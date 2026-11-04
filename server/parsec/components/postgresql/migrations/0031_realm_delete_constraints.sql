-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints to simplify removal of realm
--
-------------------------------------------------------

-- Update realm_archiving
ALTER TABLE realm_archiving
DROP CONSTRAINT realm_archiving_realm_fkey;

ALTER TABLE realm_archiving
ADD CONSTRAINT realm_archiving_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;


-- Update realm_keys_bundle_access
ALTER TABLE realm_keys_bundle_access
DROP CONSTRAINT realm_keys_bundle_access_realm_fkey;

ALTER TABLE realm_keys_bundle_access
ADD CONSTRAINT realm_keys_bundle_access_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;


-- Update realm_keys_bundle_access
ALTER TABLE realm_keys_bundle_access
DROP CONSTRAINT realm_keys_bundle_access_realm_keys_bundle_fkey;

ALTER TABLE realm_keys_bundle_access
ADD CONSTRAINT realm_keys_bundle_access_realm_keys_bundle_fkey FOREIGN KEY (
    realm_keys_bundle
) REFERENCES realm_keys_bundle (_id) ON DELETE CASCADE;


-- Update realm_keys_bundle
ALTER TABLE realm_keys_bundle
DROP CONSTRAINT realm_keys_bundle_realm_fkey;

ALTER TABLE realm_keys_bundle
ADD CONSTRAINT realm_keys_bundle_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;


-- Update realm_name
ALTER TABLE realm_name
DROP CONSTRAINT realm_name_realm_fkey;

ALTER TABLE realm_name
ADD CONSTRAINT realm_name_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;


-- Update realm
ALTER TABLE realm
DROP CONSTRAINT realm_organization_fkey;

ALTER TABLE realm
ADD CONSTRAINT realm_organization_fkey FOREIGN KEY (organization) REFERENCES organization (_id) ON DELETE CASCADE;


-- Update realm_sequester_keys_bundle_access
ALTER TABLE realm_sequester_keys_bundle_access
DROP CONSTRAINT realm_sequester_keys_bundle_access_realm_fkey;

ALTER TABLE realm_sequester_keys_bundle_access
ADD CONSTRAINT realm_sequester_keys_bundle_access_realm_fkey FOREIGN KEY (realm) REFERENCES realm (
    _id
) ON DELETE CASCADE;


-- Update realm_sequester_keys_bundle_access
ALTER TABLE realm_sequester_keys_bundle_access
DROP CONSTRAINT realm_sequester_keys_bundle_access_realm_keys_bundle_fkey;

ALTER TABLE realm_sequester_keys_bundle_access
ADD CONSTRAINT realm_sequester_keys_bundle_access_realm_keys_bundle_fkey FOREIGN KEY (
    realm_keys_bundle
) REFERENCES realm_keys_bundle (_id) ON DELETE CASCADE;


-- Update realm_user_role
ALTER TABLE realm_user_role
DROP CONSTRAINT realm_user_role_realm_fkey;

ALTER TABLE realm_user_role
ADD CONSTRAINT realm_user_role_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;

-- Update realm_topic

ALTER TABLE realm_topic
DROP CONSTRAINT realm_topic_realm_fkey;

ALTER TABLE realm_topic
ADD CONSTRAINT realm_topic_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;
