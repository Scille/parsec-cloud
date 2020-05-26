-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

ALTER TABLE user_ ADD redacted_user_certificate BYTEA;
UPDATE user_ SET redacted_user_certificate = user_certificate;
ALTER TABLE user_ ALTER COLUMN redacted_user_certificate SET NOT NULL;

CREATE TYPE user_profile AS ENUM ('ADMIN', 'REGULAR', 'OUTSIDER');
ALTER TABLE user_ ADD profile user_profile;
UPDATE user_ SET profile = (CASE WHEN is_admin IS TRUE THEN 'ADMIN' ELSE 'REGULAR' END)::user_profile;
ALTER TABLE user_ DROP COLUMN is_admin;

ALTER TABLE device ADD redacted_device_certificate BYTEA;
UPDATE device SET redacted_device_certificate = device_certificate;
ALTER TABLE device ALTER COLUMN redacted_device_certificate SET NOT NULL;
