-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS chunks (
    chunk_id BLOB PRIMARY KEY NOT NULL, -- UUID
    size INTEGER NOT NULL,
    offline BOOLEAN NOT NULL,
    accessed_on REAL, -- UNIX timestamp with microsecond precision
    data BLOB NOT NULL
)
