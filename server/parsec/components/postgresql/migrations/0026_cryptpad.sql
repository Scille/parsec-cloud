-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Support for CryptPad collaborative editing session
-------------------------------------------------------

CREATE TABLE cryptpad_session (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    document_id UUID NOT NULL,
    author INTEGER REFERENCES device (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    key_index INTEGER NOT NULL,
    encrypted_view_key BYTEA NOT NULL,
    -- NULL if the session has been created by a user without write
    -- access on the realm (hence making the session read-only).
    encrypted_edit_key BYTEA,
    UNIQUE (organization, document_id)
);
