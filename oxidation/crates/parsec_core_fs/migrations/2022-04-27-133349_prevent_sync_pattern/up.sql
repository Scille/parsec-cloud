-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS prevent_sync_pattern (
    _id INTEGER PRIMARY KEY NOT NULL,
    pattern TEXT NOT NULL,
    fully_applied BOOLEAN NOT NULL
);
