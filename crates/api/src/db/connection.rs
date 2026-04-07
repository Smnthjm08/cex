use sqlx::PgPool;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    println!("======");
    PgPool::connect(database_url)
        .await
        // .expect("Failed to connect to DB")
}
