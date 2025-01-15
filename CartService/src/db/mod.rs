use sqlx::{Pool, Postgres};

pub async fn establish_connection(db_url: &str) -> Pool<Postgres> {
    Pool::<Postgres>::builder()
        .max_connections(5)
        .build(db_url)
        .await
        .expect("Failed to connect to the database")
}
