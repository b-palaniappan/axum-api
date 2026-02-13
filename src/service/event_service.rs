use axum::extract::State;
use sqlx::PgPool;

pub async fn add_event(State(pool): State<PgPool>) {
    sqlx::query("insert into event (type, name, user_id) values ('Auth', 'APP_STARTED', 78)")
        .execute(&pool)
        .await
        .expect("panic message");
}
