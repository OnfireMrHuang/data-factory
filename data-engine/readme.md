# data-engine

## dataX依赖

数据引擎需要利用DataX来实现数据同步功能，实现方式是集成dataX的库来实现，但是需要依赖配置文件、插件等基础数据。所以需要在项目目录下添加datax目录，以此来调试或构建镜像。
步骤如下:

1、下载dataX到本地
```shell
cd ~/{your workspace}

git clone git@github.com:alibaba/DataX.git

cd DataX
```


2、maven编译

```shell
mvn -U clean package assembly:assembly -Dmaven.test.skip=true

# 编译后存在目录`target/datax/datax`
cd target/datax

```

3、将datax目录拷贝到项目目录下

```shell
cp -r datax ~/{your workspace}/data-factorydata-engine/datax

# 删除datax自身的库和运行文件，仅保留配置和插件文件
rm -fr lib bin tmp script

# 移除不需要的插件，减少镜像体积
rm -fr reeader/{unused plugins} writer/{unused plugins}}

```





