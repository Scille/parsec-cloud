# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Optional, Tuple
from parsec._parsec import DateTime

from parsec.api.protocol import OrganizationID, RealmID, DeviceID, VlobID
from parsec.backend.utils import OperationKind
from parsec.backend.postgresql.utils import (
    Q,
    q_realm_internal_id,
    q_user_internal_id,
    q_organization_internal_id,
)
from parsec.backend.realm import RealmRole
from parsec.backend.vlob import (
    VlobNotFoundError,
    VlobRealmNotFoundError,
    VlobInMaintenanceError,
    VlobNotInMaintenanceError,
    VlobEncryptionRevisionError,
    VlobAccessError,
    VlobRequireGreaterTimestampError,
)
from parsec.backend.postgresql.realm_queries.maintenance import get_realm_status, RealmNotFoundError


async def _check_realm(
    conn,
    organization_id: OrganizationID,
    realm_id: RealmID,
    encryption_revision: Optional[int],
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
            raise VlobInMaintenanceError(f"Realm `{realm_id.str}` is currently under maintenance")

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
            raise VlobNotInMaintenanceError(f"Realm `{realm_id.str}` not under maintenance")

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
    conn,
    organization_id: OrganizationID,
    realm_id: RealmID,
    author: DeviceID,
    allowed_roles: Tuple[RealmRole, ...],
) -> DateTime:
    rep = await conn.fetchrow(
        *_q_check_realm_access(
            organization_id=organization_id.str, realm_id=realm_id.uuid, user_id=author.user_id.str
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
    conn,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    encryption_revision: Optional[int],
) -> None:
    await _check_realm(
        conn, organization_id, realm_id, encryption_revision, OperationKind.DATA_READ
    )
    can_read_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR, RealmRole.READER)
    await _check_realm_access(conn, organization_id, realm_id, author, can_read_roles)


async def _check_realm_and_write_access(
    conn,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    encryption_revision: Optional[int],
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
    conn, organization_id: OrganizationID, vlob_id: VlobID
) -> RealmID:
    realm_id_uuid = await conn.fetchval(
        *_q_get_realm_id_from_vlob_id(organization_id=organization_id.str, vlob_id=vlob_id.uuid)
    )
    if not realm_id_uuid:
        raise VlobNotFoundError(f"Vlob `{vlob_id.str}` doesn't exist")
    return RealmID(realm_id_uuid)


async def _get_last_role_granted_on(
    conn, organization_id: OrganizationID, realm_id: RealmID, author: DeviceID
) -> Optional[DateTime]:
    rep = await conn.fetchrow(
        *_q_check_realm_access(
            organization_id=organization_id.str, realm_id=realm_id.uuid, user_id=author.user_id.str
        )
    )
    return None if rep is None else rep[1]
