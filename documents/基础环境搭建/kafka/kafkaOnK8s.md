## 说明

kafka作为消息中间件，主要提供给flink订阅消费使用。


参考文档:
https://mp.weixin.qq.com/s?__biz=MzI3MDM5NjgwNg==&mid=2247486178&idx=1&sn=a04a17a7d93a169f24937e99a207d7f6&scene=19&poc_token=HDX-JmijI4KTtTPs4l3PpgInmxPg_ky0-yWoyrkZ


## 安装部署

1、创建命名空间

```shell

kubectl create namespace bigdata

```

2、创建持久化存储SC（bigdata-nfs-storage）bigdata-sc.yaml

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: nfs-client-provisioner
  # replace with namespace where provisioner is deployed
  namespace: bigdata        #根据实际环境设定namespace,下面类同
---
kind: ClusterRole
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  name: nfs-client-provisioner-runner
  namespace: bigdata
rules:
  - apiGroups: [""]
    resources: ["persistentvolumes"]
    verbs: ["get", "list", "watch", "create", "delete"]
  - apiGroups: [""]
    resources: ["persistentvolumeclaims"]
    verbs: ["get", "list", "watch", "update"]
  - apiGroups: ["storage.k8s.io"]
    resources: ["storageclasses"]
    verbs: ["get", "list", "watch"]
  - apiGroups: [""]
    resources: ["events"]
    verbs: ["create", "update", "patch"]
---
kind: ClusterRoleBinding
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  name: run-nfs-client-provisioner
subjects:
  - kind: ServiceAccount
    name: nfs-client-provisioner
    namespace: bigdata
    # replace with namespace where provisioner is deployed
roleRef:
  kind: ClusterRole
  name: nfs-client-provisioner-runner
  apiGroup: rbac.authorization.k8s.io
---
kind: Role
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  name: leader-locking-nfs-client-provisioner
  namespace: bigdata
    # replace with namespace where provisioner is deployed
rules:
  - apiGroups: [""]
    resources: ["endpoints"]
    verbs: ["get", "list", "watch", "create", "update", "patch"]
---
kind: RoleBinding
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  name: leader-locking-nfs-client-provisioner
  namespace: bigdata
subjects:
  - kind: ServiceAccount
    name: nfs-client-provisioner
    # replace with namespace where provisioner is deployed
    namespace: bigdata
roleRef:
  kind: Role
  name: leader-locking-nfs-client-provisioner
  apiGroup: rbac.authorization.k8s.io
---
kind: Deployment
apiVersion: apps/v1
metadata:
  name: nfs-client-provisioner
  namespace: bigdata
spec:
  replicas: 1
  strategy:
    type: Recreate
  selector:
    matchLabels:
      app: nfs-client-provisioner
  template:
    metadata:
      labels:
        app: nfs-client-provisioner
    spec:
      serviceAccountName: nfs-client-provisioner
      containers:
        - name: nfs-client-provisioner
          image: quay.io/external_storage/nfs-client-provisioner:latest
          volumeMounts:
            - name: nfs-client-root
              mountPath: /persistentvolumes #容器内挂载点
          env:
            - name: PROVISIONER_NAME
              value: fuseim.pri/ifs
            - name: NFS_SERVER
              value: 192.168.0.113
            - name: NFS_PATH
              value: /opt/nfsdata
      volumes:
        - name: nfs-client-root #宿主机挂载点
          nfs:
            server: 192.168.0.113
            path: /opt/nfsdata
---
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: bigdata-nfs-storage
  namespace: bigdata
provisioner: fuseim.pri/ifs  # or choose another name, must match deployment's env PROVISIONER_NAME'
reclaimPolicy: Retain        #回收策略：Retain（保留）、 Recycle（回收）或者Delete（删除）
volumeBindingMode: Immediate    #volumeBindingMode存储卷绑定策略
allowVolumeExpansion: true    #pvc是否允许扩容
```

执行
```shell
kubectl apply -f bigdata-sc.yaml
kubectl get sc -n bigdata
kubectl describe sc bigdata-nfs-storage -n bigdata
```

3、安装部署zookeeper集群

```shell

# 如果没有bitnami仓库，先添加
helm repo add bitnami https://charts.bitnami.com/bitnami


# 安装zookeeper集群，注意：zookeeper集群的节点数必须是奇数，否则会出现脑裂问题
helm install zookeeper bitnami/zookeeper \
--namespace bigdata \
--set replicaCount=3 --set auth.enabled=false \
--set allowAnonymousLogin=true \
--set persistence.storageClass=bigdata-nfs-storage \
--set persistence.size=1Gi

# 查看pod
kubectl get pod,pv,svc -n bigdata -o wide

