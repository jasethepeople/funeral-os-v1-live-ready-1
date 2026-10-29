use sqlx::PgPool;
pub async fn init_pool() -> anyhow::Result<PgPool> {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://funeral:funeral@localhost:5432/funeralos".into());
    let pool = PgPool::connect(&url).await?;
    Ok(pool)
}
