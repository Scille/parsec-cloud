# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Dict, List

import triopg

from parsec._parsec import DateTime
from parsec.api.protocol import (
    DeviceID,
    MaintenanceType,
    OrganizationID,
    RealmID,
    RealmRole,
    UserID,
)
from parsec.backend.backend_events import BackendEvent
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.postgresql.message import send_message
from parsec.backend.postgresql.realm_queries.archiving import check_archiving_configuration
from parsec.backend.postgresql.utils import (
    Q,
    q_device,
    q_device_internal_id,
    q_organization_internal_id,
    q_realm,
    q_realm_internal_id,
    q_user,
    query,
)
from parsec.backend.realm import (
    RealmAccessError,
    RealmEncryptionRevisionError,
    RealmInMaintenanceError,
    RealmMaintenanceError,
    RealmNotFoundError,
    RealmNotInMaintenanceError,
    RealmParticipantsMismatchError,
    RealmStatus,
)
from parsec.backend.utils import OperationKind

_q_get_realm_status = Q(
    q_realm(
        organization_id="$organization_id",
        realm_id="$realm_id",
        select=f"encryption_revision, { q_device(_id='maintenance_started_by', select='device_id') } maintenance_started_by, maintenance_started_on, maintenance_type",
    )
)


async def get_realm_status(
    conn: triopg._triopg.TrioConnectionProxy, organization_id: OrganizationID, realm_id: RealmID
) -> RealmStatus:
    rep = await conn.fetchrow(
        *_q_get_realm_status(organization_id=organization_id.str, realm_id=realm_id)
    )
    if not rep:
        raise RealmNotFoundError(f"Realm `{realm_id.hex}` doesn't exist")

    return RealmStatus(
        maintenance_type=MaintenanceType.from_str(rep["maintenance_type"])
        if rep["maintenance_type"]
        else None,
        maintenance_started_on=rep["maintenance_started_on"],
        maintenance_started_by=DeviceID(rep["maintenance_started_by"])
        if rep["maintenance_started_by"]
        else None,
        encryption_revision=rep["encryption_revision"],
    )


_q_get_realm_role_for_not_revoked_with_users = Q(
    f"""
WITH cte_current_realm_roles AS (
    SELECT DISTINCT ON(user_) user_, role
    FROM  realm_user_role
    WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    ORDER BY user_, certified_on DESC
)
SELECT user_.user_id as user_id, user_.revoked_on as revoked_on, role
FROM user_
LEFT JOIN cte_current_realm_roles
ON user_._id = cte_current_realm_roles.user_
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND user_.user_id = ANY($users_ids::VARCHAR[])
"""
)


_q_get_realm_role_for_not_revoked = Q(
    f"""
SELECT DISTINCT ON(user_)
    { q_user(_id="realm_user_role.user_", select="user_id") } as user_id,
    { q_user(_id="realm_user_role.user_", select="revoked_on") } as revoked_on,
    role
FROM  realm_user_role
WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY user_, certified_on DESC
"""
)


async def _get_realm_role_for_not_revoked(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    realm_id: RealmID,
    users: List[UserID] | None = None,
) -> dict[UserID, RealmRole | None]:
    now = DateTime.now()

    def _cook_role(row: dict[str, Any]) -> RealmRole | None:
        if row["revoked_on"] and row["revoked_on"] <= now:
            return None
        if row["role"] is None:
            return None
        return RealmRole.from_str(row["role"])

    if users:
        rep = await conn.fetch(
            *_q_get_realm_role_for_not_revoked_with_users(
                organization_id=organization_id.str,
                realm_id=realm_id,
                users_ids=[u.str for u in users],
            )
        )
        roles = {UserID(row["user_id"]): _cook_role(row) for row in rep}
        for user in users or ():
            if user not in roles:
                raise RealmNotFoundError(f"User `{user}` doesn't exist")

    else:
        rep = await conn.fetch(
            *_q_get_realm_role_for_not_revoked(
                organization_id=organization_id.str, realm_id=realm_id
            )
        )

        roles = {
            UserID(row["user_id"]): _cook_role(row) for row in rep if _cook_role(row) is not None
        }

    return roles


_q_query_start_reencryption_maintenance_update_realm = Q(
    f"""
UPDATE realm
SET
    encryption_revision=$encryption_revision,
    maintenance_started_by={ q_device_internal_id(organization_id="$organization_id", device_id="$maintenance_started_by") },
    maintenance_started_on=$maintenance_started_on,
    maintenance_type=$maintenance_type
WHERE
    _id = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
"""
)


_q_query_start_reencryption_maintenance_update_vlob_encryption_revision = Q(
    f"""
INSERT INTO vlob_encryption_revision(
    realm,
    encryption_revision
) SELECT
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    $encryption_revision
"""
)


