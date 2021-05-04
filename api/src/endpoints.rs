use crate::channel;
use crate::channel::PartialChannel;
use crate::message;
use crate::message::PartialMessage;
use crate::State;
use serde_json::value::Value;
use tide::prelude::*;
use tide::{Request, Server};

async fn get_channel(req: Request<State>) -> tide::Result<Value> {
  let name = req
    .param("name")
    .expect("Expected a name to be passed into the request");
  let name: String = String::from(name);

  channel::get_channel(&req.state().db, name)
    .await
    .map(|ch| json!(ch))
    .map_err(|err| err.into())
}

async fn get_channels(req: Request<State>) -> tide::Result<Value> {
  channel::get_channels(&req.state().db)
    .await
    .map(|chs| json!(chs))
    .map_err(|err| err.into())
}

async fn get_messages(req: Request<State>) -> tide::Result<Value> {
  message::get_messages(&req.state().db)
    .await
    .map(|chs| json!(chs))
    .map_err(|err| err.into())
}

async fn create_channel(mut req: Request<State>) -> tide::Result<Value> {
  let channel: PartialChannel =
    req.body_json().await.expect("expected body to be JSON");

  channel::create_channel(channel, &req.state().db)
    .await
    .map(|ch| json!(ch))
    .map_err(|err| err.into())
}

async fn update_channel(mut req: Request<State>) -> tide::Result<Value> {
  let channel: PartialChannel =
    req.body_json().await.expect("expected body to be JSON");

  channel::update_channel(channel, &req.state().db)
    .await
    .map(|ch| json!(ch))
    .map_err(|err| err.into())
}

async fn create_message(mut req: Request<State>) -> tide::Result<Value> {
  let message: PartialMessage =
    req.body_json().await.expect("expected body to be JSON");

  message::create_message(message, &req.state().db)
    .await
    .map(|ch| json!(ch))
    .map_err(|err| err.into())
}

pub fn mount(mut server: Server<State>) -> Server<State> {
  let mut v1 = server.at("/api/v1");
  // GETs
  v1.at("/channel/:name").get(get_channel);
  v1.at("/channels").get(get_channels);
  v1.at("/messages/:channel").get(get_messages);

  // POSTs
  v1.at("/channel").post(create_channel);
  v1.at("/channel").post(update_channel);
  v1.at("/message/:channel").post(create_message);

  server
}
