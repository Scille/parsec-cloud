-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS remanence (
    _id INTEGER PRIMARY KEY NOT NULL,
    block_remanent INTEGER NOT NULL -- Boolean
) STRICT;
