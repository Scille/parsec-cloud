-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

ALTER TABLE organization ADD outsider_enabled BOOLEAN NOT NULL;
UPDATE organization SET outsider_enabled = False
