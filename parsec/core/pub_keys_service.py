from keybaseapi import User
from marshmallow import fields

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class PubKeysError(ParsecError):
    pass


class PubKeysNotFound(PubKeysError):
    status = 'not_found'


class cmd_GET_PUB_KEY_Schema(BaseCmdSchema):
    identity = fields.String(required=True)


class PubKeysService(BaseService):

    crypto_service = service('CryptoService')

    def __init__(self):
        super().__init__()

    @cmd('get_pub_key')
    async def _cmd_GET_PUB_KEY(self, session, msg):
        msg = cmd_GET_PUB_KEY_Schema().load(msg)
        user_key = await self.get_pub_key(msg['identity'])
        return {'status': 'ok', 'pub_key': user_key}

    async def get_pub_key(self, identity):
        if not await self.crypto_service.identity_exists(identity):
            await self.import_key_from_keybase(identity)  # TODO load at startup
        return await self.crypto_service.export_keys(identity)

    async def import_key_from_keybase(self, identity):
        user = User('key_fingerprint://' + identity)
        if user.raw_public_key:
            await self.crypto_service.import_keys(user.raw_public_key)
        else:
            raise(PubKeysNotFound('Identity not found in Keybase.'))
