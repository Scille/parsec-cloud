-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Sequester service has no certifier (since it is always
-- signed by the sequester authority).
-------------------------------------------------------

ALTER TABLE sequester_service RENAME COLUMN revoked_sequester_certificate TO sequester_revoked_service_certificate;
ALTER TABLE sequester_service DROP COLUMN revoked_sequester_certifier CASCADE;
-- No need for default value for this new column we never used this table so far !
ALTER TABLE realm_sequester_keys_bundle_access ADD COLUMN realm INTEGER REFERENCES realm (_id) NOT NULL;
