-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS vlob_atom (
    -- Compared to Parsec's datamodel, we don't store `vlob_encryption_revision` given
    -- the vlob is provided with third party encrytion key only once at creation time
    _id INTEGER PRIMARY KEY,
    vlob_id BLOB NOT NULL,
    version INTEGER NOT NULL,
    blob BLOB NOT NULL,
    size INTEGER NOT NULL,
    -- author/timestamp are required to validate the consistency of blob
    -- Care must be taken when exporting this field (and the device_ table) to
    -- keep this relationship valid !
    author INTEGER REFERENCES device (_id) NOT NULL,
    -- this field is called created_on in Parsec datamodel, but it correspond to the timestamp field in the API
    -- (the value is provided by the client when sending request and not created on backend side) so better
    -- give it the better understandable name
    timestamp REAL NOT NULL,

    UNIQUE(vlob_id)
);
