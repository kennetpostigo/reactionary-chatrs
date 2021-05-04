use redis::aio::{MultiplexedConnection, PubSub};

pub async fn create_connection(
) -> redis::RedisResult<(MultiplexedConnection, PubSub)> {
  let client = redis::Client::open(
    std::env::var("REDIS_URL")
      .expect("Missing REDIS_URL in env")
      .as_str(),
  )
  .expect("EXPECT CONNECTION STRING TO WORK");
  let publish_conn = client.get_multiplexed_async_std_connection().await?;
  let mut pubsub_conn = client.get_async_std_connection().await?.into_pubsub();
  pubsub_conn.subscribe("users").await?;
  pubsub_conn.subscribe("channels").await?;
  pubsub_conn.subscribe("messages").await?;

  Ok((publish_conn, pubsub_conn))
}
