from base64 import decodebytes

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

    @staticmethod
    def _pack_PubKeys_error(error):
        if error.message:
            return ('%s %s' % (error.status, error.message)).encode()
        else:
            return error.status.encode()

    @staticmethod
    def _extract_params(raw_data, *types):
        number = len(types)
        splitted = raw_data.split(b' ', maxsplit=number - 1)
        if len(splitted) != number:
            raise PubKeysError('bad_params', 'Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise PubKeysError('bad_params', 'Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise PubKeysError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise PubKeysError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise PubKeysError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('get_pub_key')
    async def _cmd_GET_PUB_KEY(self, msg):
        user_key = self.get_pub_key()
        return {'status': 'ok', 'user_key': user_key}

    async def list_identities(self, identity, secret=False):
        return [key['fingerprint'] for key in self.gpg.list_keys(secret)]

    async def identity_exists(self, identity, secret=False):
        return identity in await self.list_identities(identity, secret)

    async def get_pub_key(self, identity):
        if identity not in [key['fingerprint'] for key in self.gpg.list_keys()]:
            await self.fetch_pub_key_from_keybase(identity)  # TODO load at startup
        pub_key = self.gpg.export_keys(identity)
        if pub_key:
            return pub_key
        else:
            raise PubKeysNotFound()

    async def fetch_pub_key_from_keybase(self, identity):
        try:
            user = User('key_fingerprint://' + identity)
        except AttributeError:
            raise(PubKeysNotFound('Identity not found in Keybase.'))
        else:
            self.gpg.import_keys(user.raw_public_key)
