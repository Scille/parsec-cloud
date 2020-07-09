# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from triopg import UniqueViolationError
from uuid import UUID

from parsec.api.protocol import DeviceID, OrganizationID
from parsec.backend.postgresql.utils import (
    Q,
    query,
    q_organization_internal_id,
    q_device_internal_id,
    q_realm_internal_id,
    q_vlob_encryption_revision_internal_id,
)
from parsec.backend.vlob import VlobVersionError, VlobTimestampError, VlobNotFoundError
from parsec.backend.postgresql.handler import send_signal
from parsec.backend.postgresql.vlob_queries.utils import query_get_realm_id_from_vlob_id
from parsec.backend.backend_events import BackendEvent


q_vlob_updated = Q(
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


@query()
async def query_vlob_updated(
    conn, vlob_atom_internal_id, organization_id, author, realm_id, src_id, src_version=1
):

    index = await conn.fetchval(
        *q_vlob_updated(
            organization_id=organization_id,
            realm_id=realm_id,
            vlob_atom_internal_id=vlob_atom_internal_id,
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


@query()
async def query_update(
    conn,
    organization_id: OrganizationID,
    author: DeviceID,
    encryption_revision: int,
    vlob_id: UUID,
    version: int,
    timestamp: pendulum.Pendulum,
    blob: bytes,
) -> None:
    from parsec.backend.postgresql.vlob import _check_realm_and_write_access

    async with conn.transaction():
        realm_id = await query_get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
        await _check_realm_and_write_access(
            conn, organization_id, author, realm_id, encryption_revision
        )

        previous = await conn.fetchrow(
            *_q_get_vlob_version(organization_id=organization_id, vlob_id=vlob_id)
        )
        if not previous:
            raise VlobNotFoundError(f"Vlob `{vlob_id}` doesn't exist")

        elif previous["version"] != version - 1:
            raise VlobVersionError()

        elif previous["created_on"] > timestamp:
            raise VlobTimestampError()

        try:
            vlob_atom_internal_id = await conn.fetchval(
                *_q_insert_vlob_atom(
                    organization_id=organization_id,
                    author=author,
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

        await query_vlob_updated(
            conn, vlob_atom_internal_id, organization_id, author, realm_id, vlob_id, version
        )
