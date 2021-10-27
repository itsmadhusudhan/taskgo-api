use sqlx::Postgres;
use std::env;

use sqlx::postgres::PgPoolOptions;

pub async fn connect_db() -> sqlx::Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pg_pool = (PgPoolOptions::new()
        .max_connections(3)
        .connect(&database_url.to_string())
        .await)
        .expect("Db error");

    return pg_pool;
}
