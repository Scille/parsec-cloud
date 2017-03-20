from base64 import decodebytes

from parsec.crypto import RSACipher
from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError


class PubKeysError(ParsecError):
    pass


class PubKeysNotFound(PubKeysError):
    status = 'not_found'


class PubKeysService(BaseService):

    def __init__(self):
        super().__init__()
        self.pub_key = None
        self.rsa_cipher = RSACipher()

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

    @cmd('encrypt_and_sign')
    async def _cmd_CRYPT_AND_SIGN(self, msg):
        data = self._get_field('data')
        crypted_signed_data = self.crypt_and_sign(data)
        return {'status': 'ok', 'data': crypted_signed_data}

    async def get_user_key(self):
        return self.pub_key

    async def encrypt_and_sign(self, data):
        crypted_data = self.rsa_cipher.encrypt(data)
        return self.rsa_cipher.sign(crypted_data)
