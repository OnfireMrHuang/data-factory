


## 运行环境和版本

- 运行环境: 
- k8s 版本: v1.23.8
- spark 版本: v3.1.1
- spark-operator 版本: https://github.com/kubeflow/spark-operator/releases, v2.1.1


## 镜像准备

```shell

# 我是通过代理拉取下来的，如果没有代理，可以考虑使用阿里云的镜像

docker pull ghcr.io/googlecloudplatform/spark-operator:v1beta2-1.3.8-3.1.1

docker pull gcr.io/spark-operator/spark:v3.1.1
```

## 安装Spark-operator


Spark-Operator 是一个 Kubernetes 原生的 Spark 集群管理器，它可以让用户在 Kubernetes 集群上运行 Spark 应用程序。

- Helm: kubernetes的包管理工具，类似ubuntu的apt-get工具
- Chart: 应用描述，一系列用于描述k8s资源相关文件的集合,类似ubuntu的deb包
- Release: 应用实例，一个chart的具体部署实例，类似ubuntu的deb包的具体安装

参考: https://kubeflow.github.io/spark-operator/


```shell

# Add the Helm repository
helm repo add spark-operator https://kubeflow.github.io/spark-operator

# 更新
helm repo update

# 创建一个命名空间
kubectl create namespace spark-operator 

# 安装
helm install spark-operator spark-operator/spark-operator --namespace spark-operator --set sparkJobNamespace=default --set image.repository=ghcr.io/googlecloudplatform/spark-operator --set image.tag=v1beta2-1.3.8-3.1.1 


# 卸载
kubectl delete namespace spark-operator --force --grace-period=0
helm uninstall spark-operator --namespace spark-operator

```


## 运行示例Job

```shell

todo... spark的部署还存在一些问题，先搁置，后续再补充...

/usr/bin/spark-operator controller start --zap-log-level=info --namespaces=default --controller-threads=10 --enable-ui-service=true --enable-metrics=true --metrics-bind-address=:8080 --metrics-endpoint=/metrics --metrics-prefix= --metrics-labels=app_type --metrics-job-start-latency-buckets=30,60,90,120,150,180,210,240,270,300 --leader-election=true --leader-election-lock-name=spark-operator-controller-lock --leader-election-lock-namespace=spark-operator --workqueue-ratelimiter-bucket-qps=50 --workqueue-ratelimiter-bucket-size=500 --workqueue-ratelimiter-max-delay=6h --driver-pod-creation-grace-period=10s --max-tracked-executor-per-app=1000


/usr/bin/spark-operator webhook start --zap-log-level=info --namespaces=default --webhook-secret-name=spark-operator-webhook-certs --webhook-secret-namespace=spark-operator --webhook-svc-name=spark-operator-webhook-svc --webhook-svc-namespace=spark-operator --webhook-port=9443 --mutating-webhook-name=spark-operator-webhook --validating-webhook-name=spark-operator-webhook --enable-metrics=true --metrics-bind-address=:8080 --metrics-endpoint=/metrics --metrics-prefix= --metrics-labels=app_type --leader-election=true --leader-election-lock-name=spark-operator-webhook-lock --leader-election-lock-namespace=spark-operator

```


## 问题解决


1. 如果spark controller pod一直在拉取镜像，可以通过如下解决

    ```shell
        # 如果是minikube环境，并且本地已经准备好

        minikube image load <IMAGE_NAME>

    ```

2. 如果出现`ontainer has runAsNonRoot and image has non-numeric user (root), cannot verify user is non-root`,则参考:

    修改Deployment的spec.template.spec.securityContext.runAsUser为一个非0的数值，例如：

    ```yaml
    ......
    securityContext:    
        runAsUser: 1000

    ```

3. 如果出现： pkg/mod/k8s.io/client-go@v0.25.3/tools/cache/reflector.go:169: Failed to watch *v1.Pod: failed to list *v1.Pod: pods is forbidden: User "system:serviceaccount:spark-operator:spark-operator-controller" cannot list resource "pods" in API group "" at the cluster scope

    修改ClusterRole.yaml文件，将rules修改为:
    ```yaml
        apiVersion: rbac.authorization.k8s.io/v1beta1
        kind: ClusterRole
        metadata:
        name: sparkoperator
        ......
        rules:
        - apiGroups:
        - "*"
        resources:
        - "*"
        verbs:
        - "*"
    ```

4. 如果安装过程中提示scheduledsparkapplications.sparkoperator.k8s.io未找到, 解决如下

    ```shell

        # 1. 下载文件
        wget https://github.com/kubeflow/spark-operator/archive/refs/tags/v1beta2-1.3.8-3.1.1.zip  


        # 2. 传输文件(仅在集群所在节点不能访问第一步的情况下)
        scp spark-operator-1beta2-1.3.8-3.1.1.zip huangww01@xxxx:/home/workspace


        # 找到crds目录并安装资源
        cd ~/workspace/spark-operator-1beta2-1.3.8-3.1.1/manifest/crds/
        kubectl create -f sparkoperator.k8s.io_scheduledsparkapplications.yaml
        kubectl create -f sparkoperator.k8s.io_sparkapplications.yaml
    ```


## 参考资料
1. spark-operator文档: https://www.kubeflow.org/docs/components/spark-operator/getting-started/
2. `ontainer has runAsNonRoot..`问题参考文档:
    https://mirror.xyz/manjusaka.eth/7KdqhGqpmM36b_xhz9F75mxCYsmmI8yB3S52oH8Cx4Q
    https://elastisys.io/welkin/user-guide/safeguards/enforce-no-root/
3. spark-operator应用例: https://help.aliyun.com/zh/ack/ack-managed-and-ack-dedicated/use-cases/use-spark-operator-to-run-spark-jobs-on-ack







