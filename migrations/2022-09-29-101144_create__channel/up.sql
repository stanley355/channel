-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE channels (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  owner_id VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
  channel_name VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  subscribers integer NOT NULL DEFAULT 0,
  posts_number integer NOT NULL DEFAULT 0
);
