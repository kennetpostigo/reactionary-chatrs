pub mod channel;
pub mod db;
pub mod endpoints;
pub mod helpers;
pub mod message;
pub mod socket;

use crate::db::create_connection_pool;
use crate::helpers::cors_middleware;
use anyhow::Result;
use async_std::sync::{Arc, RwLock};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tide_websockets::WebSocketConnection;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  id: i64,
  username: String
}

#[derive(Debug, Clone)]
pub struct WSUser {
  id: i64,
  username: String,
  handle: WebSocketConnection
}

#[derive(Clone)]
pub struct State {
  db: Pool<Postgres>,
  wsc: Arc<RwLock<Option<WebSocketConnection>>>,
  users: Arc<RwLock<Vec<WSUser>>>,
}

impl State {
  pub fn new(db: Pool<Postgres>) -> Self {
    State {
      db,
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
  let app = tide::with_state(State::new(db));
  let app = cors_middleware(app);
  let app = socket::mount(app);
  let app = endpoints::mount(app);

  let listen_addr = String::from("localhost:8080");
  println!("Playground: http://{}", listen_addr);
  app.listen(listen_addr).await?;

  Ok(())
}
