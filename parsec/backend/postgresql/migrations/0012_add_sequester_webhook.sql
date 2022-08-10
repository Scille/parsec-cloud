-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------
CREATE TYPE sequester_service_type AS ENUM ('STORAGE', 'WEBHOOK');

ALTER TABLE sequester_service ADD webhook_url TEXT; -- NULL if service_type != WEBHOOK;
ALTER TABLE sequester_service ADD service_type sequester_service_type NOT NULL;
