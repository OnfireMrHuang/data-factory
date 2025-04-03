#!/bin/bash

# 加载配置文件: todo


# 配置JVM参数: todo


function start() {
  echo "App is running in Docker"
    # 启动应用
    exec $JAVA_HOME/bin/java $JAVA_OPTS -jar $RPG_NAME 2>&1
}
