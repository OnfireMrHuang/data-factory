## 信息架构

### 登陆页

布局为左右布局，左侧为logo，右侧为登陆表单。

### 开发页

### 一级菜单导航

<table>
    <tr>
        <th>工作空间</th>
        <th>智能助手</th>
        <th>工具</th>
    </tr>
    <tr>
        <td>首页</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>数据集成</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>数据开发</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>版本管理</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>运维监控</td>
        <td colspan="2"></td>
    </tr>
    <tr>
        <td>数据供应</td>
        <td colspan="2"></td>
    </tr>
</table>


### 二级菜单导航

- 设置
    - 资源管理
    - 工作空间管理
- 智能助手(暂时不做)
- 工具(暂时不做)
- 数据集成
    - 数据库集成
    - 自定义集成
- 数据开发
    - 宽表开发
    - 自定义开发
- 版本管理
    - 版本列表
    - 归档
    - 回滚
- 运维监控
    - 宽表任务列表
    - 流任务列表
    - 批任务列表
- 数据供应
    - 数据服务
    - 数据同步

### 菜单功能概览

- 资源管理
    - 配置数据仓库存储资源
    - 配置文件系统存储资源
    - 配置Spark批处理计算资源
    - 配置Flink流计算资源
    - 配置Kafka消息队列资源
    - 配置mailvus向量数据库资源
- 工作空间管理(catalogue、schema、table三级管理)
    - 工作空间列表
    - 工作空间创建
    - 工作空间编辑
    - 工作空间删除
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




