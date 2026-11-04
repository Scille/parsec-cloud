-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints to simplify removal of cryptpad sessions
--
-------------------------------------------------------

-- Update fkey constraint to cryptpad_session's organization
ALTER TABLE cryptpad_session
DROP CONSTRAINT cryptpad_session_organization_fkey;

ALTER TABLE cryptpad_session
ADD CONSTRAINT cryptpad_session_organization_fkey FOREIGN KEY (organization) REFERENCES organization (
    _id
) ON DELETE CASCADE;


-- Update fkey constraint to cryptpad_session's author
ALTER TABLE cryptpad_session DROP CONSTRAINT cryptpad_session_author_fkey;

ALTER TABLE cryptpad_session
ADD CONSTRAINT cryptpad_session_author_fkey FOREIGN KEY (author) REFERENCES device (_id) ON DELETE CASCADE;
