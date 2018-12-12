from triopg.exceptions import UniqueViolationError

from parsec.utils import ParsecError

# from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.blockstore import BaseBlockstoreComponent


class PGBlockstoreComponent(BaseBlockstoreComponent):
    def __init__(self, dbh):
        self.dbh = dbh

    async def get(self, id):
        async with self.dbh.pool.acquire() as conn:
            block = await conn.fetchrow("SELECT block FROM blockstore WHERE block_id = $1", id)
            if not block:
                raise NotFoundError("Unknown block id.")
        return block[0]

    async def post(self, id, block):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                try:
                    result = await conn.execute(
                        "INSERT INTO blockstore (block_id, block) VALUES ($1, $2)", id, block
                    )
                    if result != "INSERT 0 1":
                        raise ParsecError("Insertion error.")
                except UniqueViolationError as exc:
                    raise AlreadyExistsError("A block already exists with id `%s`." % id) from exc
