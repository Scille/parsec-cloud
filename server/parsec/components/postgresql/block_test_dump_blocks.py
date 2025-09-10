# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_device,
    q_organization_internal_id,
)

_q_get_all_block_meta = Q(
    f"""
SELECT
    block.block_id,
    block.size,
    block.created_on,
    block.key_index,
    realm.realm_id,
    {q_device(_id="block.author", select="device_id")} AS author  -- noqa: LT02,LT14
FROM block
LEFT JOIN realm ON block.realm = realm._id
WHERE
    realm.organization = {q_organization_internal_id("$organization_id")}  -- noqa: LT05,LT14
"""
)


async def block_test_dump_blocks(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> dict[BlockID, tuple[DateTime, DeviceID, VlobID, int, int]]:
    rows = await conn.fetch(*_q_get_all_block_meta(organization_id=organization_id.str))

    items = {}
    for row in rows:
        block_id = BlockID.from_hex(row["block_id"])
        items[block_id] = (
            row["created_on"],
            DeviceID.from_hex(row["author"]),
            VlobID.from_hex(row["realm_id"]),
            int(row["key_index"]),
            int(row["size"]),
        )

    return items
