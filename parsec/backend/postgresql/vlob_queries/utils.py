# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.backend.postgresql.utils import (
    Q,
    q_realm_internal_id,
    q_user_internal_id,
    q_organization_internal_id,
    STR_TO_REALM_ROLE,
)
from parsec.backend.realm import RealmRole
from parsec.backend.vlob import (
    VlobNotFoundError,
    VlobInMaintenanceError,
    VlobNotInMaintenanceError,
    VlobEncryptionRevisionError,
    VlobAccessError,
)
from parsec.backend.postgresql.realm_queries.maintenance import get_realm_status, RealmNotFoundError


async def _check_realm(
    conn, organization_id, realm_id, encryption_revision, expected_maintenance=False
):
    try:
        rep = await get_realm_status(conn, organization_id, realm_id)
    except RealmNotFoundError as exc:
        raise VlobNotFoundError(*exc.args) from exc
    if expected_maintenance is False:
        if rep["maintenance_type"]:
            raise VlobInMaintenanceError("Data realm is currently under maintenance")
    elif expected_maintenance is True:
        if not rep["maintenance_type"]:
            raise VlobNotInMaintenanceError(f"Realm `{realm_id}` not under maintenance")
    if encryption_revision is not None and rep["encryption_revision"] != encryption_revision:
        raise VlobEncryptionRevisionError()


_q_check_realm_access = Q(
    f"""
WITH cte_current_realm_roles AS (
    SELECT DISTINCT ON(user_) user_, role
    FROM  realm_user_role
    WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    ORDER BY user_, certified_on DESC
)
SELECT role
FROM user_
LEFT JOIN cte_current_realm_roles
ON user_._id = cte_current_realm_roles.user_
WHERE user_._id = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
"""
)


async def _check_realm_access(conn, organization_id, realm_id, author, allowed_roles):
    rep = await conn.fetchrow(
        *_q_check_realm_access(
            organization_id=organization_id, realm_id=realm_id, user_id=author.user_id
        )
    )

    if not rep:
        raise VlobNotFoundError(f"User `{author.user_id}` doesn't exist")

    if STR_TO_REALM_ROLE.get(rep[0]) not in allowed_roles:
        raise VlobAccessError()


async def _check_realm_and_write_access(
    conn, organization_id, author, realm_id, encryption_revision
):
    await _check_realm(conn, organization_id, realm_id, encryption_revision)
    can_write_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR)
    await _check_realm_access(conn, organization_id, realm_id, author, can_write_roles)


_q_get_realm_id_from_vlob_id = Q(
    f"""
SELECT
    realm.realm_id
FROM vlob_atom
INNER JOIN vlob_encryption_revision
ON  vlob_atom.vlob_encryption_revision = vlob_encryption_revision._id
INNER JOIN realm
ON vlob_encryption_revision.realm = realm._id
WHERE vlob_atom._id = (
    SELECT _id
    FROM vlob_atom
    WHERE
        organization = { q_organization_internal_id("$organization_id") }
        AND vlob_id = $vlob_id
    LIMIT 1
)
LIMIT 1
"""
)


async def _get_realm_id_from_vlob_id(conn, organization_id, vlob_id):
    realm_id = await conn.fetchval(
        *_q_get_realm_id_from_vlob_id(organization_id=organization_id, vlob_id=vlob_id)
    )
    if not realm_id:
        raise VlobNotFoundError(f"Vlob `{vlob_id}` doesn't exist")
    return realm_id
