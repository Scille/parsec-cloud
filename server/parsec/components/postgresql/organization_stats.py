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
)

# Note the `profile::text` casting here, this is a limitation of asyncpg which doesn't support
# enum within an anonymous record (see https://github.com/MagicStack/asyncpg/issues/360)
_q_get_stats = Q(
    """
WITH my_org AS (
    SELECT _id
    FROM organization
    WHERE
        organization_id = $organization_id
        AND _created_on <= $at
),

-- We cannot use `current_profile` column from `user_` since in this query we
-- want the last profile that is *older* than `$at`.
-- Hence this CTE that computes what `current_profile` was back when `$at`
-- was the current time.
my_users_last_profile_update AS (
    SELECT DISTINCT ON (user_)
        user_,
        profile
    FROM profile
    WHERE certified_on <= $at
    ORDER BY user_ ASC, certified_on DESC
),

my_realms AS (
    SELECT realm._id
    FROM realm
    INNER JOIN my_org
        ON realm.organization = my_org._id
    WHERE realm.created_on <= $at
)

SELECT
    my_users.organization_users,
    my_vlob_atoms.organization_metadata_size,
    my_blocks.organization_data_size,
    (SELECT COUNT(*) FROM my_realms) AS organization_realms
FROM
    my_org,

    LATERAL (
        SELECT ARRAY(
            SELECT
                ROW(
                    user_.revoked_on,
                    COALESCE(my_last_profile_update.profile_text, user_.initial_profile::TEXT)
                )
            FROM user_
            LEFT JOIN LATERAL (
                SELECT my_users_last_profile_update.profile::TEXT AS profile_text
                FROM my_users_last_profile_update
                WHERE my_users_last_profile_update.user_ = user_._id
                LIMIT 1
            ) AS my_last_profile_update ON TRUE
            WHERE
                user_.organization = my_org._id
                AND user_.created_on <= $at
        ) AS organization_users
    ) AS my_users,

    LATERAL (
        SELECT COALESCE(SUM(vlob_atom.size), 0) AS organization_metadata_size
        FROM vlob_atom
        INNER JOIN my_realms ON vlob_atom.realm = my_realms._id
        WHERE
            vlob_atom.created_on <= $at
            AND (vlob_atom.deleted_on IS NULL OR vlob_atom.deleted_on > $at)
    ) AS my_vlob_atoms,

    LATERAL (
        SELECT COALESCE(SUM(block.size), 0) AS organization_data_size
        FROM block
        INNER JOIN my_realms ON block.realm = my_realms._id
        WHERE
            block.created_on <= $at
            AND (block.deleted_on IS NULL OR block.deleted_on > $at)
    ) AS my_blocks
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

    # No row is returned if the organization does not exists at the specified datetime
    if row is None:
        return OrganizationStatsBadOutcome.ORGANIZATION_NOT_FOUND

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
