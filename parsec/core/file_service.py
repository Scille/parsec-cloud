from base64 import decodebytes, encodebytes
import json
import sys

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from marshmallow import fields

from parsec.core.user_manifest_service import UserManifestNotFound
from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class FileError(ParsecError):
    pass


class FileNotFound(FileError):
    status = 'not_found'


class cmd_CREATE_Schema(BaseCmdSchema):
    pass


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Integer(missing=None, validate=lambda n: n >= 1)
    size = fields.Integer(missing=None, validate=lambda n: n >= 1)
    offset = fields.Integer(missing=0, validate=lambda n: n >= 0)


class cmd_WRITE_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Integer(required=True, validate=lambda n: n >= 1)
    content = fields.String(required=True)
    offset = fields.Integer(missing=0, validate=lambda n: n >= 0)


class cmd_TRUNCATE_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Integer(required=True, validate=lambda n: n >= 1)
    length = fields.Integer(validate=lambda n: n >= 0)


class cmd_STAT_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Integer(missing=None, validate=lambda n: n >= 1)


class cmd_HISTORY_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    first_version = fields.Integer(missing=1, validate=lambda n: n >= 1)
    last_version = fields.Integer(missing=None, validate=lambda n: n >= 1)


class cmd_RESTORE_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Integer(missing=None, validate=lambda n: n >= 1)


class cmd_REENCRYPT_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class BaseFileService(BaseService):

    name = 'FileService'

    @cmd('file_create')
    async def _cmd_CREATE(self, session, msg):
        msg = cmd_CREATE_Schema().load(msg)
        file = await self.create()
        file.update({'status': 'ok'})
        return file

    @cmd('file_read')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        file = await self.read(msg['id'], msg['version'], msg['size'], msg['offset'])
        file.update({'status': 'ok'})
        return file

    @cmd('file_write')
    async def _cmd_WRITE(self, session, msg):
        msg = cmd_WRITE_Schema().load(msg)
        await self.write(msg['id'], msg['version'], msg['content'], msg['offset'])
        return {'status': 'ok'}

    @cmd('file_truncate')
    async def _cmd_TRUNCATE(self, session, msg):
        msg = cmd_TRUNCATE_Schema().load(msg)
        await self.truncate(msg['id'], msg['version'], msg['length'])
        return {'status': 'ok'}

    @cmd('file_stat')
    async def _cmd_STAT(self, session, msg):
        msg = cmd_STAT_Schema().load(msg)
        stat = await self.stat(msg['id'], msg['version'])
        stat.update({'status': 'ok'})
        return stat

    @cmd('file_history')
    async def _cmd_HISTORY(self, session, msg):
        msg = cmd_HISTORY_Schema().load(msg)
        history = await self.history(msg['id'], msg['first_version'], msg['last_version'])
        return {'status': 'ok', 'history': history}

    @cmd('file_restore')
    async def _cmd_RESTORE(self, session, msg):
        msg = cmd_RESTORE_Schema().load(msg)
        await self.restore(msg['id'], msg['version'])
        return {'status': 'ok'}

    @cmd('file_reencrypt')
    async def _cmd_REENCRYPT(self, session, msg):
        msg = cmd_REENCRYPT_Schema().load(msg)
        file = await self.reencrypt(msg['id'])
        file.update({'status': 'ok'})
        return file

    async def create(self, content=''):
        raise NotImplementedError()

    async def read(self, id, version, size, offset):
        raise NotImplementedError()

    async def write(self, id, version, content, offset):
        raise NotImplementedError()

    async def truncate(self, id, version, length):
        raise NotImplementedError()

    async def stat(self, id, version):
        raise NotImplementedError()

    async def history(self, id, first_version, last_version):
        raise NotImplementedError()

    async def restore(self, id, version):
        raise NotImplementedError()

    async def reencrypt(self, id):
        raise NotImplementedError()


