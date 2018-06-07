from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.blockstore import BaseBlockStoreComponent


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def get(self, id):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                try:
                    block, = await self.dbh.fetch_one(
                        conn, "SELECT block FROM blockstore WHERE id = $1", id
                    )
                except (TypeError, ValueError):
                    raise NotFoundError("Unknown block id.")

        return block

    async def post(self, id, block):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                exists = await self.dbh.fetch_one(
                    conn, "SELECT 1 FROM blockstore WHERE id = $1", id
                )

                if exists is not None:
                    # Should never happen
                    raise AlreadyExistsError("A block already exists with id `%s`." % id)

                await self.dbh.insert_one(
                    conn, "INSERT INTO blockstore (id, block) VALUES ($1, $2)", id, block
                )
