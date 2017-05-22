from base64 import encodebytes, decodebytes
from copy import deepcopy
import json
from os import path
import random

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from freezegun import freeze_time
import gnupg
import pytest

from parsec.core import (CryptoService, FileService,
                         IdentityService, GNUPGPubKeysService, MetaBlockService,
                         MockedBackendAPIService, MockedBlockService, MockedCacheService,
                         ShareService, UserManifestService)
from parsec.core.file_service import FileNotFound
from parsec.server import BaseServer


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


@pytest.fixture
def user_manifest_svc():
    return UserManifestService()


@pytest.fixture
def crypto_svc():
    crypto_service = CryptoService()
    crypto_service.gnupg = gnupg.GPG(homedir=GNUPG_HOME + '/secret_env')
    return crypto_service


@pytest.fixture
def file_svc(event_loop, user_manifest_svc, crypto_svc):
    identity = '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'
    service = FileService()
    mocked_cache_service = MockedCacheService()
    MockedBlockService.cache_service = mocked_cache_service
    block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
    identity_service = IdentityService()
    server = BaseServer()
    server.register_service(service)
    server.register_service(block_service)
    server.register_service(crypto_svc)
    server.register_service(identity_service)
    server.register_service(mocked_cache_service)
    server.register_service(user_manifest_svc)
    server.register_service(GNUPGPubKeysService())
    server.register_service(MockedBackendAPIService())
    server.register_service(ShareService())
    event_loop.run_until_complete(server.bootstrap_services())
    event_loop.run_until_complete(identity_service.load_identity(identity=identity))
    event_loop.run_until_complete(user_manifest_svc.load_user_manifest())
    yield service
    event_loop.run_until_complete(server.teardown_services())


