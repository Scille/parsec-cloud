from base64 import decodebytes, encodebytes
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

    async def create(self, content=''):
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
    block_service = service('BlockService')
    crypto_service = service('CryptoService')
    identity_service = service('IdentityService')
    user_manifest_service = service('UserManifestService')

    async def create(self, content=''):
        blob = await self._build_file_blocks(content)
        # Encrypt blob
        blob = json.dumps(blob)
        blob = blob.encode()
        blob_key, encrypted_blob = await self.crypto_service.sym_encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        ret = await self.backend_api_service.vlob_create(encrypted_blob)
        del ret['status']
        ret['key'] = encodebytes(blob_key).decode()
        return ret

    async def read(self, id):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except Exception:
            raise FileNotFound('Vlob not found.')
        vlob = await self.backend_api_service.vlob_read(id, properties['read_trust_seed'])
        version = vlob['version']
        blob = vlob['blob']
        encrypted_blob = decodebytes(blob.encode())
        blob_key = decodebytes(properties['key'].encode()) if properties['key'] else None
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, blob_key)
        blob = json.loads(blob.decode())
        block_key = decodebytes(blob['key'].encode())
        old_digest = decodebytes(blob['digest'].encode())
        # Get content
        content = b''
        for block_id in blob['blocks']:
            block = await self.block_service.read(id=block_id)
            # Decrypt
            encrypted_content = block['content'].encode()
            chunk_content = await self.crypto_service.sym_decrypt(encrypted_content, block_key)
            content += chunk_content
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
        blob = await self._build_file_blocks(content)
        # Encrypt blob
        blob = json.dumps(blob)
        blob = blob.encode()
        blob_key = decodebytes(properties['key'].encode())
        _, encrypted_blob = await self.crypto_service.sym_encrypt(blob, blob_key)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await self.backend_api_service.vlob_update(
            id=id, version=version, blob=encrypted_blob, trust_seed=properties['write_trust_seed'])
        return blob_key

    async def _build_file_blocks(self, content):
        if isinstance(content, str):
            content = content.encode()
        size = len(decodebytes(content))
        # Digest
        digest = hashes.Hash(hashes.SHA512(), backend=openssl)
        digest.update(content)
        content_digest = digest.finalize()  # TODO replace with hexdigest ?
        content_digest = encodebytes(content_digest).decode()
        # Create chunks
        chunk_size = 4096  # TODO modify size
        chunks = [content[i:i + chunk_size] for i in range(0, len(content), chunk_size)]
        # Force a chunk even if the content is empty
        if not chunks:
            chunks = ['']
        block_key, _ = await self.crypto_service.sym_encrypt('')
        blocks = []
        for chunk in chunks:
            # Encrypt block
            _, cypher_chunk = await self.crypto_service.sym_encrypt(chunk, block_key)
            cypher_chunk = cypher_chunk.decode()
            # Store block
            block_id = await self.block_service.create(content=cypher_chunk)
            blocks.append(block_id)
        # New vlob atom
        block_key = encodebytes(block_key).decode()
        blob = {'blocks': blocks,
                'size': size,
                'key': block_key,
                'digest': content_digest}
        return blob

    async def stat(self, id):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except Exception:
            raise FileNotFound('Vlob not found.')
        vlob = await self.backend_api_service.vlob_read(id=id,
                                                        trust_seed=properties['read_trust_seed'])
        encrypted_blob = vlob['blob']
        encrypted_blob = decodebytes(encrypted_blob.encode())
        key = decodebytes(properties['key'].encode()) if properties['key'] else None
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, key)
        blob = json.loads(blob.decode())
        stat = await self.block_service.stat(id=blob['blocks'][0])
        return {'id': id,
                'ctime': stat['creation_timestamp'],
                'mtime': stat['creation_timestamp'],
                'atime': stat['access_timestamp'],
                'size': blob['size']}

    async def history(self, id):
        # TODO raise ParsecNotImplementedError
        pass
