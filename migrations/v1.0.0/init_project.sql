-- 创建项目模版库
create database if not exists data_factory_template;


-- 数据源表
create table if not exists df_c_datasource
(
    id            char(36) not null comment '主键',
    name          varchar(64) not null comment '数据源名称',
    description   varchar(255) default '' comment '数据源描述',
    category    enum('database', 'api') not null comment '数据源分类',
    datasource_type varchar(64) not null comment '数据源类型: mysql、postgres、查询API、订阅API',
    config        json not null comment '数据源配置，存储连接信息等',
    status        enum('active', 'inactive') not null default 'active' comment '数据源状态',
    created_at    timestamp not null default current_timestamp comment '创建时间',
    updated_at    timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id)
) comment '数据源表' engine = InnoDB;
