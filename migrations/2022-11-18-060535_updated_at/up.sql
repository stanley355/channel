-- Your SQL goes here
CREATE  FUNCTION trigger_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER trigger_updated_at
    BEFORE UPDATE
    ON
        channels
    FOR EACH ROW
EXECUTE PROCEDURE trigger_updated_at();