import gnupg
from keybaseapi import User

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError


class PubKeysError(ParsecError):
    pass


class PubKeysNotFound(PubKeysError):
    status = 'not_found'


class PubKeysService(BaseService):

    def __init__(self):
        super().__init__()
        self.gpg = gnupg.GPG(binary='/usr/bin/gpg', homedir='~/.gnupg')  # TODO default params?

    @cmd('get_pub_key')
    async def _cmd_GET_PUB_KEY(self, msg):
        identity = self._get_field(msg, 'identity')
        user_key = await self.get_pub_key(identity)
        return {'status': 'ok', 'pub_key': user_key}

    async def list_identities(self, identity, secret=False):
        return [key['fingerprint'] for key in self.gpg.list_keys(secret)]

    async def identity_exists(self, identity, secret=False):
        return identity in await self.list_identities(identity, secret)

    async def get_pub_key(self, identity):
        if not await self.identity_exists(identity):
            await self.import_key_from_keybase(identity)  # TODO load at startup
        return self.gpg.export_keys(identity)

    async def import_key_from_keybase(self, identity):
        user = User('key_fingerprint://' + identity)
        if user.raw_public_key:
            self.gpg.import_keys(user.raw_public_key)
        else:
            raise(PubKeysNotFound('Identity not found in Keybase.'))
