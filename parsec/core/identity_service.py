from base64 import encodebytes
import json
import sys

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from logbook import Logger, StreamHandler
import websockets

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError

LOG_FORMAT = '[{record.time:%Y-%m-%d %H:%M:%S.%f%z}] ({record.thread_name})' \
             ' {record.level_name}: {record.channel}: {record.message}'
log = Logger('Parsec-File-Service')
StreamHandler(sys.stdout, format_string=LOG_FORMAT).push_application()


class IdentityError(ParsecError):
    pass


class IdentityNotFound(IdentityError):
    status = 'not_found'


class IdentityService(BaseService):

    crypto_service = service('CryptoService')
    pub_keys_service = service('PubKeysService')

    def __init__(self, backend_host, backend_port):
        super().__init__()
        self._backend_host = backend_host
        self._backend_port = backend_port
        self.identity = None
        self.passphrase = None

    async def send_cmd(self, **msg):
        req = json.dumps(msg).encode() + b'\n'
        log.debug('Send: %r' % req)
        websocket_path = 'ws://' + self._backend_host + ':' + str(self._backend_port)
        async with websockets.connect(websocket_path) as websocket:
            await websocket.send(req)
            raw_reps = await websocket.recv()
            log.debug('Received: %r' % raw_reps)
            return json.loads(raw_reps.decode())

    @cmd('load_user_identity')
    async def _cmd_LOAD_USER_IDENTITY(self, msg):
        identity = self._get_field(msg, 'identity', default=None)
        passphrase = self._get_field(msg, 'passphrase', default=None)
        await self.load_user_identity(identity, passphrase)
        return {'status': 'ok'}

    @cmd('get_identity')
    async def _cmd_GET_USER_IDENTITY(self, msg):
        identity = await self.get_user_identity()
        return {'status': 'ok', 'identity': identity}

    @cmd('encrypt')
    async def _cmd_ENCRYPT(self, msg):
        data = self._get_field(msg, 'data')
        encrypted_data = await self.encrypt(data)
        return {'status': 'ok', 'data': encrypted_data}

    @cmd('decrypt')
    async def _cmd_DECRYPT(self, msg):
        data = self._get_field(msg, 'data')
        decrypted_data = await self.decrypt(data)
        return {'status': 'ok', 'data': decrypted_data.decode()}

    async def load_user_identity(self, identity=None, passphrase=None):
        if identity:
            if await self.pub_keys_service.identity_exists(identity, secret=True):
                self.identity = identity
            else:
                raise IdentityNotFound('Identity not found.')
        else:
            identities = await self.pub_keys_service.list_identities(identity, secret=True)
            if len(identities) == 1:
                self.identity = identities[0]
            elif len(identities) > 1:
                raise IdentityError('Multiple identities found.')
            else:
                raise IdentityNotFound('Identity not found.')
        self.passphrase = passphrase

    async def get_user_identity(self):  # TODO identity=fingerprint?
        return self.identity

    async def encrypt(self, data, recipient=None):
        if not self.identity:
            raise(IdentityNotFound('No identity loaded.'))
        if not recipient:
            recipient = await self.get_user_identity()
        return await self.crypto_service.asym_encrypt(data, recipient)

    async def decrypt(self, data):
        if not self.identity:
            raise(IdentityNotFound('No identity loaded.'))
        return await self.crypto_service.asym_decrypt(data, self.passphrase)

    async def compute_sign_challenge(self):
        user_identity = await self.get_user_identity()
        response = await self.send_cmd(cmd='VlobService:get_sign_challenge', id=user_identity)
        if response['status'] != 'ok':
            raise IdentityError('Cannot get sign challenge.')
        # TODO should be decrypted by vblob service public key?
        encrypted_challenge = response['challenge']
        challenge = await self.crypto_service.asym_decrypt(encrypted_challenge, self.passphrase)
        return user_identity, challenge

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
