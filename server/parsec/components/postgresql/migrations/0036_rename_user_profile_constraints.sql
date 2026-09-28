-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Rename later user & profile constraints
--
-------------------------------------------------------

ALTER TABLE user_
DROP CONSTRAINT fk_user_device_user_certifier;

ALTER TABLE user_
ADD CONSTRAINT user_user_certifier_fkey FOREIGN KEY (user_certifier) REFERENCES device (_id) ON DELETE CASCADE;


ALTER TABLE user_
DROP CONSTRAINT fk_user_device_revoked_user_certifier;

ALTER TABLE user_
ADD CONSTRAINT user_revoked_user_certifier_fkey FOREIGN KEY (revoked_user_certifier) REFERENCES device (
    _id
) ON DELETE CASCADE;


ALTER TABLE profile
DROP CONSTRAINT profile_certified_by_fkey;

ALTER TABLE profile
ADD CONSTRAINT profile_certified_by_fkey FOREIGN KEY (certified_by) REFERENCES device (_id) ON DELETE CASCADE;
