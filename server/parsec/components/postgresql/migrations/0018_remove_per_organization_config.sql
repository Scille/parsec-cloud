-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Remove fields `allowed_client_agent/account_vault_strategy`
-- from table `organization` since this configuration is now
-- global to the server (instead of on a per-organization basis).
-------------------------------------------------------

ALTER TABLE organization DROP COLUMN allowed_client_agent;
ALTER TABLE organization DROP COLUMN account_vault_strategy;

DROP TYPE ALLOWED_CLIENT_AGENT;
DROP TYPE ACCOUNT_VAULT_STRATEGY;
