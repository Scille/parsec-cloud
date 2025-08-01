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
    TosLocale,
    TosUrl,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.config import AccountVaultStrategy, AllowedClientAgent

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
        minimum_archiving_period,
        tos_updated_on,
        tos_per_locale_urls,
        allowed_client_agent,
        account_vault_strategy
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
        $minimum_archiving_period,
        CASE
            WHEN $tos_per_locale_urls::JSON IS NULL
                THEN NULL::TIMESTAMPTZ
            ELSE $created_on
        END,
        $tos_per_locale_urls,
        $allowed_client_agent,
        $account_vault_strategy
    )
    -- If the organization exists but hasn't been bootstrapped yet, we can
    -- simply overwrite it.
    ON CONFLICT (organization_id) DO UPDATE
        SET
            bootstrap_token = excluded.bootstrap_token,
            active_users_limit = excluded.active_users_limit,
            user_profile_outsider_allowed = excluded.user_profile_outsider_allowed,
            _created_on = excluded._created_on,
            is_expired = excluded.is_expired,
            _expired_on = excluded._expired_on,
            minimum_archiving_period = excluded.minimum_archiving_period,
            tos_updated_on = excluded.tos_updated_on,
            tos_per_locale_urls = excluded.tos_per_locale_urls,
            allowed_client_agent = excluded.allowed_client_agent,
            account_vault_strategy = excluded.account_vault_strategy
        WHERE organization.root_verify_key IS NULL
    RETURNING _id
),

new_common_topic AS (  -- noqa: ST03
    -- New organization's `common` topic must be added in the database since it is locked
    -- during organization bootstrap to handle concurrency.
    INSERT INTO common_topic (
        organization,
        last_timestamp
    )
    -- Note the EPOCH (i.e. 1970-01-01T00:00:00Z) here!
    -- This is because the `common` topic currently has no certificates
    -- Notes:
    -- - We don't use `NULL` here given this situation is very temporary (i.e. until the
    --   organization is bootstrapped), and in the meantime the field is never read.
    -- - We don't use `$created_on` here since a created-but-not-bootstrapped organization
    --   can be overwritten, and in this case the `common` topic would also have to be
    --   updated (which is not the case when using EPOCH).
    SELECT
        _id AS organization,
        'epoch' AS last_timestamp
    FROM new_organization
    ON CONFLICT (organization) DO NOTHING
)

SELECT new_organization._id
FROM new_organization
"""
)


async def organization_create(
    conn: AsyncpgConnection,
    now: DateTime,
    id: OrganizationID,
    active_users_limit: ActiveUsersLimit,
    user_profile_outsider_allowed: bool,
    minimum_archiving_period: int,
    tos_per_locale_urls: dict[TosLocale, TosUrl] | None,
    allowed_client_agent: AllowedClientAgent,
    account_vault_strategy: AccountVaultStrategy,
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
            tos_per_locale_urls=tos_per_locale_urls,
            allowed_client_agent=allowed_client_agent.value,
            account_vault_strategy=account_vault_strategy.value,
        )
    )
    match organization_internal_id:
        case int():
            pass
        case None:
            return OrganizationCreateBadOutcome.ORGANIZATION_ALREADY_EXISTS
        case unknown:
            assert False, unknown

    return organization_internal_id
