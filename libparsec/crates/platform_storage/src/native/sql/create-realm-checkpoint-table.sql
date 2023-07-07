-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

CREATE TABLE IF NOT EXISTS realm_checkpoint (
    _id INTEGER PRIMARY KEY NOT NULL,
    checkpoint INTEGER NOT NULL
);
