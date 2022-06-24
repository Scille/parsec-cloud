 -- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------
ALTER TABLE organization ADD sequester_verify_key BYTEA;

CREATE TYPE sequester_service_type AS ENUM ('SEQUESTRE', 'WEBHOOK');

CREATE TABLE sequester_service(
    _id SERIAL PRIMARY KEY,
    service_type sequester_service_type NOT NULL,
    service_id UUID NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    encryption_key_certificate BYTEA NOT NULL,
    encryption_key_certificate_signature BYTEA NOT NULL,
    webhook_url TEXT, -- NULL if service_type != WEBHOOK
    created_on TIMESTAMPTZ NOT NULL,
    deleted_on TIMESTAMPTZ, -- NULL if not deleted

    UNIQUE(organization, service_id)
);
