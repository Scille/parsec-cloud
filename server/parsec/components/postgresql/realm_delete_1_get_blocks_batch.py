# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    BlockID,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.realm import (
    RealmDelete1GetBlocksBatchBadOutcome,
    RealmDeleteGetBlocksBatch,
    RealmDeleteGetBlocksBatchOffsetMarker,
)

_q_get_org_and_realm = Q("""
WITH my_organization AS (
    SELECT _id
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

my_realm AS (
    SELECT _id
    FROM realm
    WHERE
        organization = (SELECT my_organization._id FROM my_organization)
        AND realm_id = $realm_id
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT _id FROM my_realm) AS realm_internal_id
""")


_q_get_blocks_batch = Q("""
SELECT
    _id AS block_internal_id,
    block_id
FROM block
WHERE
    realm = $realm_internal_id
    AND _id > $batch_offset_marker
ORDER BY _id
LIMIT $batch_size
""")


async def realm_delete_1_get_blocks_batch(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    batch_offset_marker: RealmDeleteGetBlocksBatchOffsetMarker,
    batch_size: int,
) -> RealmDeleteGetBlocksBatch | RealmDelete1GetBlocksBatchBadOutcome:
    row = await conn.fetchrow(
        *_q_get_org_and_realm(organization_id=organization_id.str, realm_id=realm_id)
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int():
            pass
        case None:
            return RealmDelete1GetBlocksBatchBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, row

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case None:
            return RealmDelete1GetBlocksBatchBadOutcome.REALM_NOT_FOUND
        case _:
            assert False, row

    rows = await conn.fetch(
        *_q_get_blocks_batch(
            realm_internal_id=realm_internal_id,
            batch_offset_marker=batch_offset_marker,
            batch_size=batch_size,
        )
    )
    assert rows is not None

    blocks: list[BlockID] = []
    if not rows:
        last_block_internal_id = batch_offset_marker
    else:
        match rows[-1]["block_internal_id"]:
            case int() as last_block_internal_id:
                pass
            case _:
                assert False, row
    for row in rows:
        match row["block_id"]:
            case str() as raw_block_id:
                blocks.append(BlockID.from_hex(raw_block_id))
            case _:
                assert False, row

    return RealmDeleteGetBlocksBatch(
        blocks=blocks,
        batch_offset_marker=last_block_internal_id,
    )