@query(in_transaction=True)
async def query_start_reencryption_maintenance(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    encryption_revision: int,
    per_participant_message: Dict[UserID, bytes],
    timestamp: DateTime,
    now: DateTime,
) -> None:
    # Retrieve realm and make sure it is not under maintenance
    status = await get_realm_status(conn, organization_id, realm_id)
    if status.in_maintenance:
        raise RealmInMaintenanceError(f"Realm `{realm_id.hex}` already in maintenance")
    if encryption_revision != status.encryption_revision + 1:
        raise RealmEncryptionRevisionError("Invalid encryption revision")

    roles = await _get_realm_role_for_not_revoked(conn, organization_id, realm_id)

    if roles.get(author.user_id) != RealmRole.OWNER:
        raise RealmAccessError()

    # Make sure the realm is not deleted
    await check_archiving_configuration(
        conn, organization_id, realm_id, OperationKind.MAINTENANCE, now
    )

    if per_participant_message.keys() ^ roles.keys():
        raise RealmParticipantsMismatchError("Realm participants and message recipients mismatch")

    await conn.execute(
        *_q_query_start_reencryption_maintenance_update_realm(
            organization_id=organization_id.str,
            realm_id=realm_id,
            maintenance_started_by=author.str,
            maintenance_started_on=timestamp,
            maintenance_type="REENCRYPTION",
            encryption_revision=encryption_revision,
        )
    )

    await conn.execute(
        *_q_query_start_reencryption_maintenance_update_vlob_encryption_revision(
            organization_id=organization_id.str,
            realm_id=realm_id,
            encryption_revision=encryption_revision,
        )
    )

    await send_signal(
        conn,
        BackendEvent.REALM_MAINTENANCE_STARTED,
        organization_id=organization_id,
        author=author,
        realm_id=realm_id,
        encryption_revision=encryption_revision,
    )

    for recipient, body in per_participant_message.items():
        await send_message(conn, organization_id, author, recipient, timestamp, body)


_query_finish_reencryption_maintenance_get_info = Q(
    f"""
WITH cte_encryption_revisions AS (
    SELECT
        _id,
        encryption_revision
    FROM vlob_encryption_revision
    WHERE
        realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
        AND (encryption_revision = $encryption_revision OR encryption_revision = $encryption_revision - 1)
)
SELECT encryption_revision, COUNT(*) as count
FROM vlob_atom
INNER JOIN cte_encryption_revisions
ON cte_encryption_revisions._id = vlob_atom.vlob_encryption_revision
GROUP BY encryption_revision
ORDER BY encryption_revision
"""
)


_query_finish_reencryption_maintenance_update_realm = Q(
    f"""
UPDATE realm
SET
    maintenance_started_by=NULL,
    maintenance_started_on=NULL,
    maintenance_type=NULL
WHERE
    _id = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
"""
)


@query(in_transaction=True)
async def query_finish_reencryption_maintenance(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    encryption_revision: int,
    now: DateTime,
) -> None:
    # Retrieve realm and make sure it is not under maintenance
    status = await get_realm_status(conn, organization_id, realm_id)
    roles = await _get_realm_role_for_not_revoked(conn, organization_id, realm_id, [author.user_id])
    if roles.get(author.user_id) != RealmRole.OWNER:
        raise RealmAccessError()
    # Make sure the realm is not deleted
    await check_archiving_configuration(
        conn, organization_id, realm_id, OperationKind.MAINTENANCE, now
    )
    if not status.in_maintenance:
        raise RealmNotInMaintenanceError(f"Realm `{realm_id.hex}` not under maintenance")
    if encryption_revision != status.encryption_revision:
        raise RealmEncryptionRevisionError("Invalid encryption revision")

    # Test reencryption operations are over
    rep = await conn.fetch(
        *_query_finish_reencryption_maintenance_get_info(
            organization_id=organization_id.str,
            realm_id=realm_id,
            encryption_revision=encryption_revision,
        )
    )

    try:
        previous, current = rep
    except ValueError:
        raise RealmMaintenanceError("Reencryption operations are not over")
    assert previous["encryption_revision"] == encryption_revision - 1
    assert current["encryption_revision"] == encryption_revision
    assert previous["count"] >= current["count"]
    if previous["count"] != current["count"]:
        raise RealmMaintenanceError("Reencryption operations are not over")

    await conn.execute(
        *_query_finish_reencryption_maintenance_update_realm(
            organization_id=organization_id.str, realm_id=realm_id
        )
    )

    await send_signal(
        conn,
        BackendEvent.REALM_MAINTENANCE_FINISHED,
        organization_id=organization_id,
        author=author,
        realm_id=realm_id,
        encryption_revision=encryption_revision,
    )
