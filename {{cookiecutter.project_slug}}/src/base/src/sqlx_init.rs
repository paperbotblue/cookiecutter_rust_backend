use crate::constants;
use dotenv::dotenv;
use sqlx::PgPool;

pub async fn db_conn() -> PgPool {
    dotenv().ok();
    let database_url = constants::DATABASE_URL.clone();
    PgPool::connect(&database_url)
        .await
        .expect("unable to create sqlx db pool")
}
