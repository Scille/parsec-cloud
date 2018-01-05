from marshmallow import fields

from parsec.utils import UnknownCheckedSchema, to_jsonb64


class cmd_PUBKEY_GET_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)


class BasePubKeyComponent:

    async def api_pubkey_get(self, client_ctx, msg):
        msg = cmd_PUBKEY_GET_Schema().load(msg)
        keys = await self.get(msg['id'])
        if not keys:
            return {'pubkey_not_found', 'No public key for identity `%s`' % msg['id']}
        return {
            'status': 'ok',
            'id': msg['id'],
            'public': to_jsonb64(keys[0]),
            'verify': to_jsonb64(keys[1])
        }

    # async def api_pubkey_add(self, msg):
    #     msg = cmd_PUBKEY_ADD_Schema().load(msg)
    #     key = await Effect(EPubKeyGet(**msg, raw=True))
    #     return {'status': 'ok', 'id': msg['id'], 'key': key}

    async def add(self, id, pubkey, verifykey):
        raise NotImplementedError()

    async def get(self, intent):
        raise NotImplementedError()
