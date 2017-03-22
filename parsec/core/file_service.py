from base64 import decodebytes, encodebytes
from datetime import datetime
import json
import sys

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from logbook import Logger, StreamHandler
import websockets

from parsec.crypto import AESCipher
from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError

LOG_FORMAT = '[{record.time:%Y-%m-%d %H:%M:%S.%f%z}] ({record.thread_name})' \
             ' {record.level_name}: {record.channel}: {record.message}'
log = Logger('Parsec-File-Service')
StreamHandler(sys.stdout, format_string=LOG_FORMAT).push_application()


class FileError(ParsecError):
    pass


class FileNotFound(FileError):
    status = 'not_found'


class FileService(BaseService):

    identity_service = service('IdentityService')
    user_manifest_service = service('UserManifestService')

    def __init__(self, backend_host, backend_port):
        super().__init__()
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
    def _pack_file_error(error):
        if error.message:
            return ('%s %s' % (error.status, error.message)).encode()
        else:
            return error.status.encode()

    @staticmethod
    def _extract_params(raw_data, *types):
        number = len(types)
        splitted = raw_data.split(b' ', maxsplit=number - 1)
        if len(splitted) != number:
            raise FileError('bad_params', 'Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise FileError('bad_params', 'Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise FileError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise FileError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise FileError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('create_file')
    async def _cmd_CREATE(self, msg):
        id = await self.create()
        return {'status': 'ok', 'file': id}

    @cmd('read_file')
    async def _cmd_READ(self, msg):
        id = self._get_field(msg, 'id')
        file = await self.read(id)
        file.update({'status': 'ok'})
        return file

    @cmd('write_file')
    async def _cmd_WRITE(self, msg):
        id = self._get_field(msg, 'id')
        version = self._get_field(msg, 'version', int)
        content = self._get_field(msg, 'content')
        await self.write(id, version, content)
        return {'status': 'ok'}

    @cmd('stat_file')
    async def _cmd_STAT(self, msg):
        id = self._get_field(msg, 'id')
        stats = await self.stat(id)
        stats.update({'status': 'ok'})
        return stats

    @cmd('history')
    async def _cmd_HISTORY(self, msg):
        id = self._get_field(msg, 'id')
        history = await self.history(id)
        return {'status': 'ok', 'history': history}

    async def create(self, id=None):
        response = await self.send_cmd(cmd='VlobService:create', id=id)
        if response['status'] != 'ok':
            raise FileError('Cannot create vlob.')
        ret = {}
        for key in ('id', 'read_trust_seed', 'write_trust_seed'):
            ret[key] = response[key]
        return response

    async def read(self, id):
        properties = await self.user_manifest_service.get_properties(id)
        key = decodebytes(properties['key'].encode()) if properties['key'] else None
        trust_seed = properties['read_trust_seed']
        if not key:
            return {'content': '', 'version': 0}
        challenge, hash = await self.identity_service.compute_seed_challenge(id, trust_seed)
        response = await self.send_cmd(cmd='VlobService:read',
                                       id=id,
                                       challenge=challenge,
                                       hash=hash)
        if response['status'] != 'ok':
            raise FileError('Cannot read vlob.')
        version = response['version']
        if response['blob']:
            encryptor = AESCipher()
            encrypted_blob = decodebytes(response['blob'].encode())
            blob = encryptor.decrypt(key, encrypted_blob)
            blob = json.loads(blob.decode())
            key = decodebytes(blob['key'].encode())
            old_digest = decodebytes(blob['digest'].encode())
            # Get content
            response = await self.send_cmd(cmd='BlockService:read', id=blob['block'])
            if response['status'] != 'ok':
                raise FileError('Cannot read block.')
            # Decrypt
            encrypted_content = decodebytes(response['content'].encode())
            encryptor = AESCipher()
            content = encryptor.decrypt(key, encrypted_content)
            # Check integrity
            digest = hashes.Hash(hashes.SHA512(), backend=openssl)
            digest.update(content)
            new_digest = digest.finalize()
            assert new_digest == old_digest
        return {'content': content.decode(), 'version': version}

    async def write(self, id, version, content):
        properties = await self.user_manifest_service.get_properties(id)
        key = decodebytes(properties['key'].encode()) if properties['key'] else None
        trust_seed = properties['write_trust_seed']
        content = content.encode()
        size = len(decodebytes(content))
        # Digest
        digest = hashes.Hash(hashes.SHA512(), backend=openssl)
        digest.update(content)
        content_digest = digest.finalize()  # TODO replace with hexdigest ?
        content_digest = encodebytes(content_digest).decode()
        # Encrypt block
        encryptor = AESCipher()
        key, data = encryptor.encrypt(content)
        key = encodebytes(key).decode()
        data = encodebytes(data).decode()
        # Store block
        response = await self.send_cmd(cmd='BlockService:create', content=data)
        if response['status'] != 'ok':
            raise FileError('Cannot create block.')
        # Update vlob
        challenge, hash = await self.identity_service.compute_seed_challenge(id, trust_seed)
        blob = {'block': response['id'],
                'size': size,
                'key': key,
                'digest': content_digest}
        # Encrypt blob
        blob = json.dumps(blob)
        blob = blob.encode()
        key, encrypted_blob = encryptor.encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        response = await self.send_cmd(cmd='VlobService:update',
                                       id=id,
                                       version=version,
                                       blob=encrypted_blob,
                                       challenge=challenge,
                                       hash=hash)
        if response['status'] != 'ok':
            raise FileError('Cannot update vlob.')
        await self.user_manifest_service.update_key(id, key)  # TODO use event
        return key

    async def stat(self, id):
        properties = await self.user_manifest_service.get_properties(id)
        key = decodebytes(properties['key'].encode()) if properties['key'] else None
        trust_seed = properties['read_trust_seed']
        if not key:
            return {'id': id,
                    'ctime': datetime.utcnow().timestamp(),
                    'mtime': datetime.utcnow().timestamp(),
                    'atime': datetime.utcnow().timestamp(),
                    'size': 0}
        challenge, hash = await self.identity_service.compute_seed_challenge(id, trust_seed)
        response = await self.send_cmd(cmd='VlobService:read',
                                       id=id,
                                       challenge=challenge,
                                       hash=hash)
        if response['status'] != 'ok':
            raise FileError('Cannot read vlob.')
        encryptor = AESCipher()
        encrypted_blob = response['blob']
        encrypted_blob = decodebytes(encrypted_blob.encode())
        blob = encryptor.decrypt(key, encrypted_blob)
        blob = json.loads(blob.decode())
        response = await self.send_cmd(cmd='BlockService:stat', id=blob['block'])
        if response['status'] != 'ok':
            raise FileError('Cannot stat block.')
        return {'id': id,
                'ctime': response['creation_timestamp'],
                'mtime': response['creation_timestamp'],
                'atime': response['access_timestamp'],
                'size': blob['size']}

    async def history(self, id):
        # TODO raise ParsecNotImplementedError
        pass
