-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chunks (
    chunk_id BLOB PRIMARY KEY NOT NULL, -- UUID
    size INTEGER NOT NULL,
    offline BOOLEAN NOT NULL,
    accessed_on REAL, -- Timestamp
    data BLOB NOT NULL
);