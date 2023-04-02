create index address_city_idx on `axum`.`address` (`city`);
create index address_state_idx on `axum`.`address` (`state`);
create index address_country_idx on `axum`.`address` (`country`);
create index address_deleted_at_idx on `axum`.`address` (`deleted_at`);

alter table `axum`.`address`
  add constraint fk_address_users_user_id foreign key (`user_id`) references `axum`.`users` (`id`);
