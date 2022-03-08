-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

-- NULL if not limit
ALTER TABLE organization ADD active_users_limit INTEGER;
