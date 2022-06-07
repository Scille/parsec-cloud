 -- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------
ALTER TABLE organization ADD tpek_verify_key BYTEA;

CREATE TYPE tpek_service AS ENUM ('SEQUESTRE', 'WEBHOOK');

CREATE TABLE tpek(
    _id SERIAL PRIMARY KEY,
    service_type tpek_service NOT NULL,
    service_id TEXT NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    encryption_key BYTEA,  -- Encryption key signed by the certificate
    webhook_url TEXT, -- NULL if service_type != WEBHOOK

    UNIQUE(organization, service_id)
);
