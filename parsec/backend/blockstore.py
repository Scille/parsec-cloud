from parsec.utils import to_jsonb64
from parsec.schema import BaseCmdSchema, fields


class _cmd_GET_Schema(BaseCmdSchema):
    id = fields.String(required=True, validate=lambda n: 0 < len(n) <= 32)


cmd_GET_Schema = _cmd_GET_Schema()


class _cmd_POST_Schema(BaseCmdSchema):
    id = fields.String(required=True, validate=lambda n: 0 < len(n) <= 32)
    block = fields.Base64Bytes(required=True)


cmd_POST_Schema = _cmd_POST_Schema()


class BaseBlockStoreComponent:
    def __init__(self, signal_ns):
        pass

    async def api_blockstore_get(self, client_ctx, msg):
        msg = cmd_GET_Schema.load_or_abort(msg)
        block = await self.get(msg["id"])
        return {"status": "ok", "block": to_jsonb64(block)}

    async def api_blockstore_post(self, client_ctx, msg):
        msg = cmd_POST_Schema.load_or_abort(msg)
        await self.post(**msg)
        return {"status": "ok"}

    async def get(self, id):
        raise NotImplementedError()

    async def post(self, id, block):
        raise NotImplementedError()
