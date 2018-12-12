from uuid import UUID

from parsec.backend.blockstore import (
    BaseBlockstoreComponent,
    BlockstoreAlreadyExistsError,
    BlockstoreNotFoundError,
)


class MemoryBlockstoreComponent(BaseBlockstoreComponent):
    def __init__(self):
        self.blocks = {}

    async def read(self, id: UUID) -> bytes:
        try:
            return self.blocks[id]

        except KeyError:
            raise BlockstoreNotFoundError()

    async def create(self, id: UUID, block: bytes) -> None:
        if id in self.blocks:
            # Should never happen
            raise BlockstoreAlreadyExistsError()

        self.blocks[id] = block
