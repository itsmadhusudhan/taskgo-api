use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub async fn connect_db() -> DBPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    return pool;
}
