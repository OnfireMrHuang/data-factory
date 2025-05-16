
## 运行环境和版本

- 运行环境: 
- k8s 版本: v1.23.8
- mysql版本: 9.3.3
- hadoop 版本: 3.3.5
- minikube主机的文件系统:
    | Filesystem      | Size | Used | Avail | Use% | Mounted on     |  
  |-----------------|------|------|-------|------|----------------|  
  | overlay         | 457G | 48G  | 386G  | 12%  | /              |  
  | tmpfs           | 64M  | 0    | 64M   | 0%   | /dev           |  
  | tmpfs           | 7.8G | 0    | 7.8G  | 0%   | /sys/fs/cgroup |  
  | shm             | 64M  | 0    | 64M   | 0%   | /dev/shm       |  
  | /dev/sda3       | 457G | 48G  | 386G  | 12%  | /var           |  
  | tmpfs           | 7.8G | 42M  | 7.7G  | 1%   | /run           |  
  | tmpfs           | 7.8G | 8.0K | 7.8G  | 1%   | /tmp           |  
  | tmpfs           | 5.0M | 0    | 5.0M  | 0%   | /run/lock      |  


## 安装Hadoop


参考文档:  

1. https://www.cnblogs.com/liugp/p/17279520.html
2. https://www.cnblogs.com/liugp/p/17539121.html


### 构建镜像

所需要的Dockerfile和Chart文件都在同级目录下.

因为环境的差异，我的Doker和Chart有部分调整，所以需要调整

```shell

### 安装JDK

# jdk包在我下面提供的资源包里，当然你也可以去官网下载。
wget https://github.com/frekele/oracle-java/releases/download/8u212-b10/jdk-8u212-linux-x64.tar.gz
tar -xf jdk-8u212-linux-x64.tar.gz

# /etc/profile文件中追加如下内容：
echo "export JAVA_HOME=`pwd`/jdk1.8.0_212" >> /etc/profile
echo "export PATH=\$JAVA_HOME/bin:\$PATH" >> /etc/profile
echo "export CLASSPATH=.:\$JAVA_HOME/lib/dt.jar:\$JAVA_HOME/lib/tools.jar" >> /etc/profile

# 加载生效
source /etc/profile

# 下载mysql-8的Jdbc驱动
wget https://dev.mysql.com/get/Downloads/Connector-J/mysql-connector-java-8.0.29.zip


# 下载haoop相关的软件

### 1、Hadoop
# 下载地址：https://dlcdn.apache.org/hadoop/common/
wget https://dlcdn.apache.org/hadoop/common/hadoop-3.3.5/hadoop-3.3.5.tar.gz --no-check-certificate

### 2、hive
# 下载地址：http://archive.apache.org/dist/hive
wget http://archive.apache.org/dist/hive/hive-3.1.3/apache-hive-3.1.3-bin.tar.gz

### 2、spark
# Spark下载地址：http://spark.apache.org/downloads.html
wget https://dlcdn.apache.org/spark/spark-3.4.4/spark-3.4.4-bin-hadoop3.tgz --no-check-certificate

### 3、flink
wget https://dlcdn.apache.org/flink/flink-1.17.2/flink-1.17.2-bin-scala_2.12.tgz --no-check-certificate


# 修改config中的内容
# hadoop配置值修改参考: https://hadoop.apache.org/docs/current/hadoop-yarn/hadoop-yarn-common/yarn-default.xml
vim config/hive-config/hive-site.xml
# 将其中的用户名和密码替换掉

# 构建镜像
docker build -t hadoop:v1 . --no-cache
docker tag hadoop:v1 registry.cn-hangzhou.aliyuncs.com/bigdata_cloudnative/hadoop:v1

```


###  节点机创建挂载目录(路径有所不同)

```shell
# 如果使用pv，pvc挂载方式，就不需要在宿主机上创建目录了，非高可用可不用创建jn
sudo mkdir -p /var/bigdata/servers/hadoop/{nn,jn,dn}/data/data{1..3}
sudo chmod 777 -R /var/bigdata/servers/hadoop/
```

### 正式安装

```shell

# 将chart文件通过scp传输到开发机(并修改存储卷、密码等信息)
cd chart
# !!!主要修改chart/values.yaml、chart/templates/hive/hive-configmap.yaml

cd hadoop-on-kubernetes

# 安装
helm install hadoop ./ -n hadoop --create-namespace

# 更新
# helm upgrade hadoop ./ -n hadoop

# 卸载
# helm uninstall hadoop -n hadoop

```

输出如下:

```shell
NAME: hadoop
LAST DEPLOYED: Tue Apr 29 16:02:48 2025
NAMESPACE: hadoop
STATUS: deployed
REVISION: 1
TEST SUITE: None
NOTES:
1. You can check the status of HDFS by running this command:
   kubectl exec -n hadoop -it hadoop-hadoop-hdfs-nn-0 -- /opt/hadoop/bin/hdfs dfsadmin -report

2. You can list the yarn nodes by running this command:
   kubectl exec -n hadoop -it hadoop-hadoop-yarn-rm-0 -- /opt/hadoop/bin/yarn node -list

3. Create a port-forward to the yarn resource manager UI:
   kubectl port-forward -n hadoop hadoop-hadoop-yarn-rm-0 8088:8088

   Then open the ui in your browser:

   open http://localhost:8088

4. You can run included hadoop tests like this:
   kubectl exec -n hadoop -it hadoop-hadoop-yarn-nm-0 -- /opt/hadoop/bin/hadoop jar /opt/hadoop/share/hadoop/mapreduce/hadoop-mapreduce-client-jobclient-3.3.5-tests.jar TestDFSIO -write -nrFiles 5 -fileSize 128MB -resFile /tmp/TestDFSIOwrite.txt

5. You can list the mapreduce jobs like this:
   kubectl exec -n hadoop -it hadoop-hadoop-yarn-rm-0 -- /opt/hadoop/bin/mapred job -list

6. This chart can also be used with the zeppelin chart
    helm install --namespace hadoop --set hadoop.useConfigMap=true,hadoop.configMapName=hadoop-hadoop stable/zeppelin

7. You can scale the number of yarn nodes like this:
   helm upgrade hadoop --set yarn.nodeManager.replicas=4 stable/hadoop

   Make sure to update the values.yaml if you want to make this permanent.
```


