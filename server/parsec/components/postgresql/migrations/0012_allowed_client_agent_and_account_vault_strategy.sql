-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- See RFC 1017: add allowed client agent as a per-organization configuration
-- See RFC 1014: add account vault strategy as a per-organization configuration
-------------------------------------------------------

-- 1) allowed_client_agent

CREATE TYPE allowed_client_agent AS ENUM ('NATIVE_ONLY', 'NATIVE_OR_WEB');

ALTER TABLE organization ADD allowed_client_agent ALLOWED_CLIENT_AGENT;
UPDATE organization SET allowed_client_agent = 'NATIVE_OR_WEB';
ALTER TABLE organization ALTER COLUMN allowed_client_agent SET NOT NULL;

-- 2) account_vault_strategy

CREATE TYPE account_vault_strategy AS ENUM ('ALLOWED', 'FORBIDDEN');

ALTER TABLE organization ADD account_vault_strategy ACCOUNT_VAULT_STRATEGY;
UPDATE organization SET account_vault_strategy = 'ALLOWED';
ALTER TABLE organization ALTER COLUMN account_vault_strategy SET NOT NULL;
