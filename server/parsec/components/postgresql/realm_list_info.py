# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import DeviceID, OrganizationID, RealmRole, UserProfile, VlobID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.realm import (
    RealmInfo,
    RealmListInfoBadOutcome,
)

_q_get_realm_info = Q(
    """
WITH my_realms AS (
    SELECT
        realm._id,
        realm.realm_id
    FROM realm
    WHERE
        realm.organization = $internal_organization_id
        AND realm.status != 'DELETED'
),

-- Current role of each user in each realm (excluding NULL roles from unshare)
my_realm_current_roles AS (
    SELECT
        latest_realm_roles.realm,
        latest_realm_roles.user_,
        latest_realm_roles.role
    FROM (
        SELECT DISTINCT ON (realm_user_role.realm, realm_user_role.user_)
            realm_user_role.realm,
            realm_user_role.user_,
            realm_user_role.role
        FROM realm_user_role
        ORDER BY
            realm_user_role.realm ASC,
            realm_user_role.user_ ASC,
            realm_user_role.certified_on DESC
    ) AS latest_realm_roles
    -- Unshared users don't have a role anymore
    WHERE latest_realm_roles.role IS NOT NULL
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
)

SELECT
    my_realms._id AS realm_internal_id,
    my_realms.realm_id,
    my_roles.role_counts,
    -- Realms with no vlob/block have no row in the size CTEs
    COALESCE(my_metadata_sizes.metadata_size, 0) AS metadata_size,
    COALESCE(my_data_sizes.data_size, 0) AS data_size
FROM
    my_realms
LEFT JOIN my_metadata_sizes ON my_realms._id = my_metadata_sizes.realm
LEFT JOIN my_data_sizes ON my_realms._id = my_data_sizes.realm,

    LATERAL (
        SELECT ARRAY(
            -- Note the `role::text` casting here, this is a limitation of asyncpg which
            -- doesn't support enum within an anonymous record
            -- (see https://github.com/MagicStack/asyncpg/issues/360)
            SELECT ROW(my_realm_current_roles.role::TEXT, COUNT(*))
            FROM my_realm_current_roles
            INNER JOIN user_ ON my_realm_current_roles.user_ = user_._id
            WHERE
                my_realm_current_roles.realm = my_realms._id
                AND user_.revoked_on IS NULL
            GROUP BY my_realm_current_roles.role
        ) AS role_counts
    ) AS my_roles

ORDER BY my_realms._id
"""
)


async def realm_list_info(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    device_id: DeviceID,
) -> list[RealmInfo] | RealmListInfoBadOutcome:
    match await auth_no_lock(conn, organization_id, device_id):
        case AuthNoLockData() as db_auth:
            pass
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmListInfoBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return RealmListInfoBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return RealmListInfoBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return RealmListInfoBadOutcome.AUTHOR_REVOKED

    if db_auth.user_current_profile != UserProfile.ADMIN:
        return RealmListInfoBadOutcome.AUTHOR_NOT_ALLOWED

    rows = await conn.fetch(
        *_q_get_realm_info(internal_organization_id=db_auth.organization_internal_id)
    )

    realms_info = []
    for row in rows:
        match row["realm_id"]:
            case str() as raw_realm_id:
                realm_id = VlobID.from_hex(raw_realm_id)
            case _:
                assert False, row

        match row["metadata_size"]:
            case int() as metadata_size:
                pass
            case _:
                assert False, row

        match row["data_size"]:
            case int() as data_size:
                pass
            case _:
                assert False, row

        users_per_role = {role: 0 for role in RealmRole.VALUES}
        match row["role_counts"]:
            case list() as raw_roles:
                pass
            case _:
                assert False, row

        for raw_role, count in raw_roles:
            role = RealmRole.from_str(raw_role)
            users_per_role[role] = count

        realms_info.append(
            RealmInfo(
                realm_id=realm_id,
                metadata_size=metadata_size,
                data_size=data_size,
                users_per_role=users_per_role,
            )
        )

    return realms_info
