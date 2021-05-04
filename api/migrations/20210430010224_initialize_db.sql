-- Add migration script here
CREATE OR REPLACE FUNCTION update_updated_at_column() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = now();
RETURN NEW;
END;
$$ language 'plpgsql';
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE channels (
  id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
  idx BIGSERIAL NOT NULL,
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE messages (
  id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
  idx BIGSERIAL NOT NULL,
  username TEXT NOT NULL,
  content TEXT NOT NULL,
  channel_id uuid NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  FOREIGN KEY (channel_id) REFERENCES channels(id)
);
CREATE TRIGGER update_channels_change_updated_at BEFORE
UPDATE ON channels FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();
CREATE TRIGGER update_messages_change_updated_at BEFORE
UPDATE ON messages FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();