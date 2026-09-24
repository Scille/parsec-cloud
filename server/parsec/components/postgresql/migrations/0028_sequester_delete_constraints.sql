-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints to simplify removal of sequester
--
-------------------------------------------------------

-- Update sequester_service's organization reference constraint
ALTER TABLE sequester_service
DROP CONSTRAINT sequester_service_organization_fkey;

ALTER TABLE sequester_service
ADD CONSTRAINT sequester_service_organization_fkey
FOREIGN KEY (organization) REFERENCES organization (_id)
ON DELETE CASCADE;

-- Update sequester_topic organization's reference constraint
ALTER TABLE sequester_topic
DROP CONSTRAINT sequester_topic_organization_fkey;

ALTER TABLE sequester_topic
ADD CONSTRAINT sequester_topic_organization_fkey
FOREIGN KEY (organization) REFERENCES organization (_id)
ON DELETE CASCADE;

-- Update realm_sequester_keys_bundle_access's sequester_service reference constraint
ALTER TABLE realm_sequester_keys_bundle_access
DROP CONSTRAINT realm_sequester_keys_bundle_access_sequester_service_fkey;

ALTER TABLE realm_sequester_keys_bundle_access
ADD CONSTRAINT realm_sequester_keys_bundle_access_sequester_service_fkey
FOREIGN KEY (sequester_service) REFERENCES sequester_service (_id) ON
DELETE CASCADE;
