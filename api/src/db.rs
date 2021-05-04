use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_connection_pool() -> Result<Pool<Postgres>> {
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(
      std::env::var("DATABASE_URL")
        .expect("Missing DATABASE_URL in env")
        .as_str(),
    )
    .await?;

  Ok(pool)
}
