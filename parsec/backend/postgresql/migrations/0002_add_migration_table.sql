-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------


CREATE TABLE migration (
    _id INTEGER PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    applied TIMESTAMPTZ NOT NULL
);
