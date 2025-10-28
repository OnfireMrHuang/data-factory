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

-- 资源表
create table if not exists df_c_resource
(
    id            char(36) not null comment '主键',
    name          varchar(64) not null comment '资源名称',
    description   varchar(255) default '' comment '资源描述',
    resource_type enum('relational_database', 'file_system', 'queue') not null comment '资源类型',
    connection_config json not null comment '资源配置，存储连接信息等',
    status        enum('active', 'inactive') not null default 'active' comment '资源状态',
    created_at    timestamp not null default current_timestamp comment '创建时间',
    updated_at    timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id)
) comment '资源表' engine = InnoDB;



create table if not exists df_c_collection
(
    id            char(36) not null comment '主键',
    name          varchar(64) not null comment '采集任务名称',
    description   varchar(255) default '' comment '采集任务描述',
    category      enum('database', 'api', 'crawler') not null comment '采集分类',
    collect_type  enum('full', 'incremental') not null comment '采集类型',
    datasource_id char(36) not null comment 'source: 数据源ID',
    resource_id   char(36) not null comment 'sink: 资源ID',
    rule          json not null comment '采集规则',
    status        enum('draft', 'saved', 'applied', 'running', 'failed') not null default 'draft' comment '任务状态',
    created_at    timestamp not null default current_timestamp comment '创建时间',
    updated_at    timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    applied_at    timestamp null comment '应用到数据引擎的时间',
    primary key (id),
    key idx_datasource_id (datasource_id),
    key idx_resource_id (resource_id),
    key idx_status (status),
    key idx_category_type (category, collect_type)
) COMMENT '采集任务表' engine = InnoDB;