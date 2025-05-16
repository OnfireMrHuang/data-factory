## 说明

Apache Doris 是一款基于 MPP 架构的高性能、实时分析型数据库。它以高效、简单和统一的特性著称，能够在亚秒级的时间内返回海量数据的查询结果。Doris 既能支持高并发的点查询场景，也能支持高吞吐的复杂分析场景。

基于这些优势，Apache Doris 非常适合用于报表分析、即席查询、统一数仓构建、数据湖联邦查询加速等场景。用户可以基于 Doris 构建大屏看板、用户行为分析、AB 实验平台、日志检索分析、用户画像分析、订单分析等应用。


基于此，主要利用doris来作为查询引擎以及实时数据存储。



## Doris Operator


Doris Operator 基于 Kubernetes CustomResourceDefinitions（CRD）实现了 Doris 在 Kubernetes 平台的配置、管理和调度。Doris Operator 能够根据用户自定义的期望状态，自动创建 Pods 及其他资源以启动服务。通过自动注册机制，可将所有启动的服务整合成一个完整的 Doris 集群。这一实现显著降低了在 Doris 集群中处理配置信息、节点发现与注册、访问通信及健康检查等生产环境必备操作的复杂性和学习成本。


## 安装部署Doris Operator


step1: 安装Doris Operator CRD

```shell
kubectl create -f https://raw.githubusercontent.com/apache/doris-operator/master/config/crd/bases/doris.apache.com_dorisclusters.yaml
```

step2: 部署Doris Operator

```shell

# 提前准备镜像
docker pull apache/doris:operator-latest
minikube image load apache/doris:operator-latest

kubectl apply -f https://raw.githubusercontent.com/apache/doris-operator/master/config/operator/operator.yaml


# 修改Deployment配置，确保设置为IfNotPresent,保证优先使用本地镜像
imagePullPolicy: IfNotPresent
```

期望输出结果:

```shell
namespace/doris created
role.rbac.authorization.k8s.io/leader-election-role created
rolebinding.rbac.authorization.k8s.io/leader-election-rolebinding created
clusterrole.rbac.authorization.k8s.io/doris-operator created
clusterrolebinding.rbac.authorization.k8s.io/doris-operator-rolebinding created
serviceaccount/doris-operator created
deployment.apps/doris-operator created
```

step3: 检查Doris Operator状态

```shell
kubectl get pods -n doris
```

期望输出结果:

```shell
NAME                              READY   STATUS    RESTARTS   AGE
doris-operator-67985c6744-72q6v   1/1     Running   0          10m
```


## 部署Doris集群

1、镜像准备

```shell
docker pull apache/doris:be-2.1.8
minikube image load apache/doris:be-2.1.8

docker pull apache/doris:fe-2.1.8
minikube image load apache/doris:fe-2.1.8
```

2、应用部署

配置调整参考: https://doris.apache.org/zh-CN/docs/install/deploy-on-kubernetes/install-config-cluster

**资源在cluster目录下， 注意在部署时替换掉文件中的敏感值!!!**

```shell

# 创建fe配置
kubectl -n doris apply -f fe-conf.yaml

# 创建be配置
kubectl -n doris apply -f be-conf.yaml

# 正式部署
kubectl -n doris apply -f doriscluster.yaml

# 查看pods的状态
kubectl -n doris get pods -o wide

# 查看部署资源的状态
kubectl -n doris get dcr

```

## 访问&测试Doris集群


```shell

# 获取service
kubectl get service -n doris

# NodePort方式, 9030端口会被映射到主机，外部可以直接访问节点的映射端口
kubectl get nodes -n doris -owide

# 访问
mysql -h nodeIP -P nodePort -uroot

```





