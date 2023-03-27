create table `axum`.`event`
(
  `id`      bigint                         not null auto_increment primary key,
  `type`    enum ('User', 'Auth', 'Other') not null,
  `name`    varchar(255)                   not null,
  `info`    json,
  `time`    datetime(6)                    not null default current_timestamp(6),
  `user_id` bigint
);

