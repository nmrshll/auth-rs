use diesel::{r2d2::ConnectionManager, PgConnection};
use std::env;

const DEFAULT_DATABASE_URL: &str = "postgres://docker:docker@127.0.0.1:5432/docker";

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn connect_DB() -> Pool {
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL));
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    return pool
}

// auto-connect to DB, keep pool global
lazy_static::lazy_static! {
    pub static ref DB_CONN_POOL: Pool = connect_DB();
}

pub type PooledConnection = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
pub fn get_db_conn() -> Result<PooledConnection,r2d2::Error> {
    DB_CONN_POOL.clone().get()
}
