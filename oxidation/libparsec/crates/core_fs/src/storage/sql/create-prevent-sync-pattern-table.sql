-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS prevent_sync_pattern (
    _id INTEGER PRIMARY KEY NOT NULL,
    pattern TEXT NOT NULL,
    fully_applied BOOLEAN NOT NULL
);
