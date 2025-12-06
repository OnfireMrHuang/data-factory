-- 创建项目模版库
create database if not exists data_factory_template;

use data_factory_template;

-- 数据源表
create table if not exists df_c_datasource
(
    id            char(36) not null comment '主键',
    name          varchar(64) not null comment '数据源名称',
    description   varchar(255) default '' comment '数据源描述',
    category    enum('database', 'api') not null comment '数据源分类',
    datasource_type varchar(64) not null comment '数据源类型: mysql、postgres、查询API、订阅API',
    connection_config json not null comment '数据源配置，存储连接信息等',
    connection_status enum('connected', 'disconnected', 'error') not null default 'disconnected' comment '数据源连接状态',
    created_at    timestamp not null default current_timestamp comment '创建时间',
    updated_at    timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id)
) comment '数据源表' engine = InnoDB;



create table if not exists df_c_collection
(
    id            char(36) not null comment '主键',
    code          char(36) not null comment '任务编码',
    name          varchar(64) not null comment '采集任务名称',
    description   varchar(255) default '' comment '采集任务描述',
    category      enum('database', 'api', 'crawler') not null comment '采集分类',
    collect_type  enum('full', 'incremental') not null comment '采集类型',
    datasource_id char(36) not null comment 'source: 数据源ID',
    queue_resource_id char(36) not null comment '队列资源ID, 当采集类型为增量时必选',
    database_resource_id   char(36) not null comment '数据库资源ID, 当采集分类为database时必选',
    rule          json not null comment '采集规则',
    stage         enum('draft', 'applied') not null default 'draft' comment '开发阶段',
    created_at    timestamp not null default current_timestamp comment '创建时间',
    updated_at    timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    applied_at    timestamp null comment '应用到数据引擎的时间',
    primary key (id),
    key idx_datasource_id (datasource_id),
    key idx_queue_resource_id (queue_resource_id),
    key idx_database_resource_id (database_resource_id),
    key idx_stage (stage),
    key idx_category_type (category, collect_type)
) COMMENT '采集任务表' engine = InnoDB;


-- 测试运行表
create table if not exists df_c_collection_test_run 
(
    id               char(36) not null comment '主键',
    collection_code  char(36) not null comment '采集任务编码',
    status           enum('pending', 'running', 'success', 'failed') not null default 'pending' comment '测试运行状态',
    started_at       timestamp null comment '开始时间',
    completed_at     timestamp null comment '完成时间',
    error_message    text comment '错误消息',
    created_at       timestamp not null default current_timestamp comment '创建时间',
    updated_at       timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id),
    key idx_collection_code (collection_code),
    key idx_status (status),
    key idx_created_at (created_at)
) comment '测试运行表' engine = InnoDB;


-- 测试步骤表
create table if not exists df_c_collection_test_run_step
(
    id               char(36) not null comment '主键',
    test_run_id      char(36) not null comment '测试运行ID',
    step_id          int not null comment '步骤ID',
    title            varchar(128) not null comment '步骤标题',
    description      varchar(512) comment '步骤描述',
    status           enum('pending', 'running', 'success', 'failed', 'skipped') not null default 'pending' comment '步骤状态',
    started_at       timestamp null comment '开始时间',
    completed_at     timestamp null comment '完成时间',
    error_message    text comment '错误消息',
    created_at       timestamp not null default current_timestamp comment '创建时间',
    updated_at       timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id),
    key idx_test_run_id (test_run_id),
    key idx_step_id (step_id)
) comment '测试步骤表' engine = InnoDB;


-- 测试日志表
create table if not exists df_c_collection_test_run_log
(
    id               bigint not null auto_increment comment '主键',
    created_at    bigint not null comment '创建时间的unix毫秒时间戳',
    test_run_id      char(36) not null comment '测试运行ID',
    log_level        enum('info', 'success', 'warning', 'error') not null default 'info' comment '日志级别',
    message          varchar(512) not null comment '日志消息',
    details          text comment '详细信息',
    primary key (id),
    key idx_test_run_id (test_run_id),
    key idx_log_level (log_level),
    key idx_created_at (created_at)
) comment '测试日志表，支持unix毫秒级时间戳' engine = InnoDB;


