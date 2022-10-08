-- This file should undo anything in `up.sql`
ALTER TABLE channels 
  ALTER COLUMN owner_id TYPE varchar,
  ALTER COLUMN owner_id SET NOT NULL;