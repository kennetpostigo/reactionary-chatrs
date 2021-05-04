use crate::channel;
use crate::channel::PartialChannel;
use crate::message;
use crate::message::PartialMessage;
use crate::{State, User};
use futures_util::future::Either;
use futures_util::StreamExt;
use redis::{AsyncCommands, FromRedisValue, RedisResult, Value as RedisValue};
use tide::prelude::*;
use tide::{Request, Server};
use tide_websockets::{Message as WSMessage, WebSocket, WebSocketConnection};

#[derive(Debug, Serialize, Deserialize, Clone)]
enum MsgType {
  UserConnection,
  UserDisconnection,
  NewChannel,
  RetrieveChannels,
  RetrieveChannelMessages,
  UpdateChannel,
  NewMessage,
  UpdateMessage,
  DeleteMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SockMsg {
  _type: MsgType,
  message: Option<PartialMessage>,
  channel: Option<PartialChannel>,
  user: Option<User>,
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
  let mut pubsub = req.state().pubsub.write().await;

  let mut combined_stream = futures_util::stream::select(
    wsc.clone().map(Either::Left),
    pubsub.on_message().map(Either::Right),
  );

  while let Some(message) = combined_stream.next().await {
    match message {
      Either::Left(Ok(WSMessage::Text(string_msg))) => {
        println!("{}", string_msg.clone());
        let msg: SockMsg = serde_json::from_str(string_msg.as_str())?;
        match msg._type {
          MsgType::UserConnection => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("users", string_msg).await?;
          }
          MsgType::UserDisconnection => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("users", string_msg).await?;
          }
          MsgType::NewChannel => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("channels", string_msg).await?;
          }
          MsgType::RetrieveChannels => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("channels", string_msg).await?;
          }
          MsgType::UpdateChannel => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("channels", string_msg).await?;
          }
          MsgType::RetrieveChannelMessages => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("channels", string_msg).await?;
          }
          MsgType::NewMessage => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("messages", string_msg).await?;
          }
          MsgType::UpdateMessage => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("messages", string_msg).await?;
          }
          MsgType::DeleteMessage => {
            let mut pub_conn = req.state().broker.clone();
            pub_conn.publish("messages", string_msg).await?;
          }
        }
      }
      Either::Right(msg) => {
        let msg: SockMsg = msg.get_payload()?;
        match msg._type {
          MsgType::UserConnection => {
            let channels = channel::get_channels(&req.state().db).await?;
            let users = &req.state().users;
            let mut users = users.write().await;
            let new_user = msg.user.unwrap();
            users.push(new_user.clone());
            wsc
              .send_json(&json!({
                "msg": format!("{} has been connected", &new_user.username),
                "data": {
                  "channels": channels
                }
              }))
              .await?
          }
          MsgType::UserDisconnection => {
            let users = &req.state().users;
            let mut users = users.write().await;
            let old_user = msg.user.unwrap();
            let index = users
              .iter()
              .position(|user| user.username == old_user.username)
              .unwrap();
            users.remove(index);

            wsc
              .send_json(&json!({
                "msg": format!("{} has been disconnected", &old_user.username)
              }))
              .await?
          }
          MsgType::NewChannel => {
            let channel =
              channel::create_channel(msg.channel.unwrap(), &req.state().db)
                .await?;
            wsc
              .send_json(&json!({
                "msg": format!("{} channel has been created", &channel.name),
                "data": {
                  "channel": channel
                }
              }))
              .await?
          }
          MsgType::RetrieveChannels => {
            wsc.send_json(&json!({"hello": "world"})).await?
          }
          MsgType::UpdateChannel => {
            wsc.send_json(&json!({"hello": "world"})).await?
          }
          MsgType::RetrieveChannelMessages => {
            let channel = msg.clone().channel.unwrap();
            let messages = message::get_channel_messages(
              channel.id.unwrap(),
              &req.state().db,
            )
            .await?;
            wsc
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
              .await?
          }
          MsgType::NewMessage => {
            let new_message = msg.clone().message.unwrap();
            let message =
              message::create_message(new_message.clone(), &req.state().db)
                .await?;
            wsc
              .send_json(&json!({
                "msg": "New message recieved",
                "data": {
                  "message": {
                    "channel": &new_message.channel_id,
                    "message": message
                  }
                }
              }))
              .await?
          }
          MsgType::UpdateMessage => {
            wsc.send_json(&json!({"hello": "world"})).await?
          }
          MsgType::DeleteMessage => {
            wsc.send_json(&json!({"hello": "world"})).await?
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
