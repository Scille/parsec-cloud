# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import triopg
from typing import Dict, Tuple, Optional

from parsec._parsec import DateTime
from parsec.api.protocol import OrganizationID, DeviceID, VlobID, RealmID
from parsec.backend.vlob import VlobVersionError, VlobNotFoundError
from parsec.backend.postgresql.utils import (
    Q,
    query,
    q_device,
    q_realm_internal_id,
    q_organization_internal_id,
    q_vlob_encryption_revision_internal_id,
)
from parsec.backend.postgresql.vlob_queries.utils import (
    _get_realm_id_from_vlob_id,
    _check_realm_and_read_access,
    _get_last_role_granted_on,
)


_q_read_data_without_timestamp = Q(
    f"""
SELECT
    version,
    blob,
    { q_device(_id="author", select="device_id") } as author,
    created_on
FROM vlob_atom
WHERE
    vlob_encryption_revision = {
        q_vlob_encryption_revision_internal_id(
            organization_id="$organization_id",
            realm_id="$realm_id",
            encryption_revision="$encryption_revision",
        )
    }
    AND vlob_id = $vlob_id
ORDER BY version DESC
LIMIT 1
"""
)

_q_read_data_with_timestamp = Q(
    f"""
SELECT
    version,
    blob,
    { q_device(_id="author", select="device_id") } as author,
    created_on
FROM vlob_atom
WHERE
    vlob_encryption_revision = {
        q_vlob_encryption_revision_internal_id(
            organization_id="$organization_id",
            realm_id="$realm_id",
            encryption_revision="$encryption_revision",
        )
    }
    AND vlob_id = $vlob_id
    AND created_on <= $timestamp
ORDER BY version DESC
LIMIT 1
"""
)


_q_read_data_with_version = Q(
    f"""
SELECT
    version,
    blob,
    { q_device(_id="author", select="device_id") } as author,
    created_on
FROM vlob_atom
WHERE
    vlob_encryption_revision = {
        q_vlob_encryption_revision_internal_id(
            organization_id="$organization_id",
            realm_id="$realm_id",
            encryption_revision="$encryption_revision",
        )
    }
    AND vlob_id = $vlob_id
    AND version = $version
"""
)


@query(in_transaction=True)
async def query_read(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    encryption_revision: int,
    vlob_id: VlobID,
    version: Optional[int] = None,
    timestamp: Optional[DateTime] = None,
) -> Tuple[int, bytes, DeviceID, DateTime, DateTime]:
    realm_id = await _get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
    await _check_realm_and_read_access(conn, organization_id, author, realm_id, encryption_revision)

    if version is None:
        if timestamp is None:
            data = await conn.fetchrow(
                *_q_read_data_without_timestamp(
                    organization_id=organization_id.str,
                    realm_id=realm_id.uuid,
                    encryption_revision=encryption_revision,
                    vlob_id=vlob_id,
                )
            )
            assert data  # _get_realm_id_from_vlob_id checks vlob presence

        else:
            data = await conn.fetchrow(
                *_q_read_data_with_timestamp(
                    organization_id=organization_id.str,
                    realm_id=realm_id.uuid,
                    encryption_revision=encryption_revision,
                    vlob_id=vlob_id.uuid,
                    timestamp=timestamp,
                )
            )
            if not data:
                raise VlobVersionError()

    else:
        data = await conn.fetchrow(
            *_q_read_data_with_version(
                organization_id=organization_id.str,
                realm_id=realm_id.uuid,
                encryption_revision=encryption_revision,
                vlob_id=vlob_id.uuid,
                version=version,
            )
        )
        if not data:
            raise VlobVersionError()

    version, blob, vlob_author, created_on = data
    assert isinstance(version, int)
    assert isinstance(blob, bytes)
    vlob_author = DeviceID(vlob_author)

    author_last_role_granted_on = await _get_last_role_granted_on(
        conn, organization_id, realm_id, vlob_author
    )
    assert isinstance(author_last_role_granted_on, DateTime)
    return version, blob, vlob_author, created_on, author_last_role_granted_on


_q_poll_changes = Q(
    f"""
SELECT
    index,
    vlob_id,
    vlob_atom.version
FROM realm_vlob_update
LEFT JOIN vlob_atom ON realm_vlob_update.vlob_atom = vlob_atom._id
WHERE
    realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    AND index > $checkpoint
ORDER BY index ASC
"""
)


_q_list_versions = Q(
    f"""
SELECT
    version,
    { q_device(_id="author", select="device_id") } as author,
    created_on
FROM vlob_atom
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND vlob_id = $vlob_id
ORDER BY version DESC
"""
)


@query(in_transaction=True)
async def query_poll_changes(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: RealmID,
    checkpoint: int,
) -> Tuple[int, Dict[VlobID, int]]:
    await _check_realm_and_read_access(conn, organization_id, author, realm_id, None)

    ret = await conn.fetch(
        *_q_poll_changes(
            organization_id=organization_id.str, realm_id=realm_id.uuid, checkpoint=checkpoint
        )
    )

    changes_since_checkpoint: Dict[VlobID, int] = {
        VlobID(src_id): src_version for _, src_id, src_version in ret
    }
    new_checkpoint: int = ret[-1][0] if ret else checkpoint
    return (new_checkpoint, changes_since_checkpoint)


@query(in_transaction=True)
async def query_list_versions(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    author: DeviceID,
    vlob_id: VlobID,
) -> Dict[int, Tuple[DateTime, DeviceID]]:
    realm_id = await _get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
    await _check_realm_and_read_access(conn, organization_id, author, realm_id, None)

    rows = await conn.fetch(
        *_q_list_versions(organization_id=organization_id.str, vlob_id=vlob_id.uuid)
    )
    assert rows

    if not rows:
        raise VlobNotFoundError(f"Vlob `{vlob_id.str}` doesn't exist")

    return {
        row["version"]: (
            row["created_on"],
            DeviceID(row["author"]),
        )
        for row in rows
    }
