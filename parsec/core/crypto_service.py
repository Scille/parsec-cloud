from base64 import encodebytes, decodebytes
from os import urandom

import gnupg

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError


class CryptoError(ParsecError):
    pass


class CryptoService(BaseService):

    pub_keys_service = service('PubKeysService')

    def __init__(self):
        super().__init__()
        self.gpg = gnupg.GPG(binary='/usr/bin/gpg', homedir='~/.gnupg')  # TODO default params?

    @staticmethod
    def _pack_Crypto_error(error):
        if error.message:
            return ('%s %s' % (error.status, error.message)).encode()
        else:
            return error.status.encode()

    @staticmethod
    def _extract_params(raw_data, *types):
        number = len(types)
        splitted = raw_data.split(b' ', maxsplit=number - 1)
        if len(splitted) != number:
            raise CryptoError('bad_params', 'Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise CryptoError('bad_params', 'Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise CryptoError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise CryptoError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise CryptoError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('sym_encrypt')
    async def _cmd_SYM_ENCRYPT(self, msg):
        data = self._get_field('data')
        key, encrypted_data = self.sym_encrypt(data)
        key = encodebytes(key).decode()
        return {'status': 'ok', 'key': key, 'data': encrypted_data}

    @cmd('asym_encrypt')
    async def _cmd_ASYM_ENCRYPT(self, msg):
        recipient = self._get_field('recipient')
        data = self._get_field('data')
        passphrase = msg.get('passphrase')
        encrypted_signed_data = self.asym_encrypt(data, recipient, passphrase)
        return {'status': 'ok', 'data': encrypted_signed_data.decode()}

    @cmd('sym_decrypt')
    async def _cmd_SYM_DECRYPT(self, msg):
        data = self._get_field('data')
        key = self._get_field('key')
        decrypted_data = self.sym_decrypt(data, key)
        return {'status': 'ok', 'data': decrypted_data}

    @cmd('asym_decrypt')
    async def _cmd_ASYM_DECRYPT(self, msg):
        data = self._get_field('data')
        passphrase = msg.get('passphrase')
        decrypted_data = self.asym_decrypt(data, passphrase)
        return {'status': 'ok', 'data': decrypted_data.decode()}

    async def sym_encrypt(self, data):
        passphrase = urandom(32)
        encrypted = self.gpg.encrypt(data, passphrase=passphrase, encrypt=False, symmetric=True)
        if encrypted.status == 'encryption ok':
            return passphrase, encrypted.data
        else:
            raise CryptoError('Encryption failure.')

    async def asym_encrypt(self, data, recipient, default_key=None, passphrase=None):
        if recipient not in [key['fingerprint'] for key in self.gpg.list_keys()]:  # TODO slow?
            await self.pub_keys_service.get_pub_key(recipient)
        encrypted = self.gpg.encrypt(data,
                                     recipient,
                                     default_key=default_key,  # TODO default_key?
                                     passphrase=passphrase)
        if encrypted.status == 'encryption ok':
            return encrypted.data
        else:
            raise CryptoError('Encryption failure.')

    async def sym_decrypt(self, data, key):
        decrypted = self.gpg.decrypt(data, passphrase=key)
        if decrypted.status == 'decryption ok':
            return decrypted.data
        else:
            raise CryptoError('Decryption failure.')

    async def asym_decrypt(self, data, passphrase=None):
        decrypted = self.gpg.decrypt(data, passphrase=passphrase)
        if decrypted.status == 'decryption ok':
            return decrypted.data
        else:
            raise CryptoError('Decryption failure.')
