-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints to simplify removal of vlob
--
-------------------------------------------------------

-- Update realm_vlob_update's realm link
ALTER TABLE ONLY realm_vlob_update
DROP CONSTRAINT realm_vlob_update_realm_fkey;

ALTER TABLE ONLY realm_vlob_update
ADD CONSTRAINT realm_vlob_update_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;


-- Update realm_vlob_update's vlob_atom link
ALTER TABLE ONLY realm_vlob_update
DROP CONSTRAINT realm_vlob_update_vlob_atom_fkey;

ALTER TABLE ONLY realm_vlob_update
ADD CONSTRAINT realm_vlob_update_vlob_atom_fkey FOREIGN KEY (vlob_atom) REFERENCES vlob_atom (
    _id
) ON DELETE CASCADE;


-- Update vlob_atom's realm link
ALTER TABLE ONLY vlob_atom
DROP CONSTRAINT vlob_atom_realm_fkey;

ALTER TABLE ONLY vlob_atom
ADD CONSTRAINT vlob_atom_realm_fkey FOREIGN KEY (realm) REFERENCES realm (_id) ON DELETE CASCADE;
