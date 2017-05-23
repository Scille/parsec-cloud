from marshmallow import fields

from parsec.service import BaseService, cmd
from parsec.exceptions import PubKeyError, PubKeyNotFound
from parsec.tools import BaseCmdSchema


class cmd_PUBKEY_GET_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class BasePubKeyService(BaseService):

    name = 'PubKeyService'

    @cmd('pubkey_get')
    async def _cmd_PUBKEY_GET(self, session, msg):
        msg = cmd_PUBKEY_GET_Schema().load(msg)
        key = await self.get_pubkey(msg['id'])
        return {'status': 'ok', 'key': key}

    async def get_pubkey(self, identity):
        raise NotImplementedError()

    async def add_pubkey(self, identity):
        raise NotImplementedError()


class MockedPubKeyService(BasePubKeyService):
    def __init__(self):
        super().__init__()
        self._keys = {}

    async def add_pubkey(self, id, key):
        if id in self._keys:
            raise PubKeyError('Identity `%s` already has a public key' % id)
        else:
            self._keys[id] = key

    async def get_pubkey(self, id):
        try:
            return self._keys[id]
        except KeyError:
            raise PubKeyNotFound('No public key for identity `%s`' % id)
