# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    ActiveUsersLimit,
    BootstrapToken,
    OrganizationID,
)
from parsec.components.organization import (
    OrganizationDump,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)

_q_get_organizations = Q("""
SELECT
    organization_id,
    bootstrap_token,
    (root_verify_key IS NOT NULL) AS is_bootstrapped,
    is_expired,
    active_users_limit,
    user_profile_outsider_allowed,
    minimum_archiving_period
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
            case unknown:
                assert False, unknown

        if skip_templates and organization_id.str.endswith("Template"):
            continue

        match row["bootstrap_token"]:
            case str() as raw_bootstrap_token:
                bootstrap_token = BootstrapToken.from_hex(raw_bootstrap_token)
            case None as bootstrap_token:
                pass
            case unknown:
                assert False, unknown

        match row["is_bootstrapped"]:
            case bool() as is_bootstrapped:
                pass
            case unknown:
                assert False, unknown

        match row["is_expired"]:
            case bool() as is_expired:
                pass
            case unknown:
                assert False, unknown

        match row["active_users_limit"]:
            case None:
                active_users_limit = ActiveUsersLimit.NO_LIMIT
            case int() as raw_active_users_limit:
                active_users_limit = ActiveUsersLimit.limited_to(raw_active_users_limit)
            case unknown:
                assert False, unknown

        match row["user_profile_outsider_allowed"]:
            case bool() as user_profile_outsider_allowed:
                pass
            case unknown:
                assert False, unknown

        match row["minimum_archiving_period"]:
            case int() as minimum_archiving_period:
                pass
            case unknown:
                assert False, unknown

        items[organization_id] = OrganizationDump(
            organization_id=organization_id,
            bootstrap_token=bootstrap_token,
            is_bootstrapped=is_bootstrapped,
            is_expired=is_expired,
            active_users_limit=active_users_limit,
            user_profile_outsider_allowed=user_profile_outsider_allowed,
            minimum_archiving_period=minimum_archiving_period,
        )

    return items
