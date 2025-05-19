-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- See RFC 1017: add allowed client agent as a per-organization configuration
-------------------------------------------------------

CREATE TYPE allowed_client_agent AS ENUM ('NATIVE_ONLY', 'NATIVE_OR_WEB');

ALTER TABLE organization ADD allowed_client_agent ALLOWED_CLIENT_AGENT;
UPDATE organization SET allowed_client_agent = 'NATIVE_OR_WEB';
ALTER TABLE organization ALTER COLUMN allowed_client_agent SET NOT NULL;
