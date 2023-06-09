-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS certificates (
    certificate_index INTEGER PRIMARY KEY NOT NULL,
    certificate_timestamp DATETIME NOT NULL,
    certificate BLOB NOT NULL,
    -- We want to have a way to retrieve a singe certificate without having to iterate,
    -- decrypt and deserialize all of them.
    -- However this is tricky given we don't want to make this table dependent on the
    -- types of certificates, otherwise a migration would be required everytime we
    -- introduce a new type of certificate :(
    --
    -- Hence:
    -- - `certificate_type` is not a field with an enum type
    -- - we expose two arbitrary query fields that value depend on the actual certificate stored.
    --
    -- The format is:
    -- - User certificates: filter1=<user_id>, filter2=NULL
    -- - Revoked user certificates: filter1=<user_id>, filter2=NULL
    -- - User update certificates: filter1=<user_id>, filter2=NULL
    -- - Realm role certificate: filter1=<realm id as hex>, filter2=<user_id>
    -- - Device certificate: filter1=<device_name>, filter2=<user_id>
    -- - sequester service certificate: filter1=<service_id>, filter2=NULL
    -- - Sequester authority: filter1=NULL, filter2=NULL (nothing to filter)
    certificate_type TEXT NOT NULL,
    filter1 TEXT,
    filter2 TEXT
);

CREATE INDEX IF NOT EXISTS certificates_filter1 ON certificates (certificate_type, filter1);
CREATE INDEX IF NOT EXISTS certificates_filter2 ON certificates (certificate_type, filter2);
