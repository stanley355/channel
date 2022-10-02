-- Your SQL goes here
CREATE TABLE channels (
  id SERIAL PRIMARY KEY,
  owner_id VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
  channel_name VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  subscribers integer NOT NULL DEFAULT 0,
  posts_number integer NOT NULL DEFAULT 0,
  subscription_price INTEGER NOT NULL DEFAULT 0
);
