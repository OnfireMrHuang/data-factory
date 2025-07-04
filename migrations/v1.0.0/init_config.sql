-- 创建配置数据库
create database if not exists data_factory_config;

use data_factory_config;

create table if not exists df_c_project
(
    code          varchar(64) not null comment '项目编码',
    name          varchar(64) not null comment '项目名称',
    description   varchar(255)                                   default '' comment '项目描述',
    create_status enum ('pending', 'running', 'success', 'fail') default 'pending' comment '项目创建状态',
    create_msg    varchar(255)                                   default '' comment '项目创建失败信息',
    logo          varchar(255)                                   default '' comment '项目logo',
    created_at    timestamp   not null                           default current_timestamp comment '创建时间',
    updated_at    timestamp   not null                           default current_timestamp on update current_timestamp comment '更新时间',
    primary key (code)
) comment '项目表' engine = InnoDB;


