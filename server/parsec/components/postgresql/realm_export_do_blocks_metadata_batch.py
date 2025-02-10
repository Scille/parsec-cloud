# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    BlockID,
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q, q_device
from parsec.components.realm import (
    RealmExportBatchOffsetMarker,
    RealmExportBlocksMetadataBatch,
    RealmExportBlocksMetadataBatchItem,
    RealmExportDoBlocksBatchMetadataBadOutcome,
)

_q_get_org_and_realm = Q(
    """
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
        organization = (SELECT _id FROM my_organization)
        AND realm_id = $realm_id
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT _id FROM my_realm) AS realm_internal_id
"""
)


_q_get_blocks_metadata_batch = Q(f"""
SELECT
    _id as block_internal_id,
    block_id,
    {q_device(_id="author", select="device_id")} AS author,
    key_index,
    size
FROM block
WHERE
    realm = $realm_internal_id
    AND _id > $batch_offset_marker
ORDER BY _id
LIMIT $batch_size
""")


async def realm_export_do_blocks_metadata_batch(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    batch_offset_marker: RealmExportBatchOffsetMarker,
    batch_size: int,
) -> RealmExportBlocksMetadataBatch | RealmExportDoBlocksBatchMetadataBadOutcome:
    row = await conn.fetchrow(
        *_q_get_org_and_realm(organization_id=organization_id.str, realm_id=realm_id)
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int():
            pass
        case None:
            return RealmExportDoBlocksBatchMetadataBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case None:
            return RealmExportDoBlocksBatchMetadataBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, unknown

    rows = await conn.fetch(
        *_q_get_blocks_metadata_batch(
            realm_internal_id=realm_internal_id,
            batch_offset_marker=batch_offset_marker,
            batch_size=batch_size,
        )
    )
    assert rows is not None

    items: list[RealmExportBlocksMetadataBatchItem] = []
    for row in rows:
        match row["block_internal_id"]:
            case int() as block_internal_id:
                pass
            case unknown:
                assert False, unknown

        match row["block_id"]:
            case str() as raw_block_id:
                block_id = BlockID.from_hex(raw_block_id)
            case unknown:
                assert False, unknown

        match row["author"]:
            case str() as raw_author:
                author = DeviceID.from_hex(raw_author)
            case unknown:
                assert False, unknown

        match row["key_index"]:
            case int() as key_index:
                pass
            case unknown:
                assert False, unknown

        match row["size"]:
            case int() as size:
                pass
            case unknown:
                assert False, unknown

        items.append(
            RealmExportBlocksMetadataBatchItem(
                sequential_id=block_internal_id,
                block_id=block_id,
                author=author,
                key_index=key_index,
                size=size,
            )
        )

    return RealmExportBlocksMetadataBatch(
        items=items,
    )
