-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

CREATE TEMP TABLE unreferenced_chunks(
    chunk_id BLOB PRIMARY KEY -- UUID
);
