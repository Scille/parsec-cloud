-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

ALTER TABLE organization ADD allow_outsider_profile BOOLEAN NOT NULL;
UPDATE organization SET allow_outsider_profile = False
