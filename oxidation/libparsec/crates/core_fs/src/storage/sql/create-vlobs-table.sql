-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS vlobs (
    vlob_id BLOB PRIMARY KEY NOT NULL, -- UUID
    base_version INTEGER NOT NULL,
    remote_version INTEGER NOT NULL,
    need_sync BOOLEAN NOT NULL,
    blob BLOB NOT NULL
);
