from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.blockstore import BaseBlockStoreComponent
from .handler import atomic


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    @atomic
    async def get(self, conn, id):
        try:
            block, = await self.dbh.fetch_one(
                conn, "SELECT block FROM blockstore WHERE id = $1", id
            )
        except (TypeError, ValueError):
            raise NotFoundError("Unknown block id.")

        return block

    @atomic
    async def post(self, conn, id, block):
        # TODO: non atomic operation !
        exists = await self.dbh.fetch_one(conn, "SELECT 1 FROM blockstore WHERE id = $1", id)

        if exists is not None:
            # Should never happen
            raise AlreadyExistsError("A block already exists with id `%s`." % id)

        await self.dbh.insert_one(
            conn, "INSERT INTO blockstore (id, block) VALUES ($1, $2)", id, block
        )
