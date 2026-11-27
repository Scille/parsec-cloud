-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-- Data patch executed right before migration 0040 is applied: insert an
-- organization flagged as expired (`is_expired = TRUE`) but with `_expired_on`
-- not set, to ensure the migration backfill preserves its expiration state.

INSERT INTO organization (
    _id,
    organization_id,
    user_profile_outsider_allowed,
    is_expired,
    _created_on,
    realm_minimum_archiving_period_before_deletion
) VALUES (
    42,
    'Migration0040Org',
    FALSE,
    TRUE,
    NOW(),
    0
);
