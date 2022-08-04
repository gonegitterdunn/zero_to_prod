-- We wrap the whole migration in a transaction to make sure it succeeds or fails atomically.
-- sqlx does not automatically do this for us
-- Update status column from null to 'confirmed'
-- Make status column mandatory
BEGIN;
  -- backfill 'status' for historical entries
  UPDATE subscriptions
    SET status = 'confirmed'
    WHERE status IS NULL;
  -- make 'status' mandatory
  ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;