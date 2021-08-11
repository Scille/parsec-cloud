-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

-- NULL if not limit
ALTER TABLE organization ADD active_users_limit INTEGER;
