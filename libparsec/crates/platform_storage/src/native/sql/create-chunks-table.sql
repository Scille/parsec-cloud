-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS chunks (
    chunk_id BLOB PRIMARY KEY NOT NULL, -- UUID
    size INTEGER NOT NULL,
    offline INTEGER NOT NULL, -- Boolean
    accessed_on INTEGER, -- UNIX timestamp with microsecond precision
    data BLOB NOT NULL
) STRICT;
