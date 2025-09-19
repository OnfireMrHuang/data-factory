## 说明

Milvus是一个开源向量数据库，旨在通过向量表示来存储、索引和搜索海量非结构化数据，因此非常适合人工智能驱动的应用，如相似性搜索、语义搜索、检索增强生成（RAG）、推荐引擎和其他机器学习任务
参考文档: https://milvus.io/zh/blog/deploy-milvus-on-kubernetes-step-by-step-guide-for-k8s-users.md



## 安装Milvus

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

### 2、安装 Milvus-operator

```shell
kubectl apply -f https://raw.githubusercontent.com/zilliztech/milvus-operator/main/deploy/manifests/deployment.yaml
```


### 3、部署Milvus集群

