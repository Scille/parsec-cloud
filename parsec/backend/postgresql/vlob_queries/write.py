# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Dict

import triopg
from triopg import UniqueViolationError

from parsec._parsec import DateTime, SequesterServiceID
from parsec.api.protocol import DeviceID, OrganizationID, RealmID, VlobID
from parsec.backend.backend_events import BackendEvent
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_organization_internal_id,
    q_realm_internal_id,
    q_user_internal_id,
    q_vlob_encryption_revision_internal_id,
    query,
)
from parsec.backend.postgresql.vlob_queries.utils import (
    _check_realm_and_write_access,
    _get_realm_id_from_vlob_id,
)
from parsec.backend.vlob import (
    VlobAlreadyExistsError,
    VlobNotFoundError,
    VlobRequireGreaterTimestampError,
    VlobVersionError,
)

_q_vlob_updated = Q(
    f"""
INSERT INTO realm_vlob_update (
realm, index, vlob_atom
)
SELECT
{ q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
(
    SELECT COALESCE(MAX(index) + 1, 1)
    FROM realm_vlob_update
    WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
),
$vlob_atom_internal_id
RETURNING index
"""
)


_q_set_last_vlob_update = Q(
    f"""
INSERT INTO realm_user_change(realm, user_, last_role_change, last_vlob_update, last_archiving_change)
VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    NULL,
    $timestamp,
    NULL
)
ON CONFLICT (realm, user_)
DO UPDATE SET last_vlob_update = (
    SELECT GREATEST($timestamp, last_vlob_update)
    FROM realm_user_change
    WHERE realm={ q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    AND user_={ q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
    LIMIT 1
)
"""
)


async def _set_vlob_updated(
    conn: triopg._triopg.TrioConnectionProxy,
    vlob_atom_internal_id: int,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    src_id: VlobID,
    timestamp: DateTime,
    src_version: int = 1,
) -> None:
    index = await conn.fetchval(
        *_q_vlob_updated(
            organization_id=organization_id.str,
            realm_id=realm_id,
            vlob_atom_internal_id=vlob_atom_internal_id,
        )
    )

    await conn.execute(
        *_q_set_last_vlob_update(
            organization_id=organization_id.str,
            realm_id=realm_id,
            user_id=author.user_id.str,
            timestamp=timestamp,
        )
    )

    await send_signal(
        conn,
        BackendEvent.REALM_VLOBS_UPDATED,
        organization_id=organization_id,
        author=author,
        realm_id=realm_id,
        checkpoint=index,
        src_id=src_id,
        src_version=src_version,
    )


_q_get_vlob_version = Q(
    f"""
SELECT
    version,
    created_on
FROM vlob_atom
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND vlob_id = $vlob_id
ORDER BY version DESC LIMIT 1
"""
)


_q_insert_vlob_atom = Q(
    f"""
INSERT INTO vlob_atom (
    organization,
    vlob_encryption_revision,
    vlob_id,
    version,
    blob,
    size,
    author,
    created_on
)
SELECT
    { q_organization_internal_id(organization_id="$organization_id") },
    {
        q_vlob_encryption_revision_internal_id(
            organization_id="$organization_id",
            realm_id="$realm_id",
            encryption_revision="$encryption_revision"
        )
    },
    $vlob_id,
    $version,
    $blob,
    $blob_len,
    { q_device_internal_id(organization_id="$organization_id", device_id="$author") },
    $timestamp
RETURNING _id
"""
)


@query(in_transaction=True)
async def query_update(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    encryption_revision: int,
    vlob_id: VlobID,
    version: int,
    timestamp: DateTime,
    blob: bytes,
    sequester_blob: Dict[SequesterServiceID, bytes] | None,
    now: DateTime,
) -> None:
    realm_id = await _get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
    await _check_realm_and_write_access(
        conn, organization_id, author, realm_id, encryption_revision, timestamp, now
    )

    previous = await conn.fetchrow(
        *_q_get_vlob_version(organization_id=organization_id.str, vlob_id=vlob_id)
    )
    if not previous:
        raise VlobNotFoundError(f"Vlob `{vlob_id.hex}` doesn't exist")

    elif previous["version"] != version - 1:
        raise VlobVersionError()

    elif previous["created_on"] > timestamp:
        raise VlobRequireGreaterTimestampError(previous["created_on"])

    try:
        vlob_atom_internal_id = await conn.fetchval(
            *_q_insert_vlob_atom(
                organization_id=organization_id.str,
                author=author.str,
                realm_id=realm_id,
                encryption_revision=encryption_revision,
                vlob_id=vlob_id,
                blob=blob,
                blob_len=len(blob),
                timestamp=timestamp,
                version=version,
            )
        )

    except UniqueViolationError:
        # Should not occur in theory given we are in a transaction
        raise VlobVersionError()

    if sequester_blob:
        for service_id, blob in sequester_blob.items():
            await conn.fetchval(
                *_q_create_sequester_blob(
                    organization_id=organization_id.str,
                    service_id=service_id,
                    vlob_atom_internal_id=vlob_atom_internal_id,
                    blob=blob,
                )
            )
    await _set_vlob_updated(
        conn, vlob_atom_internal_id, organization_id, author, realm_id, vlob_id, timestamp, version
    )


_q_create = Q(
    f"""
INSERT INTO vlob_atom (
    organization,
    vlob_encryption_revision,
    vlob_id,
    version,
    blob,
    size,
    author,
    created_on
)
SELECT
    { q_organization_internal_id("$organization_id") },
    {
        q_vlob_encryption_revision_internal_id(
            organization_id="$organization_id",
            realm_id="$realm_id",
            encryption_revision="$encryption_revision"
        )
    },
    $vlob_id,
    1,
    $blob,
    $blob_len,
    { q_device_internal_id(organization_id="$organization_id", device_id="$author") },
    $timestamp
RETURNING _id
"""
)


_q_create_sequester_blob = Q(
    f"""
    INSERT INTO sequester_service_vlob_atom(service, vlob_atom, blob)
    SELECT
        (SELECT _id
            FROM sequester_service
            WHERE
                sequester_service.service_id=$service_id
                AND sequester_service.organization={ q_organization_internal_id("$organization_id") }),
        $vlob_atom_internal_id,
        $blob
    RETURNING _id
    """
)


@query(in_transaction=True)
async def query_create(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    encryption_revision: int,
    vlob_id: VlobID,
    timestamp: DateTime,
    blob: bytes,
    sequester_blob: Dict[SequesterServiceID, bytes] | None,
    now: DateTime,
) -> None:
    await _check_realm_and_write_access(
        conn, organization_id, author, realm_id, encryption_revision, timestamp, now
    )

    # Actually create the vlob
    try:
        vlob_atom_internal_id = await conn.fetchval(
            *_q_create(
                organization_id=organization_id.str,
                author=author.str,
                realm_id=realm_id,
                encryption_revision=encryption_revision,
                vlob_id=vlob_id,
                blob=blob,
                blob_len=len(blob),
                timestamp=timestamp,
            )
        )

    except UniqueViolationError:
        raise VlobAlreadyExistsError()

    if sequester_blob:
        for service_id, blob in sequester_blob.items():
            await conn.fetchval(
                *_q_create_sequester_blob(
                    organization_id=organization_id.str,
                    service_id=service_id,
                    vlob_atom_internal_id=vlob_atom_internal_id,
                    blob=blob,
                )
            )
    await _set_vlob_updated(
        conn, vlob_atom_internal_id, organization_id, author, realm_id, vlob_id, timestamp
    )
