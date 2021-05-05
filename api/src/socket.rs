use crate::message::PartialMessage;
use crate::{
  broker::InterMsg,
  broker::{Client, InterMsgType},
  channel::PartialChannel,
};
use crate::{State, User};
use async_std::sync::{Arc, RwLock};
use futures_util::StreamExt;
use redis::{FromRedisValue, RedisResult, Value as RedisValue};
use sqlx::{Pool, Postgres};
use tide::prelude::*;
use tide::{Request, Server};
use tide_websockets::{Message as WSMessage, WebSocket, WebSocketConnection};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MsgType {
  UserConnection,
  NewChannel,
  RetrieveChannels,
  RetrieveChannelMessages,
  UpdateChannel,
  NewMessage,
  UpdateMessage,
  DeleteMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SockMsg {
  pub _type: MsgType,
  pub message: Option<PartialMessage>,
  pub channel: Option<PartialChannel>,
  pub user: Option<User>,
}

#[derive(Debug, Clone)]
pub struct RedisMsg {
  pub _type: MsgType,
  pub message: Option<PartialMessage>,
  pub channel: Option<PartialChannel>,
  pub user: Option<User>,
  pub users: Arc<RwLock<Vec<User>>>,
  pub db: Pool<Postgres>,
  pub ws: WebSocketConnection,
}

impl FromRedisValue for SockMsg {
  fn from_redis_value(v: &RedisValue) -> RedisResult<Self> {
    match v {
      RedisValue::Data(data) => {
        let result: Self = serde_json::from_slice(data).unwrap();
        Ok(result)
      }
      _ => Err((redis::ErrorKind::TypeError, "Data is not valid JSON").into()),
    }
  }
}

async fn mount_ws(
  req: Request<State>,
  wsc: WebSocketConnection,
) -> tide::Result<()> {
  while let Some(message) = wsc.clone().next().await {
    match message {
      Ok(WSMessage::Text(string_msg)) => {
        println!("{}", string_msg.clone());
        let msg: SockMsg = serde_json::from_str(string_msg.as_str())?;
        match msg._type {
          MsgType::UserConnection => {
            let pubsub = req.state().pubsub.clone();
            pubsub
              .send(InterMsg {
                _type: InterMsgType::Client,
                client: Some(Client {
                  username: msg.user.unwrap().username,
                  handle: wsc.clone(),
                }),
                message: Some(string_msg.clone()),
                channel: Some(String::from("channels")),
              })
              .await?;
          }
          MsgType::NewChannel => {
            let pubsub = req.state().pubsub.clone();
            pubsub
              .send(InterMsg {
                _type: InterMsgType::Message,
                channel: Some(String::from("channels")),
                message: Some(string_msg.clone()),
                client: None,
              })
              .await?;
          }
          MsgType::RetrieveChannels => {
            let pubsub = req.state().pubsub.clone();
            pubsub
              .send(InterMsg {
                _type: InterMsgType::Message,
                channel: Some(String::from("channels")),
                message: Some(string_msg.clone()),
                client: None,
              })
              .await?;
          }
          MsgType::UpdateChannel => {
            let pubsub = req.state().pubsub.clone();
            pubsub
              .send(InterMsg {
                _type: InterMsgType::Message,
                channel: Some(String::from("channels")),
                message: Some(string_msg.clone()),
                client: None,
              })
              .await?;
          }
          MsgType::RetrieveChannelMessages => {
            let pubsub = req.state().pubsub.clone();
            pubsub
              .send(InterMsg {
                _type: InterMsgType::Message,
                channel: Some(String::from("channels")),
                message: Some(string_msg.clone()),
                client: None,
              })
              .await?;
          }
          MsgType::NewMessage => {
            let pubsub = req.state().pubsub.clone();
            pubsub
              .send(InterMsg {
                _type: InterMsgType::Message,
                channel: Some(String::from("messages")),
                message: Some(string_msg.clone()),
                client: None,
              })
              .await?;
          }
          MsgType::UpdateMessage => {
            let pubsub = req.state().pubsub.clone();
            pubsub
              .send(InterMsg {
                _type: InterMsgType::Message,
                channel: Some(String::from("messages")),
                message: Some(string_msg.clone()),
                client: None,
              })
              .await?;
          }
          MsgType::DeleteMessage => {
            let pubsub = req.state().pubsub.clone();
            pubsub
              .send(InterMsg {
                _type: InterMsgType::Message,
                channel: Some(String::from("messages")),
                message: Some(string_msg.clone()),
                client: None,
              })
              .await?;
          }
        }
      }
      _e => {
        return Err(tide::http::format_err!(
          "An event happened neither related to websockets nor redis"
        ));
      }
    }
  }

  Ok(())
}

pub fn mount(mut server: Server<State>) -> Server<State> {
  let mut v1 = server.at("/api/v1");
  v1.at("/wsc").get(WebSocket::new(mount_ws));

  server
}
