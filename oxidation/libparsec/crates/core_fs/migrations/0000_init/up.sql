-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chunks (
    chunk_id BLOB PRIMARY KEY NOT NULL, -- UUID
    size INTEGER NOT NULL,
    offline BOOLEAN NOT NULL,
    accessed_on REAL, -- Timestamp
    data BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS vlobs (
    vlob_id BLOB PRIMARY KEY NOT NULL, -- UUID
    base_version INTEGER NOT NULL,
    remote_version INTEGER NOT NULL,
    need_sync BOOLEAN NOT NULL,
    blob BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS realm_checkpoint (
    _id INTEGER PRIMARY KEY NOT NULL,
    checkpoint INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS prevent_sync_pattern (
    _id INTEGER PRIMARY KEY NOT NULL,
    pattern TEXT NOT NULL,
    fully_applied BOOLEAN NOT NULL
);
