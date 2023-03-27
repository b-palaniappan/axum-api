alter table `axum`.`event`
  add index event_type_idx (`type`);
alter table `axum`.`event`
  add index event_name_idx (`name`);
alter table `axum`.`event`
  add index event_type_name_idx (`type`, `name`);
alter table `axum`.`event`
  add index event_time_idx (`time`);
alter table `axum`.`event`
  add constraint fk_event_users_user_id foreign key (`user_id`) references `axum`.`users` (`id`);
