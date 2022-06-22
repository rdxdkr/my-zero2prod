-- performing a manual transaction because sqlx doesn't do them automatically
BEGIN;

-- Backfill `status` for historical entries
UPDATE
    subscriptions
SET
    status = 'confirmed'
WHERE
    status IS NULL;

-- Make `status` mandatory
ALTER TABLE
    subscriptions
ALTER COLUMN
    status
SET
    NOT NULL;

COMMIT;