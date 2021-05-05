use crate::channel;
use crate::channel::PartialChannel;
use crate::message;
use crate::message::PartialMessage;
use crate::{State, User, WSUser};
use futures_util::StreamExt;
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

async fn mount_ws(
  req: Request<State>,
  wsc: WebSocketConnection,
) -> tide::Result<()> {
  while let Some(Ok(WSMessage::Text(string_msg))) = wsc.clone().next().await {
    println!("{}", string_msg.clone());
    let msg: SockMsg = serde_json::from_str(string_msg.as_str())?;

    match msg._type {
      MsgType::UserConnection => {
        let channels = channel::get_channels(&req.state().db).await?;
        let users = &req.state().users;
        let mut users = users.write().await;
        let new_user = msg.user.unwrap();
        let new_user = WSUser {
          id: new_user.id,
          username: new_user.username,
          handle: wsc.clone(),
        };
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
        wsc.send_json(&json!({ "hello": "world" })).await?;
      }
      MsgType::UpdateChannel => {
        wsc.send_json(&json!({ "hello": "world" })).await?;
      }
      MsgType::RetrieveChannelMessages => {
        let channel = msg.clone().channel.unwrap();
        let messages =
          message::get_channel_messages(channel.id.unwrap(), &req.state().db)
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
          message::create_message(new_message.clone(), &req.state().db).await?;

        let  users = &req.state().users.read().await;
        let users = users.iter();
        for user in users {
          user
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
            .await?
        }
      }
      MsgType::UpdateMessage => {
        wsc.send_json(&json!({ "hello": "world" })).await?;
      }
      MsgType::DeleteMessage => {
        wsc.send_json(&json!({ "hello": "world" })).await?;
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
