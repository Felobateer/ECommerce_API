use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;

pub async fn establish_connection(db_url: &str) -> Pool<MySql> {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to connect to database")
}
