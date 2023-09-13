# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Dict, List

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
from parsec.backend.postgresql.realm_queries.archiving import check_archiving_configuration
from parsec.backend.postgresql.utils import (
    Q,
    q_device,
    q_organization_internal_id,
    q_realm,
    q_realm_internal_id,
    q_user,
    q_user_can_read_vlob,
    q_user_internal_id,
    query,
)
from parsec.backend.realm import (
    RealmAccessError,
    RealmGrantedRole,
    RealmNotFoundError,
    RealmStats,
    RealmStatus,
)
from parsec.backend.utils import OperationKind

_q_get_realm_status = Q(
    f"""
    SELECT
        { q_user_can_read_vlob(organization_id="$organization_id", realm_id="$realm_id", user_id="$user_id") } has_access,
        encryption_revision,
        { q_device(_id="maintenance_started_by", select="device_id") } maintenance_started_by,
        maintenance_started_on,
        maintenance_type
    FROM realm
    WHERE
        organization = { q_organization_internal_id("$organization_id") }
        AND realm_id = $realm_id
"""
)

_q_has_realm_access = Q(
    f"""
    SELECT
        { q_user_can_read_vlob(organization_id="$organization_id", realm_id="$realm_id", user_id="$user_id") } has_access
    FROM realm
    WHERE
        organization = { q_organization_internal_id("$organization_id") }
        AND realm_id = $realm_id
"""
)

_q_get_blocks_size_from_realm = Q(
    f"""
    SELECT SUM(size)
    FROM block
    WHERE
        realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
"""
)

_q_get_vlob_size_from_realm = Q(
    f"""
    SELECT SUM(size)
    FROM
        vlob_atom
    INNER JOIN
        realm_vlob_update ON vlob_atom._id = realm_vlob_update.vlob_atom
    WHERE
        realm_vlob_update.realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
"""
)

_q_get_current_roles = Q(
    f"""
SELECT DISTINCT ON(user_) { q_user(_id="realm_user_role.user_", select="user_id") }, role
FROM  realm_user_role
WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY user_, certified_on DESC
"""
)


_q_get_role_certificates = Q(
    f"""
SELECT { q_user(_id="realm_user_role.user_", select="user_id") }, role, certificate, certified_on
FROM  realm_user_role
WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY certified_on ASC
"""
)


_q_get_realms_for_user = Q(
    f"""
SELECT DISTINCT ON(realm) realm, { q_realm(_id="realm_user_role.realm", select="realm_id") } as realm_id, role
FROM  realm_user_role
WHERE user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
ORDER BY realm, certified_on DESC
"""
)


_q_get_organization_realms_granted_roles = Q(
    f"""
SELECT
    realm.realm_id,
    { q_user(_id="realm_user_role.user_", select="user_id") } as user_id,
    role,
    certificate,
    { q_device(_id="realm_user_role.certified_by", select="device_id") } as granted_by,
    certified_on as granted_on
FROM realm_user_role
INNER JOIN realm
ON realm_user_role.realm = realm._id
WHERE
    realm.organization = { q_organization_internal_id("$organization_id") }
"""
)


@query()
async def query_get_status(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    now: DateTime,
) -> RealmStatus:
    ret = await conn.fetchrow(
        *_q_get_realm_status(
            organization_id=organization_id.str, realm_id=realm_id, user_id=author.user_id.str
        )
    )
    if not ret:
        raise RealmNotFoundError(f"Realm `{realm_id.hex}` doesn't exist")

    if not ret["has_access"]:
        raise RealmAccessError()

    # Make sure the realm is not deleted
    await check_archiving_configuration(
        conn, organization_id, realm_id, OperationKind.CONFIGURATION, now
    )

    return RealmStatus(
        maintenance_type=MaintenanceType.from_str(ret["maintenance_type"])
        if ret["maintenance_type"]
        else None,
        maintenance_started_on=ret["maintenance_started_on"],
        maintenance_started_by=DeviceID(ret["maintenance_started_by"])
        if ret["maintenance_started_by"]
        else None,
        encryption_revision=ret["encryption_revision"],
    )


@query(in_transaction=True)
async def query_get_stats(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    now: DateTime,
) -> RealmStats:
    ret = await conn.fetchrow(
        *_q_has_realm_access(
            organization_id=organization_id.str, realm_id=realm_id, user_id=author.user_id.str
        )
    )
    if not ret:
        raise RealmNotFoundError(f"Realm `{realm_id.hex}` doesn't exist")

    if not ret["has_access"]:
        raise RealmAccessError()

    # Make sure the realm is not deleted
    await check_archiving_configuration(
        conn, organization_id, realm_id, OperationKind.CONFIGURATION, now
    )

    blocks_size_rep = await conn.fetchrow(
        *_q_get_blocks_size_from_realm(organization_id=organization_id.str, realm_id=realm_id)
    )
    vlobs_size_rep = await conn.fetchrow(
        *_q_get_vlob_size_from_realm(organization_id=organization_id.str, realm_id=realm_id)
    )

    blocks_size = blocks_size_rep["sum"] or 0
    vlobs_size = vlobs_size_rep["sum"] or 0

    return RealmStats(blocks_size=blocks_size, vlobs_size=vlobs_size)


@query()
async def query_get_current_roles(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    realm_id: RealmID,
    now: DateTime,
) -> Dict[UserID, RealmRole]:
    ret = await conn.fetch(
        *_q_get_current_roles(organization_id=organization_id.str, realm_id=realm_id)
    )

    if not ret:
        # Existing group must have at least one owner user
        raise RealmNotFoundError(f"Realm `{realm_id.hex}` doesn't exist")

    return {UserID(user_id): RealmRole.from_str(role) for user_id, role in ret if role is not None}


@query()
async def query_get_role_certificates(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    now: DateTime,
) -> List[bytes]:
    ret = await conn.fetch(
        *_q_get_role_certificates(organization_id=organization_id.str, realm_id=realm_id)
    )

    if not ret:
        # Existing group must have at least one owner user
        raise RealmNotFoundError(f"Realm `{realm_id.hex}` doesn't exist")

    out = []
    author_current_role = None
    for user_id, role, certif, _ in ret:
        user_id = UserID(user_id)
        out.append(certif)
        if user_id == author.user_id:
            author_current_role = role

    if author_current_role is None:
        raise RealmAccessError()

    return out


@query()
async def query_get_realms_for_user(
    conn: triopg._triopg.TrioConnectionProxy, organization_id: OrganizationID, user: UserID
) -> dict[RealmID, tuple[int, RealmRole]]:
    rep = await conn.fetch(
        *_q_get_realms_for_user(organization_id=organization_id.str, user_id=user.str)
    )
    return {
        RealmID.from_hex(row["realm_id"]): (row["realm"], RealmRole.from_str(row["role"]))
        for row in rep
        if row["role"] is not None
    }


@query()
async def query_dump_realms_granted_roles(
    conn: triopg._triopg.TrioConnectionProxy, organization_id: OrganizationID
) -> List[RealmGrantedRole]:
    granted_roles = []
    rows = await conn.fetch(
        *_q_get_organization_realms_granted_roles(organization_id=organization_id.str)
    )

    for row in rows:
        granted_roles.append(
            RealmGrantedRole(
                certificate=row["certificate"],
                realm_id=RealmID.from_hex(row["realm_id"]),
                user_id=UserID(row["user_id"]),
                role=RealmRole.from_str(row["role"]),
                granted_by=DeviceID(row["granted_by"]),
                granted_on=row["granted_on"],
            )
        )

    return granted_roles
