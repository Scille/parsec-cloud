 -- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

-- This migration simply creates a new table to keep track of the
-- pki certificate request and replies.


CREATE TABLE pki_certificate (
    _id SERIAL PRIMARY KEY,
    certificate_id UUID NOT NULL,
    request_id UUID NOT NULL,
    request_timestamp TIMESTAMPTZ NOT NULL,
    request_object BYTEA NOT NULL,
    reply_user_id UUID,
    reply_timestamp TIMESTAMPTZ,
    reply_object BYTEA,

    UNIQUE(certificate_id)
);
