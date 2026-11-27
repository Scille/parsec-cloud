-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-- Data patch executed right after migration 0040 is applied: ensure the
-- organization inserted by `0040_before.sql` (flagged as expired with
-- `_expired_on` not set) has been backfilled by the migration.

DO $$
DECLARE
    expired_on TIMESTAMPTZ;
BEGIN
    SELECT _expired_on
    INTO expired_on
    FROM organization
    WHERE organization_id = 'Migration0040Org';

    ASSERT expired_on IS NOT NULL, 'Backfill did not set _expired_on on the flagged organization';
END $$;
