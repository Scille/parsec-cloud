-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Rename `minimum_archiving_period` to `realm_minimum_archiving_period_before_deletion`
-- in the `organization` table
-------------------------------------------------------
ALTER TABLE organization
RENAME COLUMN minimum_archiving_period TO realm_minimum_archiving_period_before_deletion;
