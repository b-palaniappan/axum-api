create table if not exists `axum`.`users`
(
  `id`         bigint       not null auto_increment primary key,
  `first_name` varchar(255),
  `last_name`  varchar(255) not null,
  `email`      varchar(255) not null,
  `created_at` datetime(6)  not null default current_timestamp(6),
  `updated_at` datetime(6)  not null default current_timestamp(6),
  `deleted_at` datetime(6)
);
