-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Rename later user & profile constraints
-- Enable cascade deletion on foreign key related to devices
--
-------------------------------------------------------

ALTER TABLE block DROP CONSTRAINT block_author_fkey;

ALTER TABLE block
ADD CONSTRAINT block_author_fkey FOREIGN KEY (author) REFERENCES device (_id) ON DELETE CASCADE;


ALTER TABLE device DROP CONSTRAINT device_device_certifier_fkey;

ALTER TABLE device
ADD CONSTRAINT device_device_certifier_fkey FOREIGN KEY (device_certifier) REFERENCES device (
    _id
) ON DELETE CASCADE;


ALTER TABLE invitation DROP CONSTRAINT invitation_created_by_device_fkey;

ALTER TABLE invitation
ADD CONSTRAINT invitation_created_by_device_fkey FOREIGN KEY (created_by_device) REFERENCES device (
    _id
) ON DELETE CASCADE;


ALTER TABLE realm_archiving DROP CONSTRAINT realm_archiving_certified_by_fkey;

ALTER TABLE realm_archiving
ADD CONSTRAINT realm_archiving_certified_by_fkey FOREIGN KEY (certified_by) REFERENCES device (
    _id
) ON DELETE CASCADE;


ALTER TABLE realm_keys_bundle DROP CONSTRAINT realm_keys_bundle_certified_by_fkey;

ALTER TABLE realm_keys_bundle
ADD CONSTRAINT realm_keys_bundle_certified_by_fkey FOREIGN KEY (certified_by) REFERENCES device (
    _id
) ON DELETE CASCADE;


ALTER TABLE realm_name DROP CONSTRAINT realm_name_certified_by_fkey;

ALTER TABLE realm_name
ADD CONSTRAINT realm_name_certified_by_fkey FOREIGN KEY (certified_by) REFERENCES device (_id) ON DELETE CASCADE;


ALTER TABLE realm_user_role DROP CONSTRAINT realm_user_role_certified_by_fkey;

ALTER TABLE realm_user_role
ADD CONSTRAINT realm_user_role_certified_by_fkey FOREIGN KEY (certified_by) REFERENCES device (
    _id
) ON DELETE CASCADE;
