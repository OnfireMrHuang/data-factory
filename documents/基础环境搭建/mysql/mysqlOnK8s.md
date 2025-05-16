## 说明

应用元数据以及hive metastore存储使用.

参考文档: https://mp.weixin.qq.com/s?__biz=MzI3MDM5NjgwNg==&mid=2247486479&idx=1&sn=b9ed32a6cb8b495cf0e7d980f8a1ac99&chksm=ead0f0e6dda779f05ce249ba1cdcd299992a8cc2bd43ff767117342efb43466ae87a8ae32ec1#rd


## 安装Mysql

```shell
# 添加bitnami源
helm repo add bitnami https://charts.bitnami.com/bitnami

#  创建charts目录
mkdir ~/charts
cd ~/charts

# 拉取Mysql Chart
helm pull bitnami/mysql --version=10.1.1

# 解压
tar -xf mysql-10.1.1.tgz

```

修改values.yaml(因为我是minikube单节点部署，所以需要修改persistence项，将host设置为minikube,同时path分为多个目录):

```yaml
primary:
  persistence:
    enabled: true
    size: 10Gi
    storageClass: "mysql-local-storage"
    # 目录需要提前在宿主机上创建
    local:
    - name: mysql-0
      host: "minikube"
      path: "/var/bigdata/servers/mysql/data/data0"
  containerSecurityContext:
    readOnlyRootFilesystem: false # 为了方便调试，设置为false


secondary:
  replicaCount: 2
  persistence:
    enabled: true
    size: 10Gi
    storageClass: "mysql-local-storage"
    # 目录需要提前在宿主机上创建
    local:
    - name: mysql-1
      host: "minikube"
      path: "/var/bigdata/servers/mysql/data/data1"
    - name: mysql-2
      host: "minikube"
      path: "/var/bigdata/servers/mysql/data/data2"
  containerSecurityContext:
    readOnlyRootFilesystem: false # 为了方便调试，设置为false

volumePermissions:
  ## @param volumePermissions.enabled Enable init container that changes the owner and group of the persistent volume(s) mountpoint to `runAsUser:fsGroup`
  ##
  enabled: true  # 开启初始化容器来改变文件的所有者和用户组


metrics:
  ## @param metrics.enabled Start a side-car prometheus exporter
  ##
  enabled: true # 开启metrics 
  containerSecurityContext:
    readOnlyRootFilesystem: false # 为了方便调试，设置为false

# 设置密码
auth:
  rootPassword: "xxxxxx", 
  username: "your_username",
  password: "xxxxxx"
  replicationPassword: "xxxxxx"
  defaultAuthenticationPlugin: "caching_sha2_password"
```

添加mysql/templates/pv.yaml

```yaml
{{- range .Values.primary.persistence.local }}
---
apiVersion: v1
kind: PersistentVolume
metadata:
  name: {{ .name }}
  labels:
    name: {{ .name }}
spec:
  storageClassName: {{ $.Values.primary.persistence.storageClass }}
  capacity:
    storage: {{ $.Values.primary.persistence.size }}
  accessModes:
    - ReadWriteOnce
  local:
    path: {{ .path }}
  nodeAffinity:
    required:
      nodeSelectorTerms:
        - matchExpressions:
            - key: kubernetes.io/hostname
              operator: In
              values:
                - {{ .host }}
---
{{- end }}

{{- range .Values.secondary.persistence.local }}
---
apiVersion: v1
kind: PersistentVolume
metadata:
  name: {{ .name }}
  labels:
    name: {{ .name }}
spec:
  storageClassName: {{ $.Values.secondary.persistence.storageClass }}
  capacity:
    storage: {{ $.Values.secondary.persistence.size }}
  accessModes:
    - ReadWriteOnce
  local:
    path: {{ .path }}
  nodeAffinity:
    required:
      nodeSelectorTerms:
        - matchExpressions:
            - key: kubernetes.io/hostname
              operator: In
              values:
                - {{ .host }}
---
{{- end }}
```

添加mysql/templates/storage-class.yaml

```yaml
kind: StorageClass
apiVersion: storage.k8s.io/v1
metadata:
  name: {{ .Values.primary.persistence.storageClass }}
provisioner: kubernetes.io/no-provisioner
```


开始安装


```shell


# 默认挂载文件格式是9p可能有问题，先直接minikube上执行
# 将主机的目录挂载到minikue节点上(minikube cluster的资源都挂载到这个目录下, minikube是在主机上创建的虚拟机,所以需要挂载)
# nohup minikube mount /opt/bigdata:/opt/bigdata > /tmp/minikube_mount.log 2>&1 &


# 创建持久化目录
sudo mkdir -p /var/bigdata/servers/mysql/data/data0
sudo mkdir -p /var/bigdata/servers/mysql/data/data1
sudo mkdir -p /var/bigdata/servers/mysql/data/data2
sudo chown -R 1001:1001 /var/bigdata/servers/mysql
sudo chmod 777 /var/bigdata/servers/mysql

# 提前准备好镜像
docker pull docker.io/bitnami/mysql:8.0.36-debian-12-r10
docker pull docker.io/bitnami/mysqld-exporter:0.14.0-debian-11-r33

# 加载镜像到minikube节点上
minikube image load docker.io/bitnami/mysql:8.0.36-debian-12-r10
minikube image load docker.io/bitnami/mysqld-exporter:0.14.0-debian-11-r33

# 开始安装
helm install mysql ./mysql -n mysql --create-namespace

```

安装完成后，输出如下:

```shell
AME: mysql
LAST DEPLOYED: Tue Apr 29 14:16:03 2025
NAMESPACE: mysql
STATUS: deployed
REVISION: 1
TEST SUITE: None
NOTES:
CHART NAME: mysql
CHART VERSION: 10.1.1
APP VERSION: 8.0.36

** Please be patient while the chart is being deployed **

Tip:

  Watch the deployment status using the command: kubectl get pods -w --namespace mysql

Services:

  echo Primary: mysql.mysql.svc.cluster.local:3306

Execute the following to get the administrator credentials:

  echo Username: root
  MYSQL_ROOT_PASSWORD=$(kubectl get secret --namespace mysql mysql -o jsonpath="{.data.mysql-root-password}" | base64 -d)

To connect to your database:

  1. Run a pod that you can use as a client:

      kubectl run mysql-client --rm --tty -i --restart='Never' --image  docker.io/bitnami/mysql:8.0.36-debian-12-r10 --namespace mysql --env MYSQL_ROOT_PASSWORD=$MYSQL_ROOT_PASSWORD --command -- bash

  2. To connect to primary service (read/write):

      mysql -h mysql.mysql.svc.cluster.local -uroot -p"$MYSQL_ROOT_PASSWORD"



To access the MySQL Prometheus metrics from outside the cluster execute the following commands:

    kubectl port-forward --namespace mysql svc/mysql-metrics 9104:9104 &
    curl http://127.0.0.1:9104/metrics




WARNING: There are "resources" sections in the chart not set. Using "resourcesPreset" is not recommended for production. For production installations, please set the following values according to your workload needs:
  - metrics.resources
  - primary.resources
  - secondary.resources
  - volumePermissions.resources
+info https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
```

