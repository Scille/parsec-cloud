-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------

-- Remove useless underscore prefixes

ALTER TABLE organization
RENAME COLUMN _bootstrapped_on TO bootstrapped_on;

ALTER TABLE organization
RENAME COLUMN _created_on TO created_on;

ALTER TABLE organization
RENAME COLUMN _expired_on TO expired_on;

-- In other tables, `created_on` refers to an information visible to the clients,
-- typically comming from a signed timestamp.
--
-- On the other hand, blocks doesn't have timestamp from the clients point of view,
-- hence this field is useful information for database admin.
--
-- This rename helps stessing that out.
ALTER TABLE block
RENAME COLUMN created_on TO inserted_on;

-- `profile` table contains the user update certificates, hence must be
-- named accordingly

ALTER TABLE profile
RENAME TO user_update;

ALTER TABLE user_update
RENAME COLUMN profile_certificate TO user_update_certificate;

ALTER TABLE user_update
RENAME CONSTRAINT profile_pkey TO user_update_pkey;

ALTER TABLE user_update
RENAME CONSTRAINT profile_certified_by_fkey TO user_update_certified_by_fkey;

ALTER TABLE user_update
RENAME CONSTRAINT profile_user__fkey TO user_update_user__fkey;

ALTER SEQUENCE profile__id_seq
RENAME TO user_update__id_seq;

-- All tables refering to a certificate have a `organization`/`certified_by`/`certified_on`

ALTER TABLE user_update
ADD organization INTEGER REFERENCES organization (_id) NOT NULL;

ALTER TABLE user_update
ADD organization INTEGER REFERENCES organization (_id) NOT NULL;

ALTER TABLE shamir_recovery_setup
ADD certified_by INTEGER NOT NULL;

ALTER TABLE shamir_recovery_setup
ADD certified_on TIMESTAMPTZ NOT NULL;

ALTER TABLE shamir_recovery_share
ADD certified_by INTEGER NOT NULL;

ALTER TABLE shamir_recovery_share
ADD certified_on TIMESTAMPTZ NOT NULL;
