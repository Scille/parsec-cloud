from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.blockstore import BaseBlockStoreComponent


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def get(self, id):
        block = await self.dbh.fetch_one(
            'SELECT * FROM blockstore WHERE id = %s',
            (id,)
        )

        if block is None:
            raise NotFoundError('Unknown block id.')

        return block

    async def post(self, id, block):
        block = await self.dbh.fetch_one(
            'SELECT 1 FROM blockstore WHERE id = %s'
        )

        if block is not None:
            # Should never happen
            raise AlreadyExistsError(
                'A block already exists with id `%s`.' % id
            )

        await self.dbh.insert_one(
            'INSERT INTO blockstore (id, block) VALUES (%s, %s)',
            (id, block)
        )
