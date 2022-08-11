-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------

ALTER TABLE organization ADD user_profile_outsider_allowed BOOLEAN;
UPDATE organization SET user_profile_outsider_allowed = FALSE;
ALTER TABLE organization ALTER COLUMN user_profile_outsider_allowed SET NOT NULL;
