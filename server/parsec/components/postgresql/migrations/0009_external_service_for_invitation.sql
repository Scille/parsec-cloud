-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Rename `created_by` to `created_by_device` in invitation table
-- Remove NOT NULL constraint on `created_by_device` column
-- Add `created_by_service_label` column to invitation table
-- Add `claimer_user_id` column to invitation table
--
-------------------------------------------------------

ALTER TABLE invitation RENAME COLUMN created_by TO created_by_device;
ALTER TABLE invitation ALTER COLUMN created_by_device DROP NOT NULL;
ALTER TABLE invitation RENAME CONSTRAINT "invitation_created_by_fkey" TO "invitation_created_by_device_fkey";
ALTER TABLE invitation ADD COLUMN created_by_service_label VARCHAR(254);
ALTER TABLE invitation ADD COLUMN claimer_user_id INTEGER REFERENCES user_ (_id);

-- Update `claimer_user_id` using the `created_by_device` column
UPDATE invitation
SET claimer_user_id = user_._id
FROM device
INNER JOIN user_ ON device.user_ = user_._id
WHERE
    invitation.type = 'DEVICE'
    AND invitation.created_by_device = device._id;