class FileService(BaseFileService):

    backend_api_service = service('BackendAPIService')
    block_service = service('BlockService')
    crypto_service = service('CryptoService')
    identity_service = service('IdentityService')
    user_manifest_service = service('UserManifestService')

    async def create(self, content=b''):
        blob = [await self._build_file_blocks(content)]
        # Encrypt blob
        blob = json.dumps(blob)
        blob = blob.encode()
        blob_key, encrypted_blob = await self.crypto_service.sym_encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        ret = await self.backend_api_service.vlob_create(encrypted_blob)
        del ret['status']
        ret['key'] = encodebytes(blob_key).decode()
        return ret

    async def read(self, id, version=None, size=None, offset=0):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except UserManifestNotFound:
            try:
                properties = await self.user_manifest_service.get_properties(id=id, dustbin=True)
            except UserManifestNotFound:
                raise FileNotFound('Vlob not found.')
        vlob = await self.backend_api_service.vlob_read(id, properties['read_trust_seed'], version)
        version = vlob['version']
        blob = vlob['blob']
        encrypted_blob = decodebytes(blob.encode())
        blob_key = decodebytes(properties['key'].encode())
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, blob_key)
        blob = json.loads(blob.decode())
        # Get content
        matching_blocks = await self._find_matching_blocks(id, version, size, offset)
        content = b''
        content += decodebytes(matching_blocks['pre_included_content'].encode())
        for blocks_and_key in matching_blocks['included_blocks']:
            block_key = blocks_and_key['key']
            block_key = decodebytes(block_key.encode())
            for block_properties in blocks_and_key['blocks']:
                block = await self.block_service.read(id=block_properties['block'])
                # Decrypt
                chunk_content = await self.crypto_service.sym_decrypt(block['content'].encode(),
                                                                      block_key)
                chunk_content = decodebytes(chunk_content)
                # Check integrity
                digest = hashes.Hash(hashes.SHA512(), backend=openssl)
                digest.update(chunk_content)
                new_digest = digest.finalize()
                assert new_digest == decodebytes(block_properties['digest'].encode())
                content += chunk_content
        content += decodebytes(matching_blocks['post_included_content'].encode())
        content = encodebytes(content).decode()
        return {'content': content, 'version': version}

    async def write(self, id, version, content, offset=0):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except UserManifestNotFound:
            try:
                properties = await self.user_manifest_service.get_properties(id=id, dustbin=True)
            except UserManifestNotFound:
                raise FileNotFound('Vlob not found.')
        blob_key = decodebytes(properties['key'].encode())
        content = decodebytes(content.encode())
        matching_blocks = await self._find_matching_blocks(id, version - 1, len(content), offset)
        new_content = decodebytes(matching_blocks['pre_excluded_content'].encode())
        new_content += content
        new_content += decodebytes(matching_blocks['post_excluded_content'].encode())
        new_content = encodebytes(new_content).decode()
        blob = []
        blob += matching_blocks['pre_excluded_blocks']
        blob.append(await self._build_file_blocks(new_content))
        blob += matching_blocks['post_excluded_blocks']
        blob = json.dumps(blob)
        blob = blob.encode()
        _, encrypted_blob = await self.crypto_service.sym_encrypt(blob, blob_key)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await self.backend_api_service.vlob_update(
            id=id, version=version, blob=encrypted_blob, trust_seed=properties['write_trust_seed'])

    async def truncate(self, id, version, length):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except UserManifestNotFound:
            try:
                properties = await self.user_manifest_service.get_properties(id=id, dustbin=True)
            except UserManifestNotFound:
                raise FileNotFound('Vlob not found.')
        blob_key = decodebytes(properties['key'].encode())
        matching_blocks = await self._find_matching_blocks(id, version - 1, length, 0)
        blob = []
        blob += matching_blocks['included_blocks']
        blob.append(await self._build_file_blocks(matching_blocks['post_included_content']))
        blob = json.dumps(blob)
        blob = blob.encode()
        _, encrypted_blob = await self.crypto_service.sym_encrypt(blob, blob_key)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await self.backend_api_service.vlob_update(
            id=id, version=version, blob=encrypted_blob, trust_seed=properties['write_trust_seed'])

    async def stat(self, id, version=None):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except UserManifestNotFound:
            try:
                properties = await self.user_manifest_service.get_properties(id=id, dustbin=True)
            except UserManifestNotFound:
                raise FileNotFound('Vlob not found.')
        vlob = await self.backend_api_service.vlob_read(id, properties['read_trust_seed'], version)
        encrypted_blob = vlob['blob']
        encrypted_blob = decodebytes(encrypted_blob.encode())
        key = decodebytes(properties['key'].encode())
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, key)
        blob = json.loads(blob.decode())
        # TODO which block index? Or add timestamp in vlob_service ?
        stat = await self.block_service.stat(id=blob[-1]['blocks'][-1]['block'])
        size = 0
        for blocks_and_key in blob:
            for block in blocks_and_key['blocks']:
                size += block['size']
        # TODO: don't provide atime field if we don't know it?
        return {'id': id,
                'ctime': stat['creation_timestamp'],
                'mtime': stat['creation_timestamp'],
                'atime': stat['creation_timestamp'],
                'size': size,
                'version': vlob['version']}

    async def history(self, id, first_version=1, last_version=None):
        if first_version and last_version and first_version > last_version:
            raise FileError('bad_versions',
                            'First version number higher than the second one.')
        history = []
        if not last_version:
            stat = await self.stat(id)
            last_version = stat['version']
        for current_version in range(first_version, last_version + 1):
            stat = await self.stat(id, current_version)
            del stat['id']
            history.append(stat)
        return history

    async def restore(self, id, version=None):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except UserManifestNotFound:
            try:
                properties = await self.user_manifest_service.get_properties(id=id, dustbin=True)
            except UserManifestNotFound:
                raise FileNotFound('Vlob not found.')
        stat = await self.stat(id)
        if version is None:
            version = stat['version'] - 1 if stat['version'] > 1 else 1
        if version > 0 and version < stat['version']:

            vlob = await self.backend_api_service.vlob_read(
                id,
                properties['read_trust_seed'],
                version)
            await self.backend_api_service.vlob_update(
                id=id,
                version=stat['version'] + 1,
                blob=vlob['blob'],
                trust_seed=properties['write_trust_seed'])
        elif version < 1 or version > stat['version']:
            raise FileError('bad_version', 'Bad version number.')

    async def reencrypt(self, id):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except UserManifestNotFound:
            try:
                properties = await self.user_manifest_service.get_properties(id=id, dustbin=True)
            except UserManifestNotFound:
                raise FileNotFound('Vlob not found.')
        old_vlob = await self.backend_api_service.vlob_read(
            id=properties['id'],
            trust_seed=properties['read_trust_seed'])
        old_blob = old_vlob['blob']
        old_encrypted_blob = decodebytes(old_blob.encode())
        old_blob_key = decodebytes(properties['key'].encode())
        new_blob = await self.crypto_service.sym_decrypt(old_encrypted_blob, old_blob_key)
        new_key, new_encrypted_blob = await self.crypto_service.sym_encrypt(new_blob)
        new_encrypted_blob = encodebytes(new_encrypted_blob).decode()
        new_key = encodebytes(new_key).decode()
        new_vlob = await self.backend_api_service.vlob_create(new_encrypted_blob)
        del new_vlob['status']
        new_vlob['key'] = new_key
        return new_vlob

    async def _build_file_blocks(self, content):
        if isinstance(content, str):
            content = content.encode()
        content = decodebytes(content)
        # Create chunks
        chunk_size = 4096  # TODO modify size
        chunks = [content[i:i + chunk_size] for i in range(0, len(content), chunk_size)]
        # Force a chunk even if the content is empty
        if not chunks:
            chunks = [b'']
        block_key, _ = await self.crypto_service.sym_encrypt('')
        blocks = []
        for chunk in chunks:
            # Digest
            digest = hashes.Hash(hashes.SHA512(), backend=openssl)
            digest.update(chunk)
            chunk_digest = digest.finalize()  # TODO replace with hexdigest ?
            chunk_digest = encodebytes(chunk_digest).decode()
            # Encrypt block
            encoded_chunk = encodebytes(chunk).decode()
            _, cypher_chunk = await self.crypto_service.sym_encrypt(encoded_chunk, block_key)
            cypher_chunk = cypher_chunk.decode()
            # Store block
            block_id = await self.block_service.create(content=cypher_chunk)
            blocks.append({'block': block_id,
                           'digest': chunk_digest,
                           'size': len(chunk)})
        # New vlob atom
        block_key = encodebytes(block_key).decode()
        blob = {'blocks': blocks,
                'key': block_key}
        return blob

    async def _find_matching_blocks(self, id, version=None, size=None, offset=0):
        try:
            properties = await self.user_manifest_service.get_properties(id=id)
        except UserManifestNotFound:
            try:
                properties = await self.user_manifest_service.get_properties(id=id, dustbin=True)
            except UserManifestNotFound:
                raise FileNotFound('Vlob not found.')
        if size is None:
            size = sys.maxsize
        pre_excluded_blocks = []
        post_excluded_blocks = []
        vlob = await self.backend_api_service.vlob_read(id, properties['read_trust_seed'], version)
        blob = vlob['blob']
        encrypted_blob = decodebytes(blob.encode())
        blob_key = decodebytes(properties['key'].encode())
        blob = await self.crypto_service.sym_decrypt(encrypted_blob, blob_key)
        blob = json.loads(blob.decode())
        pre_excluded_blocks = []
        included_blocks = []
        post_excluded_blocks = []
        cursor = 0
        pre_excluded_content = ''
        pre_included_content = ''
        post_included_content = ''
        post_excluded_content = ''
        for blocks_and_key in blob:
            block_key = blocks_and_key['key']
            decoded_block_key = decodebytes(block_key.encode())
            for block_properties in blocks_and_key['blocks']:
                cursor += block_properties['size']
                if cursor <= offset:
                    if len(pre_excluded_blocks) and pre_excluded_blocks[-1]['key'] == block_key:
                        pre_excluded_blocks[-1]['blocks'].append(block_properties)
                    else:
                        pre_excluded_blocks.append({'blocks': [block_properties], 'key': block_key})
                elif cursor > offset and cursor - block_properties['size'] < offset:
                    delta = cursor - offset
                    block = await self.block_service.read(block_properties['block'])
                    block_content = await self.crypto_service.sym_decrypt(
                        block['content'].encode(),
                        decoded_block_key)
                    block_content = decodebytes(block_content).decode()
                    pre_excluded_content = block_content[:-delta]
                    pre_included_content = block_content[-delta:][:size]
                    if size < len(block_content[-delta:]):
                        post_excluded_content = block_content[-delta:][size:]
                elif cursor > offset and cursor <= offset + size:
                    if len(included_blocks) and included_blocks[-1]['key'] == block_key:
                        included_blocks[-1]['blocks'].append(block_properties)
                    else:
                        included_blocks.append({'blocks': [block_properties], 'key': block_key})
                elif cursor > offset + size and cursor - block_properties['size'] < offset + size:
                    delta = offset + size - (cursor - block_properties['size'])
                    block = await self.block_service.read(block_properties['block'])
                    block_content = await self.crypto_service.sym_decrypt(
                        block['content'].encode(),
                        decoded_block_key)
                    block_content = decodebytes(block_content).decode()
                    post_included_content = block_content[:delta]
                    post_excluded_content = block_content[delta:]
                else:
                    if len(post_excluded_blocks) and post_excluded_blocks[-1]['key'] == block_key:
                        post_excluded_blocks[-1]['blocks'].append(block_properties)
                    else:
                        post_excluded_blocks.append({'blocks': [block_properties],
                                                     'key': block_key})
        pre_included_content = encodebytes(pre_included_content.encode()).decode()
        pre_excluded_content = encodebytes(pre_excluded_content.encode()).decode()
        post_included_content = encodebytes(post_included_content.encode()).decode()
        post_excluded_content = encodebytes(post_excluded_content.encode()).decode()
        return {
            'pre_excluded_blocks': pre_excluded_blocks,
            'pre_excluded_content': pre_excluded_content,
            'pre_included_content': pre_included_content,
            'included_blocks': included_blocks,
            'post_included_content': post_included_content,
            'post_excluded_content': post_excluded_content,
            'post_excluded_blocks': post_excluded_blocks
        }