### 关键配置编辑

core-site.xml
```xml
<?xml version="1.0"?>
<?xml-stylesheet type="text/xsl" href="configuration.xsl"?>
<configuration>
  <property>
      <name>fs.defaultFS</name>
        <value>hdfs://hadoop-hadoop-hdfs-nn-0.hadoop-hadoop-hdfs-nn.hadoop.svc.cluster.local:9000</value>
      <description>NameNode URI</description>
  </property>
  <property>
      <name>hadoop.proxyuser.root.hosts</name>
      <value>*</value>
  </property>
  <property>
        <name>hadoop.http.staticuser.user</name>
        <value>hadoop</value>
  </property>
  <property>
    <name>dfs.permissions.enabled</name>
    <value>false</value>
  </property>
  <property>
      <name>hadoop.proxyuser.root.groups</name>
      <value>*</value>
  </property>
  <property>
      <name>hadoop.proxyuser.hadoop.hosts</name>
      <value>*</value>
  </property>
  <property>
      <name>hadoop.proxyuser.hadoop.groups</name>
      <value>*</value>
  </property>
</configuration>

```

hdfs-site.xml

```xml
<?xml version="1.0"?>
<?xml-stylesheet type="text/xsl" href="configuration.xsl"?>
<configuration><property>
      <name>dfs.webhdfs.enabled</name>
      <value>true</value>
  </property>
  <property>
    <name>dfs.datanode.use.datanode.hostname</name>
    <value>true</value>
  </property>

  <property>
    <name>dfs.client.use.datanode.hostname</name>
    <value>true</value>
  </property>

  # <property>
  #   <name>dfs.datanode.hostname</name>
  #   <value>hadoop-hadoop-hdfs-dn-0.hadoop-hadoop-hdfs-dn.hadoop.svc.cluster.local</value>
  # </property>

  <property>
    <name>dfs.namenode.datanode.registration.ip-hostname-check</name>
    <value>false</value>
  </property>

  <property>
    <name>dfs.datanode.http.address</name>
    <value>0.0.0.0:9864</value>
  </property>

  <property>
    <name>dfs.datanode.address</name>
    <value>0.0.0.0:9866</value>
  </property>

  <property>
    <name>dfs.replication</name>
      <value>1</value>
  </property>

  <property>
    <name>dfs.datanode.data.dir</name>
    <value>/opt/apache/hadoop/data/hdfs/datanode/data1</value>
    <description>DataNode directory</description>
  </property>

  <property>
    <name>dfs.namenode.name.dir</name>
    <value>/opt/apache/hadoop/data/hdfs/namenode</value>
    <description>NameNode directory for namespace and transaction logs storage.</description>
  </property>

  <property>
    <name>dfs.namenode.datanode.registration.ip-hostname-check</name>
    <value>false</value>
  </property>

  <!-- Bind to all interfaces -->
  <property>
    <name>dfs.namenode.rpc-bind-host</name>
    <value>0.0.0.0</value>
  </property>
  <property>
    <name>dfs.namenode.servicerpc-bind-host</name>
    <value>0.0.0.0</value>
  </property>
  <!-- /Bind to all interfaces -->

</configuration>
```


### 测试

```shell

kubectl exec -it hadoop-hadoop-hive-hiveserver2-0 -n hadoop -- bash

beeline -u jdbc:hive2://localhost:10000  -n hadoop

# 建表
CREATE TABLE mytable (
  id INT,
  name STRING,
  age INT,
  address STRING
)
ROW FORMAT DELIMITED
FIELDS TERMINATED BY ','
LINES TERMINATED BY '\n';

# 添加数据
INSERT INTO mytable VALUES (1, 'Alice', 25, 'F'), (2, 'Bob', 30, 'M'), (3, 'Charlie', 35, 'M');
```


## 问题与解答


1. 执行bootstrap.sh {选项}时tail -f常报错
  直接修改yaml文件，修改容器启动命令


2. hadoop配置问题
  修改hadoop-hadoop config-map, 配置绑定端口等配置


3. 资源不足导致pod被term杀掉
  调高CPU和内存资源


4. RemoteException: File /tmp/test/readme.md could only be written to 0 of the 1 minReplication nodes. There are 1 datanode(s) running and 0 node(s) are excluded in this operation.

   原因：hdfs-site.xml文件中没有配置dfs.replication属性（因为是minikube部署的单节点环境，HDFS副本数不能超过当前运行的DataNode节点数，否则写入会失败）
   解决：
    1、修改hdfs-dn的statefulSet, 将副本数降为1，将存储卷降为1
    2、修改hdfs-site.xml文件，修改dfs.replication属性，值为1; 修改dfs.datanode.data.dir属性，仅使用data1.