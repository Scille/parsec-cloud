-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- store multiple accesses for any given user/realm_keys_bundle pair.
-- Add `from_sharing` column to `realm_keys_bundle_access` table.
-------------------------------------------------------

ALTER TABLE realm_keys_bundle_access DROP CONSTRAINT realm_keys_bundle_access_realm_user__realm_keys_bundle_key;
ALTER TABLE realm_keys_bundle_access ADD COLUMN from_sharing INTEGER REFERENCES realm_user_role (_id);

CREATE UNIQUE INDEX realm_keys_bundle_access_index_from_sharing_not_null
ON realm_keys_bundle_access (
    realm, user_, realm_keys_bundle, from_sharing
) WHERE from_sharing IS NOT NULL;
CREATE UNIQUE INDEX realm_keys_bundle_access_index_from_sharing_null
ON realm_keys_bundle_access (
    realm, user_, realm_keys_bundle
) WHERE from_sharing IS NULL;
