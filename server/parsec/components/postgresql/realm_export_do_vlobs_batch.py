# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q, q_device
from parsec.components.realm import (
    RealmExportBatchOffsetMarker,
    RealmExportDoVlobsBatchBadOutcome,
    RealmExportVlobsBatch,
    RealmExportVlobsBatchItem,
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


_q_get_vlobs_batch = Q(
    f"""
SELECT
    _id as vlob_atom_internal_id,
    vlob_id,
    version,
    key_index,
    blob,
    {q_device(_id="author", select="device_id")} AS author,
    created_on
FROM vlob_atom
WHERE
    realm = $realm_internal_id
    AND _id > $batch_offset_marker
ORDER BY _id
LIMIT $batch_size
"""
)


async def realm_export_do_vlobs_batch(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    batch_offset_marker: RealmExportBatchOffsetMarker,
    batch_size: int,
) -> RealmExportVlobsBatch | RealmExportDoVlobsBatchBadOutcome:
    row = await conn.fetchrow(
        *_q_get_org_and_realm(organization_id=organization_id.str, realm_id=realm_id)
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int():
            pass
        case None:
            return RealmExportDoVlobsBatchBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case None:
            return RealmExportDoVlobsBatchBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, unknown

    rows = await conn.fetch(
        *_q_get_vlobs_batch(
            realm_internal_id=realm_internal_id,
            batch_offset_marker=batch_offset_marker,
            batch_size=batch_size,
        )
    )
    assert rows is not None

    items: list[RealmExportVlobsBatchItem] = []
    for row in rows:
        match row["vlob_atom_internal_id"]:
            case int() as vlob_atom_internal_id:
                pass
            case unknown:
                assert False, unknown

        match row["vlob_id"]:
            case str() as raw_vlob_id:
                vlob_id = VlobID.from_hex(raw_vlob_id)
            case unknown:
                assert False, unknown

        match row["version"]:
            case int() as version:
                pass
            case unknown:
                assert False, unknown

        match row["key_index"]:
            case int() as key_index:
                pass
            case unknown:
                assert False, unknown

        match row["blob"]:
            case bytes() as blob:
                pass
            case unknown:
                assert False, unknown

        match row["author"]:
            case str() as raw_author:
                author = DeviceID.from_hex(raw_author)
            case unknown:
                assert False, unknown

        match row["created_on"]:
            case DateTime() as created_on:
                pass
            case unknown:
                assert False, unknown

        items.append(
            RealmExportVlobsBatchItem(
                sequential_id=vlob_atom_internal_id,
                vlob_id=vlob_id,
                version=version,
                key_index=key_index,
                blob=blob,
                size=len(blob),
                author=author,
                timestamp=created_on,
            )
        )

    return RealmExportVlobsBatch(
        items=items,
    )
