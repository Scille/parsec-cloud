-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- See RFC 1013&1014: add account/vault/authentication methods
-------------------------------------------------------


CREATE TABLE account (
    _id SERIAL PRIMARY KEY,
    email VARCHAR(254) UNIQUE NOT NULL,
    -- Not used by Parsec Account but works as a quality-of-life feature
    -- to allow pre-filling human handle during enrollment.
    human_handle_label VARCHAR(254) NOT NULL,
    -- Account marked as deleted is a special case:
    -- - By definition, a deleted account shouldn't be present in the database...
    -- - ...however it is convenient to leave the actual deletion to a dedicated script run
    --   periodically (this typically simplifies the recovery of an account deleted by mistake).
    -- In a nutshell: an account marked as deleted is always ignored, except when creating
    -- a new account in which case the deleted account is not overwritten but instead restored
    -- (i.e. `deleted_on` is set to NULL, and a new vault & authentication method is inserted).
    --
    -- NULL if not deleted
    deleted_on TIMESTAMPTZ
);


CREATE TABLE vault (
    _id SERIAL PRIMARY KEY,
    account INTEGER REFERENCES account (_id) NOT NULL
);


CREATE TABLE vault_item (
    _id SERIAL PRIMARY KEY,
    vault INTEGER REFERENCES vault (_id) NOT NULL,
    fingerprint BYTEA NOT NULL,
    data BYTEA NOT NULL,
    UNIQUE (vault, fingerprint)
);


CREATE TYPE password_algorithm AS ENUM ('ARGON2ID');


CREATE TABLE vault_authentication_method (
    _id SERIAL PRIMARY KEY,
    auth_method_id UUID UNIQUE NOT NULL,
    vault INTEGER REFERENCES vault (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    -- IP address of the HTTP request that created the authentication method
    -- (either by account creation, vault key rotation or account recovery)
    -- Can be unknown (i.e. empty string) since this information is optional in
    -- ASGI (see
    -- https://asgi.readthedocs.io/en/latest/specs/www.html#http-connection-scope).
    created_by_ip TEXT NOT NULL,
    -- User agent header of the HTTP request that created the vault.
    created_by_user_agent TEXT NOT NULL,
    -- Secret key used for HMAC based authentication with the server
    mac_key BYTEA NOT NULL,
    -- Vault key encrypted with the `auth_method_secret_key` see rfc 1014
    vault_key_access BYTEA NOT NULL,
    -- Auth method can be of two types:
    -- - ClientProvided, for which the client is able to store
    --   `auth_method_master_secret` all by itself.
    -- - Password, for which the client must obtain some configuration
    --   (i.e. this field !) from the server in order to know how
    --   to turn the password into `auth_method_master_secret`.
    -- NULL in case of `ClientProvided`.
    password_algorithm PASSWORD_ALGORITHM,
    -- `password_algorithm_argon2id_*` fields are expected to be:
    -- - NOT NULL if `password_algorithm` == 'ARGON2ID'
    -- - NULL if `password_algorithm` != 'ARGON2ID'
    password_algorithm_argon2id_opslimit INTEGER,
    password_algorithm_argon2id_memlimit_kb INTEGER,
    password_algorithm_argon2id_parallelism INTEGER,
    -- Note that only the current vault contains auth methods that are not disabled
    -- NULL if not disabled
    disabled_on TIMESTAMPTZ
);


CREATE TABLE account_create_validation_code (
    email VARCHAR(256) PRIMARY KEY NOT NULL,
    validation_code VARCHAR(6) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    failed_attempts INTEGER DEFAULT 0
);


CREATE TABLE account_delete_validation_code (
    account INTEGER REFERENCES account (_id) NOT NULL PRIMARY KEY,
    validation_code VARCHAR(6) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    failed_attempts INTEGER DEFAULT 0
);


CREATE TABLE account_recover_validation_code (
    account INTEGER REFERENCES account (_id) NOT NULL PRIMARY KEY,
    validation_code VARCHAR(6) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    failed_attempts INTEGER DEFAULT 0
);
