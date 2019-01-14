from triopg.exceptions import UniqueViolationError
from uuid import UUID

from parsec.types import DeviceID, OrganizationID
from parsec.backend.blockstore import (
    BlockstoreError,
    BaseBlockstoreComponent,
    BlockstoreAlreadyExistsError,
    BlockstoreNotFoundError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler


class PGBlockstoreComponent(BaseBlockstoreComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def read(self, organization_id: OrganizationID, id: UUID) -> bytes:
        async with self.dbh.pool.acquire() as conn:
            block = await conn.fetchrow(
                """
SELECT block
FROM blockstore
WHERE
    organization = (
        SELECT _id from organizations WHERE organization_id = $1
    )
    AND block_id = $2
""",
                organization_id,
                id,
            )
            if not block:
                raise BlockstoreNotFoundError()
        return block[0]

    async def create(
        self, organization_id: OrganizationID, id: UUID, block: bytes, author: DeviceID
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                try:
                    result = await conn.execute(
                        """
INSERT INTO blockstore (
    organization,
    block_id,
    block,
    author
)
SELECT
    _id,
    $2,
    $3,
    (
        SELECT _id
        FROM devices
        WHERE
            organization = organizations._id
            AND device_id = $4
    )
FROM organizations
WHERE organization_id = $1
""",
                        organization_id,
                        id,
                        block,
                        author,
                    )
                    if result != "INSERT 0 1":
                        raise BlockstoreError(f"Insertion error: {result}")
                except UniqueViolationError as exc:
                    raise BlockstoreAlreadyExistsError() from exc
