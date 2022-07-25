-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------
ALTER TABLE organization ADD sequester_authority BYTEA;


CREATE TABLE sequester_service(
    _id SERIAL PRIMARY KEY,
    service_id UUID NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    service_certificate BYTEA NOT NULL,
    service_label VARCHAR(254) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    disabled_on TIMESTAMPTZ, -- NULL if not disabled
    webhook_url TEXT, -- NULL if service is not a WEBHOOK

    UNIQUE(organization, service_id)
);


CREATE TABLE sequester_service_vlob(
    _id SERIAL PRIMARY KEY,
    vlob INTEGER REFERENCES vlob_atom (_id) NOT NULL,
    service INTEGER REFERENCES sequester_service (_id) NOT NULL,
    blob BYTEA NOT NULL
);
