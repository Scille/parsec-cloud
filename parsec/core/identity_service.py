from base64 import encodebytes

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from marshmallow import fields

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class IdentityError(ParsecError):
    pass


class IdentityNotFound(IdentityError):
    status = 'not_found'


class cmd_LOAD_IDENTITY_Schema(BaseCmdSchema):
    identity = fields.String(missing=None)


class cmd_ENCRYPT_Schema(BaseCmdSchema):
    data = fields.String(required=True)


class cmd_DECRYPT_Schema(BaseCmdSchema):
    data = fields.String(required=True)


class BaseIdentityService(BaseService):

    name = 'IdentityService'

    @cmd('identity_load')
    async def _cmd_LOAD_IDENTITY(self, session, msg):
        msg = cmd_LOAD_IDENTITY_Schema().load(msg)
        await self.load_identity(msg['identity'])
        return {'status': 'ok'}

    @cmd('identity_get')
    async def _cmd_GET_IDENTITY(self, session, msg):
        identity = await self.get_identity()
        return {'status': 'ok', 'identity': identity}

    @cmd('identity_encrypt')
    async def _cmd_ENCRYPT(self, session, msg):
        msg = cmd_ENCRYPT_Schema().load(msg)
        encrypted_data = await self.encrypt(msg['data'])
        return {'status': 'ok', 'data': encrypted_data.decode()}

    @cmd('identity_decrypt')
    async def _cmd_DECRYPT(self, session, msg):
        msg = cmd_DECRYPT_Schema().load(msg)
        decrypted_data = await self.decrypt(msg['data'])
        return {'status': 'ok', 'data': decrypted_data.decode()}

    async def load_identity(self, identity=None):
        raise NotImplementedError()

    async def get_identity(self):
        raise NotImplementedError()

    async def encrypt(self, data, recipient=None):
        raise NotImplementedError()

    async def decrypt(self, data):
        raise NotImplementedError()

    async def compute_sign_challenge(self):
        raise NotImplementedError()

    async def compute_seed_challenge(self, id, trust_seed):
        raise NotImplementedError()


class IdentityService(BaseIdentityService):

    crypto_service = service('CryptoService')
    pub_keys_service = service('PubKeysService')

    def __init__(self):
        super().__init__()
        self.identity = None

    async def load_identity(self, identity=None):
        if identity:
            if await self.crypto_service.identity_exists(identity, secret=True):
                self.identity = identity
            else:
                raise IdentityNotFound('Identity not found.')
        else:
            identities = await self.crypto_service.list_identities(identity, secret=True)
            if len(identities) == 1:
                self.identity = identities[0]
            elif len(identities) > 1:
                raise IdentityError('Multiple identities found.')
            else:
                raise IdentityNotFound('Default identity not found.')

    async def get_identity(self):  # TODO identity=fingerprint?
        return self.identity

    async def encrypt(self, data, recipient=None):
        if not self.identity:
            raise(IdentityNotFound('No identity loaded.'))
        if not recipient:
            recipient = await self.get_identity()
        return await self.crypto_service.asym_encrypt(data, recipient)

    async def decrypt(self, data):
        if not self.identity:
            raise(IdentityNotFound('No identity loaded.'))
        return await self.crypto_service.asym_decrypt(data)

    async def compute_sign_challenge(self):
        identity = await self.get_identity()
        response = await self.send_cmd(cmd='VlobService:get_sign_challenge', id=identity)
        if response['status'] != 'ok':
            raise IdentityError('Cannot get sign challenge.')
        # TODO should be decrypted by vblob service public key?
        encrypted_challenge = response['challenge']
        challenge = await self.crypto_service.asym_decrypt(encrypted_challenge)
        return identity, challenge

    async def compute_seed_challenge(self, id, trust_seed):
        response = await self.send_cmd(cmd='VlobService:get_seed_challenge', id=id)
        if response['status'] != 'ok':
            raise IdentityError('Cannot get seed challenge.')
        challenge = response['challenge']
        challenge = challenge.encode()
        trust_seed = trust_seed.encode()
        digest = hashes.Hash(hashes.SHA512(), backend=openssl)
        digest.update(challenge + trust_seed)
        hash = digest.finalize()
        hash = encodebytes(hash).decode()
        return challenge.decode(), hash
