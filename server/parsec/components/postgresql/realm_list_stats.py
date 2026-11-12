# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import (
    DeviceID,
    OrganizationID,
    UserProfile,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import Q
from parsec.components.realm import (
    RealmListStatsBadOutcome,
    RealmStats,
)

_q_get_realms_stats = Q("""
WITH my_realms AS (
    SELECT
        _id,
        realm_id,
        created_on
    FROM realm
    WHERE
        organization = $organization_internal_id
        AND status != 'DELETED'
        -- Only consider workspaces: user realms never have key rotation (and a
        -- workspace without key rotation is not bootstrapped, hence has no data)
        AND key_index > 0
),

my_metadata_sizes AS (
    SELECT
        vlob_atom.realm,
        SUM(vlob_atom.size) AS metadata_size
    FROM vlob_atom
    INNER JOIN my_realms ON vlob_atom.realm = my_realms._id
    WHERE vlob_atom.deleted_on IS NULL
    GROUP BY vlob_atom.realm
),

my_data_sizes AS (
    SELECT
        block.realm,
        SUM(block.size) AS data_size
    FROM block
    INNER JOIN my_realms ON block.realm = my_realms._id
    WHERE block.deleted_on IS NULL
    GROUP BY block.realm
),

-- Last role certificate for each user in each realm (`role` is NULL if
-- the user has been unshared)
my_current_roles AS (
    SELECT DISTINCT ON (realm_user_role.realm, realm_user_role.user_)
        realm_user_role.realm,
        realm_user_role.user_,
        realm_user_role.role
    FROM realm_user_role
    INNER JOIN my_realms ON realm_user_role.realm = my_realms._id
    ORDER BY
        realm_user_role.realm ASC,
        realm_user_role.user_ ASC,
        realm_user_role.certified_on DESC
),

my_role_counts AS (
    SELECT
        my_current_roles.realm,
        COUNT(*) FILTER (WHERE my_current_roles.role = 'OWNER') AS owners,
        COUNT(*) FILTER (WHERE my_current_roles.role = 'MANAGER') AS managers,
        COUNT(*) FILTER (WHERE my_current_roles.role = 'CONTRIBUTOR') AS contributors,
        COUNT(*) FILTER (WHERE my_current_roles.role = 'READER') AS readers
    FROM my_current_roles
    INNER JOIN user_ ON my_current_roles.user_ = user_._id
    -- Revoked users still have their realm roles, but are no longer members in practice
    WHERE user_.revoked_on IS NULL
    GROUP BY my_current_roles.realm
)

SELECT
    my_realms.realm_id,
    COALESCE(my_metadata_sizes.metadata_size, 0) AS metadata_size,
    COALESCE(my_data_sizes.data_size, 0) AS data_size,
    COALESCE(my_role_counts.owners, 0) AS owners,
    COALESCE(my_role_counts.managers, 0) AS managers,
    COALESCE(my_role_counts.contributors, 0) AS contributors,
    COALESCE(my_role_counts.readers, 0) AS readers
FROM my_realms
LEFT JOIN my_metadata_sizes ON my_realms._id = my_metadata_sizes.realm
LEFT JOIN my_data_sizes ON my_realms._id = my_data_sizes.realm
LEFT JOIN my_role_counts ON my_realms._id = my_role_counts.realm
ORDER BY my_realms.created_on ASC, my_realms._id ASC
""")


async def realm_list_stats(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
) -> list[RealmStats] | RealmListStatsBadOutcome:
    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as db_auth:
            pass
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmListStatsBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return RealmListStatsBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return RealmListStatsBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return RealmListStatsBadOutcome.AUTHOR_REVOKED

    if db_auth.user_current_profile != UserProfile.ADMIN:
        return RealmListStatsBadOutcome.AUTHOR_NOT_ALLOWED

    rows = await conn.fetch(
        *_q_get_realms_stats(organization_internal_id=db_auth.organization_internal_id)
    )

    realms_stats = []
    for row in rows:
        match row["realm_id"]:
            case str() as raw_realm_id:
                realm_id = VlobID.from_hex(raw_realm_id)
            case _:
                assert False, row

        counters = {}
        for field in (
            "metadata_size",
            "data_size",
            "owners",
            "managers",
            "contributors",
            "readers",
        ):
            match row[field]:
                case int() as value:
                    counters[field] = value
                case _:
                    assert False, row

        realms_stats.append(RealmStats(realm_id=realm_id, **counters))

    return realms_stats
