# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    ActiveUsersLimit,
    BootstrapToken,
    DateTime,
    OrganizationID,
)
from parsec.components.organization import (
    OrganizationCreateBadOutcome,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)

_q_insert_organization = Q(
    """
WITH new_organization AS (
    INSERT INTO organization (
        organization_id,
        bootstrap_token,
        active_users_limit,
        user_profile_outsider_allowed,
        _created_on,
        _bootstrapped_on,
        is_expired,
        _expired_on,
        minimum_archiving_period
    )
    VALUES (
        $organization_id,
        $bootstrap_token,
        $active_users_limit,
        $user_profile_outsider_allowed,
        $created_on,
        NULL,
        FALSE,
        NULL,
        $minimum_archiving_period
    )
    -- If the organization exists but hasn't been bootstrapped yet, we can
    -- simply overwrite it.
    ON CONFLICT (organization_id) DO
        UPDATE SET
            bootstrap_token = EXCLUDED.bootstrap_token,
            active_users_limit = EXCLUDED.active_users_limit,
            user_profile_outsider_allowed = EXCLUDED.user_profile_outsider_allowed,
            _created_on = EXCLUDED._created_on,
            is_expired = EXCLUDED.is_expired,
            _expired_on = EXCLUDED._expired_on,
            minimum_archiving_period = EXCLUDED.minimum_archiving_period
        WHERE organization.root_verify_key IS NULL
    RETURNING _id
),
new_common_topic AS (
    -- New organization's `common` topic must be added in the database since it is locked
    -- during organization bootstrap to handle concurrency.
    INSERT INTO common_topic (organization, last_timestamp)
    -- Note the EPOCH (i.e. 1970-01-01T00:00:00Z) here !
    -- This is because the `common` topic currently has no certificates
    -- Notes:
    -- - We don't use `NULL` here given this situation is very temporary (i.e. until the
    --   organization is bootstrapped), and in the meantime the field is never read.
    -- - We don't use `$created_on` here since a created-but-not-bootstrapped organization
    --   can be overwritten, and in this case the `common` topic would also have to be
    --   updated (which is not the case when using EPOCH).
    SELECT _id, 'epoch' FROM new_organization
    ON CONFLICT (organization) DO NOTHING
)
SELECT _id FROM new_organization
"""
)


async def organization_create(
    conn: AsyncpgConnection,
    now: DateTime,
    id: OrganizationID,
    active_users_limit: ActiveUsersLimit,
    user_profile_outsider_allowed: bool,
    minimum_archiving_period: int,
    bootstrap_token: BootstrapToken | None,
) -> int | OrganizationCreateBadOutcome:
    organization_internal_id = await conn.fetchval(
        *_q_insert_organization(
            organization_id=id.str,
            bootstrap_token=None if bootstrap_token is None else bootstrap_token.hex,
            active_users_limit=active_users_limit
            if active_users_limit is not ActiveUsersLimit.NO_LIMIT
            else None,
            user_profile_outsider_allowed=user_profile_outsider_allowed,
            created_on=now,
            minimum_archiving_period=minimum_archiving_period,
        )
    )
    match organization_internal_id:
        case int():
            pass
        case None:
            return OrganizationCreateBadOutcome.ORGANIZATION_ALREADY_EXISTS
        case unknown:
            assert False, repr(unknown)

    return organization_internal_id
