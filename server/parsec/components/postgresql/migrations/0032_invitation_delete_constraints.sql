-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints to simplify removal of invitation
--
-------------------------------------------------------

-- Update greeting_attempt
ALTER TABLE greeting_attempt
DROP CONSTRAINT greeting_attempt_greeting_session_fkey;

ALTER TABLE greeting_attempt
ADD CONSTRAINT greeting_attempt_greeting_session_fkey FOREIGN KEY (greeting_session) REFERENCES greeting_session (
    _id
) ON DELETE CASCADE;


-- Update greeting_session
ALTER TABLE greeting_session
DROP CONSTRAINT greeting_session_invitation_fkey;

ALTER TABLE greeting_session

ADD CONSTRAINT greeting_session_invitation_fkey FOREIGN KEY (invitation) REFERENCES invitation (_id) ON DELETE CASCADE;


-- Update greeting_step
ALTER TABLE greeting_step
DROP CONSTRAINT greeting_step_greeting_attempt_fkey;

ALTER TABLE greeting_step
ADD CONSTRAINT greeting_step_greeting_attempt_fkey FOREIGN KEY (greeting_attempt) REFERENCES greeting_attempt (
    _id
) ON DELETE CASCADE;


-- Update invitation
ALTER TABLE invitation
DROP CONSTRAINT invitation_organization_fkey;

ALTER TABLE invitation
ADD CONSTRAINT invitation_organization_fkey FOREIGN KEY (organization) REFERENCES organization (_id) ON DELETE CASCADE;