class TestFileService:

    @pytest.mark.asyncio
    async def test_create(self, file_svc):
        ret = await file_svc.dispatch_msg({'cmd': 'file_create'})
        assert ret['status'] == 'ok'
        ret_2 = await file_svc.dispatch_msg({'cmd': 'file_create'})
        assert ret_2['status'] == 'ok'
        assert ret['id'] != ret_2['id']

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_create', 'id': '1234'},
        {}])
    async def test_bad_msg_create(self, file_svc, bad_msg):
        ret = await file_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_read_not_found(self, file_svc):
        ret = await file_svc.dispatch_msg({'cmd': 'file_read',
                                           'id': '5ea26ae2479c49f58ede248cdca1a3ca'})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_read(self, file_svc, user_manifest_svc):
        file_vlob = await user_manifest_svc.create_file('/test')
        id = file_vlob['id']
        # Empty file
        ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id})
        assert ret == {'status': 'ok', 'content': '', 'version': 1}
        # Not empty file
        content = 'This is a test content.'
        encoded_content = encodebytes(content.encode()).decode()
        await file_svc.write(id, 2, encoded_content)
        ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id})
        assert ret == {'status': 'ok', 'content': encoded_content, 'version': 2}
        # Offset
        offset = 5
        encoded_content = encodebytes(content[offset:].encode()).decode()
        ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id, 'offset': offset})
        assert ret == {'status': 'ok', 'content': encoded_content, 'version': 2}
        # Size
        size = 9
        encoded_content = encodebytes(content[offset:][:size].encode()).decode()
        ret = await file_svc.dispatch_msg({'cmd': 'file_read',
                                           'id': id,
                                           'size': size,
                                           'offset': offset})
        assert ret == {'status': 'ok', 'content': encoded_content, 'version': 2}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_read', 'id': 42},
        {'cmd': 'file_read', 'id': None},
        {'cmd': 'file_read', 'id': '<id-here>', 'version': -1},
        {'cmd': 'file_read', 'id': '<id-here>', 'version': '<version-here>', 'size': 0},
        {'cmd': 'file_read', 'id': '<id-here>', 'version': '<version-here>', 'offset': -1},
        {'cmd': 'file_read', 'id': '<id-here>', 'version': '<version-here>', 'size': 1,
         'offset': -1},
        {'cmd': 'file_read'}, {}])
    async def test_bad_msg_read(self, file_svc, user_manifest_svc, bad_msg):
        file_vlob = await user_manifest_svc.create_file('/test')
        file_vlob = await file_svc.stat(file_vlob['id'])
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = file_vlob['id']
        if bad_msg.get('version') == '<version-here>':
            bad_msg['version'] = file_vlob['version']
        ret = await file_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_write_not_found(self, file_svc):
        content = encodebytes('foo'.encode()).decode()
        ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                           'id': '1234',
                                           'version': 1,
                                           'content': content})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_write(self, file_svc, user_manifest_svc):
        file_vlob = await user_manifest_svc.create_file('/test')
        id = file_vlob['id']
        # Check with empty and not empty file
        for version, content in enumerate(('this is v2 content', 'this is v3 content'), 2):
            encoded_content = encodebytes(content.encode()).decode()
            ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                               'id': id,
                                               'version': version,
                                               'content': encoded_content})
            assert ret == {'status': 'ok'}
            file = await file_svc.read(id)
            assert file == {'content': encoded_content, 'version': version}
        # Offset
        encoded_content = encodebytes('v4'.encode()).decode()
        version = 4
        ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                           'id': id,
                                           'version': version,
                                           'content': encoded_content,
                                           'offset': 8})
        assert ret == {'status': 'ok'}
        file = await file_svc.read(id)
        encoded_content = encodebytes('this is v4 content'.encode()).decode()
        assert file == {'content': encoded_content, 'version': version}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_write', 'id': 42},
        {'cmd': 'file_write', 'id': None},
        {'cmd': 'file_write', 'id': '<id-here>', 'content': 'foo'},
        {'cmd': 'file_write', 'id': '<id-here>', 'version': -1, 'content': 'foo'},
        {'cmd': 'file_write', 'id': '<id-here>', 'version': '<version-here>', 'content': 'foo',
         'offset': -1},
        {'cmd': 'file_write'}, {}])
    async def test_bad_msg_write(self, file_svc, user_manifest_svc, bad_msg):
        file_vlob = await user_manifest_svc.create_file('/test')
        file_vlob = await file_svc.stat(file_vlob['id'])
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = file_vlob['id']
        if bad_msg.get('version') == '<version-here>':
            bad_msg['version'] = file_vlob['version']
        ret = await file_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_truncate_not_found(self, file_svc):
        ret = await file_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'id': '1234',
                                           'version': 1,
                                           'length': 7})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_truncate(self, file_svc, user_manifest_svc, crypto_svc):
        # Encoded contents
        block_size = 4096
        content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, block_size + 1)])
        encoded_content = encodebytes(content).decode()
        # Blocks
        blocks = await file_svc._build_file_blocks(encoded_content)
        # Create file
        blob = json.dumps([blocks])
        blob = blob.encode()
        file_vlob = await user_manifest_svc.create_file('/foo')
        blob_key = decodebytes(file_vlob['key'].encode())
        _, encrypted_blob = await crypto_svc.sym_encrypt(blob, blob_key)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        # Write content
        id = file_vlob['id']
        await file_svc.backend_api_service.vlob_update(
            id=id,
            version=2,
            blob=encrypted_blob,
            trust_seed=file_vlob['write_trust_seed'])
        # Truncate full length
        ret = await file_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'id': id,
                                           'version': 3,
                                           'length': block_size + 1})
        assert ret == {'status': 'ok'}
        file = await file_svc.read(id)
        encoded_content = encodebytes(content[:block_size + 1]).decode()
        assert file == {'content': encoded_content, 'version': 3}
        # Truncate block length
        ret = await file_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'id': id,
                                           'version': 4,
                                           'length': block_size})
        assert ret == {'status': 'ok'}
        file = await file_svc.read(id)
        encoded_content = encodebytes(content[:block_size]).decode()
        assert file == {'content': encoded_content, 'version': 4}
        # Truncate shorter than block length
        ret = await file_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'id': id,
                                           'version': 5,
                                           'length': block_size - 1})
        assert ret == {'status': 'ok'}
        file = await file_svc.read(id)
        encoded_content = encodebytes(content[:block_size - 1]).decode()
        assert file == {'content': encoded_content, 'version': 5}
        # Truncate empty
        ret = await file_svc.dispatch_msg({'cmd': 'file_truncate',
                                           'id': id,
                                           'version': 6,
                                           'length': 0})
        assert ret == {'status': 'ok'}
        file = await file_svc.read(id)
        assert file == {'content': '', 'version': 6}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_truncate', 'id': 42},
        {'cmd': 'file_truncate', 'id': None},
        {'cmd': 'file_truncate', 'id': '<id-here>', 'length': -1},
        {'cmd': 'file_truncate', 'id': '<id-here>', 'version': '<version-here>', 'length': -1},
        {'cmd': 'file_truncate'}, {}])
    async def test_bad_msg_truncate(self, file_svc, user_manifest_svc, bad_msg):
        file_vlob = await user_manifest_svc.create_file('/test')
        file_vlob = await file_svc.stat(file_vlob['id'])
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = file_vlob['id']
        if bad_msg.get('version') == '<version-here>':
            bad_msg['version'] = file_vlob['version']
        ret = await file_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_stat_not_found(self, file_svc):
        ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': '1234'})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_stat(self, file_svc, user_manifest_svc):
        # Good file
        with freeze_time('2012-01-01') as frozen_datetime:
            file_vlob = await user_manifest_svc.create_file('/test')
            id = file_vlob['id']
            ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
            ctime = frozen_datetime().timestamp()
            assert ret == {'status': 'ok',
                           'id': id,
                           'ctime': ctime,
                           'mtime': ctime,
                           'atime': ctime,
                           'size': 0,
                           'version': 1}
            frozen_datetime.tick()
            mtime = frozen_datetime().timestamp()
            content = encodebytes('foo'.encode()).decode()
            await file_svc.write(id, 2, content)
            ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
            assert ret == {'status': 'ok',
                           'id': id,
                           'ctime': mtime,
                           'mtime': mtime,
                           'atime': mtime,
                           'size': 3,
                           'version': 2}
            frozen_datetime.tick()
            await file_svc.read(id)  # TODO useless if atime is not modified
            ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
            assert ret == {'status': 'ok',
                           'id': id,
                           'ctime': mtime,
                           'mtime': mtime,
                           'atime': mtime,
                           'size': 3,
                           'version': 2}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_truncate', 'id': 42},
        {'cmd': 'file_truncate', 'id': None},
        {'cmd': 'file_truncate', 'id': '<id-here>', 'version': -1},
        {'cmd': 'file_truncate'}, {}])
    async def test_bad_msg_stat(self, file_svc, bad_msg):
        file_vlob = await file_svc.create()
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = file_vlob['id']
        ret = await file_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_history_not_found(self, file_svc):
        ret = await file_svc.dispatch_msg({'cmd': 'file_history', 'id': '1234'})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_history(self, file_svc, user_manifest_svc):
        with freeze_time('2012-01-01') as frozen_datetime:
            file_vlob = await user_manifest_svc.create_file('/test')
            id = file_vlob['id']
            original_time = frozen_datetime().timestamp()
            for version, content in enumerate(('this is v2', 'this is v3...'), 2):
                frozen_datetime.tick()
                encoded_content = encodebytes(content.encode()).decode()
                await file_svc.write(id, version, encoded_content)
        # Full history
        ret = await file_svc.dispatch_msg({'cmd': 'file_history', 'id': id})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 1,
                    'ctime': original_time,
                    'mtime': original_time,
                    'atime': original_time,
                    'size': 0
                },
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                },
                {
                    'version': 3,
                    'ctime': original_time + 2,
                    'mtime': original_time + 2,
                    'atime': original_time + 2,
                    'size': 13
                }
            ]
        }
        # Partial history starting at version 2
        ret = await file_svc.dispatch_msg({'cmd': 'file_history', 'id': id, 'first_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                },
                {
                    'version': 3,
                    'ctime': original_time + 2,
                    'mtime': original_time + 2,
                    'atime': original_time + 2,
                    'size': 13
                }
            ]
        }
        # Partial history ending at version 2
        ret = await file_svc.dispatch_msg({'cmd': 'file_history', 'id': id, 'last_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 1,
                    'ctime': original_time,
                    'mtime': original_time,
                    'atime': original_time,
                    'size': 0
                },
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                }
            ]
        }
        # First version = last version
        ret = await file_svc.dispatch_msg({'cmd': 'file_history',
                                           'id': id,
                                           'first_version': 2,
                                           'last_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                }
            ]
        }
        # First version > last version
        ret = await file_svc.dispatch_msg({'cmd': 'file_history',
                                           'id': id,
                                           'first_version': 3,
                                           'last_version': 2})
        assert ret == {'status': 'bad_versions',
                       'label': 'First version number higher than the second one.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_history', 'id': 42},
        {'cmd': 'file_history', 'id': '<id-here>', 'first_version': -1},
        {'cmd': 'file_history', 'id': '<id-here>', 'last_version': -1},
        {'cmd': 'file_history'}, {}])
    async def test_bad_msg_history(self, file_svc, user_manifest_svc, bad_msg):
        file_vlob = await user_manifest_svc.create_file('/test')
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = file_vlob['id']
        ret = await file_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_restore_not_found(self, file_svc):
        ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': '1234', 'version': 10})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_restore(self, file_svc, user_manifest_svc):
        encoded_content = encodebytes('initial'.encode()).decode()
        file_vlob = await user_manifest_svc.create_file('/test', encoded_content)
        id = file_vlob['id']
        # Restore file with version 1
        file = await file_svc.read(id)
        assert file == {'content': encoded_content, 'version': 1}
        ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': id})
        file = await file_svc.read(id)
        assert file == {'content': encoded_content, 'version': 1}
        # Restore previous version
        for version, content in enumerate(('this is v2', 'this is v3', 'this is v4'), 2):
            encoded_content = encodebytes(content.encode()).decode()
            await file_svc.write(id, version, encoded_content)
        file = await file_svc.read(id)
        encoded_content = encodebytes('this is v4'.encode()).decode()
        assert file == {'content': encoded_content, 'version': 4}
        ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': id})
        file = await file_svc.read(id)
        encoded_content = encodebytes('this is v3'.encode()).decode()
        assert file == {'content': encoded_content, 'version': 5}
        # Restore old version
        ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': id, 'version': 2})
        file = await file_svc.read(id)
        encoded_content = encodebytes('this is v2'.encode()).decode()
        assert file == {'content': encoded_content, 'version': 6}
        # Bad version
        ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': id, 'version': 10})
        assert ret == {'status': 'bad_version', 'label': 'Bad version number.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_restore', 'id': 42},
        {'cmd': 'file_restore', 'id': '<id-here>', 'version': 0},
        {'cmd': 'file_restore'}, {}])
    async def test_bad_msg_restore(self, file_svc, user_manifest_svc, bad_msg):
        file_vlob = await user_manifest_svc.create_file('/test')
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = file_vlob['id']
        ret = await file_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_reencrypt_not_found(self, file_svc):
        ret = await file_svc.dispatch_msg({'cmd': 'file_reencrypt', 'id': '1234'})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_reencrypt(self, file_svc, user_manifest_svc):
        encoded_content_initial = encodebytes('content 1'.encode()).decode()
        encoded_content_final = encodebytes('content 2'.encode()).decode()
        old_vlob = await user_manifest_svc.create_file('/foo', encoded_content_initial)
        ret = await file_svc.dispatch_msg({'cmd': 'file_reencrypt', 'id': old_vlob['id']})
        assert ret['status'] == 'ok'
        del ret['status']
        new_vlob = ret
        for property in old_vlob.keys():
            assert old_vlob[property] != new_vlob[property]
        await user_manifest_svc.import_file_vlob('/bar', new_vlob)
        await file_svc.write(new_vlob['id'], 2, encoded_content_final)
        file = await file_svc.read(old_vlob['id'])
        assert file == {'content': encoded_content_initial, 'version': 1}
        file = await file_svc.read(new_vlob['id'])
        assert file == {'content': encoded_content_final, 'version': 2}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'file_reencrypt', 'id': 42},
        {'cmd': 'file_reencrypt'}, {}])
    async def test_bad_msg_reencrypt(self, file_svc, user_manifest_svc, bad_msg):
        file_vlob = await user_manifest_svc.create_file('/test')
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = file_vlob['id']
        ret = await file_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    @pytest.mark.parametrize('length', [0, 4095, 4096, 4097])
    async def test_build_file_blocks(self, file_svc, crypto_svc, length):

        def digest(chunk):
            digest = hashes.Hash(hashes.SHA512(), backend=openssl)
            digest.update(chunk)
            chunk_digest = digest.finalize()  # TODO replace with hexdigest ?
            chunk_digest = encodebytes(chunk_digest).decode()
            return chunk_digest

        block_size = 4096
        content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, length)])
        encoded_content = encodebytes(content).decode()
        blocks = await file_svc._build_file_blocks(encoded_content)
        assert sorted(blocks.keys()) == ['blocks', 'key']
        assert isinstance(blocks['blocks'], list)
        required_blocks = int(len(content) / block_size)
        if not len(content) or len(content) % block_size:
            required_blocks += 1
        assert len(blocks['blocks']) == required_blocks
        for index, block in enumerate(blocks['blocks']):
            assert sorted(block.keys()) == ['block', 'digest', 'size']
            assert block['block']
            length = len(content) - index * block_size
            length = block_size if length > block_size else length
            assert block['size'] == length
            assert block['digest'] == digest(content[index * block_size:index + 1 * block_size])

    @pytest.mark.asyncio
    async def test_find_matching_blocks_not_found(self, file_svc):
        with pytest.raises(FileNotFound):
            await file_svc._find_matching_blocks('1234')

    @pytest.mark.asyncio
    async def test_find_matching_blocks(self, file_svc, crypto_svc, user_manifest_svc):
        block_size = 4096
        # Contents
        contents = {}
        total_length = 0
        for index, length in enumerate([block_size + 1,
                                        block_size - 1,
                                        block_size,
                                        2 * block_size + 2,
                                        2 * block_size - 2,
                                        2 * block_size]):
            content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, length)])
            contents[index] = content
            total_length += length
        # Encoded contents
        encoded_contents = {}
        for index, content in contents.items():
            encoded_contents[index] = encodebytes(contents[index]).decode()
        # Blocks
        blocks = {}
        for index, encoded_content in encoded_contents.items():
            blocks[index] = await file_svc._build_file_blocks(encoded_content)
        # Create file
        blob = json.dumps([blocks[i] for i in range(0, len(blocks))])
        blob = blob.encode()
        file_vlob = await user_manifest_svc.create_file('/foo')
        blob_key = decodebytes(file_vlob['key'].encode())
        _, encrypted_blob = await crypto_svc.sym_encrypt(blob, blob_key)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await file_svc.backend_api_service.vlob_update(
            id=file_vlob['id'],
            version=2,
            blob=encrypted_blob,
            trust_seed=file_vlob['write_trust_seed'])
        # All matching blocks
        matching_blocks = await file_svc._find_matching_blocks(file_vlob['id'])
        assert matching_blocks == {'pre_excluded_blocks': [],
                                   'pre_excluded_content': '',
                                   'pre_included_content': '',
                                   'included_blocks': [blocks[i] for i in range(0, len(blocks))],
                                   'post_included_content': '',
                                   'post_excluded_content': '',
                                   'post_excluded_blocks': []
                                   }
        # With offset
        delta = 10
        offset = (blocks[0]['blocks'][0]['size'] + blocks[0]['blocks'][1]['size'] +
                  blocks[1]['blocks'][0]['size'] + blocks[2]['blocks'][0]['size'] - delta)
        matching_blocks = await file_svc._find_matching_blocks(file_vlob['id'],
                                                               version=2,
                                                               offset=offset)
        pre_excluded_content = contents[2][:blocks[2]['blocks'][0]['size'] - delta]
        pre_included_content = contents[2][-delta:]
        encoded_pre_excluded_content = encodebytes(pre_excluded_content).decode()
        encoded_pre_included_content = encodebytes(pre_included_content).decode()
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_content': encoded_pre_excluded_content,
                                   'pre_included_content': encoded_pre_included_content,
                                   'included_blocks': [blocks[i] for i in range(3, 6)],
                                   'post_included_content': '',
                                   'post_excluded_content': '',
                                   'post_excluded_blocks': []
                                   }
        # With small size
        delta = 10
        size = 5
        offset = (blocks[0]['blocks'][0]['size'] + blocks[0]['blocks'][1]['size'] +
                  blocks[1]['blocks'][0]['size'] + blocks[2]['blocks'][0]['size'] - delta)
        matching_blocks = await file_svc._find_matching_blocks(file_vlob['id'],
                                                               version=2,
                                                               offset=offset,
                                                               size=size)
        pre_excluded_content = contents[2][:blocks[2]['blocks'][0]['size'] - delta]
        pre_included_content = contents[2][-delta:][:size]
        post_excluded_content = contents[2][-delta:][size:]
        encoded_pre_excluded_content = encodebytes(pre_excluded_content).decode()
        encoded_pre_included_content = encodebytes(pre_included_content).decode()
        encoded_post_excluded_content = encodebytes(post_excluded_content).decode()
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_content': encoded_pre_excluded_content,
                                   'pre_included_content': encoded_pre_included_content,
                                   'included_blocks': [],
                                   'post_included_content': '',
                                   'post_excluded_content': encoded_post_excluded_content,
                                   'post_excluded_blocks': [blocks[i] for i in range(3, 6)]
                                   }
        # With big size
        delta = 10
        size = delta
        size += blocks[3]['blocks'][0]['size']
        size += blocks[3]['blocks'][1]['size']
        size += blocks[3]['blocks'][2]['size']
        size += 2 * delta
        offset = (blocks[0]['blocks'][0]['size'] + blocks[0]['blocks'][1]['size'] +
                  blocks[1]['blocks'][0]['size'] + blocks[2]['blocks'][0]['size'] - delta)
        matching_blocks = await file_svc._find_matching_blocks(file_vlob['id'],
                                                               version=2,
                                                               offset=offset,
                                                               size=size)
        pre_excluded_content = contents[2][:-delta]
        pre_included_content = contents[2][-delta:]
        post_included_content = contents[4][:2 * delta]
        post_excluded_content = contents[4][:block_size][2 * delta:]
        encoded_pre_excluded_content = encodebytes(pre_excluded_content).decode()
        encoded_pre_included_content = encodebytes(pre_included_content).decode()
        encoded_post_included_content = encodebytes(post_included_content).decode()
        encoded_post_excluded_content = encodebytes(post_excluded_content).decode()
        partial_block_4 = deepcopy(blocks[4])
        del partial_block_4['blocks'][0]
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_content': encoded_pre_excluded_content,
                                   'pre_included_content': encoded_pre_included_content,
                                   'included_blocks': [blocks[3]],
                                   'post_included_content': encoded_post_included_content,
                                   'post_excluded_content': encoded_post_excluded_content,
                                   'post_excluded_blocks': [partial_block_4, blocks[5]]
                                   }
        # With big size and no delta
        size = blocks[3]['blocks'][0]['size']
        size += blocks[3]['blocks'][1]['size']
        size += blocks[3]['blocks'][2]['size']
        offset = (blocks[0]['blocks'][0]['size'] + blocks[0]['blocks'][1]['size'] +
                  blocks[1]['blocks'][0]['size'] + blocks[2]['blocks'][0]['size'])
        matching_blocks = await file_svc._find_matching_blocks(file_vlob['id'],
                                                               version=2,
                                                               offset=offset,
                                                               size=size)
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1], blocks[2]],
                                   'pre_excluded_content': '',
                                   'pre_included_content': '',
                                   'included_blocks': [blocks[3]],
                                   'post_included_content': '',
                                   'post_excluded_content': '',
                                   'post_excluded_blocks': [blocks[4], blocks[5]]
                                   }
        # # With total size
        matching_blocks = await file_svc._find_matching_blocks(file_vlob['id'],
                                                               version=2,
                                                               offset=0,
                                                               size=total_length)
        assert matching_blocks == {'pre_excluded_blocks': [],
                                   'pre_excluded_content': '',
                                   'pre_included_content': '',
                                   'included_blocks': [blocks[i] for i in range(0, 6)],
                                   'post_included_content': '',
                                   'post_excluded_content': '',
                                   'post_excluded_blocks': []
                                   }
