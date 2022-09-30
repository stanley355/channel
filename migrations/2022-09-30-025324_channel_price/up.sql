-- Your SQL goes here
ALTER TABLE channels
  ADD COLUMN subscription_price INTEGER NOT NULL DEFAULT 0;