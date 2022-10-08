-- Your SQL goes here
ALTER TABLE channels 
  ALTER COLUMN owner_id TYPE uuid USING (uuid_generate_v4()),
  ALTER COLUMN owner_id SET NOT NULL;