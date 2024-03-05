-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------

-- Add frozen column, defaulting to False
ALTER TABLE user_ ADD frozen BOOLEAN NOT NULL DEFAULT False;
