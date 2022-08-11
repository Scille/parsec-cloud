-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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

ALTER TABLE organization ADD _bootstrapped_on TIMESTAMPTZ;
UPDATE organization SET _bootstrapped_on = (
    CASE WHEN root_verify_key IS NOT NULL THEN
        -- If bootstrapped, use creation date of the first user
        (SELECT user_.created_on FROM user_ WHERE user_.organization = organization._id AND user_.user_certifier IS NULL)
    ELSE
        -- Else fallback by using the expired date if any
        organization._expired_on
    END
);

ALTER TABLE organization ADD _created_on TIMESTAMPTZ;
-- _created_on is mandatory, so fallback to current date
UPDATE organization SET _created_on = LEAST(_bootstrapped_on, NOW());
ALTER TABLE organization ALTER COLUMN _created_on SET NOT NULL;

-- This UPDATE shouldn't be needed in theory, but just to be sure...
UPDATE user_ SET profile = 'STANDARD' WHERE profile IS NULL;
-- Add missing constraint on user_.profile
ALTER TABLE user_ ALTER COLUMN profile SET NOT NULL;
