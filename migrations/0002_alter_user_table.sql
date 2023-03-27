alter table `axum`.`users`
  add index users_deleted_at_idx (`deleted_at`);
alter table `axum`.`users`
  add index users_email_idx (`email`);
