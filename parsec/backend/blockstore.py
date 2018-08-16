from parsec.utils import to_jsonb64
from parsec.schema import BaseCmdSchema, fields


class _cmd_GET_Schema(BaseCmdSchema):
    id = fields.UUID(required=True)


cmd_GET_Schema = _cmd_GET_Schema()


class _cmd_POST_Schema(BaseCmdSchema):
    id = fields.UUID(required=True)
    block = fields.Base64Bytes(required=True)


cmd_POST_Schema = _cmd_POST_Schema()


class BaseBlockStoreComponent:
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
