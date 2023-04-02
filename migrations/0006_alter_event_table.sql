create index event_type_idx on `axum`.`event` (`type`);
create index event_name_idx on `axum`.`event` (`name`);
create index event_type_name_idx on `axum`.`event` (`type`, `name`);
create index event_time_idx on `axum`.`event` (`time`);

alter table `axum`.`event`
  add constraint fk_event_users_user_id foreign key (`user_id`) references `axum`.`users` (`id`);
