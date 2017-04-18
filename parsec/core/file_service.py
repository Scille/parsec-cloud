from base64 import decodebytes, encodebytes
from datetime import datetime
import json

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from marshmallow import fields

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


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


class BaseFileService(BaseService):

    name = 'FileService'

    @cmd('file_create')
    async def _cmd_CREATE(self, session, msg):
        file = await self.create()
        file.update({'status': 'ok'})
        return file

    @cmd('file_read')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        file = await self.read(msg['id'])
        file.update({'status': 'ok'})
        return file

    @cmd('file_write')
    async def _cmd_WRITE(self, session, msg):
        msg = cmd_WRITE_Schema().load(msg)
        await self.write(msg['id'], msg['version'], msg['content'])
        return {'status': 'ok'}

    @cmd('file_stat')
    async def _cmd_STAT(self, session, msg):
        msg = cmd_STAT_Schema().load(msg)
        stat = await self.stat(msg['id'])
        stat.update({'status': 'ok'})
        return stat

    @cmd('file_history')
    async def _cmd_HISTORY(self, session, msg):
        msg = cmd_STAT_Schema().load(msg)
        history = await self.history(msg['id'])
        return {'status': 'ok', 'history': history}

    async def create(self):
        raise NotImplementedError()

    async def read(self, id):
        raise NotImplementedError()

    async def write(self, id, version, content):
        raise NotImplementedError()

    async def stat(self, id):
        raise NotImplementedError()

    async def history(self, id):
        raise NotImplementedError()


class FileService(BaseFileService):

    backend_api_service = service('BackendAPIService')
    crypto_service = service('CryptoService')
    identity_service = service('IdentityService')
    user_manifest_service = service('UserManifestService')

    async def create(self):
        vlob = await self.backend_api_service.vlob_create()
        return {'id': vlob.id,
                'read_trust_seed': vlob.read_trust_seed,
                'write_trust_seed': vlob.write_trust_seed}

    async def read(self, id):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except Exception:
            raise FileNotFound('Vlob not found.')
        vlob = await self.backend_api_service.vlob_read(id=id)
        version = len(vlob.blob_versions)
        if version == 0:
            return {'content': '', 'version': 0}
        blob = vlob.blob_versions[version - 1]
        encrypted_blob = decodebytes(blob.encode())
        blob_key = decodebytes(properties['key'].encode()) if properties['key'] else None
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, blob_key)
        blob = json.loads(blob.decode())
        block_key = decodebytes(blob['key'].encode())
        old_digest = decodebytes(blob['digest'].encode())
        # Get content
        block = await self.backend_api_service.block_read(id=blob['block'])
        # Decrypt
        encrypted_content = block['content'].encode()
        content = await self.crypto_service.sym_decrypt(encrypted_content, block_key)
        # Check integrity
        digest = hashes.Hash(hashes.SHA512(), backend=openssl)
        digest.update(content)
        new_digest = digest.finalize()
        assert new_digest == old_digest
        return {'content': content.decode(), 'version': version}

    async def write(self, id, version, content):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except Exception:
            raise FileNotFound('Vlob not found.')
        content = content.encode()
        size = len(decodebytes(content))
        # Digest
        digest = hashes.Hash(hashes.SHA512(), backend=openssl)
        digest.update(content)
        content_digest = digest.finalize()  # TODO replace with hexdigest ?
        content_digest = encodebytes(content_digest).decode()
        # Encrypt block
        block_key, data = await self.crypto_service.sym_encrypt(content)
        block_key = encodebytes(block_key).decode()
        data = data.decode()
        # Store block
        block_id = await self.backend_api_service.block_create(content=data)
        # Update vlob
        blob = {'block': block_id,
                'size': size,
                'key': block_key,
                'digest': content_digest}
        # Encrypt blob
        blob = json.dumps(blob)
        blob = blob.encode()
        blob_key = decodebytes(properties['key'].encode())
        _, encrypted_blob = await self.crypto_service.sym_encrypt(blob, blob_key)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await self.backend_api_service.vlob_update(
            id=id, next_version=version, blob=encrypted_blob)
        return block_key

    async def stat(self, id):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except Exception:
            raise FileNotFound('Vlob not found.')
        vlob = await self.backend_api_service.vlob_read(id=id)
        version = len(vlob.blob_versions)
        if version == 0:
            return {'id': id,
                    'ctime': datetime.utcnow().timestamp(),
                    'mtime': datetime.utcnow().timestamp(),
                    'atime': datetime.utcnow().timestamp(),
                    'size': 0}
        encrypted_blob = vlob.blob_versions[version - 1]
        encrypted_blob = decodebytes(encrypted_blob.encode())
        key = decodebytes(properties['key'].encode()) if properties['key'] else None
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, key)
        blob = json.loads(blob.decode())
        stat = await self.backend_api_service.block_stat(id=blob['block'])
        return {'id': id,
                'ctime': stat['creation_timestamp'],
                'mtime': stat['creation_timestamp'],
                'atime': stat['access_timestamp'],
                'size': blob['size']}

    async def history(self, id):
        # TODO raise ParsecNotImplementedError
        pass
