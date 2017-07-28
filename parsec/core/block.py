import attr
from effect2 import TypeDispatcher

from parsec.exceptions import BlockError, BlockNotFound


@attr.s
class EBlockCreate:
    id = attr.ib()
    content = attr.ib()


@attr.s
class EBlockRead:
    id = attr.ib()


@attr.s
class Block:
    id = attr.ib()
    content = attr.ib()


def in_memory_block_dispatcher_factory():
    blocks = {}

    def perform_block_read(intent):
        try:
            return Block(id=intent.id, content=blocks[intent.id])
        except KeyError:
            raise BlockNotFound('Block %s not found' % intent.id)

    def perform_block_create(intent):
        if intent.id in blocks:
            raise BlockError('Block %s already exists' % intent.id)
        blocks[intent.id] = intent.content
        return Block(id=intent.id, content=intent.content)

    dispatcher = TypeDispatcher({
        EBlockCreate: perform_block_create,
        EBlockRead: perform_block_read
    })
    return dispatcher
