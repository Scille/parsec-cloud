-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS realm_checkpoint (
    _id INTEGER PRIMARY KEY NOT NULL,
    checkpoint INTEGER NOT NULL
) STRICT;
