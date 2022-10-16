-- This file should undo anything in `up.sql`
ALTER TABLE channels
  DROP COLUMN profile_img_url,
  DROP COLUMN background_img_url;