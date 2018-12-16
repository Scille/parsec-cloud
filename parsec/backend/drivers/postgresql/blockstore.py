from triopg.exceptions import UniqueViolationError
from uuid import UUID

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

    async def read(self, id: UUID) -> bytes:
        async with self.dbh.pool.acquire() as conn:
            block = await conn.fetchrow("SELECT block FROM blockstore WHERE block_id = $1", id)
            if not block:
                raise BlockstoreNotFoundError()
        return block[0]

    async def create(self, id: UUID, block: bytes) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                try:
                    result = await conn.execute(
                        "INSERT INTO blockstore (block_id, block) VALUES ($1, $2)", id, block
                    )
                    if result != "INSERT 0 1":
                        raise BlockstoreError(f"Insertion error: {result}")
                except UniqueViolationError as exc:
                    raise BlockstoreAlreadyExistsError() from exc
