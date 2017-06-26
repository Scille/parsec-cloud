from base64 import encodebytes, decodebytes
from copy import deepcopy
import json
# from io import BytesIO
import random

from freezegun import freeze_time
import pytest

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes

from parsec.core import (MockedBackendAPIService, MetaBlockService, MockedBlockService,
                         SynchronizerService)
from parsec.core.file import File
from parsec.crypto import generate_sym_key, load_sym_key
from parsec.exceptions import BlockNotFound, FileError, VlobNotFound
from parsec.server import BaseServer


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
        # Reload commited file
        file = await File.create(synchronizer_svc)
        await file.write(b'bar', 0)
        file_vlob = await file.commit()
        file = await File.load(synchronizer_svc,
                               file_vlob['id'],
                               file_vlob['key'],
                               file_vlob['read_trust_seed'],
                               file_vlob['write_trust_seed'])
        assert await file.read() == b'bar'
        await file.write(b'foo', 0)
        # Reload not commited file
        file = await File.create(synchronizer_svc)
        file_vlob = await file.get_vlob()
        await file.write(b'foo', 0)
        file = await File.load(synchronizer_svc,
                               file_vlob['id'],
                               file_vlob['key'],
                               file_vlob['read_trust_seed'],
                               file_vlob['write_trust_seed'])
        await file.write(b'foo', 0)

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
        assert await file.get_version() == 1

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
        assert read_content == b''
        # Not empty file
        content = b'This is a test content.'
        await file.write(content, 0)
        read_content = await file.read()
        assert read_content == content
        # Offset
        offset = 5
        read_content = await file.read(offset=offset)
        assert read_content == content[offset:]
        # Size
        size = 9
        read_content = await file.read(size=size, offset=offset)
        assert read_content == content[offset:][:size]

    @pytest.mark.asyncio
    async def test_write(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        block_ids = await file.get_blocks()
        # Check with empty and not empty file
        for content in [b'this is v2 content', b'this is v3 content']:
            await file.write(content, 0)
            synchronizer_block_list = await synchronizer_svc.block_list()
            for block_id in block_ids[1:]:
                assert block_id not in synchronizer_block_list
            block_ids = await file.get_blocks()
            read_content = await file.read()
            assert read_content == content
        # Offset
        await file.write(data=b'v4', offset=8)
        read_content = await file.read()
        assert read_content == b'this is v4 content'
        assert await file.get_version() == 1

    @pytest.mark.asyncio
    async def test_truncate(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        # Encoded contents
        block_size = 4096
        content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, block_size + 1)])
        # Blocks
        blocks = await file._build_file_blocks(content)
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
        assert read_content == content[:block_size + 1]
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id in synchronizer_block_list
        # Truncate block length
        await file.truncate(block_size)
        read_content = await file.read()
        assert read_content == content[:block_size]
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids[:-1]:
            assert block_id in synchronizer_block_list
        assert block_ids[-1] not in synchronizer_block_list
        # Truncate shorter than block length
        await file.truncate(block_size - 1)
        read_content = await file.read()
        assert read_content == content[:block_size - 1]
        new_synchronizer_block_list = await synchronizer_svc.block_list()
        new_blocks = [block_id for block_id in new_synchronizer_block_list if block_id not in synchronizer_block_list]
        assert len(new_blocks) == 1
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id not in synchronizer_block_list
        # Truncate empty
        await file.truncate(0)
        read_content = await file.read()
        assert read_content == b''
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids + new_blocks:
            assert block_id not in synchronizer_block_list
        assert await file.get_version() == 1

    @pytest.mark.asyncio
    async def test_stat(self, synchronizer_svc):
        # Good file
        with freeze_time('2012-01-01') as frozen_datetime:
            file = await File.create(synchronizer_svc)
            file_vlob = await file.get_vlob()
            ret = await file.stat()
            ctime = frozen_datetime().isoformat()
            assert ret == {'type': 'file',
                           'id': file_vlob['id'],
                           'created': ctime,
                           'updated': ctime,
                           'size': 0,
                           'version': 1}
            frozen_datetime.tick()
            mtime = frozen_datetime().isoformat()
            await file.write(b'foo', 0)
            ret = await file.stat()
            assert ret == {'type': 'file',
                           'id': file_vlob['id'],
                           'created': mtime,  # TODO ctime?
                           'updated': mtime,
                           'size': 3,
                           'version': 1}
            frozen_datetime.tick()
            await file.read()  # TODO useless if atime is not modified
            ret = await file.stat()
            assert ret == {'type': 'file',
                           'id': file_vlob['id'],
                           'created': mtime,
                           'updated': mtime,
                           'size': 3,
                           'version': 1}

    @pytest.mark.asyncio
    async def test_restore(self, synchronizer_svc):
        # Restore not commited initial file
        content_1 = b'This is content 1.'
        file = await File.create(synchronizer_svc)
        await file.write(content_1, 0)
        with pytest.raises(FileError):
            await file.restore()
        # Restore dirty file to previous version
        await file.commit()
        content_2 = b'This is content 2.'
        await file.write(content_2, 0)
        await file.commit()
        content_3 = b'This is content 3.'
        await file.write(content_3, 0)
        block_ids = await file.get_blocks()
        await file.restore()
        assert await file.get_version() == 3
        content = await file.read()
        assert content == content_1
        synchronizer_block_list = await synchronizer_svc.block_list()
        for block_id in block_ids:
            assert block_id not in synchronizer_block_list
        # Restore file to specific version
        await file.commit()
        await file.restore(2)
        assert await file.get_version() == 4
        content = await file.read()
        assert content == content_2

    @pytest.mark.asyncio
    async def test_reencrypt(self, synchronizer_svc):
        content_1 = b'This is content 1.'
        content_2 = b'This is content 2.'
        file = await File.create(synchronizer_svc)
        await file.write(content_1, 0)
        await file.commit()
        old_vlob = await file.get_vlob()
        await file.reencrypt()
        new_vlob = await file.get_vlob()
        for property in old_vlob.keys():
            assert old_vlob[property] != new_vlob[property]
        old_file = await File.load(synchronizer_svc, **old_vlob)
        await old_file.write(content_2, 0)
        await old_file.commit()
        assert await file.get_version() == 2
        file = await File.load(synchronizer_svc, **new_vlob)
        content = await file.read()
        assert content == content_1
        assert await file.get_version() == 1

    async def test_commit(self, synchronizer_svc):
        file = await File.create(synchronizer_svc)
        vlob = await file.get_vlob()
        await file.write(b'foo', 0)
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
        await file.write(b'bar', 0)
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
        await file.write(b'foo', 0)
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
        blocks = await file._build_file_blocks(content)
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
        # Blocks
        blocks = {}
        for index, content in contents.items():
            blocks[index] = await file._build_file_blocks(content)
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
                                   'pre_excluded_data': b'',
                                   'pre_included_data': b'',
                                   'included_blocks': [blocks[i] for i in range(0, len(blocks))],
                                   'post_included_data': b'',
                                   'post_excluded_data': b'',
                                   'post_excluded_blocks': []
                                   }
        # With offset
        delta = 10
        offset = (blocks[0]['blocks'][0]['size'] + blocks[0]['blocks'][1]['size'] +
                  blocks[1]['blocks'][0]['size'] + blocks[2]['blocks'][0]['size'] - delta)
        matching_blocks = await file._find_matching_blocks(None, offset)
        pre_excluded_data = contents[2][:blocks[2]['blocks'][0]['size'] - delta]
        pre_included_data = contents[2][-delta:]
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_data': pre_excluded_data,
                                   'pre_included_data': pre_included_data,
                                   'included_blocks': [blocks[i] for i in range(3, 6)],
                                   'post_included_data': b'',
                                   'post_excluded_data': b'',
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
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_data': pre_excluded_data,
                                   'pre_included_data': pre_included_data,
                                   'included_blocks': [],
                                   'post_included_data': b'',
                                   'post_excluded_data': post_excluded_data,
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
        partial_block_4 = deepcopy(blocks[4])
        del partial_block_4['blocks'][0]
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1]],
                                   'pre_excluded_data': pre_excluded_data,
                                   'pre_included_data': pre_included_data,
                                   'included_blocks': [blocks[3]],
                                   'post_included_data': post_included_data,
                                   'post_excluded_data': post_excluded_data,
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
                                   'pre_excluded_data': b'',
                                   'pre_included_data': b'',
                                   'included_blocks': [blocks[3]],
                                   'post_included_data': b'',
                                   'post_excluded_data': b'',
                                   'post_excluded_blocks': [blocks[4], blocks[5]]
                                   }
        # # With total size
        matching_blocks = await file._find_matching_blocks(total_length, 0)
        assert matching_blocks == {'pre_excluded_blocks': [],
                                   'pre_excluded_data': b'',
                                   'pre_included_data': b'',
                                   'included_blocks': [blocks[i] for i in range(0, 6)],
                                   'post_included_data': b'',
                                   'post_excluded_data': b'',
                                   'post_excluded_blocks': []
                                   }
