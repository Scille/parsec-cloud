-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------
ALTER TABLE organization ADD sequester_authority_certificate BYTEA;
ALTER TABLE organization ADD sequester_authority_verify_key_der BYTEA;


CREATE TABLE sequester_service(
    _id SERIAL PRIMARY KEY,
    service_id UUID NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    service_certificate BYTEA NOT NULL,
    service_label VARCHAR(254) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    disabled_on TIMESTAMPTZ, -- NULL if not disabled

    UNIQUE(organization, service_id)
);


CREATE TABLE sequester_service_vlob_atom(
    _id SERIAL PRIMARY KEY,
    vlob_atom INTEGER REFERENCES vlob_atom (_id) NOT NULL,
    service INTEGER REFERENCES sequester_service (_id) NOT NULL,
    blob BYTEA NOT NULL
);