# 内部测试连接
export POD_NAME=$(kubectl get pods --namespace bigdata -l "app.kubernetes.io/name=zookeeper,app.kubernetes.io/instance=zookeeper,app.kubernetes.io/component=zookeeper" -o jsonpath="{.items[0].metadata.name}")

kubectl exec -it $POD_NAME -n bigdata -- zkCli.sh


# 外部测试连接

# 先删掉本地端口对应的进程，要不然就得换连接端口了
netstat -tnlp|grep 127.0.0.1:2181|awk '{print int($NF)}'|xargs kill -9
# 外部连接测试
 ubectl port-forward --namespace bigdata svc/zookeeper 2181:2181 &
# 需要本机安装zk客户端
zkCli.sh 127.0.0.1:21
```

4、安装部署kafka集群

```shell

# 查看zoopeeper的集群状态
helm status zookeeper -n bigdata

# 安装kafka
helm install kafka bitnami/kafka \
--namespace bigdata \
--set zookeeper.enabled=false \
--set replicaCount=3 \
--set externalZookeeper.servers=zookeeper.bigdata.svc.cluster.local \
--set persistence.storageClass=bigdata-nfs-storage

# 查看pod
kubectl get pod,svc -n bigdata

# 简单实用验证
# 先创建一个client
kubectl run kafka-client --restart='Always' --image docker.io/bitnami/kafka:2.8.1-debian-10-r57 --namespace bigdata --command -- sleep infinity

# 打开两个窗口（一个作为生产者：producer，一个作为消费者：consumer），但是两个窗口都得先登录客户端,在producer端输入，consumer会实时打印
# 生产者
kubectl exec --tty -i kafka-client --namespace bigdata -- bash
kafka-console-producer.sh \
--broker-list kafka-0.kafka-headless.bigdata.svc.cluster.local:9092,kafka-1.kafka-headless.bigdata.svc.cluster.local:9092,kafka-2.kafka-headless.bigdata.svc.cluster.local:9092 \
--topic test

# 消费者
kubectl exec --tty -i kafka-client --namespace bigdata -- bash
kafka-console-consumer.sh \
--bootstrap-server kafka.bigdata.svc.cluster.local:9092 \
--topic test \
--from-beginning

# 创建topic
kafka-topics.sh --create --topic mytest --zookeeper zookeeper.bigdata.svc.cluster.local:2181 --partitions 1 --replication-factor 1

# 查看topic
kafka-topics.sh --describe --zookeeper zookeeper.bigdata.svc.cluster.local:2181  --topic mytest

# 先查看topic列表
kafka-topics.sh --list --zookeeper zookeeper.bigdata.svc.cluster.local:2181

# 删除topic
kafka-topics.sh --delete --topic mytest --zookeeper zookeeper.bigdata.svc.cluster.local:2181

# 再查看,发现topic还在(其实上面没删除，只是标记了（只会删除zookeeper中的元数据，消息文件须手动删除）)
kafka-topics.sh --list --zookeeper zookeeper.bigdata.svc.cluster.local:2181

# 修改Topic信息
# 先创建一个topic
kafka-topics.sh --create --topic test001 --zookeeper zookeeper.bigdata.svc.cluster.local:2181 --partitions 1 --replication-factor 1

# 修改，设置数据过期时间（-1表示不过期）
kafka-topics.sh --zookeeper zookeeper.bigdata.svc.cluster.local:2181 -topic test001 --alter --config retention.ms=259200000

# 修改多字段
kafka-topics.sh --zookeeper zookeeper.bigdata.svc.cluster.local:2181 -topic test001 --alter --config max.message.bytes=128000 retention.ms=259200000
kafka-topics.sh --describe --zookeeper zookeeper.bigdata.svc.cluster.local:2181  --topic test001

# 增加topic分区数
kafka-topics.sh --zookeeper zookeeper.bigdata.svc.cluster.local:2181 --alter --topic test --partitions 10
kafka-topics.sh --describe --zookeeper zookeeper.bigdata.svc.cluster.local:2181  --topic test

# 列出所有主题中的所有用户组
kafka-consumer-groups.sh --bootstrap-server kafka-0.kafka-headless.bigdata.svc.cluster.local:9092 --list

# 查询消费者组详情（数据积压情况）
# 生产者
kafka-console-producer.sh \
--broker-list kafka-0.kafka-headless.bigdata.svc.cluster.local:9092,kafka-1.kafka-headless.bigdata.svc.cluster.local:9092,kafka-2.kafka-headless.bigdata.svc.cluster.local:9092 \
--topic test
# 消费者带group.id
kafka-console-consumer.sh --bootstrap-server kafka-0.kafka-headless.bigdata.svc.cluster.local:9092 --topic test --consumer-property group.id=mygroup
# 查看消费组情况
kafka-consumer-groups.sh --bootstrap-server kafka-0.kafka-headless.bigdata.svc.cluster.local:9092 --describe --group mygroup

```

