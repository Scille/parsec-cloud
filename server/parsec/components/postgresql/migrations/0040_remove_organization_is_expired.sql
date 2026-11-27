-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
--  Remove the `is_expired` column from the `organization` table.
--  An organization is now considered expired if `_expired_on` is not
--  NULL and the date is due.
-------------------------------------------------------

-- Defensive backfill: `_expired_on` should always be set when `is_expired` is
-- true, but this is not guaranteed for databases manipulated by older
-- versions / admin-sys.
UPDATE organization
SET _expired_on = NOW()
WHERE is_expired AND _expired_on IS NULL;

ALTER TABLE organization DROP COLUMN is_expired;
