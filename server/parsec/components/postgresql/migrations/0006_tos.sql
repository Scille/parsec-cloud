-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
--  Terms of Services (ToS) can now be defined and updated per-organization.
--  Users are requested to accept this ToS on first connection and whenever
--  the ToS are updated. Acceptance date is registered for each user.
-------------------------------------------------------

ALTER TABLE organization ADD tos_updated_on TIMESTAMPTZ;
ALTER TABLE organization ADD tos_per_locale_urls JSON;

ALTER TABLE user_ ADD tos_accepted_on TIMESTAMPTZ;
