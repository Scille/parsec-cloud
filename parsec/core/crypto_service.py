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

    @cmd('sym_encrypt')
    async def _cmd_SYM_ENCRYPT(self, msg):
        data = self._get_field(msg, 'data')
        key, encrypted_data = await self.sym_encrypt(data)
        key = encodebytes(key).decode()
        return {'status': 'ok', 'key': key, 'data': encrypted_data.decode()}

    @cmd('asym_encrypt')
    async def _cmd_ASYM_ENCRYPT(self, msg):
        recipient = self._get_field(msg, 'recipient')
        data = self._get_field(msg, 'data')
        encrypted_signed_data = await self.asym_encrypt(data, recipient)
        return {'status': 'ok', 'data': encrypted_signed_data.decode()}

    @cmd('sym_decrypt')
    async def _cmd_SYM_DECRYPT(self, msg):
        data = self._get_field(msg, 'data')
        key = self._get_field(msg, 'key')
        key = decodebytes(key.encode())
        decrypted_data = await self.sym_decrypt(data, key)
        return {'status': 'ok', 'data': decrypted_data.decode()}

    @cmd('asym_decrypt')
    async def _cmd_ASYM_DECRYPT(self, msg):
        data = self._get_field(msg, 'data')
        passphrase = self._get_field(msg, 'passphrase', default=None)
        decrypted_data = await self.asym_decrypt(data, passphrase)
        return {'status': 'ok', 'data': decrypted_data.decode()}

    async def sym_encrypt(self, data):
        passphrase = urandom(32)
        encrypted = self.gpg.encrypt(data, passphrase=passphrase, encrypt=False, symmetric=True)
        assert encrypted.status == 'encryption ok'
        return passphrase, encrypted.data

    async def asym_encrypt(self, data, recipient):
        try:
            await self.pub_keys_service.get_pub_key(recipient)
        except Exception:
            raise CryptoError('Encryption failure.')
        encrypted = self.gpg.encrypt(data, recipient)
        assert encrypted.status == 'encryption ok'
        return encrypted.data

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
