
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


## 安装Mysql(为Hive metastore服务)

参考文档: https://mp.weixin.qq.com/s?__biz=MzI3MDM5NjgwNg==&mid=2247486479&idx=1&sn=b9ed32a6cb8b495cf0e7d980f8a1ac99&chksm=ead0f0e6dda779f05ce249ba1cdcd299992a8cc2bd43ff767117342efb43466ae87a8ae32ec1#rd


```shell
# 添加bitnami源
helm repo add bitnami https://charts.bitnami.com/bitnami

#  创建charts目录
mkdir ~/charts
cd ~/charts

# 拉取Mysql Chart
helm pull bitnami/mysql --version=10.1.1

# 解压
tar -xf mysql-10.1.1.tgz

```

修改values.yaml(因为我是minikube单节点部署，所以需要修改persistence项，将host设置为minikube,同时path分为多个目录):

```yaml
primary:
  persistence:
    enabled: true
    size: 10Gi
    storageClass: "mysql-local-storage"
    # 目录需要提前在宿主机上创建
    local:
    - name: mysql-0
      host: "minikube"
      path: "/var/bigdata/servers/mysql/data/data0"
  containerSecurityContext:
    readOnlyRootFilesystem: false # 为了方便调试，设置为false


secondary:
  replicaCount: 2
  persistence:
    enabled: true
    size: 10Gi
    storageClass: "mysql-local-storage"
    # 目录需要提前在宿主机上创建
    local:
    - name: mysql-1
      host: "minikube"
      path: "/var/bigdata/servers/mysql/data/data1"
    - name: mysql-2
      host: "minikube"
      path: "/var/bigdata/servers/mysql/data/data2"
  containerSecurityContext:
    readOnlyRootFilesystem: false # 为了方便调试，设置为false

volumePermissions:
  ## @param volumePermissions.enabled Enable init container that changes the owner and group of the persistent volume(s) mountpoint to `runAsUser:fsGroup`
  ##
  enabled: true  # 开启初始化容器来改变文件的所有者和用户组


metrics:
  ## @param metrics.enabled Start a side-car prometheus exporter
  ##
  enabled: true # 开启metrics 
  containerSecurityContext:
    readOnlyRootFilesystem: false # 为了方便调试，设置为false

# 设置密码
auth:
  rootPassword: "xxxxxx", 
  username: "your_username",
  password: "xxxxxx"
  replicationPassword: "xxxxxx"
  defaultAuthenticationPlugin: "caching_sha2_password"
```

添加mysql/templates/pv.yaml

```yaml
{{- range .Values.primary.persistence.local }}
---
apiVersion: v1
kind: PersistentVolume
metadata:
  name: {{ .name }}
  labels:
    name: {{ .name }}
spec:
  storageClassName: {{ $.Values.primary.persistence.storageClass }}
  capacity:
    storage: {{ $.Values.primary.persistence.size }}
  accessModes:
    - ReadWriteOnce
  local:
    path: {{ .path }}
  nodeAffinity:
    required:
      nodeSelectorTerms:
        - matchExpressions:
            - key: kubernetes.io/hostname
              operator: In
              values:
                - {{ .host }}
---
{{- end }}

{{- range .Values.secondary.persistence.local }}
---
apiVersion: v1
kind: PersistentVolume
metadata:
  name: {{ .name }}
  labels:
    name: {{ .name }}
spec:
  storageClassName: {{ $.Values.secondary.persistence.storageClass }}
  capacity:
    storage: {{ $.Values.secondary.persistence.size }}
  accessModes:
    - ReadWriteOnce
  local:
    path: {{ .path }}
  nodeAffinity:
    required:
      nodeSelectorTerms:
        - matchExpressions:
            - key: kubernetes.io/hostname
              operator: In
              values:
                - {{ .host }}
---
{{- end }}
```

添加mysql/templates/storage-class.yaml

```yaml
kind: StorageClass
apiVersion: storage.k8s.io/v1
metadata:
  name: {{ .Values.primary.persistence.storageClass }}
provisioner: kubernetes.io/no-provisioner
```


开始安装


```shell


# 默认挂载文件格式是9p可能有问题，先直接minikube上执行
# 将主机的目录挂载到minikue节点上(minikube cluster的资源都挂载到这个目录下, minikube是在主机上创建的虚拟机,所以需要挂载)
# nohup minikube mount /opt/bigdata:/opt/bigdata > /tmp/minikube_mount.log 2>&1 &


# 创建持久化目录
sudo mkdir -p /var/bigdata/servers/mysql/data/data0
sudo mkdir -p /var/bigdata/servers/mysql/data/data1
sudo mkdir -p /var/bigdata/servers/mysql/data/data2
sudo chown -R 1001:1001 /var/bigdata/servers/mysql
sudo chmod 777 /var/bigdata/servers/mysql

# 提前准备好镜像
docker pull docker.io/bitnami/mysql:8.0.36-debian-12-r10
docker pull docker.io/bitnami/mysqld-exporter:0.14.0-debian-11-r33

# 加载镜像到minikube节点上
minikube image load docker.io/bitnami/mysql:8.0.36-debian-12-r10
minikube image load docker.io/bitnami/mysqld-exporter:0.14.0-debian-11-r33

# 开始安装
helm install mysql ./mysql -n mysql --create-namespace

```

安装完成后，输出如下:

```shell
AME: mysql
LAST DEPLOYED: Tue Apr 29 14:16:03 2025
NAMESPACE: mysql
STATUS: deployed
REVISION: 1
TEST SUITE: None
NOTES:
CHART NAME: mysql
CHART VERSION: 10.1.1
APP VERSION: 8.0.36

** Please be patient while the chart is being deployed **

Tip:

  Watch the deployment status using the command: kubectl get pods -w --namespace mysql

Services:

  echo Primary: mysql.mysql.svc.cluster.local:3306

Execute the following to get the administrator credentials:

  echo Username: root
  MYSQL_ROOT_PASSWORD=$(kubectl get secret --namespace mysql mysql -o jsonpath="{.data.mysql-root-password}" | base64 -d)

To connect to your database:

  1. Run a pod that you can use as a client:

      kubectl run mysql-client --rm --tty -i --restart='Never' --image  docker.io/bitnami/mysql:8.0.36-debian-12-r10 --namespace mysql --env MYSQL_ROOT_PASSWORD=$MYSQL_ROOT_PASSWORD --command -- bash

  2. To connect to primary service (read/write):

      mysql -h mysql.mysql.svc.cluster.local -uroot -p"$MYSQL_ROOT_PASSWORD"



To access the MySQL Prometheus metrics from outside the cluster execute the following commands:

    kubectl port-forward --namespace mysql svc/mysql-metrics 9104:9104 &
    curl http://127.0.0.1:9104/metrics




WARNING: There are "resources" sections in the chart not set. Using "resourcesPreset" is not recommended for production. For production installations, please set the following values according to your workload needs:
  - metrics.resources
  - primary.resources
  - secondary.resources
  - volumePermissions.resources
+info https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
```


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




