# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncpg

from parsec._parsec import DateTime, DeviceID, OrganizationID, VlobID
from parsec.components.postgresql.realm_queries.maintenance import (
    RealmNotFoundError,
    get_realm_status,
)
from parsec.components.postgresql.utils import (
    Q,
    q_organization_internal_id,
    q_realm_internal_id,
    q_user_internal_id,
)
from parsec.components.realm import RealmRole
from parsec.components.vlob import (
    VlobAccessError,
    VlobEncryptionRevisionError,
    VlobInMaintenanceError,
    VlobNotFoundError,
    VlobNotInMaintenanceError,
    VlobRealmNotFoundError,
    VlobRequireGreaterTimestampError,
)
from parsec.types import OperationKind


async def _check_realm(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    encryption_revision: int | None,
    operation_kind: OperationKind,
) -> None:
    # Get the current realm status
    try:
        status = await get_realm_status(conn, organization_id, realm_id)
    except RealmNotFoundError as exc:
        raise VlobRealmNotFoundError(*exc.args) from exc

    # Special case of reading while in reencryption
    if operation_kind == OperationKind.DATA_READ and status.in_reencryption:
        # Starting a reencryption maintenance bumps the encryption revision.
        # Hence if we are currently in reencryption maintenance, last encryption revision is not ready
        # to be used (it will be once the reencryption is over !).
        # So during this intermediary state, we allow read access to the previous encryption revision instead.

        # Note that `encryption_revision` might also be `None` in the case of `poll_changes` and `list_versions`
        # requests, which should also be allowed during a reencryption

        # The vlob is not available yet for the current revision
        if encryption_revision is not None and encryption_revision == status.encryption_revision:
            raise VlobInMaintenanceError(f"Realm `{realm_id.hex}` is currently under maintenance")

        # The vlob is only available at the previous revision
        if (
            encryption_revision is not None
            and encryption_revision != status.encryption_revision - 1
        ):
            raise VlobEncryptionRevisionError()

    # In all other cases
    else:
        # Access during maintenance is forbidden
        if operation_kind != OperationKind.MAINTENANCE and status.in_maintenance:
            raise VlobInMaintenanceError("Data realm is currently under maintenance")

        # A maintenance state was expected
        if operation_kind == OperationKind.MAINTENANCE and not status.in_maintenance:
            raise VlobNotInMaintenanceError(f"Realm `{realm_id.hex}` not under maintenance")

        # Otherwise simply check that the revisions match
        if encryption_revision is not None and status.encryption_revision != encryption_revision:
            raise VlobEncryptionRevisionError()


_q_check_realm_access = Q(
    f"""
WITH cte_current_realm_roles AS (
    SELECT DISTINCT ON(user_) user_, role, certified_on
    FROM  realm_user_role
    WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    ORDER BY user_, certified_on DESC
)
SELECT role, certified_on
FROM user_
LEFT JOIN cte_current_realm_roles
ON user_._id = cte_current_realm_roles.user_
WHERE user_._id = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
"""
)


async def _check_realm_access(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    author: DeviceID,
    allowed_roles: tuple[RealmRole, ...],
) -> DateTime:
    rep = await conn.fetchrow(
        *_q_check_realm_access(
            organization_id=organization_id.str, realm_id=realm_id, user_id=author.user_id.str
        )
    )

    if not rep:
        raise VlobNotFoundError(f"User `{author.user_id.str}` doesn't exist")

    role = RealmRole.from_str(rep[0]) if rep[0] is not None else None
    if role not in allowed_roles:
        raise VlobAccessError()

    role_granted_on = rep[1]
    return role_granted_on


async def _check_realm_and_read_access(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    encryption_revision: int | None,
) -> None:
    await _check_realm(
        conn, organization_id, realm_id, encryption_revision, OperationKind.DATA_READ
    )
    can_read_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR, RealmRole.READER)
    await _check_realm_access(conn, organization_id, realm_id, author, can_read_roles)


async def _check_realm_and_write_access(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    encryption_revision: int | None,
    timestamp: DateTime,
) -> None:
    await _check_realm(
        conn, organization_id, realm_id, encryption_revision, OperationKind.DATA_WRITE
    )
    can_write_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR)
    last_role_granted_on = await _check_realm_access(
        conn, organization_id, realm_id, author, can_write_roles
    )
    # Write operations should always occurs strictly after the last change of role for this user
    if last_role_granted_on >= timestamp:
        raise VlobRequireGreaterTimestampError(last_role_granted_on)


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


async def _get_realm_id_from_vlob_id(
    conn: asyncpg.Connection, organization_id: OrganizationID, vlob_id: VlobID
) -> VlobID:
    realm_id_uuid = await conn.fetchval(
        *_q_get_realm_id_from_vlob_id(organization_id=organization_id.str, vlob_id=vlob_id)
    )
    if not realm_id_uuid:
        raise VlobNotFoundError(f"Vlob `{vlob_id.hex}` doesn't exist")
    return VlobID.from_hex(realm_id_uuid)


async def _get_last_role_granted_on(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    author: DeviceID,
) -> DateTime | None:
    rep = await conn.fetchrow(
        *_q_check_realm_access(
            organization_id=organization_id.str, realm_id=realm_id, user_id=author.user_id.str
        )
    )
    return None if rep is None else rep[1]
