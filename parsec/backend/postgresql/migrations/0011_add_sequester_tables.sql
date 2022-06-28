-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------
ALTER TABLE organization ADD sequester_authority_certificate BYTEA;


CREATE TABLE sequester_service(
    _id SERIAL PRIMARY KEY,
    service_id UUID NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    encryption_key_certificate BYTEA NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    deleted_on TIMESTAMPTZ, -- NULL if not deleted
    webhook_url TEXT, -- NULL if service is not a WEBHOOK

    UNIQUE(organization, service_id)
);