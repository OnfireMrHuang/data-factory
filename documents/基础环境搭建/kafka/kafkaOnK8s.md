## 说明

kafka作为消息中间件，主要提供给flink订阅消费使用。


参考文档:
https://mp.weixin.qq.com/s?__biz=MzI3MDM5NjgwNg==&mid=2247486178&idx=1&sn=a04a17a7d93a169f24937e99a207d7f6&scene=19&poc_token=HDX-JmijI4KTtTPs4l3PpgInmxPg_ky0-yWoyrkZ


## 安装部署

### 1、创建命名空间

```shell
# 创建zookeeper和kafka的命名空间
kubectl create namespace zookeeper
kubectl create namespace kafka
```

### 2、创建持久化存储


```shell

# 宿主机创建存储目录
sudo mkdir -p /var/bigdata/servers/zookeeper/data
sudo mkdir -p /var/bigdata/servers/kafka/data

# 宿主机修改权限
sudo chmod -R 777 /var/bigdata/servers/zookeeper
sudo chmod -R 777 /var/bigdata/servers/kafka

# 应用资源
kubectl apply -n zookeeper -f zookeeper-local-storage.yaml
kubectl apply -n kafka -f kafka-local-storage.yaml
```

### 3、安装部署zookeeper集群

```shell

# 设置代理
export HTTP_PROXY=http://10.11.71.41:7890
export HTTPS_PROXY=http://10.11.71.41:7890
export NO_PROXY=localhost,127.0.0.1,.svc,.cluster.local

# 如果没有bitnami仓库，先添加
helm repo add bitnami https://charts.bitnami.com/bitnami

# 在有网络的机器上执行
helm pull oci://registry-1.docker.io/bitnamicharts/zookeeper --version 13.8.2
tar xvf zookeeper-13.8.2.tgz

# 3.9.3-debian-12-r15替换为3.9.3-debian-12-r8
sed -i 's/3.9.3-debian-12-r15/3.9.3-debian-12-r8/g' zookeeper/values.yaml

# 提前拉取镜像
minikube ssh

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/zookeeper:3.9.3-debian-12-r8
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/zookeeper:3.9.3-debian-12-r8  docker.io/bitnami/zookeeper:3.9.3-debian-12-r8

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/os-shell:12-debian-12-r43
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/os-shell:12-debian-12-r43  docker.io/bitnami/os-shell:12-debian-12-r43

# 创建命名空间
kubectl create namespace zookeeper

# 离线安装
helm install zookeeper ./zookeeper \
  --namespace zookeeper \
  --set replicaCount=1 \
  --set auth.enabled=false \
  --set allowAnonymousLogin=true \
  --set persistence.storageClass=zookeeper-local-storage


# 查看pod
kubectl get pod,pv,svc -n zookeeper -o wide

# 内部测试连接
export POD_NAME=$(kubectl get pods --namespace zookeeper -l "app.kubernetes.io/name=zookeeper,app.kubernetes.io/instance=zookeeper,app.kubernetes.io/component=zookeeper" -o jsonpath="{.items[0].metadata.name}")

kubectl exec -it $POD_NAME -n zookeeper -- zkCli.sh

# 外部测试连接
# 先删掉本地端口对应的进程，要不然就得换连接端口了
netstat -tnlp|grep 127.0.0.1:2181|awk '{print int($NF)}'|xargs kill -9
# 外部连接测试
 ubectl port-forward --namespace zookeeper svc/zookeeper 2181:2181 &
# 需要本机安装zk客户端
zkCli.sh 127.0.0.1:21
```

### 4、安装部署kafka集群

```shell

# 查看zoopeeper的集群状态
helm status zookeeper -n zookeeper


# 在有网络的机器上执行
helm pull oci://registry-1.docker.io/bitnamicharts/kafka --version 31.5.0
tar xvf kafka-31.5.0.tgz

# 替换镜像
sed -i 's/12-debian-12-r39/12-debian-12-r43/g' kafka/values.yaml
sed -i 's/1.1.0-debian-12-r7/1.1.0-debian-12-r9/g' kafka/values.yaml
sed -i 's/1.32.2-debian-12-r3/1.32.2-debian-12-r4/g' kafka/values.yaml

# 提前拉取镜像
minikube ssh

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/jmx-exporter:1.0.1-debian-12-r9
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/jmx-exporter:1.0.1-debian-12-r9  docker.io/bitnami/jmx-exporter:1.0.1-debian-12-r9

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/kafka:3.9.0-debian-12-r12
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/kafka:3.9.0-debian-12-r12  docker.io/bitnami/kafka:3.9.0-debian-12-r12

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/kubectl:1.32.3-debian-12-r4
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/kubectl:1.32.3-debian-12-r4  docker.io/bitnami/kubectl:1.32.3-debian-12-r4


# 创建命名空间
kubectl create namespace kafka

# 离线安装
helm install kafka ./kafka \
  --namespace kafka \
  --set replicaCount=1 \
  --set zookeeper.enabled=false \
  --set externalZookeeper.servers=zookeeper.zookeeper.svc.cluster.local \
  --set persistence.enabled=true \
  --set global.storageClass=kafka-local-storage \
  --set controller.replicaCount=0 \
  --set kraft.enabled=false \
  --set "broker.replicaCount=1" \
  --set "broker.id=1" 


# 查看pod
kubectl get pod,svc -n kafka

# 简单实用验证
# 先创建一个client
kubectl run kafka-client --restart='Never' --image docker.io/bitnami/kafka:3.9.0-debian-12-r12 --namespace kafka --command -- sleep infinity
kubectl cp --namespace kafka /home/huangww01/workspace/kafka-depoly/client.properties kafka-client:/tmp/client.properties


# 创建topic
kubectl exec --tty -i kafka-client --namespace kafka -- bash

# 创建topic
kafka-topics.sh --create --topic test --bootstrap-server kafka-broker-0.kafka-broker-headless.kafka.svc.cluster.local:9092 --command-config /tmp/client.properties --partitions 1 --replication-factor 1 

# 查看topic
kafka-topics.sh --describe --topic test --bootstrap-server kafka-broker-0.kafka-broker-headless.kafka.svc.cluster.local:9092 --command-config /tmp/client.properties 

# 查看topic列表
kafka-topics.sh --list  --bootstrap-server kafka-broker-0.kafka-broker-headless.kafka.svc.cluster.local:9092 --command-config /tmp/client.properties 

# 删除topic
kafka-topics.sh --delete --topic test --bootstrap-server kafka-broker-0.kafka-broker-headless.kafka.svc.cluster.local:9092 --command-config /tmp/client.properties 

# 消费者
kafka-console-consumer.sh \
            --consumer.config /tmp/client.properties \
            --bootstrap-server kafka.kafka.svc.cluster.local:9092 \
            --topic test \
            --from-beginning

# 生产者
kubectl exec --tty -i kafka-client --namespace kafka -- bash

kafka-console-producer.sh \
            --producer.config /tmp/client.properties \
            --bootstrap-server kafka.kafka.svc.cluster.local:9092 \
            --topic test
```

