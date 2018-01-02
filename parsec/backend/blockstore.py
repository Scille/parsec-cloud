import attr
from marshmallow import fields
from uuid import uuid4

from parsec.utils import UnknownCheckedSchema, to_jsonb64, ParsecError


class BlockStoreError(ParsecError):
    status = 'block_store_error'


class BlockNotFoundError(ParsecError):
    status = 'block_not_found'


class cmd_GET_Schema(UnknownCheckedSchema):
    id = fields.String(required=True, validate=lambda n: 0 < len(n) <= 32)


class cmd_POST_Schema(UnknownCheckedSchema):
    block = fields.Base64Bytes(required=True)


class BaseBlockStoreComponent:

    def __init__(self, signal_ns):
        pass

    async def api_blockstore_get(self, client_ctx, msg):
        msg = cmd_GET_Schema().load(msg)
        block = await self.get(msg['id'])
        return {
            'status': 'ok',
            'block': to_jsonb64(block)
        }

    async def api_blockstore_post(self, client_ctx, msg):
        msg = cmd_POST_Schema().load(msg)
        id = uuid4().hex
        await self.post(id, msg['block'])
        return {'status': 'ok', 'id': id}

    async def get(self, id):
        raise NotImplementedError()

    async def post(self, id, block):
        raise NotImplementedError()


class MockedBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, *args):
        super().__init__(*args)
        self.blocks = {}

    async def get(self, id):
        try:
            return self.blocks[id]
        except KeyError:
            raise BlockNotFoundError('Unknown block id.')

    async def post(self, id, block):
        if id in self.blocks:
            # Should never happen
            raise BlockStoreError('A block already exists with id `%s`.' % id)
        self.blocks[id] = block
