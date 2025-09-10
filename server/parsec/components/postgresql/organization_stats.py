# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    UserProfile,
)
from parsec.components.organization import (
    OrganizationStats,
    OrganizationStatsBadOutcome,
    OrganizationStatsProfileDetailItem,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_organization_internal_id,
)

# Note the `profile::text` casting here, this is a limitation of asyncpg which doesn't support
# enum within an anonymous record (see https://github.com/MagicStack/asyncpg/issues/360)
_q_get_stats = Q(
    f"""
SELECT
    (
        SELECT COALESCE(_created_on <= $at, FALSE)
        FROM organization
        WHERE organization_id = $organization_id
    ) AS organization_found,
    (
        SELECT ARRAY(
            SELECT (
                revoked_on,
                COALESCE(
                    (
                        SELECT profile.profile::TEXT
                        FROM profile
                        WHERE profile.user_ = user_._id
                        ORDER BY profile.certified_on DESC LIMIT 1
                    ),
                    initial_profile::TEXT
                )
            )
            FROM user_
            WHERE
                organization = {q_organization_internal_id("$organization_id")}  -- noqa: LT05,LT14
                AND created_on <= $at
        )
    ) AS organization_users,
    (
        SELECT COUNT(DISTINCT realm._id)
        FROM realm
        LEFT JOIN realm_user_role ON realm._id = realm_user_role.realm
        WHERE
            realm.organization = {q_organization_internal_id("$organization_id")}  -- noqa: LT05,LT14
            AND realm_user_role.certified_on <= $at
    ) AS organization_realms,
    (
        SELECT COALESCE(SUM(vlob_atom.size), 0)
        FROM vlob_atom
        LEFT JOIN realm ON vlob_atom.realm = realm._id
        WHERE
            realm.organization = {q_organization_internal_id("$organization_id")}  -- noqa: LT05,LT14
            AND vlob_atom.created_on <= $at
            AND (vlob_atom.deleted_on IS NULL OR vlob_atom.deleted_on > $at)
    ) AS organization_metadata_size,
    (
        SELECT COALESCE(SUM(block.size), 0)
        FROM block
        LEFT JOIN realm ON block.realm = realm._id
        WHERE
            realm.organization = {q_organization_internal_id("$organization_id")}  -- noqa: LT05,LT14
            AND block.created_on <= $at
            AND (block.deleted_on IS NULL OR block.deleted_on > $at)
    ) AS organization_data_size
"""
)

_q_get_organizations = Q("""
SELECT organization_id AS id
FROM organization
ORDER BY id
""")


async def _get_organization_stats(
    connection: AsyncpgConnection,
    organization: OrganizationID,
    at: DateTime | None = None,
) -> OrganizationStats | OrganizationStatsBadOutcome:
    at = at or DateTime.now()
    row = await connection.fetchrow(*_q_get_stats(organization_id=organization.str, at=at))
    assert row is not None

    match row["organization_found"]:
        case True:
            pass
        # `None` if the orga doesn't exist, `False` if the orga exists but is too recent
        case False | None:
            return OrganizationStatsBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, row

    users = 0
    active_users = 0
    users_per_profile = {profile: {"active": 0, "revoked": 0} for profile in UserProfile.VALUES}
    match row["organization_users"]:
        case list() as raw_users:
            pass
        case _:
            assert False, row

    for u in raw_users:
        revoked_on, profile = u
        users += 1
        user_profile = UserProfile.from_str(profile)
        if revoked_on:
            users_per_profile[user_profile]["revoked"] += 1
        else:
            active_users += 1
            users_per_profile[user_profile]["active"] += 1

    users_per_profile_detail = tuple(
        OrganizationStatsProfileDetailItem(profile=profile, **data)
        for profile, data in users_per_profile.items()
    )

    match row["organization_data_size"]:
        case int() as data_size:
            pass
        case _:
            assert False, row

    match row["organization_metadata_size"]:
        case int() as metadata_size:
            pass
        case _:
            assert False, row

    match row["organization_realms"]:
        case int() as realms:
            pass
        case _:
            assert False, row

    return OrganizationStats(
        data_size=data_size,
        metadata_size=metadata_size,
        realms=realms,
        users=users,
        active_users=active_users,
        users_per_profile_detail=users_per_profile_detail,
    )


async def organization_stats(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
) -> OrganizationStats | OrganizationStatsBadOutcome:
    return await _get_organization_stats(conn, organization_id)


async def organization_server_stats(
    conn: AsyncpgConnection, at: DateTime | None = None
) -> dict[OrganizationID, OrganizationStats]:
    results: dict[OrganizationID, OrganizationStats] = {}
    for org in await conn.fetch(*_q_get_organizations()):
        org_id = OrganizationID(org["id"])
        org_stats = await _get_organization_stats(conn, org_id, at=at)
        match org_stats:
            case OrganizationStats() as org_stats:
                results[org_id] = org_stats
            # The organization didn't exist at `at` time
            case OrganizationStatsBadOutcome.ORGANIZATION_NOT_FOUND:
                pass
    return results
