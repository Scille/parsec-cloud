# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID

import pendulum
from triopg.exceptions import UniqueViolationError

from parsec.backend.block import (
    BaseBlockComponent,
    BlockAccessError,
    BlockAlreadyExistsError,
    BlockError,
    BlockInMaintenanceError,
    BlockNotFoundError,
)
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.drivers.postgresql.handler import PGHandler
from parsec.backend.drivers.postgresql.realm import RealmNotFoundError, get_realm_status
from parsec.backend.vlob import BaseVlobComponent
from parsec.types import DeviceID, OrganizationID


async def _check_realm(conn, organization_id, realm_id):
    try:
        rep = await get_realm_status(conn, organization_id, realm_id)

    except RealmNotFoundError as exc:
        raise BlockNotFoundError(*exc.args) from exc

    if rep["maintenance_type"]:
        raise BlockInMaintenanceError("Data realm is currently under maintenance")


async def _get_realm_id_from_block_id(conn, organization_id, block_id):
    realm_id = await conn.fetchval(
        """
SELECT get_realm_id(realm)
FROM block
WHERE _id = get_block_internal_id($1, $2)
LIMIT 1
            """,
        organization_id,
        block_id,
    )
    if not realm_id:
        raise BlockNotFoundError(f"Realm `{realm_id}` doesn't exist")
    return realm_id


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
        self, organization_id: OrganizationID, author: DeviceID, block_id: UUID
    ) -> bytes:
        async with self.dbh.pool.acquire() as conn:
            realm_id = await _get_realm_id_from_block_id(conn, organization_id, block_id)
            await _check_realm(conn, organization_id, realm_id)
            ret = await conn.fetchrow(
                """
SELECT
    deleted_on,
    user_can_read_vlob(
        get_user_internal_id($1, $2),
        realm
    )
FROM block
WHERE
    organization = get_organization_internal_id($1)
    AND block_id = $3
""",
                organization_id,
                author.user_id,
                block_id,
            )
            if not ret or ret[0]:
                raise BlockNotFoundError()

            elif not ret[1]:
                raise BlockAccessError()

        return await self._blockstore_component.read(organization_id, block_id)

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: UUID,
        realm_id: UUID,
        block: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await _check_realm(conn, organization_id, realm_id)

            # 1) Check access rights and block unicity
            ret = await conn.fetchrow(
                """
SELECT
    user_can_write_vlob(
        get_user_internal_id($1, $2),
        get_realm_internal_id($1, $4)
    ),
    EXISTS (
        SELECT _id
        FROM block
        WHERE
            organization = get_organization_internal_id($1)
            AND block_id = $3
    )
""",
                organization_id,
                author.user_id,
                block_id,
                realm_id,
            )

            if not ret[0]:
                raise BlockAccessError()

            elif ret[1]:
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
                """
INSERT INTO block (
    organization,
    block_id,
    realm,
    author,
    size,
    created_on
)
SELECT
    get_organization_internal_id($1),
    $3,
    get_realm_internal_id($1, $4),
    get_device_internal_id($1, $2),
    $5,
    $6
""",
                organization_id,
                author,
                block_id,
                realm_id,
                len(block),
                pendulum.now(),
            )

            if ret != "INSERT 0 1":
                raise BlockError(f"Insertion error: {ret}")


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def read(self, organization_id: OrganizationID, id: UUID) -> bytes:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetchrow(
                    """
SELECT data
FROM block_data
WHERE
    organization_id = $1 AND
    block_id = $2
""",
                    organization_id,
                    id,
                )
                if not ret:
                    raise BlockNotFoundError()

                return ret[0]

    async def create(self, organization_id: OrganizationID, id: UUID, block: bytes) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                try:
                    ret = await conn.execute(
                        """
INSERT INTO block_data (
    organization_id,
    block_id,
    data
)
SELECT
    $1,
    $2,
    $3
""",
                        organization_id,
                        id,
                        block,
                    )
                    if ret != "INSERT 0 1":
                        raise BlockError(f"Insertion error: {ret}")

                except UniqueViolationError:
                    # Keep calm and stay idempotent
                    pass
