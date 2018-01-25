import trio
import pytest
from pendulum import datetime

from tests.common import freeze_time
from parsec.core.fs import *



def create_file(fs, name, parent=None, is_placeholder=False,
                is_not_loaded=False, need_sync=False, need_flush=False,
                size=0, blocks_accesses=None, dirty_blocks_accesses=None):
    if is_placeholder:
        access = fs._placeholder_access_cls(
            '<%s id>' % name, b'<%s key>' % name.encode())
        base_version = 0
    else:
        access = fs._vlob_access_cls(
            '<%s id>' % name, '<%s rts>' % name, '<%s wts>' % name,
            b'<%s key>' % name.encode()
        )
        base_version = 1
    if is_not_loaded:
        entry = fs._not_loaded_entry_cls(access=access)
    else:
        entry = fs._file_entry_cls(
            access=access,
            need_flush=need_flush,
            need_sync=need_sync,
            created=datetime(2017, 1, 1),
            updated=datetime(2017, 12, 31, 23, 59, 59),
            base_version=base_version,
            name=name,
            parent=parent,
            size=size,
            blocks_accesses=blocks_accesses,
            dirty_blocks_accesses=dirty_blocks_accesses,
        )
    if parent:
        parent._children[name] = entry
    return entry


def _mocked_lookup_in_data(mock):
    mock.data = {}

    def lookup_in_data(id, key):
        return mock.data.get(id)

    return lookup_in_data


def add_block(file, data, offset=0, in_local=False, name=None, dirty=False, dirty_not_flushed=False):
    if file._fs.blocks_manager.fetch_from_backend.side_effect is None:
        file._fs.blocks_manager.fetch_from_backend.side_effect = _mocked_lookup_in_data(
            file._fs.blocks_manager.fetch_from_backend)

    if file._fs.blocks_manager.fetch_from_local.side_effect is None:
        file._fs.blocks_manager.fetch_from_local.side_effect = _mocked_lookup_in_data(
            file._fs.blocks_manager.fetch_from_local)

    count = (len(file._fs.blocks_manager.fetch_from_backend.data) +
             len(file._fs.blocks_manager.fetch_from_local.data))
    # count = len(file._blocks) + len(file._dirty_blocks)
    name = name or 'block-%s' % count
    id = '<%s id>' % name
    key = ('<%s key>' % name).encode()
    if dirty:
        access = file._fs._dirty_block_access_cls(id, key, offset, len(data))
    else:
        access = file._fs._block_access_cls(id, key, offset, len(data))
    block = file._fs._block_cls(access)

    if not dirty:
        file._blocks.append(block)
        file._fs.blocks_manager.fetch_from_backend.data[access.id] = data
        if in_local:
            file._fs.blocks_manager.fetch_from_local.data[access.id] = data
    else:
        file._need_sync = True
        file._dirty_blocks.append(block)
        if dirty_not_flushed:
            file._need_flush = True
            block._data = data
        else:
            file._fs.blocks_manager.fetch_from_local.data[access.id] = data

    if file.size < len(data) + offset:
        file._size = len(data) + offset
    return block


def add_dirty_block(*args, not_flushed=False, **kwargs):
    return add_block(*args, **kwargs, dirty=True, dirty_not_flushed=not_flushed)


@pytest.fixture
def root(fs):
    return fs._folder_entry_cls(
        access=fs._user_vlob_access_cls(b'<foo key>'),
        need_flush=False,
        need_sync=False,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=1,
        name='root',
        parent=None,
        children_accesses={},
    )


@pytest.fixture
def bar_txt(fs, root):
    bar = fs._file_entry_cls(
        access=fs._vlob_access_cls('<bar id>', '<bar rts>', '<bar wts>', b'<bar key>'),
        need_flush=False,
        need_sync=True,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=0,
        name='bar.txt',
        parent=root,
    )
    root._children['bar.txt'] = bar
    return bar


@pytest.mark.trio
async def test_attributes(bar_txt, root):

    assert bar_txt.parent is root
    with pytest.raises(AttributeError):
        bar_txt.parent = None

    assert bar_txt.name == 'bar.txt'
    with pytest.raises(AttributeError):
        bar_txt.name = None

    assert not bar_txt.is_placeholder
    with pytest.raises(AttributeError):
        bar_txt.is_placeholder = None

    assert bar_txt.created == datetime(2017, 1, 1)
    with pytest.raises(AttributeError):
        bar_txt.created = None

    assert bar_txt.updated == datetime(2017, 12, 31, 23, 59, 59)
    with pytest.raises(AttributeError):
        bar_txt.updated = None

    assert bar_txt.base_version == 0
    with pytest.raises(AttributeError):
        bar_txt.base_version = None

    assert bar_txt.size == 0
    with pytest.raises(AttributeError):
        bar_txt.size = None


