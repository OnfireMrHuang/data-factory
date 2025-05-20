## 说明

实时计算使用。

参考文档: 
1、https://www.cnblogs.com/liugp/p/16755095.html
2、https://zhuanlan.zhihu.com/p/621173037


## FlinkOperator

Kubernetes模式下，Flink又细分为Native Kubernetes和Flink Kubernetes Operator两种模式，在实际应用中，比较少使用Native Kubernetes，而是使用Flink Kubernetes Operator居多。此外，Flink Kubernetes Operator也是Apache Flink官方提供和推荐的，它可以极大地简化将Flink应用部署到K8s上的配置。

Flink Kubernetes Operator介绍: https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/zh/docs/concepts/overview/


## 安装部署FinkOperator


### 1、安装cert-manager依赖

Jetstack/cert-managerr版本: 1.17.2
Jetstack/cert-manager 是 Kubernetes 生态系统中的一款开源项目，它提供了一种自动化的方式来管理 TLS 证书的生命周期

```shell

# 提前准备镜像

minikube ssh

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/quay.io/jetstack/cert-manager-webhook:v1.17.2
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/quay.io/jetstack/cert-manager-webhook:v1.17.2  quay.io/jetstack/cert-manager-webhook:v1.17.2

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/quay.io/jetstack/cert-manager-cainjector:v1.17.2
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/quay.io/jetstack/cert-manager-cainjector:v1.17.2  quay.io/jetstack/cert-manager-cainjector:v1.17.2

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/quay.io/jetstack/cert-manager-controller:v1.17.2
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/quay.io/jetstack/cert-manager-controller:v1.17.2  quay.io/jetstack/cert-manager-controller:v1.17.2

kubectl create -f https://github.com/jetstack/cert-manager/releases/download/v1.17.2/cert-manager.yaml

```


### 2、安装FlinkOperator

FlinkOperator版本: 1.11.0

```shell

# 预拉取镜像

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/apache/flink-kubernetes-operator:1.11.0
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/apache/flink-kubernetes-operator:1.11.0  docker.io/apache/flink-kubernetes-operator:1.11.0

helm repo add flink-operator-repo https://downloads.apache.org/flink/flink-kubernetes-operator-1.11.0/

# 在线安装方式
# 其默认镜像为ghcr.io/apache/flink-kubernetes-operator:cc9c6cb, 可以应用后修改为ghcr.io/apache/flink-kubernetes-operator:1.11.0
helm install flink-kubernetes-operator flink-operator-repo/flink-kubernetes-operator  --namespace flink --create-namespace 

# 离线安装方式
# 先下载
wget 'https://downloads.apache.org/flink/flink-kubernetes-operator-1.11.0/flink-kubernetes-operator-1.11.0-helm.tgz'
# 解压进入目录, 根据需要修改value.yaml文件,然后执行安装
helm install -f values.yaml flink-kubernetes-operator . --namespace flink --create-namespace


# 检查安装情况
helm list -n flink

```


## 测试运行示例Job

Flink版本: 1.19.2
官方运行例子: https://github.com/apache/flink-kubernetes-operator/tree/main/examples
以basic.yaml为例子:

```yaml
apiVersion: flink.apache.org/v1beta1
kind: FlinkDeployment
metadata:
  name: basic-example
spec:
  image: flink:1.19.2
  flinkVersion: v1_19
  flinkConfiguration:
    taskmanager.numberOfTaskSlots: "2"
  serviceAccount: flink
  jobManager:
    resource:
      memory: "1024m"
      cpu: 1
  taskManager:
    resource:
      memory: "1024m"
      cpu: 1
  job:
    jarURI: local:///opt/flink/examples/streaming/StateMachineExample.jar
    parallelism: 2
    upgradeMode: stateless
```

```shell

# 准备镜像

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/flink:1.19.2
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/flink:1.19.2  docker.io/flink:1.19.2

# 运行作业
kubectl apply -f basic.yaml -n flink

# 查看作业运行情况
kubectl get all -n flink
kubectl get FlinkDeployment -n flink

# 终止作业
kubectl delete -f basic.yaml -n flink
kubectl delete FlinkDeployment basic-example -n flink

```


## 部署模式介绍

| Feature                 | Session 模式                          | Per-Job 模式                         | Application 模式                     |
|-------------------------|---------------------------------------|--------------------------------------|--------------------------------------|
| 集群生命周期            | 独立于作业，长期运行                  | 与作业生命周期一致                   | 与应用程序生命周期一致               |
| 资源隔离                | 多作业共享资源，隔离性差              | 每个作业独享资源，隔离性好           | 每个应用独享资源，隔离性好           |
| 资源管理                | 预先分配固定资源                      | 按作业需求动态分配                   | 按应用需求动态分配                   |
| 适用场景                | 开发测试、短时间运行的小作业          | 生产环境，重要作业                   | 生产环境，完整应用部署               |
| 作业提交方式            | 通过客户端提交到现有集群              | 每个作业启动独立集群                 | 应用自包含，直接启动集群             |
| 资源利用率              | 可能资源浪费(空闲时)                  | 较高                                 | 较高                                 |
| 故障影响范围            | 一个作业失败可能影响其他作业          | 仅影响当前作业                       | 仅影响当前应用                       |
| 启动速度                | 作业启动快(集群已存在)                | 作业启动慢(需先启动集群)             | 应用启动慢(需先启动集群)             |
| 典型部署工具            | Flink standalone/YARN/K8s session     | Flink on YARN/K8s (per-job)          | Flink on YARN/K8s (application)      |
| main()方法执行位置      | 客户端机器                            | 客户端机器                           | JobManager 容器内                    |
| 作业依赖管理            | 需客户端有所有依赖                    | 需客户端有所有依赖                   | 依赖打包在应用镜像/JAR中             |


**经过对比，主要选择Application模式运行Flink job.**

提交Job的详细方式请参考: https://zhuanlan.zhihu.com/p/621177585



