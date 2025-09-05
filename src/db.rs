use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub type DbPool = PgPool;

pub async fn init_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:secret@127.0.0.1:5432/todo".into());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}
