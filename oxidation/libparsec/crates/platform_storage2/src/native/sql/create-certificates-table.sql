-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS certificates (
    index_ INTEGER PRIMARY KEY NOT NULL,
    certificate BLOB NOT NULL,
    -- We want to have a way to retreive a singe certificate without having to iterate,
    -- decrypt and deserialize all of them.
    -- However this is tricky given we don't want to make this table dependent on the
    -- types of certificates, otherwise a migration would be required everytime we
    -- introduce a new type of certificate :(
    --
    -- Hence:
    -- - `certificate_type` field is not an enum
    -- - `hint` field contains a serialization the fields that we want to query using SQL LIKE
    --
    -- The format for `hint` is "<field_name>:<field_value> <field_name>:<field_value>..."
    -- For instance:
    -- - User & revoked user certificates: "user_id:2199bb7d21ec4988825db6bcf9d7a43e"
    -- - Realm role certficate: "user_id:2199bb7d21ec4988825db6bcf9d7a43e realm_id:dcf41c521cae4682a4cf29302e2af1b6"
    -- - Device certificate: "user_id:2199bb7d21ec4988825db6bcf9d7a43e device_name:78c339d140664e909961c05b4d9add4c"
    -- - Sequester authority & sequester service certificate: "" (nothing to index)
    certificate_type TEXT NOT NULL,
    hint TEXT NOT NULL
)
