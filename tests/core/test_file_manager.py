import pytest
from trio.testing import trio_test
from unittest.mock import Mock
from nacl.secret import SecretBox

from parsec.core.file_manager import _merge_patches, _try_merge_two_patches, Patch
from parsec.utils import to_jsonb64

from tests.common import mocked_local_storage_cls_factory


@pytest.mark.parametrize('data', [
    {
        'label': 'No patches',
        'patches': [],
        'expected': []
    },
    {
        'label': 'Contiguous patches',
        'patches': [(0, 'hello '), (6, 'world !')],
        'expected': [(0, 'hello world !')]
    },
    {
        'label': 'Non-contiguous patches',
        'patches': [(0, 'hello'), (10, 'world !')],
        'expected': [(0, 'hello'), (10, 'world !')]
    },
    {
        'label': 'Overwrite single patch',
        'patches': [(0, 'hello '), (6, 'world !'), (6, 'SPARTAAAA !')],
        'expected': [(0, 'hello SPARTAAAA !')]
    },
    {
        'label': 'Overwrite multiple patches',
        'patches': [(0, 'hello '), (6, 'world'), (11, ' !'), (6, 'SPARTAAAA !')],
        'expected': [(0, 'hello SPARTAAAA !')]
    },
    {
        'label': 'Cascade overwrite patches',
        'patches': [(0, 'hello '), (6, 'world'), (11, ' !'), (6, 'SPARTAAAA !')],
        'expected': [(0, 'hello SPARTAAAA !')]
    },
    {
        'label': 'Out of order',
        'patches': [(13, 'third'), (6, 'second'), (0, 'first')],
        'expected': [(0, 'first'), (6, 'second'), (13, 'third')]
    },
], ids=lambda x: x['label'])
def test_merge_patches(data):
    patches = [Patch(None, o, len(b), buffer=b) for o, b in data['patches']]
    expected = [Patch(None, o, len(b), buffer=b) for o, b in data['expected']]
    merged = _merge_patches(patches)
    assert merged == expected


@pytest.mark.parametrize('data', [
    {
        'label': 'Contiguous patches',
        'patches': [(0, 'hello ', 6), (6, 'world !', 7)],
        'expected': (0, 'hello world !', 13)
    },
    {
        'label': 'Non-contiguous patches',
        'patches': [(0, 'hello', 5), (10, 'world !', 7)],
        'expected': None
    },
    {
        'label': 'P1 in P2',
        'patches': [(3, 'abc', 3), (0, '123456789', 9)],
        'expected': (0, '123456789', 9)
    },
    {
        'label': 'P2 in P1',
        'patches': [(0, '123456789', 9), (3, 'abc', 3)],
        'expected': (0, '123abc789', 9)
    },
    {
        'label': "P1 on P2's left",
        'patches': [(0, 'abcdef', 6), (3, '456789', 6)],
        'expected': (0, 'abc456789', 9)
    },
    {
        'label': "P1 on P2's right",
        'patches': [(3, 'defghi', 6), (0, '123456', 6)],
        'expected': (0, '123456ghi', 9)
    },
    {
        'label': "Same size, same pos",
        'patches': [(3, 'def', 3), (3, '456', 3)],
        'expected': (3, '456', 3)
    },
], ids=lambda x: x['label'])
def test_try_merge_two_patches(data):
    patches = [Patch(None, o, s, buffer=b) for o, b, s in data['patches']]
    expected = data['expected']
    if expected:
        expected = Patch(None, expected[0], expected[2], buffer=expected[1])
    ret = _try_merge_two_patches(*patches)
    assert ret == expected


def _local_file_factory(with_blocks=False):
    mocked_file_manager = Mock()
    mls_cls = mocked_local_storage_cls_factory()
    mocked_file_manager.local_storage = mls_cls()

    phf, key = PlaceHolderFile.create(mocked_file_manager)
    if with_blocks:
        block_1_id = 'faa4e1068dad47b4a758a73102478388'
        block_1_key = b'\xab\xcfn\xc8*\xe8|\xc42\xf2\xfao\x1b\xc1Xm\xb4\xb9JBe\x9a1W\r(\xcc\xbd1\x12RB'
        mls_cls.test_storage.blocks[block_1_id] = SecretBox(block_1_key).encrypt(b'abcdefghij')

        block_2_id = '4c5b4338a47c462098d6c98856f5bf56'
        block_2_key = b'\xcb\x1c\xe4\x80\x8d\xca\rl?z\xa4\x82J7\xc5\xd5\xed5^\xb6\x05\x8cR;A\xbd\xb1 \xbd\xc2?\xe9'
        mls_cls.test_storage.blocks[block_2_id] = SecretBox(block_2_key).encrypt(b'ABCDEFGHIJ')

        phf.data['blocks'] = [
            {'id': block_1_id, 'key': to_jsonb64(block_1_key), 'offset': 0, 'size': 10},
            {'id': block_2_id, 'key': to_jsonb64(block_2_key), 'offset': 10, 'size': 10},
        ]
        phf.data['size'] = 20
    return phf, key


@pytest.mark.xfail
@trio_test
async def test_file_read_empty():
    phf, _ = _local_file_factory()
    out = await phf.read()
    assert out == b''


@pytest.mark.xfail
@trio_test
async def test_file_read_too_big_size():
    phf, _ = _local_file_factory()
    out = await phf.read(40)
    assert out == b''
    phf.write(b'foo')
    out = await phf.read(40)
    assert out == b'foo'


@pytest.mark.xfail
@trio_test
async def test_file_write():
    phf, _ = _local_file_factory()
    phf.write(b'foo')
    out = await phf.read()
    assert out == b'foo'


@pytest.mark.xfail
@trio_test
async def test_file_write_overwrite():
    phf, _ = _local_file_factory()
    phf.write(b'123')
    phf.write(b'abc')
    out = await phf.read()
    assert out == b'abc'


@pytest.mark.xfail
@trio_test
async def test_file_multi_write():
    phf, _ = _local_file_factory()
    phf.write(b'123')
    phf.write(b'456', offset=3)
    phf.write(b'789', offset=6)
    out = await phf.read(6, offset=2)
    assert out == b'345678'


@pytest.mark.xfail
@trio_test
async def test_file_write_and_truncate():
    phf, _ = _local_file_factory()
    phf.write(b'123')
    phf.write(b'456', offset=3)
    phf.write(b'789', offset=6)
    phf.truncate(5)
    out = await phf.read()
    assert out == b'12345'


@pytest.mark.xfail
@trio_test
async def test_file_read_with_blocks():
    phf, _ = _local_file_factory(with_blocks=True)
    out = await phf.read()
    assert out == b'abcdefghijABCDEFGHIJ'


@pytest.mark.xfail
@trio_test
async def test_file_write_with_blocks():
    phf, _ = _local_file_factory(with_blocks=True)
    phf.write(b'1')
    phf.write(b'2', offset=10)
    phf.write(b'3', offset=20)
    out = await phf.read()
    assert out == b'1bcdefghij2BCDEFGHIJ3'


@pytest.mark.xfail
@trio_test
async def test_file_truncate_with_blocks():
    phf, _ = _local_file_factory(with_blocks=True)
    phf.truncate(5)
    out = await phf.read()
    assert out == b'abcde'
