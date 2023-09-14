-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------


-- The default value value is used to fill the columns with 2592000 seconds is 30 days
-- It is then dropped as it is only meant to be used during the migration
ALTER TABLE organization ADD minimum_archiving_period INTEGER NOT NULL DEFAULT 2592000;
ALTER TABLE organization ALTER COLUMN minimum_archiving_period DROP DEFAULT;

ALTER TABLE realm_user_change ADD last_archiving_change TIMESTAMPTZ;

CREATE TYPE realm_archiving_configuration AS ENUM ('AVAILABLE', 'ARCHIVED', 'DELETION_PLANNED');

CREATE TABLE realm_archiving (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    configuration realm_archiving_configuration NOT NULL,
    -- NULL if not DELETION_PLANNED
    deletion_date TIMESTAMPTZ,
    certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device(_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL
);
