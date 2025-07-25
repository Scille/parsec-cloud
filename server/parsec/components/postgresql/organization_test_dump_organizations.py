# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    ActiveUsersLimit,
    BootstrapToken,
    DateTime,
    OrganizationID,
)
from parsec.components.organization import (
    OrganizationDump,
    TermsOfService,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.config import AccountVaultStrategy, AllowedClientAgent

_q_get_organizations = Q("""
SELECT
    organization_id,
    bootstrap_token,
    (root_verify_key IS NOT NULL) AS is_bootstrapped,
    is_expired,
    active_users_limit,
    user_profile_outsider_allowed,
    minimum_archiving_period,
    tos_updated_on,
    tos_per_locale_urls,
    allowed_client_agent,
    account_vault_strategy
FROM organization
ORDER BY organization_id
""")


async def organization_test_dump_organizations(
    conn: AsyncpgConnection, skip_templates: bool = True
) -> dict[OrganizationID, OrganizationDump]:
    items = {}
    rows = await conn.fetch(*_q_get_organizations())

    for row in rows:
        match row["organization_id"]:
            case str() as raw_organization_id:
                organization_id = OrganizationID(raw_organization_id)
            case _:
                assert False, row

        if skip_templates and organization_id.str.endswith("Template"):
            continue

        match row["bootstrap_token"]:
            case str() as raw_bootstrap_token:
                bootstrap_token = BootstrapToken.from_hex(raw_bootstrap_token)
            case None as bootstrap_token:
                pass
            case _:
                assert False, row

        match row["is_bootstrapped"]:
            case bool() as is_bootstrapped:
                pass
            case _:
                assert False, row

        match row["is_expired"]:
            case bool() as is_expired:
                pass
            case _:
                assert False, row

        match row["active_users_limit"]:
            case None:
                active_users_limit = ActiveUsersLimit.NO_LIMIT
            case int() as raw_active_users_limit:
                active_users_limit = ActiveUsersLimit.limited_to(raw_active_users_limit)
            case _:
                assert False, row

        match row["user_profile_outsider_allowed"]:
            case bool() as user_profile_outsider_allowed:
                pass
            case _:
                assert False, row

        match row["minimum_archiving_period"]:
            case int() as minimum_archiving_period if minimum_archiving_period >= 0:
                pass
            case _:
                assert False, row

        match (row["tos_updated_on"], row["tos_per_locale_urls"]):
            case (None, None):
                tos = None
            case (DateTime() as tos_updated_on, dict() as tos_per_locale_urls):
                for k, v in tos_per_locale_urls.items():
                    assert isinstance(k, str), tos_per_locale_urls
                    assert isinstance(v, str), tos_per_locale_urls
                tos = TermsOfService(updated_on=tos_updated_on, per_locale_urls=tos_per_locale_urls)
            case _:
                assert False, row

        match row["allowed_client_agent"]:
            case str() as allowed_client_agent_raw:
                allowed_client_agent = AllowedClientAgent(allowed_client_agent_raw)
            case _:
                assert False, row

        match row["account_vault_strategy"]:
            case str() as account_vault_strategy_raw:
                account_vault_strategy = AccountVaultStrategy(account_vault_strategy_raw)
            case _:
                assert False, row

        items[organization_id] = OrganizationDump(
            organization_id=organization_id,
            bootstrap_token=bootstrap_token,
            is_bootstrapped=is_bootstrapped,
            is_expired=is_expired,
            active_users_limit=active_users_limit,
            user_profile_outsider_allowed=user_profile_outsider_allowed,
            minimum_archiving_period=minimum_archiving_period,
            tos=tos,
            allowed_client_agent=allowed_client_agent,
            account_vault_strategy=account_vault_strategy,
        )

    return items
