-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints to simplify removal of totp opaque keys
--
-------------------------------------------------------

ALTER TABLE totp_opaque_key DROP CONSTRAINT totp_opaque_key_user__fkey;

ALTER TABLE totp_opaque_key
ADD CONSTRAINT totp_opaque_key_user_fkey FOREIGN KEY (user_) REFERENCES user_ (_id) ON DELETE CASCADE;
