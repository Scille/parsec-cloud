from base64 import encodebytes, decodebytes
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

    def __init__(self, backend_host, backend_port):
        super().__init__()
        self.email = None
        self._backend_host = backend_host
        self._backend_port = backend_port

    async def send_cmd(self, **msg):
        req = json.dumps(msg).encode() + b'\n'
        log.debug('Send: %r' % req)
        websocket_path = 'ws://' + self._backend_host + ':' + str(self._backend_port)
        async with websockets.connect(websocket_path) as websocket:
            await websocket.send(req)
            raw_reps = await websocket.recv()
            log.debug('Received: %r' % raw_reps)
            return json.loads(raw_reps.decode())

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
        return {'status': 'ok', 'data': decrypted_data}

    async def load_user_identity(self, email, key_file):
        self.email = email
        with open(key_file, 'rb') as file:
            key = file.read()
            # TODO accept passphrase
            await self.crypto_service.load_key(key=key.decode(), passphrase='')

    async def get_user_identity(self):
        return self.email

    async def encrypt(self, data):
        return self.crypto_service.encrypt(data)

    async def decrypt(self, key, key_signature, content, signature):
        return await self.crypto_service.decrypt(key, key_signature, content, signature)

    async def simple_decrypt(self, content):  # TODO remove this
        return self.crypto_service._asym.decrypt(content)

    async def compute_sign_challenge(self):
        user_identity = await self.get_user_identity()
        response = await self.send_cmd(cmd='VlobService:get_sign_challenge',
                                       id=user_identity)
        if response['status'] != 'ok':
            raise IdentityError('Cannot get sign challenge.')
        # TODO should be decrypted by vblob service public key?
        encrypted_challenge = response['challenge']
        encrypted_challenge = decodebytes(encrypted_challenge.encode())
        challenge = await self.simple_decrypt(encrypted_challenge)
        challenge = challenge.decode()  # TODO encodebytes challenge?
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
