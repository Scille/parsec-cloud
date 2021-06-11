-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

ALTER TABLE organization ADD user_profile_outsider_allowed BOOLEAN NOT NULL;
UPDATE organization SET user_profile_outsider_allowed = False
