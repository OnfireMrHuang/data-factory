
## 说明

给应用提供远程缓存使用

## 安装部署


1、配置存储卷

```shell


```


2、安装redis

```shell

helm install redis bitnami/redis \
--set global.storageClass={你的本地存储} \
--set global.redis.password={你的密码} \
--set image.registry=registry.cn-shanghai.aliyuncs.com \
--set image.repository=wanfei/redis \
--set architecture=standalone \
--set master.service.type=NodePort \
--set master.service.nodePorts.redis=30919 \
--version 17.7.3

```

3、验证
```shell
# 获取redis的密码
export REDIS_PASSWORD=$(kubectl get secret --namespace default redis -o jsonpath="{.data.redis-password}" | base64 -d)


# 运行redis-client容器
kubectl run --namespace default redis-client --restart='Never'  --env REDIS_PASSWORD=$REDIS_PASSWORD  --image docker.io/bitnami/redis:7.2.3-debian-11-r2 --command -- sleep infinity

# 进入redis-client容器
kubectl exec --tty -i redis-client \
   --namespace default -- bash

   
# 在redis-cli容器中使用redis-cli命令连接集群
REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h redis -p 6379 # Read only operations
REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h redis -p 26379 # Sentinel access

# 暴露redis的端口，使外部的服务可以访问redis集群
kubectl port-forward --namespace default svc/redis 6379:6379 & REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h 127.0.0.1 -p 6379

```

