from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.blockstore import BaseBlockStoreComponent


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def get(self, id):
        try:
            block, = await self.dbh.fetch_one("SELECT block FROM blockstore WHERE id = %s", (id,))
        except (TypeError, ValueError):
            raise NotFoundError("Unknown block id.")

        return block.tobytes()

    async def post(self, id, block):
        # TODO: non atomic operation !
        exists = await self.dbh.fetch_one("SELECT 1 FROM blockstore WHERE id = %s", (id,))

        if exists is not None:
            # Should never happen
            raise AlreadyExistsError("A block already exists with id `%s`." % id)

        await self.dbh.insert_one("INSERT INTO blockstore (id, block) VALUES (%s, %s)", (id, block))
