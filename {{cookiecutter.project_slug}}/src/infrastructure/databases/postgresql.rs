use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;

use crate::domain::constants;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub fn db_pool() -> DBConn {
    dotenv().ok();
    let database_url = constants::DATABASE_URL.clone();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
