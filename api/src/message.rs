use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
  pub id: Uuid,
  pub idx: i64,
  pub username: String,
  pub content: String,
  pub channel_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartialMessage {
  pub id: Option<Uuid>,
  pub idx: Option<i64>,
  pub username: String,
  pub content: String,
  pub channel_id: Uuid,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}

pub async fn get_channel_messages(
  channel: Uuid,
  db: &Pool<Postgres>,
) -> Result<Vec<Message>> {
  let messages: Result<Vec<Message>, sqlx::Error> = query_as!(
    Message,
    "SELECT * FROM messages WHERE channel_id = $1",
    channel
  )
  .fetch_all(db)
  .await;

  match messages {
    Ok(msgs) => Ok(msgs),
    Err(e) => Err(e.into()),
  }
}

pub async fn get_messages(db: &Pool<Postgres>) -> Result<Vec<Message>> {
  let messages: Result<Vec<Message>, sqlx::Error> =
    query_as!(Message, "SELECT * FROM messages")
      .fetch_all(db)
      .await;

  match messages {
    Ok(msgs) => Ok(msgs),
    Err(e) => Err(e.into()),
  }
}

pub async fn create_message(
  message: PartialMessage,
  db: &Pool<Postgres>,
) -> Result<Message> {
  let new_message: Result<Message, sqlx::Error> = query_as!(
    Message,
    "INSERT INTO messages(
        username,
        content,
        channel_id
      ) VALUES (
        $1,
        $2,
        $3
      ) RETURNING *;",
    message.username,
    message.content,
    message.channel_id
  )
  .fetch_one(db)
  .await;

  match new_message {
    Ok(msg) => Ok(msg),
    Err(e) => Err(e.into()),
  }
}

pub async fn update_message(
  message: PartialMessage,
  db: &Pool<Postgres>,
) -> Result<Message> {
  let update_message: Result<Message, sqlx::Error> = query_as!(
    Message,
    "UPDATE messages
        SET content = $1 
      WHERE id = $2
      RETURNING
        id,
        idx,
        username,
        content,
        channel_id,
        created_at,
        updated_at
      ;",
    message.content,
    message.id
  )
  .fetch_one(db)
  .await;

  match update_message {
    Ok(msg) => Ok(msg),
    Err(e) => Err(e.into()),
  }
}
