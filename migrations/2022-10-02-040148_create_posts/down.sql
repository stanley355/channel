-- This file should undo anything in `up.sql`
ALTER TABLE channels 
  ALTER COLUMN id TYPE uuid PRIMARY KEY DEFAULT uuid_generate_v4();
DROP TABLE posts;