from base64 import encodebytes, decodebytes
from os import urandom

import gnupg
from marshmallow import fields

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class CryptoError(ParsecError):
    pass


class cmd_SYM_ENCRYPT_Schema(BaseCmdSchema):
    data = fields.String(required=True)


class cmd_ASYM_ENCRYPT_Schema(BaseCmdSchema):
    data = fields.String(required=True)
    recipient = fields.String(required=True)


class cmd_SYM_DECRYPT_Schema(BaseCmdSchema):
    data = fields.String(required=True)
    key = fields.String(required=True)


class cmd_ASYM_DECRYPT_Schema(BaseCmdSchema):
    data = fields.String(required=True)
    passphrase = fields.String(missing=None)


class CryptoService(BaseService):

    pub_keys_service = service('PubKeysService')

    def __init__(self):
        super().__init__()
        self.gpg = gnupg.GPG(binary='/usr/bin/gpg', homedir='~/.gnupg')  # TODO default params?

    @cmd('sym_encrypt')
    async def _cmd_SYM_ENCRYPT(self, session, msg):
        msg = cmd_SYM_ENCRYPT_Schema().load(msg)
        key, encrypted_data = await self.sym_encrypt(msg['data'])
        key = encodebytes(key).decode()
        return {'status': 'ok', 'key': key, 'data': encrypted_data.decode()}

    @cmd('asym_encrypt')
    async def _cmd_ASYM_ENCRYPT(self, session, msg):
        msg = cmd_ASYM_ENCRYPT_Schema().load(msg)
        encrypted_signed_data = await self.asym_encrypt(msg['data'], msg['recipient'])
        return {'status': 'ok', 'data': encrypted_signed_data.decode()}

    @cmd('sym_decrypt')
    async def _cmd_SYM_DECRYPT(self, session, msg):
        msg = cmd_SYM_DECRYPT_Schema().load(msg)
        msg['key'] = decodebytes(msg['key'].encode())  # TODO use marshmallow
        decrypted_data = await self.sym_decrypt(msg['data'], msg['key'])
        return {'status': 'ok', 'data': decrypted_data.decode()}

    @cmd('asym_decrypt')
    async def _cmd_ASYM_DECRYPT(self, session, msg):
        msg = cmd_ASYM_DECRYPT_Schema().load(msg)
        decrypted_data = await self.asym_decrypt(msg['data'], msg['passphrase'])
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

    async def list_identities(self, identity, secret=False):
        return [key['fingerprint'] for key in self.gpg.list_keys(secret)]

    async def identity_exists(self, identity, secret=False):
        return identity in await self.list_identities(identity, secret)

    async def import_keys(self, keys):
        self.gpg.import_keys(keys)

    async def export_keys(self, identity):
        return self.gpg.export_keys(identity)
