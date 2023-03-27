alter table `axum`.`address`
  add index address_city_idx (`city`);
alter table `axum`.`address`
  add index address_state_idx (`state`);
alter table `axum`.`address`
  add index address_country_idx (`country`);
alter table `axum`.`address`
  add index address_deleted_at_idx (`deleted_at`);
alter table `axum`.`address`
  add CONSTRAINT fk_address_users_user_id foreign key (`user_id`) references `axum`.`users` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;
