-- Added salt column for user password
ALTER TABLE
    users
ADD
    COLUMN salt TEXT NOT NULL;