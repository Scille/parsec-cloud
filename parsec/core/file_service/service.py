from base64 import decodebytes, encodebytes
from datetime import datetime
import json
import sys

from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import hashes
from logbook import Logger, StreamHandler
import websockets

from parsec.crypto import AESCipher
from parsec.service import BaseService, cmd
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

    def __init__(self, backend_host, backend_port):
        super().__init__()
        self.files = {}
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
        trust_seed = self._get_field(msg, 'trust_seed')
        response = await self.read(id, trust_seed)
        response.update({'status': 'ok'})
        return response

    @cmd('write_file')
    async def _cmd_WRITE(self, msg):
        id = self._get_field(msg, 'id')
        trust_seed = self._get_field(msg, 'trust_seed')
        version = self._get_field(msg, 'version', int)
        content = self._get_field(msg, 'content')
        await self.write(id, trust_seed, version, content)
        return {'status': 'ok'}

    @cmd('stat_file')
    async def _cmd_STAT(self, msg):
        id = self._get_field(msg, 'id')
        stats = await self.stat(id)
        return {'status': 'ok', 'stats': stats}

    @cmd('history')
    async def _cmd_HISTORY(self, msg):
        id = self._get_field(msg, 'id')
        history = await self.history(id)
        return {'status': 'ok', 'history': history}

    async def create(self):
        response = await self.send_cmd(cmd='VlobService:create')
        if response['status'] != 'ok':
            raise FileError()
        file = {
            'ctime': datetime.utcnow().timestamp(),
            'mtime': datetime.utcnow().timestamp(),
            'atime': datetime.utcnow().timestamp(),
            'size': 0
        }
        self.files[response['id']] = file
        ret = {}
        for key in ('id', 'read_trust_seed', 'write_trust_seed'):
            ret[key] = response[key]
        return ret

    async def read(self, id, trust_seed):
        if id not in self.files:
            raise FileNotFound('File not found.')
        self.files[id]['atime'] = datetime.utcnow().timestamp()
        response = await self.send_cmd(cmd='VlobService:read', id=id, trust_seed=trust_seed)
        if response['status'] != 'ok':
            raise FileError()
        file = {}
        version = response['version']
        block_id, key, sha = response['content']
        if block_id:
            key = decodebytes(key.encode())
            sha = decodebytes(sha.encode())
            # Get content
            response = await self.send_cmd(cmd='BlockService:read', id=block_id)
            if response['status'] != 'ok':
                raise FileError()
            # Decrypt
            encrypted_content = decodebytes(response['content'].encode())
            encryptor = AESCipher()
            content = encryptor.decrypt(key, encrypted_content)
            # Check integrity
            digest = hashes.Hash(hashes.SHA512(), backend=default_backend())
            digest.update(content)
            new_sha = digest.finalize()
            assert new_sha == sha
        else:
            content = b''
        file['content'] = content.decode()
        file['version'] = version
        return file

    async def write(self, id, trust_seed, version, content):
        if id not in self.files:
            raise FileNotFound('File not found.')
        content = content.encode()
        self.files[id]['mtime'] = datetime.utcnow().timestamp()
        self.files[id]['size'] = len(decodebytes(content))
        # Digest
        digest = hashes.Hash(hashes.SHA512(), backend=default_backend())
        digest.update(content)
        sha = digest.finalize()  # TODO replace with hexdigest ?
        # Encrypt
        encryptor = AESCipher()
        key, data = encryptor.encrypt(content)
        # Store
        response = await self.send_cmd(cmd='BlockService:create',
                                       content=encodebytes(data).decode())
        if response['status'] != 'ok':
            raise FileError()
        block_id = response['id']
        blob = (block_id, encodebytes(key).decode(), encodebytes(sha).decode())
        response = await self.send_cmd(cmd='VlobService:update',
                                       id=id,
                                       trust_seed=trust_seed,
                                       version=version,
                                       blob=blob)
        if response['status'] != 'ok':
            raise FileError()

    async def stat(self, id):
        if id not in self.files:
            raise FileNotFound('File not found.')
        file = self.files[id]
        return {'id': id,
                'ctime': file['ctime'],
                'mtime': file['mtime'],
                'atime': file['atime'],
                'size': file['size']}

    async def history(self, id):
        # TODO raise ParsecNotImplementedError
        pass
