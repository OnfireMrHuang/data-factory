## 信息架构

### 登陆页

布局为左右布局，左侧为logo，右侧为登陆表单。

### 开发页

### 一级菜单导航

<table>
    <tr>
        <th>项目</th>
        <th>快捷搜索</th>
        <th>AI</th>
        <th>工具箱</th>
        <th>设置</th>
    </tr>
    <tr>
        <td>首页</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>数据采购</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>数据加工</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>数据供应</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>运维监控</td>
        <td colspan="2"></td>
    </tr>
</table>


### 二级菜单导航

- 项目面板
- 快捷搜索(暂时不做)
- 设置
    - 资源管理
- AI(暂时不做)
- 工具箱(暂时不做)
- 数据采购
    - 数据库集成
    - 自定义集成
- 数据加工
    - 主题管理
    - 宽表开发
    - 文件开发
    - 流开发
- 运维监控
    - 表任务列表
    - 流任务列表
    - 批任务列表
- 数据供应
    - 数据服务
    - 数据同步

### 菜单功能概览

- 资源管理
    - 配置数据仓库存储资源(doris、mysql、postgres)
    - 配置文件系统存储资源(hdfs、host file、emptyfile)
    - 配置Spark批处理计算资源
    - 配置Flink流计算资源
    - 配置Kafka消息队列资源
    - 配置mailvus向量数据库资源
- 数据库集成
    - 配置Mysql数据库
    - 配置Postgresql数据库
- 自定义集成
    - 输出:
        -  streaming、file、hive table、doris table、milvus table
    - 开发类型:
        - 常驻容器(container)
        - 任务(job)
    - 开发语言:
        -  python
        -  java
        -  scala
    - 开发资源
        - cpu、内存、磁盘
        - 中间件
- 宽表开发(仅支持Doris语法，类Mysql语法)
    - 表定义
        - 主键、字段、索引、分区
    - 实时模型
        - 监听规则配置
        - 清洗规则配置
    - 调度模式
        - 任务定义: 同步任务、SQL清洗任务
        - 任务DAG编排
        - 任务调度配置
- 自定义开发
    - 输入: 
        -  streaming、file、hive table、doris table、milvus table 
    - 输出:
        -  streaming、file、hive table、doris table、milvus table
    - 开发类型:
        - 常驻容器(container)
        - 任务(job)
    - 开发语言:
        -  python
        -  java
        -  scala
        -  SQL
    - 开发资源
        - cpu、内存、磁盘
        - 中间件
    - DAG编排
    - 调度配置
- 宽表任务
    - 宽表批任务
    - 宽表流任务
- 流任务
- 批任务
- 数据服务
    - 元数据查询接口
    - 取数接口
- 数据同步
    - 目标数据库定义
    - 同步任务定义


## 方案设计

### 登陆页




