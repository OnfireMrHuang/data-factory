
-- auto-generated definition
create table user (
    id          char(36)     not null comment '用户ID (uuid)' primary key,
    name        varchar(255) not null comment '用户名称',
    email       varchar(255) not null comment '用户注册邮箱',
    phone       varchar(255) null comment '用户移动手机号',
    password    varchar(255) not null comment '用户密码',
    create_time datetime     not null comment '创建时间',
    update_time datetime     not null comment '更新时间',
    constraint email unique (email),
    constraint phone unique (phone)
) comment '用户表' engine = InnoDB;

