-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints to simplify removal of block
--
-------------------------------------------------------

-- Update block's block_realm link
ALTER TABLE ONLY block
DROP CONSTRAINT block_realm_fkey;

ALTER TABLE ONLY block
ADD CONSTRAINT block_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;
