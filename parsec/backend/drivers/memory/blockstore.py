from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.blockstore import BaseBlockStoreComponent


class MemoryBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self):
        self.blocks = {}

    async def get(self, id):
        try:
            return self.blocks[id]

        except KeyError:
            raise NotFoundError("Unknown block id.")

    async def post(self, id, block):
        if id in self.blocks:
            # Should never happen
            raise AlreadyExistsError("A block already exists with id `%s`." % id)

        self.blocks[id] = block
