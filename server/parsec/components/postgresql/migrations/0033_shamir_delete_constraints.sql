-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints to simplify removal of shamir recovery
--
-------------------------------------------------------

ALTER TABLE ONLY shamir_recovery_setup
DROP CONSTRAINT shamir_recovery_setup_organization_fkey;

ALTER TABLE ONLY shamir_recovery_setup
ADD CONSTRAINT shamir_recovery_setup_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE ONLY shamir_recovery_setup
DROP CONSTRAINT shamir_recovery_setup_user__fkey;

ALTER TABLE ONLY shamir_recovery_setup
ADD CONSTRAINT shamir_recovery_setup_user_fkey FOREIGN KEY (user_) REFERENCES user_ (_id) ON DELETE CASCADE;


ALTER TABLE ONLY shamir_recovery_share
DROP CONSTRAINT shamir_recovery_share_organization_fkey;

ALTER TABLE ONLY shamir_recovery_share
ADD CONSTRAINT shamir_recovery_share_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE ONLY shamir_recovery_share
DROP CONSTRAINT shamir_recovery_share_recipient_fkey;

ALTER TABLE ONLY shamir_recovery_share
ADD CONSTRAINT shamir_recovery_share_recipient_fkey FOREIGN KEY (recipient) REFERENCES user_ (_id) ON DELETE CASCADE;


ALTER TABLE ONLY shamir_recovery_share
DROP CONSTRAINT shamir_recovery_share_shamir_recovery_fkey;

ALTER TABLE ONLY shamir_recovery_share
ADD CONSTRAINT shamir_recovery_share_shamir_recovery_fkey FOREIGN KEY (
    shamir_recovery
) REFERENCES shamir_recovery_setup (_id) ON DELETE CASCADE;


ALTER TABLE ONLY shamir_recovery_topic
DROP CONSTRAINT shamir_recovery_topic_organization_fkey;

ALTER TABLE ONLY shamir_recovery_topic
ADD CONSTRAINT shamir_recovery_topic_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE ONLY user_
DROP CONSTRAINT user__shamir_recovery_fkey;

ALTER TABLE ONLY user_
ADD CONSTRAINT user_shamir_recovery_fkey FOREIGN KEY (shamir_recovery) REFERENCES shamir_recovery_setup (
    _id
) ON DELETE CASCADE;


ALTER TABLE ONLY invitation
DROP CONSTRAINT invitation_shamir_recovery_fkey;

ALTER TABLE ONLY invitation
ADD CONSTRAINT invitation_shamir_recovery_fkey FOREIGN KEY (shamir_recovery) REFERENCES shamir_recovery_setup (
    _id
) ON DELETE CASCADE;
