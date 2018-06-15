from parsec.utils import ParsecError
from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.blockstore import BaseBlockStoreComponent


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def get(self, id):
        async with self.dbh.pool.acquire() as conn:
            block = await conn.fetchrow("SELECT block FROM blockstore WHERE id = $1", id)
            if not block:
                raise NotFoundError("Unknown block id.")
        return block[0]

    async def post(self, id, block):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                exists = await conn.fetchrow("SELECT 1 FROM blockstore WHERE id = $1", id)

                if exists:
                    # Should never happen
                    raise AlreadyExistsError("A block already exists with id `%s`." % id)

                result = await conn.execute(
                    "INSERT INTO blockstore (id, block) VALUES ($1, $2)", id, block
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")
