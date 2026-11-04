-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS vlobs (
    vlob_id BLOB PRIMARY KEY NOT NULL, -- UUID
    base_version INTEGER NOT NULL,
    remote_version INTEGER NOT NULL,
    need_sync INTEGER NOT NULL, -- Boolean
    blob BLOB NOT NULL --noqa: references.keywords
) STRICT;
