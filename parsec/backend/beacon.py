from parsec.schema import BaseCmdSchema, fields


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    offset = fields.Integer(required=True)


class BaseBeaconComponent:
    async def api_beacon_read(self, client_ctx, msg):
        msg = cmd_READ_Schema().load_or_abort(msg)
        items = await self.read(**msg)
        return {
            "status": "ok",
            "id": msg["id"],
            "offset": msg["offset"],
            "items": items,
            "count": len(items),
        }

    async def read(self, id, offset):
        raise NotImplementedError()

    async def update(self, id, src_id, src_version, author="anonymous"):
        raise NotImplementedError()
