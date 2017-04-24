from base64 import encodebytes, decodebytes
from os import urandom

import gnupg
from marshmallow import fields

from parsec.service import BaseService, cmd
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


class BaseCryptoService(BaseService):

    name = 'CryptoService'

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
        decrypted_data = await self.asym_decrypt(msg['data'])
        return {'status': 'ok', 'data': decrypted_data.decode()}

    async def sym_encrypt(self, data):
        raise NotImplementedError()

    async def asym_encrypt(self, data, recipient):
        raise NotImplementedError()

    async def sym_decrypt(self, data, key):
        raise NotImplementedError()

    async def asym_decrypt(self, data):
        raise NotImplementedError()

    async def list_identities(self, identity, secret=False):
        raise NotImplementedError()

    async def identity_exists(self, identity, secret=False):
        raise NotImplementedError()

    async def import_keys(self, keys):
        raise NotImplementedError()

    async def export_keys(self, identity):
        raise NotImplementedError()


class CryptoService(BaseCryptoService):

    def __init__(self, homedir='~/.gnupg'):
        super().__init__()
        self.gnupg = gnupg.GPG(homedir=homedir, use_agent=True)
        self.gnupg_agentless = gnupg.GPG(homedir=homedir, use_agent=False)  # Cleaner way?

    async def sym_encrypt(self, data, passphrase=None):
        if not passphrase:
            passphrase = urandom(32)
        encrypted = self.gnupg_agentless.encrypt(data,
                                                 passphrase=passphrase,
                                                 encrypt=False,
                                                 symmetric=True)
        assert encrypted.status == 'encryption ok'
        return passphrase, encrypted.data

    async def asym_encrypt(self, data, recipient):
        encrypted = self.gnupg.encrypt(data, recipient)
        if encrypted.status != 'encryption ok':
            raise CryptoError('Encryption failure.')
        return encrypted.data

    async def sym_decrypt(self, data, key):
        decrypted = self.gnupg_agentless.decrypt(data, passphrase=key)
        if decrypted.status == 'decryption ok':
            return decrypted.data
        else:
            raise CryptoError('Decryption failure.')

    async def asym_decrypt(self, data):
        decrypted = self.gnupg.decrypt(data)
        if decrypted.status == 'decryption ok':
            return decrypted.data
        else:
            raise CryptoError('Decryption failure.')

    async def list_identities(self, identity, secret=False):
        return [key['fingerprint'] for key in self.gnupg.list_keys(secret)]

    async def identity_exists(self, identity, secret=False):
        ids = await self.list_identities(identity, secret)
        # Handle both short and long identity format
        return any(id for id in ids if id == identity or id.endswith(identity))

    async def import_keys(self, keys):
        self.gnupg.import_keys(keys)

    async def export_keys(self, identity):
        return self.gnupg.export_keys(identity)
