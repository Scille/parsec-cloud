-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

-- Tables used by APIv1 invitation
DROP TABLE user_invitation;
DROP TABLE device_invitation;

ALTER TABLE organization ADD is_expired BOOLEAN;
UPDATE organization SET is_expired = expiration_date IS NOT NULL;
ALTER TABLE organization ALTER COLUMN is_expired SET NOT NULL;

ALTER TABLE organization RENAME COLUMN expiration_date TO _expired_on;

ALTER TABLE organization ADD _created_on TIMESTAMPTZ;
UPDATE organization SET _created_on = (
    CASE WHEN root_verify_key is NULL THEN
        -- If bootsrapped, use creation date of the first user
        (SELECT created_on FROM user_ WHERE user_.organization = organization._id AND user_.user_certifier IS NULL)
    ELSE
        -- Else fallback by using the expired date if any or the current date
        LEAST(organization._expired_on, NOW())
    END
);
ALTER TABLE organization ALTER COLUMN _created_on SET NOT NULL;
