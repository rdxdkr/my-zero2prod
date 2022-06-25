-- Removed salt column for user password
ALTER TABLE
    users DROP COLUMN salt;