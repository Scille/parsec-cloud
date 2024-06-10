-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS prevent_sync_pattern (
    _id INTEGER PRIMARY KEY NOT NULL,
    pattern TEXT NOT NULL,
    fully_applied INTEGER NOT NULL -- Boolean
) STRICT;
