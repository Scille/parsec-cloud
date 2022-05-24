-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS vlobs (
    vlob_id BLOB PRIMARY KEY NOT NULL, -- UUID
    base_version INTEGER NOT NULL,
    remote_version INTEGER NOT NULL,
    need_sync BOOLEAN NOT NULL,
    blob BLOB NOT NULL
);
