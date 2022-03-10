# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from triopg.exceptions import UniqueViolationError
import pendulum

from parsec.api.protocol import OrganizationID, DeviceID, RealmID, BlockID
from parsec.backend.utils import OperationKind
from parsec.backend.vlob import BaseVlobComponent
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.block import (
    BaseBlockComponent,
    BlockError,
    BlockAlreadyExistsError,
    BlockNotFoundError,
    BlockAccessError,
    BlockInMaintenanceError,
)
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.utils import (
    Q,
    q_organization_internal_id,
    q_user_internal_id,
    q_user_can_read_vlob,
    q_user_can_write_vlob,
    q_device_internal_id,
    q_realm,
    q_realm_internal_id,
    q_block,
)
from parsec.backend.postgresql.realm_queries.maintenance import get_realm_status, RealmNotFoundError


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
    conn, organization_id: OrganizationID, realm_id: RealmID, operation_kind: OperationKind
) -> None:
    # Fetch the realm status maintenance type
    try:
        status = await get_realm_status(conn, organization_id, realm_id)
    except RealmNotFoundError as exc:
        raise BlockNotFoundError(*exc.args) from exc

    # Special case of reading while in reencryption is authorized
    if operation_kind == OperationKind.DATA_READ and status.in_reencryption:
        pass

    # Access is not allowed while in maintenance
    elif status.in_maintenance:
        raise BlockInMaintenanceError("Data realm is currently under maintenance")


class PGBlockComponent(BaseBlockComponent):
    def __init__(
        self,
        dbh: PGHandler,
        blockstore_component: BaseBlockStoreComponent,
        vlob_component: BaseVlobComponent,
    ):
        self.dbh = dbh
        self._blockstore_component = blockstore_component
        self._vlob_component = vlob_component

    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: BlockID
    ) -> bytes:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            realm_id_uuid = await conn.fetchval(
                *_q_get_realm_id_from_block_id(
                    organization_id=organization_id.str, block_id=block_id.uuid
                )
            )
            if not realm_id_uuid:
                raise BlockNotFoundError()
            realm_id = RealmID(realm_id_uuid)
            await _check_realm(conn, organization_id, realm_id, OperationKind.DATA_READ)
            ret = await conn.fetchrow(
                *_q_get_block_meta(
                    organization_id=organization_id.str,
                    block_id=block_id.uuid,
                    user_id=author.user_id.str,
                )
            )
            if not ret or ret["deleted_on"]:
                raise BlockNotFoundError()

            elif not ret["has_access"]:
                raise BlockAccessError()

        return await self._blockstore_component.read(organization_id, block_id)

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        realm_id: RealmID,
        block: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await _check_realm(conn, organization_id, realm_id, OperationKind.DATA_WRITE)

            # 1) Check access rights and block unicity
            ret = await conn.fetchrow(
                *_q_get_block_write_right_and_unicity(
                    organization_id=organization_id.str,
                    user_id=author.user_id.str,
                    realm_id=realm_id.uuid,
                    block_id=block_id.uuid,
                )
            )

            if not ret["has_access"]:
                raise BlockAccessError()

            elif ret["exists"]:
                raise BlockAlreadyExistsError()

            # 2) Upload block data in blockstore under an arbitrary id
            # Given block metadata and block data are stored on different
            # storages, beeing atomic is not easy here :(
            # For instance step 2) can be successful (or can be successful on
            # *some* blockstores in case of a RAID blockstores configuration)
            # but step 4) fails. To avoid deadlock in such case (i.e.
            # blockstores with existing block raise `BlockAlreadyExistsError`)
            # blockstore are idempotent (i.e. if a block id already exists a
            # blockstore return success without any modification).
            await self._blockstore_component.create(organization_id, block_id, block)

            # 3) Insert the block metadata into the database
            ret = await conn.execute(
                *_q_insert_block(
                    organization_id=organization_id.str,
                    block_id=block_id.uuid,
                    realm_id=realm_id.uuid,
                    author=author.str,
                    size=len(block),
                    created_on=pendulum.now(),
                )
            )

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

    async def read(self, organization_id: OrganizationID, id: BlockID) -> bytes:
        async with self.dbh.pool.acquire() as conn:
            ret = await conn.fetchrow(
                *_q_get_block_data(organization_id=organization_id.str, block_id=id.uuid)
            )
            if not ret:
                raise BlockNotFoundError()

            return ret[0]

    async def create(self, organization_id: OrganizationID, id: BlockID, block: bytes) -> None:
        async with self.dbh.pool.acquire() as conn:
            try:
                ret = await conn.execute(
                    *_q_insert_block_data(
                        organization_id=organization_id.str, block_id=id.uuid, data=block
                    )
                )
                if ret != "INSERT 0 1":
                    raise BlockError(f"Insertion error: {ret}")

            except UniqueViolationError:
                # Keep calm and stay idempotent
                pass
