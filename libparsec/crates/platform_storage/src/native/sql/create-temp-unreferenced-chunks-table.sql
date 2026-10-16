-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

CREATE TEMP TABLE unreferenced_chunks (
    chunk_id BLOB PRIMARY KEY -- UUID
) STRICT;
