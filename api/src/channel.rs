use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::message::Message;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
  pub id: Uuid,
  pub idx: i64,
  pub name: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartialChannel {
  pub id: Option<Uuid>,
  pub idx: Option<i64>,
  pub name: String,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}

pub async fn get_channel(
  db: &Pool<Postgres>,
  name: String,
) -> Result<Option<Channel>> {
  let channel: Result<Channel, sqlx::Error> =
    query_as!(Channel, "SELECT * FROM channels WHERE name = $1", name)
      .fetch_one(db)
      .await;

  match channel {
    Ok(ch) => Ok(Some(ch)),
    Err(sqlx::Error::RowNotFound) => Ok(None),
    Err(e) => Err(e.into()),
  }
}

pub async fn get_channels(db: &Pool<Postgres>) -> Result<Vec<Channel>> {
  let channels: Result<Vec<Channel>, sqlx::Error> =
    query_as!(Channel, "SELECT * FROM channels")
      .fetch_all(db)
      .await;

  match channels {
    Ok(chs) => Ok(chs),
    Err(e) => Err(e.into()),
  }
}

pub async fn get_channel_messages(
  channel: Uuid,
  db: &Pool<Postgres>,
) -> Result<Vec<Message>> {
  let channels: Result<Vec<Message>, sqlx::Error> = query_as!(
    Message,
    "SELECT * FROM messages WHERE channel_id = $1",
    channel
  )
  .fetch_all(db)
  .await;

  match channels {
    Ok(msgs) => Ok(msgs),
    Err(e) => Err(e.into()),
  }
}

pub async fn create_channel(
  channel: PartialChannel,
  db: &Pool<Postgres>,
) -> Result<Channel> {
  let new_channel: Result<Channel, sqlx::Error> = query_as!(
    Channel,
    "INSERT INTO channels(
        name
      ) VALUES (
        $1
      ) RETURNING *;",
    channel.name
  )
  .fetch_one(db)
  .await;

  match new_channel {
    Ok(ch) => Ok(ch),
    Err(e) => Err(e.into()),
  }
}

pub async fn update_channel(
  channel: PartialChannel,
  db: &Pool<Postgres>,
) -> Result<Channel> {
  let update_channel: Result<Channel, sqlx::Error> = query_as!(
    Channel,
    "UPDATE channels
        SET name = $1 
      WHERE id = $2
      RETURNING
        id,
        idx,
        name,
        created_at,
        updated_at
      ;",
    channel.name,
    channel.id
  )
  .fetch_one(db)
  .await;

  match update_channel {
    Ok(ch) => Ok(ch),
    Err(e) => Err(e.into()),
  }
}
