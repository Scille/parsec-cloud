-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- TOTP support
-------------------------------------------------------

ALTER TABLE user_
ADD COLUMN totp_setup_completed BOOLEAN NOT NULL DEFAULT FALSE,
-- Cannot be NULL if `totp_setup_completed` is TRUE
ADD COLUMN totp_secret BYTEA,
-- NULL if no reset token is active
-- Must be NULL if `totp_setup_completed` is TRUE
ADD COLUMN totp_reset_token VARCHAR(32),

ADD CONSTRAINT check_totp_secret_when_setup_completed
CHECK (totp_setup_completed = FALSE OR totp_secret IS NOT NULL),

ADD CONSTRAINT check_totp_reset_token_when_setup_completed
CHECK (totp_setup_completed = FALSE OR totp_reset_token IS NULL);

CREATE TABLE totp_opaque_key (
    _id SERIAL PRIMARY KEY,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    opaque_key_id UUID NOT NULL UNIQUE,
    opaque_key BYTEA NOT NULL,
    -- Throttle fields for rate-limiting OTP attempts
    last_failed_attempt TIMESTAMPTZ,
    failed_attempts INTEGER NOT NULL DEFAULT 0
);
