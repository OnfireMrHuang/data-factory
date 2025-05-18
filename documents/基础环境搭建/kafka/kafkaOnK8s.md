## 说明

kafka作为消息中间件，主要提供给flink订阅消费使用。


参考文档:
https://mp.weixin.qq.com/s?__biz=MzI3MDM5NjgwNg==&mid=2247486178&idx=1&sn=a04a17a7d93a169f24937e99a207d7f6&scene=19&poc_token=HDX-JmijI4KTtTPs4l3PpgInmxPg_ky0-yWoyrkZ


## 安装部署

1、创建命名空间

```shell
# 创建zookeeper和kafka的命名空间
kubectl create namespace zookeeper
kubectl create namespace kafka
```

2、创建持久化存储


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

3、安装部署zookeeper集群

```shell

# 如果没有bitnami仓库，先添加
helm repo add bitnami https://charts.bitnami.com/bitnami


# 安装zookeeper集群，注意：zookeeper集群的节点数必须是奇数，否则会出现脑裂问题
helm install zookeeper bitnami/zookeeper \
--namespace zookeeper \
--set replicaCount=1 --set auth.enabled=false \
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

4、安装部署kafka集群

```shell

# 查看zoopeeper的集群状态
helm status zookeeper -n zookeeper

# 安装kafka
helm install kafka bitnami/kafka \
--namespace kafka \
--set zookeeper.enabled=false \
--set replicaCount=1 \
--set externalZookeeper.servers=zookeeper.zookeeper.svc.cluster.local \
--set persistence.storageClass=kafka-local-storage 

# 查看pod
kubectl get pod,svc -n kafka

# 简单实用验证
# 先创建一个client
kubectl run kafka-client --restart='Always' --image docker.io/bitnami/kafka:2.8.1-debian-10-r57 --namespace kafka --command -- sleep infinity

# 打开两个窗口（一个作为生产者：producer，一个作为消费者：consumer），但是两个窗口都得先登录客户端,在producer端输入，consumer会实时打印
# 生产者
kubectl exec --tty -i kafka-client --namespace kafka -- bash
kafka-console-producer.sh \
--broker-list kafka-0.kafka-headless.kafka.svc.cluster.local:9092
--topic test

# 消费者
kubectl exec --tty -i kafka-client --namespace kafka -- bash
kafka-console-consumer.sh \
--bootstrap-server kafka.kafka.svc.cluster.local:9092 \
--topic test \
--from-beginning

# 创建topic
kafka-topics.sh --create --topic mytest --zookeeper zookeeper.zookeeper.svc.cluster.local:2181 --partitions 1 --replication-factor 1

# 查看topic
kafka-topics.sh --describe --zookeeper zookeeper.zookeeper.svc.cluster.local:2181  --topic mytest

# 先查看topic列表
kafka-topics.sh --list --zookeeper zookeeper.zookeeper.svc.cluster.local:2181

# 删除topic
kafka-topics.sh --delete --topic mytest --zookeeper zookeeper.zookeeper.svc.cluster.local:2181

# 再查看,发现topic还在(其实上面没删除，只是标记了（只会删除zookeeper中的元数据，消息文件须手动删除）)
kafka-topics.sh --list --zookeeper zookeeper.zookeeper.svc.cluster.local:2181

# 修改Topic信息
# 先创建一个topic
kafka-topics.sh --create --topic test001 --zookeeper zookeeper.zookeeper.svc.cluster.local:2181 --partitions 1 --replication-factor 1

# 修改，设置数据过期时间（-1表示不过期）
kafka-topics.sh --zookeeper zookeeper.zookeeper.svc.cluster.local:2181 -topic test001 --alter --config retention.ms=259200000

# 修改多字段
kafka-topics.sh --zookeeper zookeeper.zookeeper.svc.cluster.local:2181 -topic test001 --alter --config max.message.bytes=128000 retention.ms=259200000
kafka-topics.sh --describe --zookeeper zookeeper.zookeeper.svc.cluster.local:2181  --topic test001

# 增加topic分区数
kafka-topics.sh --zookeeper zookeeper.zookeeper.svc.cluster.local:2181 --alter --topic test --partitions 10
kafka-topics.sh --describe --zookeeper zookeeper.zookeeper.svc.cluster.local:2181  --topic test

# 列出所有主题中的所有用户组
kafka-consumer-groups.sh --bootstrap-server kafka-0.kafka-headless.kafka.svc.cluster.local:9092 --list

# 查询消费者组详情（数据积压情况）
# 生产者
kafka-console-producer.sh \
--broker-list kafka-0.kafka-headless.kafka.svc.cluster.local:9092
--topic test

# 消费者带group.id
kafka-console-consumer.sh --bootstrap-server kafka-0.kafka-headless.kafka.svc.cluster.local:9092 --topic test --consumer-property group.id=mygroup

# 查看消费组情况
kafka-consumer-groups.sh --bootstrap-server kafka-0.kafka-headless.kafka.svc.cluster.local:9092 --describe --group mygroup

```

