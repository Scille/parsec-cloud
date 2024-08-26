# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_device,
)

_q_dump_vlobs = Q(
    f"""
SELECT
    vlob_atom.vlob_id,
    realm.realm_id,
    { q_device(_id="vlob_atom.author", select="device_id") } AS author,
    vlob_atom.created_on,
    vlob_atom.blob
FROM vlob_atom
INNER JOIN realm
ON realm._id = vlob_atom.realm
INNER JOIN organization
ON organization._id = realm.organization
WHERE organization_id = $organization_id
"""
)


async def vlob_test_dump_vlobs(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> dict[VlobID, list[tuple[DeviceID, DateTime, VlobID, bytes]]]:
    rows = await conn.fetch(*_q_dump_vlobs(organization_id=organization_id.str))
    result: dict[VlobID, list[tuple[DeviceID, DateTime, VlobID, bytes]]] = {}
    for row in rows:
        vlob_id = VlobID.from_hex(row["vlob_id"])
        realm_id = VlobID.from_hex(row["realm_id"])
        author = DeviceID.from_hex(row["author"])
        if vlob_id not in result:
            result[vlob_id] = []
        result[vlob_id].append(
            (
                author,
                row["created_on"],
                realm_id,
                row["blob"],
            )
        )
    return result
