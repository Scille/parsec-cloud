from base64 import decodebytes

from parsec.crypto import RSACipher
from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError


class IdentityError(ParsecError):
    pass


class IdentityNotFound(IdentityError):
    status = 'not_found'


class IdentityService(BaseService):

    def __init__(self):
        super().__init__()
        self.email = None
        self.rsa_cipher = RSACipher()

    @staticmethod
    def _pack_identity_error(error):
        if error.message:
            return ('%s %s' % (error.status, error.message)).encode()
        else:
            return error.status.encode()

    @staticmethod
    def _extract_params(raw_data, *types):
        number = len(types)
        splitted = raw_data.split(b' ', maxsplit=number - 1)
        if len(splitted) != number:
            raise IdentityError('bad_params', 'Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise IdentityError('bad_params', 'Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise IdentityError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise IdentityError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise IdentityError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('load_identity')
    async def _cmd_LOAD(self, msg):
        email = self._get_field(msg, 'email')
        key_file = self._get_field(msg, 'key_file')
        await self.load_user_identity(email, key_file)
        return {'status': 'ok'}

    @cmd('get_identity')
    async def _cmd_GET(self, msg):
        identity = self.get_user_identity()
        return {'status': 'ok', 'identity': identity}

    @cmd('encrypt')
    async def _cmd_ENCRYPT(self, msg):
        data = self._get_field(msg, 'data')
        encrypted_data = self.encrypt(data)
        return {'status': 'ok', 'data': encrypted_data}

    @cmd('decrypt')
    async def _cmd_DECRYPT(self, msg):
        data = self._get_field(msg, 'data')
        decrypted_data = self.decrypt(data)
        return {'status': 'ok', 'data': decrypted_data}

    @cmd('sign')
    async def _cmd_SIGN(self, msg):
        data = self._get_field(msg, 'data')
        signed_data = self.sign(data)
        return {'status': 'ok', 'data': signed_data}

    async def load_user_identity(self, email, key_file):
        self.email = email
        with open(key_file, 'rb') as file:
            self.rsa_cipher.load_key(file.read(), b'')

    async def get_user_identity(self):
        return self.email

    async def encrypt(self, data):
        return self.rsa_cipher.encrypt(data)

    async def decrypt(self, data):
        return self.rsa_cipher.decrypt(data)

    async def sign(self, data):
        return self.rsa_cipher.sign(data)
