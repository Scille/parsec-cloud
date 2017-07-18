from copy import deepcopy
import random

from effect2.testing import const, noop, perform_sequence, raise_
import pytest

from parsec.core.file import ContentBuilder, File
from parsec.core.synchronizer import (EVlobCreate, EVlobList, EVlobRead, EVlobUpdate, EVlobDelete,
                                      EVlobSynchronize, EBlockCreate, EBlockSynchronize, EBlockRead,
                                      EBlockDelete)
from parsec.exceptions import BlockNotFound, FileError, VlobNotFound
from tests.test_crypto import mock_crypto_passthrough
from parsec.tools import to_jsonb64, ejson_dumps, digest


@pytest.fixture
def file(mock_crypto_passthrough):
        block_id = '4567'
        blob = [{'blocks': [{'block': block_id, 'digest': digest(b''), 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EBlockCreate(''),
                const(block_id)),
            (EVlobCreate(blob),
                const({'id': '1234', 'read_trust_seed': '42', 'write_trust_seed': '43'})),
        ]
        return perform_sequence(sequence, File.create())


class TestContentBuilder:

    def test_init(self):
        builder = ContentBuilder()
        assert builder.contents == {}

    def test_write(self):
        builder = ContentBuilder()
        builder.write(b'0123456789', 20)
        assert builder.contents == {20: b'0123456789'}
        # Insert just before
        builder.write(b'cde', 17)
        assert builder.contents == {17: b'cde0123456789'}
        # Insert just febore and collision
        builder.write(b'ABC', 15)
        assert builder.contents == {15: b'ABCde0123456789'}
        # Insert just after
        builder.write(b'xyz', 30)
        assert builder.contents == {15: b'ABCde0123456789xyz'}
        # Insert inside
        builder.write(b'XY', 30)
        assert builder.contents == {15: b'ABCde0123456789XYz'}
        # Insert before
        builder.write(b'before', 8)
        assert builder.contents == {8: b'before', 15: b'ABCde0123456789XYz'}
        # Insert after
        builder.write(b'after', 34)
        assert builder.contents == {8: b'before', 15: b'ABCde0123456789XYz', 34: b'after'}

    def test_truncate(self):
        builder = ContentBuilder()
        builder.write(b'0123456789', 20)
        builder.write(b'abc', 40)
        builder.truncate(25)
        assert builder.contents == {20: b'01234'}


class TestFile:

    def test_create_file(self, file):
        assert file.dirty is True
        assert file.version == 0
        assert file.get_vlob() == {'id': '1234',
                                   'key': to_jsonb64(b'<dummy-key-00000000000000000002>'),
                                   'read_trust_seed': '42',
                                   'write_trust_seed': '43'}

    def test_load_file(self, file):
        vlob_id = '1234'
        other_vlob_id = '5678'
        read_trust_seed = '42'
        version = 1
        # Load from open files
        file2 = perform_sequence([], File.load(vlob_id,
                                               to_jsonb64(b'<dummy-key-00000000000000000001>'),
                                               read_trust_seed,
                                               '43'))
        assert file == file2
        File.files = {}
        # Test reloading commited and not commited file
        for synchronizer_vlob_list in [[vlob_id, other_vlob_id], [other_vlob_id]]:
            key = to_jsonb64(b'<dummy-key-00000000000000000001>')
            sequence = [
                (EVlobRead(vlob_id, read_trust_seed, None),
                    const({'id': vlob_id, 'blob': 'foo', 'version': version})),
                (EVlobList(),
                    const(synchronizer_vlob_list)),
            ]
            file = perform_sequence(sequence, File.load(vlob_id, key, read_trust_seed, '43'))
            assert file.dirty is (vlob_id in synchronizer_vlob_list)
            assert file.version == (version - 1 if file.dirty else version)
            File.files = {}

    def test_get_vlob(self, file):
        assert file.get_vlob() == {'id': '1234',
                                   'key': to_jsonb64(b'<dummy-key-00000000000000000002>'),
                                   'read_trust_seed': '42',
                                   'write_trust_seed': '43'}

    def test_get_blocks(self, file):
        file.dirty = False
        file.version = 1
        vlob_id = '1234'
        block_ids = ['4567', '5678', '6789']
        chunk_digest = digest(b'')
        blob = [{'blocks': [{'block': block_ids[0], 'digest': chunk_digest, 'size': 0},
                            {'block': block_ids[1], 'digest': chunk_digest, 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                {'blocks': [{'block': block_ids[2], 'digest': chunk_digest, 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
        ]
        ret = perform_sequence(sequence, file.get_blocks())
        assert ret == block_ids
        assert file.dirty is False
        assert file.version == 1

    def test_get_version(self, file):
        assert file.version == 0
        assert file.dirty is True
        assert file.get_version() == 1
        file.dirty = False
        assert file.get_version() == 0

    def test_read(self, file):
        file.dirty = False
        file.version = 1
        # Empty file
        vlob_id = '1234'
        chunk_digest = digest(b'')
        blob = [{'blocks': [{'block': '4567', 'digest': chunk_digest, 'size': 0}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
        ]
        read_content = perform_sequence(sequence, file.read())
        assert read_content == b''
        # Not empty file
        content = b'This is a test content.'
        block_ids = ['4567', '5678', '6789']
        chunk_1 = content[:5]
        chunk_2 = content[5:14]
        chunk_3 = content[14:]
        blob = [{'blocks': [{'block': block_ids[0],
                             'digest': digest(chunk_1),
                             'size': len(chunk_1)},
                            {'block': block_ids[1],
                             'digest': digest(chunk_2),
                             'size': len(chunk_2)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                {'blocks': [{'block': block_ids[2],
                             'digest': digest(chunk_3),
                             'size': len(chunk_3)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
            (EBlockRead(block_ids[0]),
                const({'content': to_jsonb64(chunk_1), 'creation_date': '2012-01-01T00:00:00'})),
            (EBlockRead(block_ids[1]),
                const({'content': to_jsonb64(chunk_2), 'creation_date': '2012-01-01T00:00:00'})),
            (EBlockRead(block_ids[2]),
                const({'content': to_jsonb64(chunk_3), 'creation_date': '2012-01-01T00:00:00'}))
        ]
        read_content = perform_sequence(sequence, file.read())
        assert read_content == content
        # Offset
        offset = 5
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
            (EBlockRead(block_ids[1]),
                const({'content': to_jsonb64(chunk_2), 'creation_date': '2012-01-01T00:00:00'})),
            (EBlockRead(block_ids[2]),
                const({'content': to_jsonb64(chunk_3), 'creation_date': '2012-01-01T00:00:00'}))
        ]
        read_content = perform_sequence(sequence, file.read(offset=offset))
        assert read_content == content[offset:]
        # Size
        size = 9
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
            (EBlockRead(block_ids[1]),
                const({'content': to_jsonb64(chunk_2), 'creation_date': '2012-01-01T00:00:00'}))
        ]
        read_content = perform_sequence(sequence, file.read(offset=offset, size=size))
        assert read_content == content[offset:][:size]
        assert file.dirty is False
        assert file.version == 1

    def test_write(self, file):
        file.dirty = False
        file.version = 2
        ret = file.write(b'foo', 0)
        assert ret is None
        file.write(b'bar', 1)
        assert file.modifications == [(file.write, b'foo', 0), (file.write, b'bar', 1)]
        assert file.dirty is False
        assert file.version == 2

    def test_truncate(self, file):
        file.dirty = False
        file.version = 2
        ret = file.truncate(0)
        assert ret is None
        file.truncate(1)  # TODO should raise error?
        assert file.modifications == [(file.truncate, 0), (file.truncate, 1)]
        assert file.dirty is False
        assert file.version == 2

    def test_stat(self, file):
        vlob_id = '1234'
        content = b'This is a test content.'
        block_ids = ['4567', '5678', '6789']
        # Original content
        chunk_1 = content[:5]
        chunk_2 = content[5:14]
        chunk_3 = content[14:]
        blob = [{'blocks': [{'block': block_ids[0],
                             'digest': digest(chunk_1),
                             'size': len(chunk_1)},
                            {'block': block_ids[1],
                             'digest': digest(chunk_2),
                             'size': len(chunk_2)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                {'blocks': [{'block': block_ids[2],
                             'digest': digest(chunk_3),
                             'size': len(chunk_3)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1}))
        ]
        ret = perform_sequence(sequence, file.stat())
        assert ret == {'type': 'file',
                       'id': vlob_id,
                       'created': '2012-01-01T00:00:00',
                       'updated': '2012-01-01T00:00:00',
                       'size': 23,
                       'version': 1}
        # TODO check created and updated time are different
        # Truncate in buffer
        file.truncate(20)
        ret = perform_sequence(sequence, file.stat())
        assert ret == {'type': 'file',
                       'id': vlob_id,
                       'created': '2012-01-01T00:00:00',
                       'updated': '2012-01-01T00:00:00',
                       'size': 20,
                       'version': 1}
        # Write in buffer
        file.write(b'foo', 30)
        ret = perform_sequence(sequence, file.stat())
        assert ret == {'type': 'file',
                       'id': vlob_id,
                       'created': '2012-01-01T00:00:00',
                       'updated': '2012-01-01T00:00:00',
                       'size': 33,
                       'version': 1}

    def test_restore(self, file):
        vlob_id = '1234'
        content = b'This is a test content.'
        block_ids = ['4567', '5678', '6789']
        # Original content
        chunk_1 = content[:5]
        chunk_2 = content[5:14]
        chunk_3 = content[14:]
        blob = [{'blocks': [{'block': block_ids[0],
                             'digest': digest(chunk_1),
                             'size': len(chunk_1)},
                            {'block': block_ids[1],
                             'digest': digest(chunk_2),
                             'size': len(chunk_2)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                {'blocks': [{'block': block_ids[2],
                             'digest': digest(chunk_3),
                             'size': len(chunk_3)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        # New content
        new_chuck_2 = b'is A test'
        new_block_id = '7654'
        new_blob = [{'blocks': [{'block': block_ids[0],
                                 'digest': digest(chunk_1),
                                 'size': len(chunk_1)}],
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                    {'blocks': [{'block': new_block_id,
                                 'digest': digest(new_chuck_2),
                                 'size': len(new_chuck_2)}],
                     'key': to_jsonb64(b'<dummy-key-00000000000000000003>')},
                    {'blocks': [{'block': block_ids[2],
                                 'digest': digest(chunk_3),
                                 'size': len(chunk_3)}],
                     'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        new_blob = ejson_dumps(new_blob).encode()
        new_blob = to_jsonb64(new_blob)
        # Restore not commited file with version = 1
        file.dirty = False
        with pytest.raises(FileError):
            perform_sequence([], file.restore())
        assert file.dirty is False
        file.dirty = True
        # Restore commited file with version = 1
        file.dirty = False
        file.version = 1
        with pytest.raises(FileError):
            perform_sequence([], file.restore())
        assert file.dirty is False
        # Restore not commited file with version = current version
        file.dirty = True
        file.version = 5
        with pytest.raises(FileError):
            perform_sequence([], file.restore(6))
        assert file.dirty is True
        # Restore commited file with version = current version
        file.dirty = False
        file.version = 6
        with pytest.raises(FileError):
            perform_sequence([], file.restore(6))
        assert file.dirty is False
        # Restore previous version
        sequence = [
            (EVlobRead(vlob_id, '42', 6),  # Discard
                const({'id': vlob_id, 'blob': blob, 'version': 6})),
            (EBlockDelete('4567'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EBlockDelete('5678'),
                noop),
            (EBlockDelete('6789'),
                noop),
            (EVlobDelete('1234'),
                noop),
            (EVlobRead('1234', '42', 5),
                const({'id': vlob_id, 'blob': new_blob, 'version': 5})),
            (EVlobUpdate(vlob_id, '43', 7, new_blob),
                noop)
        ]
        ret = perform_sequence(sequence, file.restore())
        assert ret is None
        assert file.dirty is True
        assert file.version == 6
        # Restore specific version
        sequence = [
            (EVlobRead(vlob_id, '42', 7),
                const({'id': vlob_id, 'blob': new_blob, 'version': 7})),
            (EBlockDelete('4567'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EBlockDelete('7654'),
                noop),
            (EBlockDelete('6789'),
                noop),
            (EVlobDelete('1234'),
                noop),
            (EVlobRead('1234', '42', 2),
                const({'id': vlob_id, 'blob': blob, 'version': 2})),
            (EVlobUpdate(vlob_id, '43', 7, blob),
                noop)
        ]
        ret = perform_sequence(sequence, file.restore(2))
        assert ret is None
        assert file.dirty is True
        assert file.version == 6

    def test_reencrypt(self, file):
        old_vlob = file.get_vlob()
        content = b'This is a test content.'
        block_ids = ['4567', '5678', '6789']
        # Original content
        chunk_1 = content[:5]
        chunk_2 = content[5:14]
        chunk_3 = content[14:]
        blob = [{'blocks': [{'block': block_ids[0],
                             'digest': digest(chunk_1),
                             'size': len(chunk_1)},
                            {'block': block_ids[1],
                             'digest': digest(chunk_2),
                             'size': len(chunk_2)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                {'blocks': [{'block': block_ids[2],
                             'digest': digest(chunk_3),
                             'size': len(chunk_3)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        sequence = [
            (EVlobRead('1234', '42', 1),
                const({'id': '1234', 'blob': blob, 'version': 1})),
            (EVlobCreate(blob),  # TODO check re-encryption
                const({'id': '2345', 'read_trust_seed': '21', 'write_trust_seed': '22'}))
        ]
        ret = perform_sequence(sequence, file.reencrypt())
        assert ret is None
        file.reencrypt()
        new_vlob = file.get_vlob()
        for property in old_vlob.keys():
            assert old_vlob[property] != new_vlob[property]

    def test_flush(self, file):
        file.truncate(9)
        file.write(b'IS', 5)
        file.write(b'IS a nice test content.', 5)
        file.dirty = False
        file.version = 2
        vlob_id = '1234'
        content = b'This is a test content.'
        block_ids = ['4567', '5678', '6789']
        # Original content
        chunk_1 = content[:5]
        chunk_2 = content[5:14]
        chunk_3 = content[14:]
        blob = [{'blocks': [{'block': block_ids[0],
                             'digest': digest(chunk_1),
                             'size': len(chunk_1)},
                            {'block': block_ids[1],
                             'digest': digest(chunk_2),
                             'size': len(chunk_2)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                {'blocks': [{'block': block_ids[2],
                             'digest': digest(chunk_3),
                             'size': len(chunk_3)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000002>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        # New content after truncate
        new_chuck_2 = b'is a'
        new_block_id = '7654'
        new_blob = [{'blocks': [{'block': block_ids[0],
                                 'digest': digest(chunk_1),
                                 'size': len(chunk_1)}],
                     'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                    {'blocks': [{'block': new_block_id,
                                 'digest': digest(new_chuck_2),
                                 'size': len(new_chuck_2)}],
                     'key': to_jsonb64(b'<dummy-key-00000000000000000003>')}]
        new_blob = ejson_dumps(new_blob).encode()
        new_blob = to_jsonb64(new_blob)
        # New content after write
        new_block_2_id = '6543'
        new_chunk_4 = b'IS a nice test content.'
        new_blob_2 = [{'blocks': [{'block': block_ids[0],
                                   'digest': digest(chunk_1),
                                   'size': len(chunk_1)}],
                       'key': to_jsonb64(b'<dummy-key-00000000000000000001>')},
                      {'blocks': [{'block': new_block_2_id,
                                   'digest': digest(new_chunk_4),
                                   'size': len(new_chunk_4)}],
                       'key': to_jsonb64(b'<dummy-key-00000000000000000004>')}]
        new_blob_2 = ejson_dumps(new_blob_2).encode()
        new_blob_2 = to_jsonb64(new_blob_2)
        sequence = [
            (EVlobRead(vlob_id, '42', 2),  # Get blocks
                const({'id': vlob_id, 'blob': blob, 'version': 2})),
            (EVlobRead(vlob_id, '42', 2),  # Matching blocks
                const({'id': vlob_id, 'blob': blob, 'version': 2})),
            (EBlockRead(block_ids[1]),
                const({'content': to_jsonb64(chunk_2), 'creation_date': '2012-01-01T00:00:00'})),
            (EBlockCreate(to_jsonb64(new_chuck_2)),
                const(new_block_id)),
            (EVlobUpdate(vlob_id, '43', 3, new_blob),
                noop),
            (EVlobRead(vlob_id, '42', 3),  # Matching blocks
                const({'id': vlob_id, 'blob': new_blob, 'version': 3})),
            (EBlockCreate(to_jsonb64(new_chunk_4)),
                const(new_block_2_id)),
            (EVlobUpdate(vlob_id, '43', 3, new_blob_2),
                noop),
            (EVlobRead(vlob_id, '42', 3),
                const({'id': vlob_id, 'blob': new_blob_2, 'version': 3})),
            (EBlockDelete('5678'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EBlockDelete('6789'),
                noop),
        ]
        ret = perform_sequence(sequence, file.flush())
        assert ret is None
        assert file.dirty is True
        assert file.version == 2

    def test_commit(self, file):
        vlob_id = '1234'
        content = b'This is a test content.'
        block_ids = ['4567', '5678', '6789']
        new_vlob = {'id': '2345', 'read_trust_seed': 'ABC', 'write_trust_seed': 'DEF'}
        # Original content
        chunk_1 = content[:5]
        chunk_2 = content[5:14]
        chunk_3 = content[14:]
        blob = [{'blocks': [{'block': block_ids[0],
                             'digest': digest(chunk_1),
                             'size': len(chunk_1)},
                            {'block': block_ids[1],
                             'digest': digest(chunk_2),
                             'size': len(chunk_2)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000003>')},
                {'blocks': [{'block': block_ids[2],
                             'digest': digest(chunk_3),
                             'size': len(chunk_3)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000004>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        # New content after truncate
        new_chuck_2 = b'is a'
        new_block_id = '7654'
        new_blob = [{'blocks': [{'block': block_ids[0],
                                 'digest': digest(chunk_1),
                                 'size': len(chunk_1)}],
                     'key': to_jsonb64(b'<dummy-key-00000000000000000003>')},
                    {'blocks': [{'block': new_block_id,
                                 'digest': digest(new_chuck_2),
                                 'size': len(new_chuck_2)}],
                     'key': to_jsonb64(b'<dummy-key-00000000000000000003>')}]
        new_blob = ejson_dumps(new_blob).encode()
        new_blob = to_jsonb64(new_blob)
        file.truncate(9)
        sequence = [
            (EVlobRead('1234', '42', 1),
                const({'id': '1234', 'blob': blob, 'version': 1})),
            (EVlobRead('1234', '42', 1),
                const({'id': '1234', 'blob': blob, 'version': 1})),
            (EBlockRead(block_ids[1]),
                const({'content': to_jsonb64(chunk_2), 'creation_date': '2012-01-01T00:00:00'})),
            (EBlockCreate(to_jsonb64(new_chuck_2)),
                const(new_block_id)),
            (EVlobUpdate(vlob_id, '43', 1, new_blob),
                noop),
            (EVlobRead('1234', '42', 1),
                const({'id': '1234', 'blob': new_blob, 'version': 1})),
            (EBlockDelete('5678'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EBlockDelete('6789'),
                noop),
            (EVlobRead('1234', '42', 1),
                const({'id': '1234', 'blob': new_blob, 'version': 1})),
            (EBlockSynchronize('4567'),
                const(True)),
            (EBlockSynchronize('7654'),
                const(False)),
            (EVlobSynchronize('1234'),
                const(new_vlob))
        ]
        ret = perform_sequence(sequence, file.commit())
        new_vlob['key'] = to_jsonb64(b'<dummy-key-00000000000000000002>')
        assert ret == new_vlob
        assert file.dirty is False
        assert file.version == 1

    def test_discard(self, file):
        content = b'This is a test content.'
        block_ids = ['4567', '5678', '6789']
        # Original content
        chunk_1 = content[:5]
        chunk_2 = content[5:14]
        chunk_3 = content[14:]
        blob = [{'blocks': [{'block': block_ids[0],
                             'digest': digest(chunk_1),
                             'size': len(chunk_1)},
                            {'block': block_ids[1],
                             'digest': digest(chunk_2),
                             'size': len(chunk_2)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000003>')},
                {'blocks': [{'block': block_ids[2],
                             'digest': digest(chunk_3),
                             'size': len(chunk_3)}],
                 'key': to_jsonb64(b'<dummy-key-00000000000000000004>')}]
        blob = ejson_dumps(blob).encode()
        blob = to_jsonb64(blob)
        # Already synchronized
        sequence = [
            (EVlobRead('1234', '42', 1),
                const({'id': '1234', 'blob': blob, 'version': 1})),
            (EBlockDelete('4567'),
                lambda _: raise_(BlockNotFound('Block not found.'))),
            (EBlockDelete('5678'),
                noop),
            (EBlockDelete('6789'),
                noop),
            (EVlobDelete('1234'),
                lambda _: raise_(VlobNotFound('Block not found.')))  # TODO vlob OR block exceptin
        ]
        ret = perform_sequence(sequence, file.discard())
        assert ret is False
        # Not already synchronized
        file.dirty = True
        file.version = 0
        sequence = [
            (EVlobRead('1234', '42', 1),
                const({'id': '1234', 'blob': blob, 'version': 1})),
            (EBlockDelete('4567'),
                noop),
            (EBlockDelete('5678'),
                noop),
            (EBlockDelete('6789'),
                noop),
            (EVlobDelete('1234'),
                noop)
        ]
        ret = perform_sequence(sequence, file.discard())
        assert ret is True
        assert file.dirty is False

    @pytest.mark.parametrize('length', [0, 4095, 4096, 4097])
    def test_build_file_blocks(self, file, length):
        file.dirty = False
        block_size = 4096
        content = b''.join([str(random.randint(1, 9)).encode() for i in range(0, length)])
        chunks = [content[i:i + block_size] for i in range(0, len(content), block_size)]
        if not chunks:
            chunks = [b'']
        sequence = []
        for chunk in chunks:
            sequence.append((EBlockCreate(to_jsonb64(chunk)), const('4567')))
        blocks = perform_sequence(sequence, file._build_file_blocks(content))
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
        assert file.dirty is True

    def test_find_matching_blocks(self, file):
        vlob_id = '1234'
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

        def generator():
            i = 2000
            while True:
                yield str(i)
                i += 1
        gen = generator()

        blocks = {}
        block_contents = {}
        block_id = 2000
        for index, content in contents.items():
            chunks = [content[i:i + block_size] for i in range(0, len(content), block_size)]
            if not chunks:
                chunks = [b'']
            sequence = []
            for chunk in chunks:
                encoded_chunk = to_jsonb64(chunk)
                sequence.append((EBlockCreate(encoded_chunk), lambda id=id: next(gen)))  # TODO dirty
                block_contents[str(block_id)] = encoded_chunk
                block_id += 1
            blocks[index] = perform_sequence(sequence, file._build_file_blocks(content))
        # Create file
        blob = ejson_dumps([blocks[i] for i in range(0, len(blocks))]).encode()
        blob = to_jsonb64(blob)
        # All matching blocks
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1}))
        ]
        matching_blocks = perform_sequence(sequence, file._find_matching_blocks())
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
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
            (EBlockRead('2003'),
                const({'content': block_contents['2003'],
                           'creation_date': '2012-01-01T00:00:00'}))
        ]
        matching_blocks = perform_sequence(sequence, file._find_matching_blocks(None, offset))
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
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
            (EBlockRead(id='2003'),
                const({'content': block_contents['2003'],
                       'creation_date': '2012-01-01T00:00:00'}))
        ]
        matching_blocks = perform_sequence(sequence, file._find_matching_blocks(size, offset))
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
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
            (EBlockRead('2003'),
                const({'content': block_contents['2003'],
                       'creation_date': '2012-01-01T00:00:00'})),
            (EBlockRead('2007'),
                const({'content': block_contents['2007'],
                       'creation_date': '2012-01-01T00:00:00'}))
        ]
        matching_blocks = perform_sequence(sequence, file._find_matching_blocks(size, offset))
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
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
        ]
        matching_blocks = perform_sequence(sequence, file._find_matching_blocks(size, offset))
        assert matching_blocks == {'pre_excluded_blocks': [blocks[0], blocks[1], blocks[2]],
                                   'pre_excluded_data': b'',
                                   'pre_included_data': b'',
                                   'included_blocks': [blocks[3]],
                                   'post_included_data': b'',
                                   'post_excluded_data': b'',
                                   'post_excluded_blocks': [blocks[4], blocks[5]]
                                   }
        # With total size
        sequence = [
            (EVlobRead(vlob_id, '42', 1),
                const({'id': vlob_id, 'blob': blob, 'version': 1})),
        ]
        matching_blocks = perform_sequence(sequence, file._find_matching_blocks(total_length, 0))
        assert matching_blocks == {'pre_excluded_blocks': [],
                                   'pre_excluded_data': b'',
                                   'pre_included_data': b'',
                                   'included_blocks': [blocks[i] for i in range(0, 6)],
                                   'post_included_data': b'',
                                   'post_excluded_data': b'',
                                   'post_excluded_blocks': []
                                   }