@pytest.mark.trio
async def test_read_empty(fs):
    foo = create_file(fs, 'foo')
    content = await foo.read()
    assert content == b''


@pytest.mark.trio
async def test_read_blocks(fs, mocked_blocks_manager):
    blocks_accesses = [
        fs._block_access_cls('<block 1 id>', b'<block 1 key>', 0, 9),
        fs._block_access_cls('<block 2 id>', b'<block 2 key>', 9, 7),
    ]
    foo = create_file(fs, 'foo', blocks_accesses=blocks_accesses, size=16)
    mocked_blocks_manager.fetch_from_local.side_effect = [
        b'Hello... ',
        b'world !',
    ]
    content = await foo.read()
    assert content == b'Hello... world !'
    assert mocked_blocks_manager.fetch_from_local.call_args_list == [
        [('<block 1 id>', b'<block 1 key>'), {}],
        [('<block 2 id>', b'<block 2 key>'), {}],
    ]


@pytest.mark.trio
async def test_file_read_too_big_size(fs, mocked_blocks_manager):
    file = create_file(fs, 'foo')
    add_block(file, b'Hello... ', offset=0, in_local=True)
    add_block(file, b'world !', offset=9, in_local=True)
    out = await file.read(40)
    assert out == b'Hello... world !'
    out = await file.read(-1)
    assert out == b'Hello... world !'


@pytest.mark.trio
async def test_write(fs):
    file = create_file(fs, 'foo')
    await file.write(b'foo')
    out = await file.read()
    assert out == b'foo'


@pytest.mark.trio
async def test_write_overwrite(fs):
    file = create_file(fs, 'foo')
    await file.write(b'123')
    await file.write(b'abc')
    out = await file.read()
    assert out == b'abc'


@pytest.mark.trio
async def test_multi_write(fs):
    file = create_file(fs, 'foo')
    await file.write(b'123')
    await file.write(b'456', offset=3)
    await file.write(b'789', offset=6)
    out = await file.read(6, offset=2)
    assert out == b'345678'


@pytest.mark.trio
async def test_write_and_truncate(fs):
    file = create_file(fs, 'foo')
    await file.write(b'123')
    await file.write(b'456', offset=3)
    await file.write(b'789', offset=6)
    await file.truncate(5)
    out = await file.read()
    assert out == b'12345'


@pytest.mark.trio
async def test_truncate_too_long(fs):
    file = create_file(fs, 'foo')
    add_block(file, b'Hello... ', offset=0, in_local=True)
    add_block(file, b'world !', offset=9, in_local=True)
    await file.truncate(100)
    out = await file.read()
    assert out == b'Hello... world !'


# TODO: test read/write/flush/sync with dirty blocks, not flushed dirty blocks etc.


@pytest.mark.trio
async def test_flush_not_needed(bar_txt, mocked_manifests_manager):
    await bar_txt.flush()
    assert not bar_txt.need_flush
    mocked_manifests_manager.flush_on_local.assert_not_called()


@pytest.mark.xfail
@pytest.mark.trio
async def test_flush(fs, mocked_manifests_manager, mocked_blocks_manager):
    file = create_file(fs, 'foo')
    add_block(file, b'1' * 10, offset=0, name='1', in_local=True)
    add_block(file, b'2' * 10, offset=10, name='2', in_local=False)
    add_dirty_block(file, b'A' * 5, offset=5, name='A', not_flushed=False)
    add_dirty_block(file, b'B' * 5, offset=7, name='B', not_flushed=True)

    await file.flush()

    assert mocked_manifests_manager.flush_on_local.call_count
    mocked_manifests_manager.flush_on_local.assert_called_with(
        '<foo id>', b'<foo key>', {
            'format': 1,
            'type': 'local_file_manifest',
            'need_sync': True,
            'base_version': 1,
            'created': datetime(2017, 1, 1),
            'updated': datetime(2017, 12, 31, 23, 59, 59),
            'blocks': [
                {'id': '<1 id>', 'key': 'PDEga2V5Pg==\n', 'offset': 0, 'size': 10},
                {'id': '<2 id>', 'key': 'PDIga2V5Pg==\n', 'offset': 10, 'size': 10},
            ],
            'dirty_blocks': [
                {'id': '<A id>', 'key': 'PEEga2V5Pg==\n', 'offset': 5, 'size': 5},
                {'id': '<B id>', 'key': 'PEIga2V5Pg==\n', 'offset': 7, 'size': 5}
            ]
        }
    )

    assert mocked_blocks_manager.flush_on_local.call_count == 1
    mocked_blocks_manager.flush_on_local.assert_called_with(b'BBBBB')

    assert not file.need_flush
    assert file.need_sync


