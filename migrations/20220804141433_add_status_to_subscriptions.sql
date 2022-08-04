-- Add column status will null values to subscriptions table
ALTER TABLE subscriptions ADD COLUMN status TEXT null;