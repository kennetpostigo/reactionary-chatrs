use crate::channel;
use crate::message;
use crate::socket::{MsgType, SockMsg};
use async_channel as achannel;
use async_channel::{Receiver, Sender};
use async_std::task;
use futures_util::future::Either;
use futures_util::StreamExt;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use sqlx::{Pool, Postgres};
use tide::prelude::*;
use tide_websockets::WebSocketConnection;

#[derive(Clone)]
pub enum InterMsgType {
  Message,
  Client,
}

#[derive(Clone)]
pub struct InterMsg {
  pub _type: InterMsgType,
  pub channel: Option<String>,
  pub message: Option<String>,
  pub client: Option<Client>,
}

#[derive(Clone)]
pub struct Client {
  pub username: String,
  pub handle: WebSocketConnection,
}

#[derive(Clone)]
struct Intermediary {
  pub receiver: Receiver<InterMsg>,
  pub sender: Sender<InterMsg>,
  pub redis: MultiplexedConnection,
  pub postgres: Pool<Postgres>,
  clients: Vec<Client>,
}

impl Intermediary {
  pub fn new(postgres: Pool<Postgres>, redis: MultiplexedConnection) -> Self {
    let (sender, receiver) = achannel::unbounded();
    Intermediary {
      receiver,
      sender,
      redis,
      postgres,
      clients: vec![],
    }
  }
}

pub async fn create_connection(
  postgres: Pool<Postgres>,
) -> redis::RedisResult<(MultiplexedConnection, Sender<InterMsg>)> {
  let client = redis::Client::open(
    std::env::var("REDIS_URL")
      .expect("Missing REDIS_URL in env")
      .as_str(),
  )
  .expect("EXPECT CONNECTION STRING TO WORK");
  let publish_conn = client.get_multiplexed_async_std_connection().await?;

  let mut inter = Intermediary::new(postgres, publish_conn.clone());
  let inter_handle = inter.sender.clone();
  task::spawn(async move {
    let mut pubsub_conn = client
      .get_async_std_connection()
      .await
      .unwrap()
      .into_pubsub();
    pubsub_conn.subscribe("users").await.unwrap();
    pubsub_conn.subscribe("channels").await.unwrap();
    pubsub_conn.subscribe("messages").await.unwrap();

    let mut combined_stream = futures_util::stream::select(
      inter.receiver.clone().map(Either::Left),
      pubsub_conn.on_message().map(Either::Right),
    );

    while let Some(stream) = combined_stream.next().await {
      match stream {
        Either::Left(msg) => match msg._type {
          InterMsgType::Client => {
            let clients = &mut inter.clients;
            let index = clients
              .iter()
              .position(|c| c.username == msg.client.clone().unwrap().username);
            match index {
              Some(_idx) => (),
              None => clients.push(msg.client.clone().unwrap()),
            }
            inter
              .sender
              .send(InterMsg {
                _type: InterMsgType::Message,
                client: None,
                message: msg.message,
                channel: msg.channel,
              })
              .await
              .unwrap();
          }
          InterMsgType::Message => inter
            .redis
            .publish(msg.channel.unwrap(), msg.message.unwrap())
            .await
            .unwrap(),
        },
        Either::Right(msg) => {
          let msg: SockMsg = msg.get_payload().unwrap();
          match msg._type {
            MsgType::UserConnection => {
              let channels =
                channel::get_channels(&inter.postgres).await.unwrap();
              let client = inter
                .clients
                .iter()
                .find(|c| c.username == msg.user.clone().unwrap().username);
              if let Some(client) = client {
                client
                  .handle
                  .send_json(&json!({
                    "data": {
                      "channels": channels
                    }
                  }))
                  .await
                  .unwrap()
              }
            }
            MsgType::NewChannel => {
              let channel =
                channel::create_channel(msg.channel.unwrap(), &inter.postgres)
                  .await
                  .unwrap();

              for client in inter.clients.iter() {
                client
                      .handle
                      .send_json(&json!({
                        "msg": format!("{} channel has been created", &channel.name),
                        "data": {
                          "channel": channel
                        }
                      }))
                      .await.unwrap();
              }
            }
            MsgType::RetrieveChannels => {
              todo!()
            }
            MsgType::UpdateChannel => {
              todo!()
            }
            MsgType::RetrieveChannelMessages => {
              let channel = msg.clone().channel.unwrap();
              let messages = message::get_channel_messages(
                channel.id.unwrap(),
                &inter.postgres,
              )
              .await
              .unwrap();

              for client in inter.clients.iter() {
                client
                  .handle
                  .send_json(&json!({
                    "msg":
                    format!(
                      "{} channel messages have been retrieved",
                      channel.name.clone()
                    ),
                  "data": {
                    "messages": {
                      "channel": channel.clone(),
                      "messages": messages
                    }
                  }
                  }))
                  .await
                  .unwrap();
              }
            }
            MsgType::NewMessage => {
              let new_message = msg.clone().message.unwrap();
              let message =
                message::create_message(new_message.clone(), &inter.postgres)
                  .await
                  .unwrap();

              for client in inter.clients.iter() {
                client
                  .handle
                  .send_json(&json!({
                    "msg": "New message recieved",
                    "data": {
                      "message": {
                        "channel": &new_message.channel_id,
                        "message": message
                      }
                    }
                  }))
                  .await
                  .unwrap();
              }
            }
            MsgType::UpdateMessage => {
              todo!()
            }
            MsgType::DeleteMessage => {
              todo!()
            }
          }
        }
      }
    }
  });

  Ok((publish_conn, inter_handle))
}
