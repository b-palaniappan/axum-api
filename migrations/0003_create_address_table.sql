
create table if not exists `axum`.`address`
(
  `id`         bigint       not null auto_increment primary key,
  `user_id`    bigint       not null,
  `line_one`   varchar(255) not null,
  `line_two`   varchar(255),
  `city`       varchar(255) not null,
  `state`      varchar(50)  not null,
  `country`    varchar(2)   not null,
  `geocode`    JSON         NOT NULL,
  `created_at` datetime(6)  not null default current_timestamp(6),
  `updated_at` datetime(6)  not null default current_timestamp(6),
  `deleted_at` datetime(6)
);
