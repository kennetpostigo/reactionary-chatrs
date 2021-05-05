pub mod broker;
pub mod channel;
pub mod db;
pub mod endpoints;
pub mod helpers;
pub mod message;
pub mod socket;

use crate::broker::create_connection;
use crate::db::create_connection_pool;
use crate::helpers::cors_middleware;
use anyhow::Result;
use async_channel::Sender;
use async_std::sync::{Arc, RwLock};
use broker::InterMsg;
use dotenv::dotenv;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tide_websockets::WebSocketConnection;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  id: i64,
  username: String,
}

#[derive(Clone)]
pub struct State {
  db: Pool<Postgres>,
  broker: MultiplexedConnection,
  pubsub: Sender<InterMsg>,
  wsc: Arc<RwLock<Option<WebSocketConnection>>>,
  users: Arc<RwLock<Vec<User>>>,
}

impl State {
  pub fn new(
    db: Pool<Postgres>,
    broker: (MultiplexedConnection, Sender<InterMsg>),
  ) -> Self {
    let (broker, pubsub) = broker;
    State {
      db,
      broker,
      pubsub,
      wsc: Arc::new(RwLock::new(None)),
      users: Arc::new(RwLock::new(vec![])),
    }
  }
}

#[async_std::main]
async fn main() -> Result<()> {
  tide::log::start();
  dotenv().ok();

  let db = create_connection_pool().await?;
  let broker = create_connection(db.clone()).await?;
  let app = tide::with_state(State::new(db, broker));
  let app = cors_middleware(app);
  let app = socket::mount(app);
  let app = endpoints::mount(app);

  let listen_addr = String::from("localhost:8080");
  println!("Playground: http://{}", listen_addr);
  app.listen(listen_addr).await?;

  Ok(())
}
