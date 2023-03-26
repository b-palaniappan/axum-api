use axum::extract::State;
use sqlx::MySqlPool;

pub async fn app_event(State(pool): State<MySqlPool>) {
  sqlx::query("insert into events (type, name, user_key) values ('AUTH', 'APP_STARTED', 'random_user')").execute(&pool).await.expect("TODO: panic message");
}
