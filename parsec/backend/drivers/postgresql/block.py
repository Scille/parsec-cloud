# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from triopg.exceptions import UniqueViolationError
from uuid import UUID
import pendulum

from parsec.types import DeviceID, OrganizationID
from parsec.backend.vlob import BaseVlobComponent
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.block import (
    BaseBlockComponent,
    BlockError,
    BlockAlreadyExistsError,
    BlockNotFoundError,
    BlockAccessError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler


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

    async def read(self, organization_id: OrganizationID, author: DeviceID, id: UUID) -> bytes:
        async with self.dbh.pool.acquire() as conn:
            ret = await conn.fetchrow(
                """
SELECT
    deleted_on,
    user_has_vlob_group_read_right(
        get_user_internal_id($1, $2),
        vlob_group
    )
FROM block
WHERE
    organization = get_organization_internal_id($1)
    AND block_id = $3
""",
                organization_id,
                author.user_id,
                id,
            )
            if not ret or ret[0]:
                raise BlockNotFoundError()

            elif not ret[1]:
                raise BlockAccessError()

        return await self._blockstore_component.read(organization_id, id)

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        id: UUID,
        vlob_group: UUID,
        block: bytes,
    ) -> None:

        async with self.dbh.pool.acquire() as conn:
            # 1) Check access rights and block unicity
            ret = await conn.fetchrow(
                """
SELECT
    user_has_vlob_group_write_right(
        get_user_internal_id($1, $2),
        get_vlob_group_internal_id($1, $4)
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
                id,
                vlob_group,
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
            await self._blockstore_component.create(organization_id, id, block)

            # 3) Insert the block metadata into the database
            ret = await conn.execute(
                """
INSERT INTO block (
    organization,
    block_id,
    vlob_group,
    author,
    size,
    created_on
)
SELECT
    get_organization_internal_id($1),
    $3,
    get_vlob_group_internal_id($1, $4),
    get_device_internal_id($1, $2),
    $5,
    $6
""",
                organization_id,
                author,
                id,
                vlob_group,
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
