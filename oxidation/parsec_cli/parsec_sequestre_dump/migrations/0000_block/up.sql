-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS block (
    -- _id is not SERIAL given we will take the one present in the Parsec database
    _id INTEGER PRIMARY KEY,
    block_id BLOB NOT NULL,
    data BLOB NOT NULL,
    -- TODO: are author/size/created_on useful ?
    author INTEGER REFERENCES device (_id) NOT NULL,
    size INTEGER NOT NULL,
    -- this field is created by the backend when inserting (unlike vlob's timestamp, see below)
    created_on REAL NOT NULL,

    UNIQUE(block_id)
);