@pytest.mark.xfail
@pytest.mark.trio
async def test_simple_sync(fs, mocked_manifests_manager):
    file = create_file(fs, 'foo')
    add_block(file, b'1' * 10, offset=0, name='1', in_local=True)
    add_block(file, b'2' * 10, offset=10, name='2', in_local=False)
    add_dirty_block(file, b'A' * 5, offset=5, name='A', not_flushed=False)
    add_dirty_block(file, b'B' * 5, offset=7, name='B', not_flushed=True)

    await file.sync()


@pytest.mark.trio
async def test_get_merged_blocks(fs):
    file = create_file(fs, 'foo')
    add_block(file, b'1' * 100, offset=0, name='1', in_local=True)
    add_block(file, b'2' * 100, offset=100, name='2', in_local=False)
    add_block(file, b'3' * 100, offset=200, name='3', in_local=True)
    add_block(file, b'4' * 100, offset=300, name='4', in_local=False)
    add_block(file, b'5' * 100, offset=400, name='5', in_local=True)
    add_dirty_block(file, b'A' * 140, offset=340, name='A', not_flushed=True)
    add_dirty_block(file, b'B' * 50, offset=250, name='B', not_flushed=True)
    add_dirty_block(file, b'C' * 40, offset=30, name='C', not_flushed=False)
    add_dirty_block(file, b'D' * 20, offset=60, name='D', not_flushed=True)
    add_dirty_block(file, b'E' * 120, offset=350, name='E', not_flushed=False)
    add_dirty_block(file, b'F' * 60, offset=330, name='F', not_flushed=True)
    add_dirty_block(file, b'G' * 100, offset=500, name='G', not_flushed=False)
    add_dirty_block(file, b'H' * 1, offset=0, name='H', not_flushed=True)
    await file.truncate(475)

    blocks = await file.get_merged_blocks()
    assert len(blocks) == 12
    results = [
        (0, 1, b'H'),
        (1, 30, b'1'),
        (30, 60, b'C'),
        (60, 80, b'D'),
        (80, 100, b'1'),
        (100, 200, b'2'),
        (200, 250, b'3'),
        (250, 300, b'B'),
        (300, 330, b'4'),
        (330, 390, b'F'),
        (390, 470, b'E'),
        (470, 475, b'A'),
    ]
    for index, (offset, end, data) in enumerate(results):
        assert blocks[index].offset == offset
        assert blocks[index].end == end
        assert await blocks[index].fetch_data() == data * (end - offset)


@pytest.mark.trio
async def test_get_normalized_blocks(fs):
    file = create_file(fs, 'foo')
    add_block(file, b'1' * 90, offset=0, name='1', in_local=True)
    add_block(file, b'2' * 100, offset=90, name='2', in_local=False)
    add_block(file, b'3' * 200, offset=190, name='3', in_local=True)
    add_block(file, b'4' * 100, offset=490, name='4', in_local=False)
    add_block(file, b'5' * 110, offset=600, name='5', in_local=True)
    add_block(file, b'6' * 100, offset=600, name='6', in_local=False)
    add_block(file, b'7' * 10, offset=700, name='7', in_local=True)

    block_size = 100
    blocks = await file.get_normalized_blocks(file._blocks, block_size)
    assert len(blocks) == 8
    results = [
        (0, 100, b'1' * 90 + b'2' * 10),
        (100, 200, b'2' * 90 + b'3' * 10),
        (200, 300, b'3' * 100),
        (300, 400, b'3' * 90 + b'4' * 10),
        (400, 500, b'4' * 90 + b'5' * 10),
        (500, 600, b'5' * 100),
        (600, 700, b'6' * 100),
        (700, 710, b'7' * 10),
    ]
    for index, (offset, end, data) in enumerate(results):
        assert blocks[index].offset == offset
        assert blocks[index].end == end
        assert await blocks[index].fetch_data() == data
