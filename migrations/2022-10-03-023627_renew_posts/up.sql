-- Your SQL goes here
CREATE TABLE posts (
  id VARCHAR PRIMARY KEY,
  channels_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
  img_url VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  likes integer NOT NULL DEFAULT 0,
  post_type VARCHAR NOT NULL,
  is_free BOOLEAN NOT NULL DEFAULT FALSE,
  FOREIGN KEY (channels_id) REFERENCES channels (id)
);
