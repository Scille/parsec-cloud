from base64 import encodebytes, decodebytes
from copy import deepcopy
import json
# from io import BytesIO
import random

# from freezegun import freeze_time
import pytest

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes

# from parsec.core.buffers import BufferedBlock, BufferedUserVlob, BufferedVlob
# from parsec.core import (CoreService, IdentityService, MetaBlockService,
#                          MockedBackendAPIService, MockedBlockService)
# from parsec.core.manifest import GroupManifest, Manifest, UserManifest
from parsec.core import (MockedBackendAPIService, MetaBlockService, MockedBlockService,
                         SynchronizerService)
from parsec.core.file import File
from parsec.crypto import generate_sym_key, load_sym_key
from parsec.exceptions import BlockNotFound, FileError, FileNotFound, VlobNotFound
# from parsec.exceptions import UserManifestError, UserManifestNotFound, FileNotFound, VlobNotFound
from parsec.server import BaseServer


JOHN_DOE_IDENTITY = 'John_Doe'
JOHN_DOE_PRIVATE_KEY = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICWgIBAAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO04KJSq1cH87KtmkqI3qewvXtW
qFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/mG/U9gURDi4aTTXT02RbHESBp
yMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49SLhqGNmNAnH2E3lxAgMBAAEC
gYBY2S0QFJG8AwCdfKKUK+t2q+UO6wscwdtqSk/grBg8MWXTb+8XjteRLy3gD9Eu
E1IpwPStjj7KYEnp2blAvOKY0E537d2a4NLrDbSi84q8kXqvv0UeGMC0ZB2r4C89
/6BTZii4mjIlg3iPtkbRdLfexjqmtELliPkHKJIIMH3fYQJBAKd/k1hhnoxEx4sq
GRKueAX7orR9iZHraoR9nlV69/0B23Na0Q9/mP2bLphhDS4bOyR8EXF3y6CjSVO4
LBDPOmUCQQCV5hr3RxGPuYi2n2VplocLK/6UuXWdrz+7GIqZdQhvhvYSKbqZ5tvK
Ue8TYK3Dn4K/B+a7wGTSx3soSY3RBqwdAkAv94jqtooBAXFjmRq1DuGwVO+zYIAV
GaXXa2H8eMqr2exOjKNyHMhjWB1v5dswaPv25tDX/caCqkBFiWiVJ8NBAkBnEnqo
Xe3tbh5btO7+08q4G+BKU9xUORURiaaELr1GMv8xLhBpkxy+2egS4v+Y7C3zPXOi
1oB9jz1YTnt9p6DhAkBy0qgscOzo4hAn062MAYWA6hZOTkvzRbRpnyTRctKwZPSC
+tnlGk8FAkuOm/oKabDOY1WZMkj5zWAXrW4oR3Q2
-----END RSA PRIVATE KEY-----
"""
JOHN_DOE_PUBLIC_KEY = b"""
-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO0
4KJSq1cH87KtmkqI3qewvXtWqFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/m
G/U9gURDi4aTTXT02RbHESBpyMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49
SLhqGNmNAnH2E3lxAgMBAAE=
-----END PUBLIC KEY-----
"""


@pytest.fixture
def synchronizer_svc(event_loop):
    service = SynchronizerService()
    server = BaseServer()
    server.register_service(service)
    server.register_service(MetaBlockService(backends=[MockedBlockService, MockedBlockService]))
    server.register_service(MockedBackendAPIService())
    event_loop.run_until_complete(server.bootstrap_services())
    yield service
    event_loop.run_until_complete(server.teardown_services())


# @pytest.fixture
# def identity_svc(event_loop):
#     identity = JOHN_DOE_IDENTITY
#     identity_key = BytesIO(JOHN_DOE_PRIVATE_KEY)
#     service = IdentityService()
#     event_loop.run_until_complete(service.load(identity, identity_key.read()))
#     return service


class TestFile:

    @pytest.mark.asyncio
    async def test_create_file(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        version = await file.get_version()
        assert version == 0
        file_vlob = await file.get_vlob()
        vlob = await synchronizer_svc.vlob_read(file_vlob['id'],
                                                file_vlob['read_trust_seed'],
                                                version)
        file_vlob = await file.get_vlob()
        key = file_vlob['key']
        key = decodebytes(key.encode())
        encryptor = load_sym_key(key)
        blob = vlob['blob']
        blob = decodebytes(blob.encode())
        blocks = encryptor.decrypt(blob)
        blocks = json.loads(blocks.decode())
        assert len(blocks) == 1
        assert len(blocks[0]['blocks']) == 1
        assert blocks[0]['blocks'][0]['size'] == 0
        await synchronizer_svc.vlob_update(vlob['id'],
                                           version + 1,
                                           file_vlob['write_trust_seed'],
                                           'foo')

    @pytest.mark.asyncio
    async def test_load_file(self, synchronizer_svc):
        encryptor = generate_sym_key()
        key = encodebytes(encryptor.key).decode()
        vlob = await synchronizer_svc.vlob_create('foo')
        new_vlob = await synchronizer_svc.vlob_synchronize(vlob['id'])
        new_vlob_id = new_vlob['id']
        read_trust_seed = new_vlob['read_trust_seed']
        write_trust_seed = new_vlob['write_trust_seed']
        await synchronizer_svc.vlob_update(new_vlob_id, 2, write_trust_seed, 'bar')
        await synchronizer_svc.vlob_synchronize(new_vlob_id)
        file = await File.load(synchronizer_svc,
                               new_vlob_id,
                               key,
                               read_trust_seed,
                               write_trust_seed)
        file_vlob = await file.get_vlob()
        assert {
            'id': new_vlob_id,
            'key': key,
            'read_trust_seed': read_trust_seed,
            'write_trust_seed': write_trust_seed
        } == file_vlob
        assert file.version == 2

    @pytest.mark.asyncio
    async def test_get_vlob(self, synchronizer_svc):
        encryptor = generate_sym_key()
        key = encodebytes(encryptor.key).decode()
        vlob = await synchronizer_svc.vlob_create('foo')
        file = await File.load(synchronizer_svc,
                               vlob['id'],
                               key,
                               vlob['read_trust_seed'],
                               vlob['write_trust_seed'])
        file_vlob = await file.get_vlob()
        assert {
            'id': vlob['id'],
            'key': key,
            'read_trust_seed': vlob['read_trust_seed'],
            'write_trust_seed': vlob['write_trust_seed']
        } == file_vlob
        assert file.version == 1

    @pytest.mark.asyncio
    async def test_get_blocks(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        # Encoded contents
        block_size = 10000
        content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, block_size + 1)])
        encoded_content = encodebytes(content).decode()
        # Blocks
        block_ids = []
        blob = []
        blob.append(await file._build_file_blocks(encoded_content))
        for blocks_and_key in blob:
            for block_properties in blocks_and_key['blocks']:
                block_ids.append(block_properties['block'])
        blob = json.dumps(blob)
        blob = blob.encode()
        encrypted_blob = file.encryptor.encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        vlob = await file.get_vlob()
        await synchronizer_svc.vlob_update(vlob['id'],
                                           1,
                                           vlob['write_trust_seed'],
                                           encrypted_blob)
        ret = await file.get_blocks()
        assert ret == block_ids

    @pytest.mark.asyncio
    async def test_get_version(self, synchronizer_svc):
        encryptor = generate_sym_key()
        key = encodebytes(encryptor.key).decode()
        vlob = await synchronizer_svc.vlob_create('foo')
        new_vlob = await synchronizer_svc.vlob_synchronize(vlob['id'])
        new_vlob_id = new_vlob['id']
        read_trust_seed = new_vlob['read_trust_seed']
        write_trust_seed = new_vlob['write_trust_seed']
        await synchronizer_svc.vlob_update(new_vlob_id, 2, write_trust_seed, 'bar')
        await synchronizer_svc.vlob_synchronize(new_vlob_id)
        file = await File.load(synchronizer_svc,
                               new_vlob_id,
                               key,
                               read_trust_seed,
                               write_trust_seed)
        version = await file.get_version()
        assert version == 2

    @pytest.mark.asyncio
    async def test_read(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        # Empty file
        read_content = await file.read()
        assert read_content == ''
        # Not empty file
        content = 'This is a test content.'
        encoded_content = encodebytes(content.encode()).decode()
        await file.write(encoded_content, 0)
        read_content = await file.read()
        assert read_content == encoded_content
        # Offset
        offset = 5
        encoded_content = encodebytes(content[offset:].encode()).decode()
        read_content = await file.read(offset=offset)
        assert read_content == encoded_content
        # Size
        size = 9
        encoded_content = encodebytes(content[offset:][:size].encode()).decode()
        read_content = await file.read(size=size, offset=offset)
        assert read_content == encoded_content

    @pytest.mark.asyncio
    async def test_write(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        block_ids = await file.get_blocks()
        # Check with empty and not empty file
        for content in ['this is v2 content', 'this is v3 content']:
            encoded_data = encodebytes(content.encode()).decode()
            await file.write(encoded_data, 0)
            synchronizer_block_list = await synchronizer_svc.block_list()
            for block_id in block_ids[1:]:
                assert block_id not in synchronizer_block_list
            block_ids = await file.get_blocks()
            read_content = await file.read()
            assert read_content == encoded_data
        # Offset
        encoded_data = encodebytes('v4'.encode()).decode()
        await file.write(data=encoded_data, offset=8)
        read_content = await file.read()
        encoded_data = encodebytes('this is v4 content'.encode()).decode()
        assert read_content == encoded_data
        assert await file.get_version() == 1

    @pytest.mark.asyncio
    async def test_truncate(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        # Encoded contents
        block_size = 4096
        content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, block_size + 1)])
        encoded_content = encodebytes(content).decode()
        # Blocks
        blocks = await file._build_file_blocks(encoded_content)
        # Write content
        blob = json.dumps([blocks])
        blob = blob.encode()
        file_vlob = await file.get_vlob()
        key = decodebytes(file_vlob['key'].encode())
        encryptor = load_sym_key(key)
        encrypted_blob = encryptor.encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        id = file_vlob['id']
        await synchronizer_svc.vlob_update(id, 1, file_vlob['write_trust_seed'], encrypted_blob)
        block_ids = await file.get_blocks()
        # Truncate full length
        await file.truncate(block_size + 1)
        read_content = await file.read()
        encoded_content = encodebytes(content[:block_size + 1]).decode()
        assert read_content == encoded_content
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id in synchronizer_block_list
        # Truncate block length
        await file.truncate(block_size)
        read_content = await file.read()
        encoded_content = encodebytes(content[:block_size]).decode()
        assert read_content == encoded_content
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids[:-1]:
            assert block_id in synchronizer_block_list
        assert block_ids[-1] not in synchronizer_block_list
        # Truncate shorter than block length
        await file.truncate(block_size - 1)
        read_content = await file.read()
        encoded_content = encodebytes(content[:block_size - 1]).decode()
        assert read_content == encoded_content
        new_synchronizer_block_list = await synchronizer_svc.block_list()
        new_blocks = [block_id for block_id in new_synchronizer_block_list if block_id not in synchronizer_block_list]
        assert len(new_blocks) == 1
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id not in synchronizer_block_list
        # Truncate empty
        await file.truncate(0)
        read_content = await file.read()
        assert read_content == ''
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids + new_blocks:
            assert block_id not in synchronizer_block_list
        assert await file.get_version() == 1

    @pytest.mark.asyncio
    async def test_restore(self, synchronizer_svc):
        # Restore not commited initial file
        content = 'This is content 1.'
        encoded_content_1 = encodebytes(content.encode()).decode()
        file = await File.create(synchronizer_svc)
        await file.write(encoded_content_1, 0)
        with pytest.raises(FileError):
            await file.restore()
        # Restore dirty file to previous version
        await file.commit()
        content = 'This is content 2.'
        encoded_content_2 = encodebytes(content.encode()).decode()
        await file.write(encoded_content_2, 0)
        await file.commit()
        content = 'This is content 3.'
        encoded_content_3 = encodebytes(content.encode()).decode()
        await file.write(encoded_content_3, 0)
        block_ids = await file.get_blocks()
        await file.restore()
        assert await file.get_version() == 3
        content = await file.read()
        assert content == encoded_content_1
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id not in synchronizer_block_list
        # Restore file to specific version
        await file.commit()
        await file.restore(2)
        assert await file.get_version() == 4
        content = await file.read()
        assert content == encoded_content_2

    @pytest.mark.asyncio
    async def test_reencrypt(self, synchronizer_svc):
        encoded_content_1 = encodebytes('This is content 1.'.encode()).decode()
        encoded_content_2 = encodebytes('This is content 2.'.encode()).decode()
        file = await File.create(synchronizer_svc)
        await file.write(encoded_content_1, 0)
        await file.commit()
        old_vlob = await file.get_vlob()
        await file.reencrypt()
        new_vlob = await file.get_vlob()
        for property in old_vlob.keys():
            assert old_vlob[property] != new_vlob[property]
        old_file = await File.load(synchronizer_svc, **old_vlob)
        await old_file.write(encoded_content_2, 0)
        await old_file.commit()
        assert await file.get_version() == 2
        file = await File.load(synchronizer_svc, **new_vlob)
        content = await file.read()
        assert content == encoded_content_1
        assert await file.get_version() == 1

    async def test_commit(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        encoded_data = encodebytes('foo'.encode()).decode()
        await file.write(encoded_data, 0)
        block_ids = await file.get_blocks()
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id in synchronizer_block_list
        assert await file.get_version() == 1
        new_vlob = await file.commit()
        assert new_vlob != vlob
        assert sorted(list(new_vlob.keys())) == ['id', 'key', 'read_trust_seed', 'write_trust_seed']
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id not in synchronizer_block_list
        encoded_data = encodebytes('bar'.encode()).decode()
        await file.write(encoded_data, 0)
        assert await file.get_version() == 2
        new_vlob = await file.commit()
        assert new_vlob is True
        block_ids = await file.get_blocks()
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id not in synchronizer_block_list
        assert await file.get_version() == 2
        ret = await file.commit()
        assert ret is False

    @pytest.mark.asyncio
    async def test_discard(self, synchronizer_svc):
        # Not synchronized
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        encoded_data = encodebytes('foo'.encode()).decode()
        await file.write(encoded_data, 0)
        block_ids = await file.get_blocks()
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id in synchronizer_block_list
        assert await file.get_version() == 1
        ret = await file.discard()
        assert ret is True
        with pytest.raises(VlobNotFound):
            await synchronizer_svc.vlob_read(vlob['id'], vlob['read_trust_seed'])
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            with pytest.raises(BlockNotFound):
                await synchronizer_svc.block_read(block_id)
        assert await file.get_version() == 0
        # Synchronized
        file = await File.create(synchronizer_svc)
        ret = await file.commit()
        ret = await file.discard()
        assert ret is False

    @pytest.mark.asyncio
    @pytest.mark.parametrize('length', [0, 4095, 4096, 4097])
    async def test_build_file_blocks(self, synchronizer_svc, length):

        def digest(chunk):
            digest = hashes.Hash(hashes.SHA512(), backend=openssl)
            digest.update(chunk)
            chunk_digest = digest.finalize()  # TODO replace with hexdigest ?
            chunk_digest = encodebytes(chunk_digest).decode()
            return chunk_digest

        file = await File.create(synchronizer_svc)
        block_size = 4096
        content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, length)])
        encoded_content = encodebytes(content).decode()
        blocks = await file._build_file_blocks(encoded_content)
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
        assert await file.get_version() == 1

    @pytest.mark.asyncio
    async def test_find_matching_blocks(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
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
            blocks[index] = await file._build_file_blocks(encoded_content)
        # Create file
        blob = json.dumps([blocks[i] for i in range(0, len(blocks))])
        blob = blob.encode()
        file_vlob = await file.get_vlob()
        key = decodebytes(file_vlob['key'].encode())
        encryptor = load_sym_key(key)
        encrypted_blob = encryptor.encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await synchronizer_svc.vlob_update(file_vlob['id'],
                                           1,
                                           file_vlob['write_trust_seed'],
                                           encrypted_blob)
        # All matching blocks
        matching_blocks = await file._find_matching_blocks()
        assert matching_blocks == {'pre_excluded_blocks': [],
                                   'pre_excluded_data': '',
                                   'pre_included_data': '',
                                   'included_blocks': [blocks[i] for i in range(0, len(blocks))],
                                   'post_included_data': '',
                                   'post_excluded_data': '',
                                   'post_excluded_blocks': []
                                   }
        # With offset
        delta = 10
        offset = (blocks[0]['blocks'][0]['size'] + blocks[0]['blocks'][1]['size'] +
                  blocks[1]['blocks'][0]['size'] + blocks[2]['blocks'][0]['size'] - delta)
        matching_blocks = await file._find_matching_blocks(None, offset)
        pre_excluded_data = contents[2][:blocks[2]['blocks'][0]['size'] - delta]
        pre_included_data = contents[2][-delta:]
        encoded_pre_excluded_data = encodebytes(pre_excluded_data).decode()
        encoded_pre_included_data = encodebytes(pre_included_data).decode()
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_data': encoded_pre_excluded_data,
                                   'pre_included_data': encoded_pre_included_data,
                                   'included_blocks': [blocks[i] for i in range(3, 6)],
                                   'post_included_data': '',
                                   'post_excluded_data': '',
                                   'post_excluded_blocks': []
                                   }
        # With small size
        delta = 10
        size = 5
        offset = (blocks[0]['blocks'][0]['size'] + blocks[0]['blocks'][1]['size'] +
                  blocks[1]['blocks'][0]['size'] + blocks[2]['blocks'][0]['size'] - delta)
        matching_blocks = await file._find_matching_blocks(size, offset)
        pre_excluded_data = contents[2][:blocks[2]['blocks'][0]['size'] - delta]
        pre_included_data = contents[2][-delta:][:size]
        post_excluded_data = contents[2][-delta:][size:]
        encoded_pre_excluded_data = encodebytes(pre_excluded_data).decode()
        encoded_pre_included_data = encodebytes(pre_included_data).decode()
        encoded_post_excluded_data = encodebytes(post_excluded_data).decode()
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_data': encoded_pre_excluded_data,
                                   'pre_included_data': encoded_pre_included_data,
                                   'included_blocks': [],
                                   'post_included_data': '',
                                   'post_excluded_data': encoded_post_excluded_data,
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
        matching_blocks = await file._find_matching_blocks(size, offset)
        pre_excluded_data = contents[2][:-delta]
        pre_included_data = contents[2][-delta:]
        post_included_data = contents[4][:2 * delta]
        post_excluded_data = contents[4][:block_size][2 * delta:]
        encoded_pre_excluded_data = encodebytes(pre_excluded_data).decode()
        encoded_pre_included_data = encodebytes(pre_included_data).decode()
        encoded_post_included_data = encodebytes(post_included_data).decode()
        encoded_post_excluded_data = encodebytes(post_excluded_data).decode()
        partial_block_4 = deepcopy(blocks[4])
        del partial_block_4['blocks'][0]
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_data': encoded_pre_excluded_data,
                                   'pre_included_data': encoded_pre_included_data,
                                   'included_blocks': [blocks[3]],
                                   'post_included_data': encoded_post_included_data,
                                   'post_excluded_data': encoded_post_excluded_data,
                                   'post_excluded_blocks': [partial_block_4, blocks[5]]
                                   }
        # With big size and no delta
        size = blocks[3]['blocks'][0]['size']
        size += blocks[3]['blocks'][1]['size']
        size += blocks[3]['blocks'][2]['size']
        offset = (blocks[0]['blocks'][0]['size'] + blocks[0]['blocks'][1]['size'] +
                  blocks[1]['blocks'][0]['size'] + blocks[2]['blocks'][0]['size'])
        matching_blocks = await file._find_matching_blocks(size, offset)
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1], blocks[2]],
                                   'pre_excluded_data': '',
                                   'pre_included_data': '',
                                   'included_blocks': [blocks[3]],
                                   'post_included_data': '',
                                   'post_excluded_data': '',
                                   'post_excluded_blocks': [blocks[4], blocks[5]]
                                   }
        # # With total size
        matching_blocks = await file._find_matching_blocks(total_length, 0)
        assert matching_blocks == {'pre_excluded_blocks': [],
                                   'pre_excluded_data': '',
                                   'pre_included_data': '',
                                   'included_blocks': [blocks[i] for i in range(0, 6)],
                                   'post_included_data': '',
                                   'post_excluded_data': '',
                                   'post_excluded_blocks': []
                                   }
