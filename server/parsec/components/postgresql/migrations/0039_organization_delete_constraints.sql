-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Enable cascade deletion on foreign key related to organization
--
-------------------------------------------------------

ALTER TABLE async_enrollment DROP CONSTRAINT async_enrollment_organization_fkey;

ALTER TABLE async_enrollment
ADD CONSTRAINT async_enrollment_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE common_topic DROP CONSTRAINT common_topic_organization_fkey;

ALTER TABLE common_topic
ADD CONSTRAINT common_topic_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE device DROP CONSTRAINT device_organization_fkey;

ALTER TABLE device
ADD CONSTRAINT device_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE greeting_attempt DROP CONSTRAINT greeting_attempt_organization_fkey;

ALTER TABLE greeting_attempt
ADD CONSTRAINT greeting_attempt_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE human DROP CONSTRAINT human_organization_fkey;

ALTER TABLE human
ADD CONSTRAINT human_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE realm_topic DROP CONSTRAINT realm_topic_organization_fkey;

ALTER TABLE realm_topic
ADD CONSTRAINT realm_topic_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


ALTER TABLE user_ DROP CONSTRAINT user__organization_fkey;

ALTER TABLE user_ ADD CONSTRAINT user_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;
