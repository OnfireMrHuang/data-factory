


FROM registry.cn-hangzhou.aliyuncs.com/bigdata_cloudnative/centos:7.7.1908

RUN rm -f /etc/localtime && ln -sv /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo "Asia/Shanghai" > /etc/timezone

RUN export LANG=zh_CN.UTF-8

# 替换 yum 源
RUN curl -o /etc/yum.repos.d/CentOS-Base.repo https://mirrors.aliyun.com/repo/Centos-7.repo && \
    yum clean all && \
    yum makecache

# 创建用户和用户组，跟yaml编排里的user: 10000:10000
RUN groupadd --system --gid=10000 hadoop && useradd --system --home-dir /home/hadoop --uid=10000 --gid=hadoop hadoop

# 安装sudo
RUN yum -y install sudo ; chmod 640 /etc/sudoers

# 给hadoop添加sudo权限
RUN echo "hadoop ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

RUN yum -y install install net-tools telnet wget nc expect which

RUN mkdir /opt/apache/

# 安装 JDK
ADD jdk-8u212-linux-x64.tar.gz /opt/apache/
ENV JAVA_HOME /opt/apache/jdk1.8.0_212
ENV PATH $JAVA_HOME/bin:$PATH

# 配置 Hadoop
ENV HADOOP_VERSION 3.3.5
ADD hadoop-${HADOOP_VERSION}.tar.gz /opt/apache/
ENV HADOOP_HOME /opt/apache/hadoop
RUN ln -s /opt/apache/hadoop-${HADOOP_VERSION} $HADOOP_HOME

ENV HADOOP_COMMON_HOME=${HADOOP_HOME} \
    HADOOP_HDFS_HOME=${HADOOP_HOME} \
    HADOOP_MAPRED_HOME=${HADOOP_HOME} \
    HADOOP_YARN_HOME=${HADOOP_HOME} \
    HADOOP_CONF_DIR=${HADOOP_HOME}/etc/hadoop \
    PATH=${PATH}:${HADOOP_HOME}/bin

# 配置Hive
ENV HIVE_VERSION 3.1.3
ADD apache-hive-${HIVE_VERSION}-bin.tar.gz /opt/apache/
ENV HIVE_HOME=/opt/apache/hive
ENV PATH=$HIVE_HOME/bin:$PATH
RUN ln -s /opt/apache/apache-hive-${HIVE_VERSION}-bin ${HIVE_HOME}

# 配置spark(可选)
# ENV SPARK_VERSION 3.4.4
# ADD spark-${SPARK_VERSION}-bin-hadoop3.tgz /opt/apache/
# ENV SPARK_HOME=/opt/apache/spark
# ENV PATH=$SPARK_HOME/bin:$PATH
# RUN ln -s /opt/apache/spark-${SPARK_VERSION}-bin-hadoop3 ${SPARK_HOME}

# 配置 flink（可选）
# ENV FLINK_VERSION 1.17.2
# ADD flink-${FLINK_VERSION}-bin-scala_2.12.tgz /opt/apache/
# ENV FLINK_HOME=/opt/apache/flink
# ENV PATH=$FLINK_HOME/bin:$PATH
# RUN ln -s /opt/apache/flink-${FLINK_VERSION} ${FLINK_HOME}

# 创建namenode、datanode存储目录
RUN mkdir -p /opt/apache/hadoop/data/{hdfs,yarn} /opt/apache/hadoop/data/hdfs/namenode /opt/apache/hadoop/data/hdfs/datanode/data{1..3} /opt/apache/hadoop/data/yarn/{local-dirs,log-dirs,apps}

COPY bootstrap.sh /opt/apache/

# hadoop config
COPY config/hadoop-config/* ${HADOOP_HOME}/etc/hadoop/

# hive  config
COPY config/hive-config/* ${HIVE_HOME}/conf/

# mysql 8的驱动
COPY mysql-connector-java-8.0.29/mysql-connector-java-8.0.29.jar ${HIVE_HOME}/lib/

RUN sudo mkdir -p /home/hadoop/ && sudo chown -R hadoop:hadoop /home/hadoop/

#RUN yum -y install which
ENV ll "ls -l"

RUN chmod +x /opt/apache/bootstrap.sh
RUN chown -R hadoop:hadoop /opt/apache
WORKDIR /opt/apache
