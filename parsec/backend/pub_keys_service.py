from base64 import decodebytes
from os import listdir
from os.path import isfile, join

from parsec.crypto import RSACipher
from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError


class PubKeysError(ParsecError):
    pass


class PubKeysNotFound(PubKeysError):
    status = 'not_found'


class PubKeysService(BaseService):

    def __init__(self, pub_keys_directory):
        super().__init__()
        self.pub_keys = {}
        self.pub_keys_directory = pub_keys_directory

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

    @cmd('get_user_key')
    async def _cmd_GET_USER_KEY(self, msg):
        user_key = self.get_user_key()
        return {'status': 'ok', 'user_key': user_key}

    @cmd('sign_encrypt')
    async def _cmd_SIGN_ENCRYPT(self, msg):
        data = self._get_field('data')
        crypted_signed_data = self.sign_and_encrypt(data)
        return {'status': 'ok', 'data': crypted_signed_data}

    async def get_user_key(self, identity):
        if identity not in self.pub_keys:
            await self.load_keys()  # TODO load at startup
        if identity in self.pub_keys:
            return self.pub_keys[identity]
        else:
            raise PubKeysNotFound()

    async def encrypt(self, identity, data):  # TODO sign with public key??
        if identity not in self.pub_keys:
            await self.load_keys()  # TODO load at startup
        if identity in self.pub_keys:
            pub_key = self.pub_keys[identity]
            return pub_key.encrypt(data)
        else:
            raise PubKeysNotFound()

    async def load_keys(self):
        filenames = [join(self.pub_keys_directory, file)
                     for file in listdir(self.pub_keys_directory)
                     if isfile(join(self.pub_keys_directory, file))]
        for file in filenames:
            identity = 'francois.rossigneux@scille.fr'  # TODO remove this, read identity
            self.pub_keys[identity] = RSACipher(file)  # TODO doesn't work with public
