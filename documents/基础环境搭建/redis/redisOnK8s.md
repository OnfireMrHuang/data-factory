
## 说明

给应用提供远程缓存使用

## 安装部署


### 1、配置存储卷

```shell

# 主机创建存储卷
sudo mkdir -p /var/bigdata/servers/redis/data

# 修改权限
sudo chmod 777 -R /var/bigdata/servers/redis

kubectl create namespace  redis

kubectl apply -n redis -f redis-local-storage.yaml

```


### 2、安装redis

```shell

# 设置代理
export HTTP_PROXY=http://10.11.71.41:7890
export HTTPS_PROXY=http://10.11.71.41:7890
export NO_PROXY=localhost,127.0.0.1,.svc,.cluster.local

# 如果没有bitnami仓库，先添加
helm repo add bitnami https://charts.bitnami.com/bitnami

# 在有网络的机器上执行
helm pull oci://registry-1.docker.io/bitnamicharts/redis --version 20.6.1
tar xvf redis-20.6.1.tgz

# redis镜像替换
sed -i 's/7.4.1-debian-12-r3/7.4.1-debian-12-r2/g' redis/values.yaml
sed -i 's/12-debian-12-r34/12-debian-12-r43/g' redis/values.yaml
sed -i 's/1.32.0-debian-12-r0/1.32.0-debian-12-r4/g' redis/values.yaml

# 预准备镜像
minikube ssh
docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/redis:7.4.1-debian-12-r2
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/redis:7.4.1-debian-12-r2  docker.io/bitnami/redis:7.4.1-debian-12-r2

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/redis-sentinel:7.4.1-debian-12-r2
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/redis-sentinel:7.4.1-debian-12-r2  docker.io/bitnami/redis-sentinel:7.4.1-debian-12-r2

docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/redis-exporter:1.67.0-debian-12-r0
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/bitnami/redis-exporter:1.67.0-debian-12-r0  docker.io/bitnami/redis-exporter:1.67.0-debian-12-r0

unset http_proxy
unset https_proxy


helm install redis ./redis \
--namespace redis \
--set global.storageClass=redis-local-storage \
--set global.redis.password=redis@123 \
--set architecture=standalone 

```

### 3、验证
```shell
# 获取redis的密码
export REDIS_PASSWORD=$(kubectl get secret --namespace redis redis -o jsonpath="{.data.redis-password}" | base64 -d)


# 运行redis-client容器
kubectl run --namespace redis redis-client --restart='Never'  --env REDIS_PASSWORD=$REDIS_PASSWORD  --image docker.io/bitnami/redis:7.4.1-debian-12-r2 --command -- sleep infinity

# 进入redis-client容器
kubectl exec --tty -i redis-client \
   --namespace redis -- bash

   
# 在redis-cli容器中使用redis-cli命令连接集群
REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h redis-master

```

