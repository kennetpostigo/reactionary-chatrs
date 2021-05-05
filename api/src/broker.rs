use crate::channel;
use crate::channel::PartialChannel;
use crate::message;
use crate::socket::{MsgType, SockMsg};
use anyhow::Result;
use async_channel as achannel;
use async_channel::{Receiver, Sender};
use async_std::task;
use futures_util::future::Either;
use futures_util::StreamExt;
use redis::{
  aio::{MultiplexedConnection, PubSub},
  AsyncCommands,
};
use sqlx::{Pool, Postgres};
use tide::prelude::*;
use tide_websockets::WebSocketConnection;

pub struct IntermediaryMsg {
  pub channel: Option<String>,
  pub message: Option<String>,
  pub client: Option<Client>,
  pub db: Option<Pool<Postgres>>,
}

#[derive(Clone)]
struct Client {
  pub username: String,
  pub handle: WebSocketConnection,
}

#[derive(Clone)]
struct Intermediary {
  pub receiver: Receiver<IntermediaryMsg>,
  pub sender: Sender<IntermediaryMsg>,
  pub redis: MultiplexedConnection,
  pub db: Option<Pool<Postgres>>,
  pub clients: Vec<Client>,
}

impl Intermediary {
  pub fn new(redis: MultiplexedConnection) -> Self {
    let (sender, receiver) = achannel::unbounded();
    Intermediary {
      receiver,
      sender,
      redis,
      db: None,
      clients: vec![],
    }
  }

  pub async fn publish(&mut self, msg: IntermediaryMsg) -> Result<()> {
    self.sender.send(msg).await.map_err(|op| op.into())
  }
}

pub async fn create_connection(
) -> redis::RedisResult<(MultiplexedConnection, Sender<IntermediaryMsg>)> {
  let client = redis::Client::open(
    std::env::var("REDIS_URL")
      .expect("Missing REDIS_URL in env")
      .as_str(),
  )
  .expect("EXPECT CONNECTION STRING TO WORK");
  let publish_conn = client.get_multiplexed_async_std_connection().await?;
  let mut pubsub_conn = client.get_async_std_connection().await?.into_pubsub();
  let inter = Intermediary::new(publish_conn.clone());

  pubsub_conn.subscribe("users").await?;
  pubsub_conn.subscribe("channels").await?;
  pubsub_conn.subscribe("messages").await?;

  let mut combined_stream = futures_util::stream::select(
    inter.receiver.clone().map(Either::Left),
    pubsub_conn.on_message().map(Either::Right),
  );

  task::spawn(async move {
    while let Some(stream) = combined_stream.next().await {
      match stream {
        Either::Left(msg) => {}
        Either::Right(msg) => {
          let msg: SockMsg = msg.get_payload().unwrap();
        }
      }
    }
  });

  Ok((publish_conn, inter.sender.clone()))
}
