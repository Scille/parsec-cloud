-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add delete constraints for account tables to simplify removal of account
--
-- NOTE: With an inline REFERENCES like that (no explicit CONSTRAINT name given),
-- Postgres auto-generates the name using the pattern: `<table>_<column>_fkey`
--
-------------------------------------------------------


-- Update vault account reference constraint
ALTER TABLE vault
DROP CONSTRAINT vault_account_fkey;

ALTER TABLE vault
ADD CONSTRAINT vault_account_fkey
FOREIGN KEY (account) REFERENCES account (_id)
ON DELETE CASCADE;

-- Update vault_item vault reference constraint
ALTER TABLE vault_item
DROP CONSTRAINT vault_item_vault_fkey;

ALTER TABLE vault_item
ADD CONSTRAINT vault_item_vault_fkey
FOREIGN KEY (vault) REFERENCES vault (_id)
ON DELETE CASCADE;

-- Update vault_authentication_method vault reference constraint
ALTER TABLE vault_authentication_method
DROP CONSTRAINT vault_authentication_method_vault_fkey;

ALTER TABLE vault_authentication_method
ADD CONSTRAINT vault_authentication_method_vault_fkey
FOREIGN KEY (vault) REFERENCES vault (_id)
ON DELETE CASCADE;

-- Update account_delete_validation_code account reference constraint
ALTER TABLE account_delete_validation_code
DROP CONSTRAINT account_delete_validation_code_account_fkey;

ALTER TABLE account_delete_validation_code
ADD CONSTRAINT account_delete_validation_code_account_fkey
FOREIGN KEY (account) REFERENCES account (_id)
ON DELETE CASCADE;

-- Update account_recover_validation_code account reference constraint
ALTER TABLE account_recover_validation_code
DROP CONSTRAINT account_recover_validation_code_account_fkey;

ALTER TABLE account_recover_validation_code
ADD CONSTRAINT account_recover_validation_code_account_fkey
FOREIGN KEY (account) REFERENCES account (_id)
ON DELETE CASCADE;
