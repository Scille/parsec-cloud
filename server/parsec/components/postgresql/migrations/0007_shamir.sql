-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Update shamir recovery setup table to allow for deletion.
-- The reveal token is now a string instead of an UUID, similar to the invitation token.
-- Use the same unique index trick as the greeting attempt to ensure that there is only
-- one active recovery setup per user.
-------------------------------------------------------


ALTER TABLE shamir_recovery_setup ADD created_on TIMESTAMPTZ NOT NULL;
ALTER TABLE shamir_recovery_setup ADD deleted_on TIMESTAMPTZ;
ALTER TABLE shamir_recovery_setup ADD deletion_certificate BYTEA;
ALTER TABLE shamir_recovery_setup ALTER COLUMN reveal_token TYPE VARCHAR(32);


-- Makes sure that there is only one active setup per user
CREATE UNIQUE INDEX unique_active_setup ON shamir_recovery_setup (user_)
WHERE deleted_on IS NULL;
