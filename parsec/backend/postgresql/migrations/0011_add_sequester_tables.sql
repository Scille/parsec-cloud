 -- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------
ALTER TABLE organization ADD sequester_verify_key BYTEA;

CREATE TYPE sequester_service AS ENUM ('SEQUESTRE', 'WEBHOOK');

CREATE TABLE sequester(
    _id SERIAL PRIMARY KEY,
    service_type sequester_service NOT NULL,
    service_id TEXT NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    encryption_key_payload BYTEA NOT NULL,
    encryption_key__payload_signature BYTEA NOT NULLS,
    webhook_url TEXT,

    UNIQUE(organization, service_id)
);
