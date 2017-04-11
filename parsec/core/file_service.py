from base64 import decodebytes, encodebytes
from datetime import datetime
import json
import sys

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from logbook import Logger, StreamHandler
from marshmallow import fields

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


LOG_FORMAT = '[{record.time:%Y-%m-%d %H:%M:%S.%f%z}] ({record.thread_name})' \
             ' {record.level_name}: {record.channel}: {record.message}'
log = Logger('Parsec-File-Service')
StreamHandler(sys.stdout, format_string=LOG_FORMAT).push_application()


class FileError(ParsecError):
    pass


class FileNotFound(FileError):
    status = 'not_found'


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class cmd_WRITE_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Integer(required=True)
    content = fields.String(required=True)


class cmd_STAT_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class cmd_HISTORY_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class FileService(BaseService):

    backend_api_service = service('BackendAPIService')
    crypto_service = service('CryptoService')
    identity_service = service('IdentityService')
    user_manifest_service = service('UserManifestService')

    def __init__(self):
        super().__init__()

    @cmd('create_file')
    async def _cmd_CREATE(self, session, msg):
        id = await self.create()
        return {'status': 'ok', 'file': id}

    @cmd('read_file')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        file = await self.read(msg['id'])
        file.update({'status': 'ok'})
        return file

    @cmd('write_file')
    async def _cmd_WRITE(self, session, msg):
        msg = cmd_WRITE_Schema().load(msg)
        await self.write(msg['id'], msg['version'], msg['content'])
        return {'status': 'ok'}

    @cmd('stat_file')
    async def _cmd_STAT(self, session, msg):
        msg = cmd_STAT_Schema().load(msg)
        stats = await self.stat(msg['id'])
        stats.update({'status': 'ok'})
        return stats

    @cmd('history')
    async def _cmd_HISTORY(self, session, msg):
        msg = cmd_STAT_Schema().load(msg)
        history = await self.history(msg['id'])
        return {'status': 'ok', 'history': history}

    async def create(self):
        vlob = await self.backend_api_service.vlob_service.create()
        return {'id': vlob.id,
                'read_trust_seed': vlob.read_trust_seed,
                'write_trust_seed': vlob.write_trust_seed}

    async def read(self, id):
        try:
            properties = await self.user_manifest_service.get_properties(id)
        except Exception:
            raise FileNotFound('Vlob not found.')
        key = decodebytes(properties['key'].encode()) if properties['key'] else None
        if not key:
            return {'content': '', 'version': 0}
        vlob = await self.backend_api_service.vlob_service.read(id=id)
        version = len(vlob.blob_versions)
        blob = vlob.blob_versions[version - 1]
        encrypted_blob = decodebytes(blob.encode())
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, key)
        blob = json.loads(blob.decode())
        key = decodebytes(blob['key'].encode())
        old_digest = decodebytes(blob['digest'].encode())
        # Get content
        block = await self.backend_api_service.block_service.read(id=blob['block'])
        # Decrypt
        encrypted_content = block['content'].encode()
        content = await self.crypto_service.sym_decrypt(encrypted_content, key)
        # Check integrity
        digest = hashes.Hash(hashes.SHA512(), backend=openssl)
        digest.update(content)
        new_digest = digest.finalize()
        assert new_digest == old_digest
        return {'content': content.decode(), 'version': version}

    async def write(self, id, version, content):
        try:
            properties = await self.user_manifest_service.get_properties(id)
        except Exception:
            raise FileNotFound('Vlob not found.')
        key = decodebytes(properties['key'].encode()) if properties['key'] else None
        content = content.encode()
        size = len(decodebytes(content))
        # Digest
        digest = hashes.Hash(hashes.SHA512(), backend=openssl)
        digest.update(content)
        content_digest = digest.finalize()  # TODO replace with hexdigest ?
        content_digest = encodebytes(content_digest).decode()
        # Encrypt block
        key, data = await self.crypto_service.sym_encrypt(content)
        key = encodebytes(key).decode()
        data = data.decode()
        # Store block
        block_id = await self.backend_api_service.block_service.create(content=data)
        # Update vlob
        blob = {'block': block_id,
                'size': size,
                'key': key,
                'digest': content_digest}
        # Encrypt blob
        blob = json.dumps(blob)
        blob = blob.encode()
        key, encrypted_blob = await self.crypto_service.sym_encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await self.backend_api_service.vlob_service.update(id=id,
                                                           next_version=version,
                                                           blob=encrypted_blob)
        await self.user_manifest_service.update_key(id, key)  # TODO use event
        return key

    async def stat(self, id):
        try:
            properties = await self.user_manifest_service.get_properties(id)
        except Exception:
            raise FileNotFound('Vlob not found.')
        key = decodebytes(properties['key'].encode()) if properties['key'] else None
        if not key:
            return {'id': id,
                    'ctime': datetime.utcnow().timestamp(),
                    'mtime': datetime.utcnow().timestamp(),
                    'atime': datetime.utcnow().timestamp(),
                    'size': 0}
        vlob = await self.backend_api_service.vlob_service.read(id=id)
        encrypted_blob = vlob.blob_versions[-1]
        encrypted_blob = decodebytes(encrypted_blob.encode())
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, key)
        blob = json.loads(blob.decode())
        stat = await self.backend_api_service.block_service.stat(id=blob['block'])
        return {'id': id,
                'ctime': stat['creation_timestamp'],
                'mtime': stat['creation_timestamp'],
                'atime': stat['access_timestamp'],
                'size': blob['size']}

    async def history(self, id):
        # TODO raise ParsecNotImplementedError
        pass
