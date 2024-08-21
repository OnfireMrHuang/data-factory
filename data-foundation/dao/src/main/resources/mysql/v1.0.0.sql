
-- 用户表(设置邮箱、手机号唯一)
create table user (
    id char(36) comment '用户ID (uuid)' primary key,
    name varchar(255) comment '用户名称' not null,
    email varchar(255) comment '用户注册邮箱' not null,
    phone varchar(255) comment '用户移动手机号',
    password varchar(255) comment '用户密码' not null,
    create_time datetime comment '创建时间' not null,
    update_time datetime comment '更新时间' not null,
    unique key (email),
    unique key (phone)
) engine=innodb default charset=utf8mb4 comment='用户表';

