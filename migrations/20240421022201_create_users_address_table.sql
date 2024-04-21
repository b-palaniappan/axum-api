create table users
(
    id         bigint auto_increment primary key not null,
    first_name varchar(255) not null,
    last_name  varchar(255) not null,
    email      varchar(255) not null,
    created_at timestamp    not null default current_timestamp,
    updated_at timestamp    not null default current_timestamp on update current_timestamp,
    deleted_at timestamp
);

create table address
(
    id         bigint auto_increment primary key not null,
    user_id    bigint       not null,
    line_one   varchar(255) not null,
    line_two   varchar(255) not null,
    city       varchar(255) not null,
    state      varchar(255) not null,
    country    varchar(255) not null,
    created_at timestamp    not null default current_timestamp,
    updated_at timestamp    not null default current_timestamp on update current_timestamp,
    deleted_at timestamp
);-- Add migration script here
