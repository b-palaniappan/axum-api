create index event_type_idx on `axum`.`events` (`type`);
create index event_name_idx on `axum`.`events` (`name`);
create index event_type_name_idx on `axum`.`events` (`type`, `name`);
create index event_time_idx on `axum`.`events` (`time`);

alter table `axum`.`events`
  add constraint fk_event_users_user_id foreign key (`user_id`) references `axum`.`users` (`id`);
