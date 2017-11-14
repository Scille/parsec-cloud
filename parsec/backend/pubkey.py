import attr
from marshmallow import fields

from nacl.public import PublicKey
from nacl.signing import VerifyKey

from parsec.utils import UnknownCheckedSchema, ParsecError, to_jsonb64


class PubKeyError(ParsecError):
    status = 'pubkey_error'


class PubKeyNotFound(PubKeyError):
    status = 'pubkey_not_found'


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


@attr.s
class MockedPubKeyComponent(BasePubKeyComponent):
    _keys = attr.ib(default=attr.Factory(dict))

    async def add(self, id, pubkey, verifykey):
        assert isinstance(pubkey, (bytes, bytearray))
        assert isinstance(verifykey, (bytes, bytearray))
        if id in self._keys:
            raise PubKeyError('Identity `%s` already has a public key' % id)
        else:
            self._keys[id] = (pubkey, verifykey)

    async def get(self, id, cooked=False):
        try:
            keys = self._keys[id]
            return (PublicKey(keys[0]), VerifyKey(keys[1])) if cooked else keys
        except KeyError:
            raise PubKeyNotFound('No public key for identity `%s`' % id)
