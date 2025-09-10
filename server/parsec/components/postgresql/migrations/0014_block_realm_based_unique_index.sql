-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Block IDs are UUID so they should be globally unique.
-- A unique index on (organization, block ID) was specified, however
-- it is better to specify (realm, block ID) since a block is
-- always considered as part of realm.
--
-- This notably avoid full-scan on the block table during block_create when
-- checking that the requested block ID doesn't already exist in the realm.
--
-- Finally note that since all realm are part of an organization, we are relaxing
-- the unique index (i.e. we allow multiple realm to have the same block ID which
-- was previously forbidden). Hence there is no risk of having data preventing the
-- index change.
-------------------------------------------------------

ALTER TABLE block DROP CONSTRAINT block_organization_block_id_key;
ALTER TABLE block DROP COLUMN organization;
ALTER TABLE block ADD CONSTRAINT block_realm_block_id_key UNIQUE (realm, block_id);
