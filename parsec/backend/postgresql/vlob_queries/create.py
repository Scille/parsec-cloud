# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from uuid import UUID
from triopg import UniqueViolationError

from parsec.api.protocol import DeviceID, OrganizationID
from parsec.backend.vlob import VlobAlreadyExistsError
from parsec.backend.postgresql.queries import (
    Q,
    q_organization_internal_id,
    q_realm_internal_id,
    q_device_internal_id,
)
from parsec.backend.postgresql.vlob_queries.write import query_vlob_updated


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
    (SELECT _id
        FROM vlob_encryption_revision
            WHERE realm= { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
            AND encryption_revision=$encryption_revision),
    $vlob_id,
    1,
    $blob,
    $blob_len,
    { q_device_internal_id(organization_id="$organization_id", device_id="$author") },
    $timestamp
RETURNING _id
"""
)

# .format(
#                q_organization_internal_id(organization_id=Parameter("$1")),
#                Query.from_(t_vlob_encryption_revision)
#                .where(
#                    (
#                        t_vlob_encryption_revision.realm
#                        == q_realm_internal_id(
#                            organization_id=Parameter("$1"), realm_id=Parameter("$3")
#                        )
#                    )
#                    & (t_vlob_encryption_revision.encryption_revision == Parameter("$4"))
#                )
#                .select("_id"),
#                q_device_internal_id(
#                    organization_id=Parameter("$1"), device_id=Parameter("$2")
#                ),
#            )

print(_q_create)


async def query_create(
    conn,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: UUID,
    encryption_revision: int,
    vlob_id: UUID,
    timestamp: pendulum.Pendulum,
    blob: bytes,
) -> None:
    from parsec.backend.postgresql.vlob import _check_realm_and_write_access

    async with conn.transaction():

        await _check_realm_and_write_access(
            conn, organization_id, author, realm_id, encryption_revision
        )

        # Actually create the vlob
        try:
            vlob_atom_internal_id = await conn.fetchval(
                *_q_create(
                    organization_id=organization_id,
                    author=author,
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

        await query_vlob_updated(
            conn, vlob_atom_internal_id, organization_id, author, realm_id, vlob_id
        )
