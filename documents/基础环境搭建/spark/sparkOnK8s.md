## 需求

在Spark之前已经创建了一个hadoop集群，其中结构化表使用hive管理，非结构化数据使用hdfs管理。当然，spark可以直接访问创建和访问结构化与非结构化数据，考虑到后续可能需要兼容trino计算引擎，所以选择hive作为结构化表管理。

### spark On hive


Spark on Hive 是Hive只作为存储角色，Spark负责sql解析优化，执行。这里可以理解为Spark 通过Spark SQL 使用Hive 语句操作Hive表 ,底层运行的还是 Spark RDD。具体步骤如下：

- 通过SparkSQL，加载Hive的配置文件，获取到Hive的元数据信息；
- 获取到Hive的元数据信息之后可以拿到Hive表的数据；
- 通过SparkSQL来操作Hive表中的数据


其中hive-site.xml存储在configMap中，metastore存储在mysql中，因此spark需要能访问configMap和mysql。


### spark部署选型

在下面列出一个表格来对比spark on kubernetes和spark-operator的区别：
| 对比项 | Spark on Kubernetes | Spark Operator |
| --- | --- | --- |
| 管理抽象层级	 | 更高层(CRD抽象)	 | 更底层(直接使用Spark提交机制) |
| 任务提交方式 | 通过K8s自定义资源(SparkApplication) | 直接使用spark-submit --k8s |
| 任务生命周期	 | 完整生命周期管理(包括重试、状态监控等) | 基础生命周期管理 |
| 与Hive集成 | 需要额外配置Hive Metastore连接 | 同样需要配置Hive Metastore连接 |
| 资源管理 | 支持细粒度资源控制 | 基础资源控制 |
| 监控集成 | 内置Prometheus指标暴露 | 需要手动配置监控 |


通过对比，spark-operator在监控和资源控制上更好一些，因此采用spark-operator来部署spark集群。


| 版本 | API Version | Kubernetes Version | Base Spark Version |
| --- | --- | --- | --- |
| v2.1.x | v1beta2 | 1.16+ | 3.5.3 |
| v2.0.x | v1beta2 | 1.16+ | 3.5.2 |
| v1beta2-1.6.x-3.5.0 | v1beta2 | 1.16+ | 3.5.0 |
| v1beta2-1.5.x-3.5.0 | v1beta2 | 1.16+ | 3.5.0 |
| v1beta2-1.4.x-3.5.0 | v1beta2 | 1.16+ | 3.5.0 |
| v1beta2-1.3.x-3.1.1 | v1beta2 | 1.16+ | 3.1.1 |
| v1beta2-1.2.3-3.1.1 | v1beta2 | 1.13+ | 3.1.1 |
| v1beta2-1.2.2-3.0.0 | v1beta2 | 1.13+ | 3.0.0 |
| v1beta2-1.2.1-3.0.0 | v1beta2 | 1.13+ | 3.0.0 |
| v1beta2-1.2.0-3.0.0 | v1beta2 | 1.13+ | 3.0.0 |
| v1beta2-1.1.x-2.4.5 | v1beta2 | 1.13+ | 2.4.5 |
| v1beta2-1.0.x-2.4.4 | v1beta2 | 1.13+ | 2.4.4 |


## 安装Spark-operator

- 运行环境: 
- k8s 版本: v1.23.8
- spark 版本: v3.5.3
- spark-operator 版本: https://github.com/kubeflow/spark-operator/releases, v2.1.0


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

# 准备镜像
docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/kubeflow/spark-operator:2.1.0
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/kubeflow/spark-operator:2.1.0  docker.io/kubeflow/spark-operator:2.1.0

# 安装
helm install spark-operator spark-operator/spark-operator --namespace spark-operator --set sparkJobNamespace=default --set image.tag=2.1.0

# 卸载
kubectl delete namespace spark-operator --force --grace-period=0
helm uninstall spark-operator --namespace spark-operator

```

## 运行示例Job

```shell

# 准备镜像
docker pull swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/spark:3.5.3
docker tag  swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/spark:3.5.3  docker.io/spark:3.5.3


# 运行例子
kubectl apply -f https://raw.githubusercontent.com/kubeflow/spark-operator/refs/heads/release-2.1/examples/spark-pi.yaml

# 查询状态
kubectl get sparkapp spark-pi


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
        runAsUser: 10000
        runAsNonRoot: true
        fsGroup: 0
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
        - apiGroups: ["*"]    
        resources: ["*"]    
        verbs: ["*"]
    ```



## 参考资料
1. spark-operator文档: https://www.kubeflow.org/docs/components/spark-operator/getting-started/
2. `ontainer has runAsNonRoot..`问题参考文档:
    https://mirror.xyz/manjusaka.eth/7KdqhGqpmM36b_xhz9F75mxCYsmmI8yB3S52oH8Cx4Q
    https://elastisys.io/welkin/user-guide/safeguards/enforce-no-root/
3. spark-operator应用例: https://help.aliyun.com/zh/ack/ack-managed-and-ack-dedicated/use-cases/use-spark-operator-to-run-spark-jobs-on-ack



