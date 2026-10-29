use async_nats::jetstream;
pub async fn init() -> anyhow::Result<jetstream::Context> {
    let url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".into());
    let client = async_nats::connect(url).await?;
    Ok(jetstream::new(client))
}
