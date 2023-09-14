# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import triopg
from triopg.exceptions import UniqueViolationError

from parsec._parsec import DateTime
from parsec.api.protocol import BlockID, DeviceID, OrganizationID, RealmID
from parsec.backend.block import (
    BaseBlockComponent,
    BlockAccessError,
    BlockAlreadyExistsError,
    BlockError,
    BlockInMaintenanceError,
    BlockNotFoundError,
    BlockRealmArchivedError,
    BlockRealmDeletedError,
    BlockStoreError,
)
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.realm_queries.archiving import (
    RealmArchivedError,
    RealmDeletedError,
    check_archiving_configuration,
)
from parsec.backend.postgresql.realm_queries.maintenance import RealmNotFoundError, get_realm_status
from parsec.backend.postgresql.utils import (
    Q,
    q_block,
    q_device_internal_id,
    q_organization_internal_id,
    q_realm,
    q_realm_internal_id,
    q_user_can_read_vlob,
    q_user_can_write_vlob,
    q_user_internal_id,
)
from parsec.backend.utils import OperationKind

_q_get_realm_id_from_block_id = Q(
    f"""
SELECT
    { q_realm(_id="block.realm", select="realm.realm_id") }
FROM block
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND block_id = $block_id
"""
)


_q_get_block_meta = Q(
    f"""
SELECT
    deleted_on,
    {
        q_user_can_read_vlob(
            user=q_user_internal_id(
                organization_id="$organization_id",
                user_id="$user_id"
            ),
            realm="block.realm"
        )
    } as has_access
FROM block
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND block_id = $block_id
"""
)


_q_get_block_write_right_and_unicity = Q(
    f"""
SELECT
    {
        q_user_can_write_vlob(
            organization_id="$organization_id",
            user_id="$user_id",
            realm_id="$realm_id"
        )
    } as has_access,
    EXISTS({
        q_block(
            organization_id="$organization_id",
            block_id="$block_id"
        )
    }) as exists
"""
)


_q_insert_block = Q(
    f"""
INSERT INTO block (organization, block_id, realm, author, size, created_on)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $block_id,
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_device_internal_id(organization_id="$organization_id", device_id="$author") },
    $size,
    $created_on
)
"""
)


async def _check_realm(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    realm_id: RealmID,
    operation_kind: OperationKind,
    now: DateTime,
) -> None:
    # Fetch the realm status maintenance type
    try:
        status = await get_realm_status(conn, organization_id, realm_id)
    except RealmNotFoundError as exc:
        raise BlockNotFoundError(*exc.args) from exc

    # Check archiving status
    try:
        await check_archiving_configuration(conn, organization_id, realm_id, operation_kind, now)
    except RealmArchivedError:
        raise BlockRealmArchivedError()
    except RealmDeletedError:
        raise BlockRealmDeletedError()

    # Special case of reading while in reencryption is authorized
    if operation_kind == OperationKind.DATA_READ and status.in_reencryption:
        pass

    # Access is not allowed while in maintenance
    elif status.in_maintenance:
        raise BlockInMaintenanceError("Data realm is currently under maintenance")


class PGBlockComponent(BaseBlockComponent):
    def __init__(self, dbh: PGHandler, blockstore_component: BaseBlockStoreComponent):
        self.dbh = dbh
        self._blockstore_component = blockstore_component

    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: BlockID, now: DateTime
    ) -> bytes:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            realm_id_uuid = await conn.fetchval(
                *_q_get_realm_id_from_block_id(
                    organization_id=organization_id.str, block_id=block_id
                )
            )
            if not realm_id_uuid:
                raise BlockNotFoundError()
            realm_id = RealmID.from_hex(realm_id_uuid)
            await _check_realm(conn, organization_id, realm_id, OperationKind.DATA_READ, now)
            ret = await conn.fetchrow(
                *_q_get_block_meta(
                    organization_id=organization_id.str,
                    block_id=block_id,
                    user_id=author.user_id.str,
                )
            )
            if not ret or ret["deleted_on"]:
                raise BlockNotFoundError()

            elif not ret["has_access"]:
                raise BlockAccessError()

        # We can do the blockstore read outside of the transaction given the block
        # are never modified/removed
        return await self._blockstore_component.read(organization_id, block_id)

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        realm_id: RealmID,
        block: bytes,
        now: DateTime,
    ) -> None:
        created_on = now
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await _check_realm(conn, organization_id, realm_id, OperationKind.DATA_WRITE, now)

            # 1) Check access rights and block unicity
            # Note it's important to check unicity here because blockstore create
            # overwrite existing data !
            ret = await conn.fetchrow(
                *_q_get_block_write_right_and_unicity(
                    organization_id=organization_id.str,
                    user_id=author.user_id.str,
                    realm_id=realm_id,
                    block_id=block_id,
                )
            )

            if not ret["has_access"]:
                raise BlockAccessError()

            elif ret["exists"]:
                raise BlockAlreadyExistsError()

            # 2) Upload block data in blockstore under an arbitrary id
            # Given block metadata and block data are stored on different storages,
            # being atomic is not easy here :(
            # For instance step 2) can be successful (or can be successful on *some*
            # blockstores in case of a RAID blockstores configuration) but step 4) fails.
            # This is solved by the fact blockstores are considered idempotent and two
            # create operations with the same orgID/ID couple are expected to have the
            # same block data.
            # Hence any blockstore create failure result in postgres transaction
            # cancellation, and blockstore create success can be overwritten by another
            # create in case the postgres transaction was cancelled in step 3)
            await self._blockstore_component.create(organization_id, block_id, block)

            # 3) Insert the block metadata into the database
            try:
                ret = await conn.execute(
                    *_q_insert_block(
                        organization_id=organization_id.str,
                        block_id=block_id,
                        realm_id=realm_id,
                        author=author.str,
                        size=len(block),
                        created_on=created_on,
                    )
                )
            except UniqueViolationError as exc:
                # Given step 1) hasn't locked anything in the database, concurrent block
                # create operations may end up with one of them in unique violation error
                raise BlockAlreadyExistsError() from exc

            if ret != "INSERT 0 1":
                raise BlockError(f"Insertion error: {ret}")


_q_get_block_data = Q(
    """
SELECT
    data
FROM block_data
WHERE
    organization_id = $organization_id
    AND block_id = $block_id
"""
)


_q_insert_block_data = Q(
    """
INSERT INTO block_data (organization_id, block_id, data)
VALUES ($organization_id, $block_id, $data)
"""
)


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def read(self, organization_id: OrganizationID, block_id: BlockID) -> bytes:
        async with self.dbh.pool.acquire() as conn:
            ret = await conn.fetchrow(
                *_q_get_block_data(organization_id=organization_id.str, block_id=block_id)
            )
            if not ret:
                raise BlockStoreError("Block not found")

            return ret[0]

    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            try:
                ret = await conn.execute(
                    *_q_insert_block_data(
                        organization_id=organization_id.str, block_id=block_id, data=block
                    )
                )
                if ret != "INSERT 0 1":
                    raise BlockStoreError(f"Insertion error: {ret}")

            except UniqueViolationError:
                # Keep calm and stay idempotent
                pass
