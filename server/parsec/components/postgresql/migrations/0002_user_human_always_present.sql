-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------

ALTER TABLE user_ ALTER COLUMN human SET NOT NULL;
