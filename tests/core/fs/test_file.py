import pytest
import attr
import nacl.encoding
import nacl.hash
from pendulum import datetime
from hypothesis import given, strategies as st, note, assume

from parsec.core.fs import *


def create_file(
    fs,
    name,
    parent=None,
    is_placeholder=False,
    is_not_loaded=False,
    need_sync=False,
    need_flush=False,
    size=0,
    blocks_accesses=None,
    dirty_blocks_accesses=None,
):
    if is_placeholder:
        access = fs._placeholder_access_cls(
            "<%s id>" % name, b"<%s key>" % name.encode()
        )
        base_version = 0
    else:
        access = fs._vlob_access_cls(
            "<%s id>" % name,
            "<%s rts>" % name,
            "<%s wts>" % name,
            b"<%s key>" % name.encode(),
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


def add_block(
    file,
    data,
    offset=0,
    in_local=False,
    name=None,
    dirty=False,
    dirty_not_flushed=False,
):
    if file._fs.blocks_manager.fetch_from_backend.side_effect is None:
        file._fs.blocks_manager.fetch_from_backend.side_effect = _mocked_lookup_in_data(
            file._fs.blocks_manager.fetch_from_backend
        )

    if file._fs.blocks_manager.fetch_from_local.side_effect is None:
        file._fs.blocks_manager.fetch_from_local.side_effect = _mocked_lookup_in_data(
            file._fs.blocks_manager.fetch_from_local
        )

    count = (
        len(file._fs.blocks_manager.fetch_from_backend.data)
        + len(file._fs.blocks_manager.fetch_from_local.data)
    )
    # count = len(file._blocks) + len(file._dirty_blocks)
    name = name or "block-%s" % count
    id = "<%s id>" % name
    key = ("<%s key>" % name).encode()
    digest = nacl.hash.sha256(data, encoder=nacl.encoding.Base64Encoder)
    if dirty:
        access = file._fs._dirty_block_access_cls(id, key, offset, len(data), digest)
    else:
        access = file._fs._block_access_cls(id, key, offset, len(data), digest)
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
        access=fs._user_vlob_access_cls(b"<foo key>"),
        need_flush=False,
        need_sync=False,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=1,
        name="root",
        parent=None,
        children_accesses={},
    )


@pytest.fixture
def bar_txt(fs, root):
    bar = fs._file_entry_cls(
        access=fs._vlob_access_cls("<bar id>", "<bar rts>", "<bar wts>", b"<bar key>"),
        need_flush=False,
        need_sync=True,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=0,
        name="bar.txt",
        parent=root,
    )
    root._children["bar.txt"] = bar
    return bar


@pytest.mark.trio
async def test_attributes(bar_txt, root):

    assert bar_txt.parent is root
    with pytest.raises(AttributeError):
        bar_txt.parent = None

    assert bar_txt.name == "bar.txt"
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
    foo = create_file(fs, "foo")
    content = await foo.read()
    assert content == b""


@pytest.mark.trio
async def test_read_blocks(fs, mocked_blocks_manager):
    blocks_data = [b"Hello... ", b"world !"]
    digests = [
        nacl.hash.sha256(data, encoder=nacl.encoding.Base64Encoder)
        for data in blocks_data
    ]
    blocks_accesses = [
        fs._block_access_cls("<block 1 id>", b"<block 1 key>", 0, 9, digests[0]),
        fs._block_access_cls("<block 2 id>", b"<block 2 key>", 9, 7, digests[1]),
    ]
    foo = create_file(fs, "foo", blocks_accesses=blocks_accesses, size=16)
    mocked_blocks_manager.fetch_from_local.side_effect = blocks_data
    content = await foo.read()
    assert content == b"Hello... world !"
    assert (
        mocked_blocks_manager.fetch_from_local.call_args_list
        == [
            [("<block 1 id>", b"<block 1 key>"), {}],
            [("<block 2 id>", b"<block 2 key>"), {}],
        ]
    )


@pytest.mark.trio
async def test_file_read_too_big_size(fs, mocked_blocks_manager):
    file = create_file(fs, "foo")
    add_block(file, b"Hello... ", offset=0, in_local=True)
    add_block(file, b"world !", offset=9, in_local=True)
    out = await file.read(40)
    assert out == b"Hello... world !"
    out = await file.read(-1)
    assert out == b"Hello... world !"


@pytest.mark.trio
async def test_write(fs):
    file = create_file(fs, "foo")
    await file.write(b"foo")
    out = await file.read()
    assert out == b"foo"


@pytest.mark.trio
async def test_write_empty(fs):
    file = create_file(fs, "foo")
    await file.write(b"")
    out = await file.read()
    assert out == b""


@pytest.mark.trio
async def test_write_overwrite(fs):
    file = create_file(fs, "foo")
    await file.write(b"123")
    await file.write(b"abc")
    out = await file.read()
    assert out == b"abc"


@pytest.mark.trio
async def test_multi_write(fs):
    file = create_file(fs, "foo")
    await file.write(b"123")
    await file.write(b"456", offset=3)
    await file.write(b"789", offset=6)
    out = await file.read(6, offset=2)
    assert out == b"345678"


@pytest.mark.trio
async def test_write_and_truncate(fs):
    file = create_file(fs, "foo")
    await file.write(b"123")
    await file.write(b"456", offset=3)
    await file.write(b"789", offset=6)
    await file.truncate(5)
    out = await file.read()
    assert out == b"12345"


@pytest.mark.trio
async def test_truncate_too_long(fs):
    file = create_file(fs, "foo")
    add_block(file, b"Hello... ", offset=0, in_local=True)
    add_block(file, b"world !", offset=9, in_local=True)
    await file.truncate(100)
    out = await file.read()
    assert out == b"Hello... world !"


# TODO: test read/write/flush/sync with dirty blocks, not flushed dirty blocks etc.


@pytest.mark.trio
async def test_flush_not_needed(bar_txt, mocked_manifests_manager):
    await bar_txt.flush()
    assert not bar_txt.need_flush
    mocked_manifests_manager.flush_on_local.assert_not_called()


@pytest.mark.trio
async def test_flush(fs, mocked_manifests_manager, mocked_blocks_manager):
    file = create_file(fs, "foo")
    add_block(file, b"1" * 10, offset=0, name="1", in_local=True)
    add_block(file, b"2" * 10, offset=10, name="2", in_local=False)
    add_dirty_block(file, b"A" * 5, offset=5, name="A", not_flushed=False)
    add_dirty_block(file, b"B" * 5, offset=7, name="B", not_flushed=True)

    await file.flush()

    digests = [
        nacl.hash.sha256(data, encoder=nacl.encoding.Base64Encoder)
        for data in [b"1" * 10, b"2" * 10, b"A" * 5, b"B" * 5]
    ]

    assert mocked_manifests_manager.flush_on_local.call_count
    mocked_manifests_manager.flush_on_local.assert_called_with(
        "<foo id>",
        b"<foo key>",
        {
            "format": 1,
            "type": "local_file_manifest",
            "need_sync": True,
            "base_version": 1,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "size": 20,
            "blocks": [
                {
                    "id": "<1 id>",
                    "key": b"<1 key>",
                    "offset": 0,
                    "size": 10,
                    "digest": digests[0],
                },
                {
                    "id": "<2 id>",
                    "key": b"<2 key>",
                    "offset": 10,
                    "size": 10,
                    "digest": digests[1],
                },
            ],
            "dirty_blocks": [
                {
                    "id": "<A id>",
                    "key": b"<A key>",
                    "offset": 5,
                    "size": 5,
                    "digest": digests[2],
                },
                {
                    "id": "<B id>",
                    "key": b"<B key>",
                    "offset": 7,
                    "size": 5,
                    "digest": digests[3],
                },
            ],
        },
    )

    assert mocked_blocks_manager.flush_on_local.call_count == 1
    mocked_blocks_manager.flush_on_local.assert_called_with(
        "<B id>", b"<B key>", b"BBBBB"
    )

    assert not file.need_flush
    assert file.need_sync


@pytest.mark.trio
@pytest.mark.xfail
async def test_simple_sync(fs, mocked_manifests_manager, mocked_blocks_manager):
    file = create_file(fs, "foo")
    add_block(file, b"1" * 10, offset=0, name="1", in_local=True)
    add_block(file, b"2" * 10, offset=10, name="2", in_local=False)
    add_dirty_block(file, b"A" * 5, offset=5, name="A", not_flushed=False)
    add_dirty_block(file, b"B" * 5, offset=7, name="B", not_flushed=True)

    await file.sync()

    digests = [
        nacl.hash.sha256(data, encoder=nacl.encoding.Base64Encoder)
        for data in [b"1" * 10, b"2" * 10]
    ]

    mocked_manifests_manager.sync_with_backend.assert_called_with(
        "<foo id>",
        "<foo wts>",
        b"<foo key>",
        {
            "format": 1,
            "type": "file_manifest",
            "version": 2,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "size": 20,
            "blocks": [
                {
                    "id": "<1 id>",
                    "key": b"<1 key>",
                    "offset": 0,
                    "size": 10,
                    "digest": digests[0],
                },
                {
                    "id": "<2 id>",
                    "key": b"<2 key>",
                    "offset": 10,
                    "size": 10,
                    "digest": digests[1],
                },
            ],
        },
    )


# TODO: use assert_called_with on mocked_blocks_manager


@attr.s(init=False)
class MockBlock:
    data = attr.ib()
    offset = attr.ib()
    end = attr.ib()
    size = attr.ib()

    def __init__(self, data, offset):
        self.data = data
        self.offset = offset
        self.end = offset + len(data)
        self.size = len(data)


def test_get_merged_blocks_like_middleage():
    blocks = []
    blocks.append(MockBlock(b"1" * 100, 0))
    blocks.append(MockBlock(b"2" * 100, 100))
    blocks.append(MockBlock(b"3" * 100, 200))
    blocks.append(MockBlock(b"4" * 100, 300))
    blocks.append(MockBlock(b"5" * 100, 400))

    dirty_blocks = []
    dirty_blocks.append(MockBlock(b"A" * 140, 340))
    dirty_blocks.append(MockBlock(b"B" * 50, 250))
    dirty_blocks.append(MockBlock(b"C" * 40, 30))
    dirty_blocks.append(MockBlock(b"D" * 20, 60))
    dirty_blocks.append(MockBlock(b"E" * 120, 350))
    dirty_blocks.append(MockBlock(b"F" * 60, 330))
    dirty_blocks.append(MockBlock(b"G" * 100, 500))
    dirty_blocks.append(MockBlock(b"H" * 1, 0))

    results = [
        (dirty_blocks[7], 0, 1, b"H"),
        (blocks[0], 1, 30, b"1"),
        (dirty_blocks[2], 30, 60, b"C"),
        (dirty_blocks[3], 60, 80, b"D"),
        (blocks[0], 80, 100, b"1"),
        (blocks[1], 100, 200, b"2"),
        (blocks[2], 200, 250, b"3"),
        (dirty_blocks[1], 250, 300, b"B"),
        (blocks[3], 300, 330, b"4"),
        (dirty_blocks[5], 330, 390, b"F"),
        (dirty_blocks[4], 390, 470, b"E"),
        (dirty_blocks[0], 470, 475, b"A"),
    ]

    merged_blocks = file.get_merged_blocks(blocks, dirty_blocks, 475)

    assert len(merged_blocks) == 12

    for index, (base_block, offset, end, data) in enumerate(results):
        block, block_offset, block_end = merged_blocks[index]
        if not block_offset and not block_end:
            block_offset = 0
            block_end = block.end - block.offset
        block_data = merged_blocks[index][0].data
        block_data = block_data[block_offset:block_end]
        assert block_data == data * (end - offset)
        block_offset += merged_blocks[index][0].offset
        block_end += merged_blocks[index][0].offset
        assert block == base_block
        assert block == merged_blocks[index][0]
        assert block_offset == offset
        assert block_end == end


def _generate_contiguous_blocks(random, slices, size):
    blocks = []
    curr_offset = 0
    for i in range(slices):
        curr_block_size = random.randint(0, size - curr_offset)
        # Keep payload different for each block
        curr_block_payload = chr(ord("a") + i).encode() * curr_block_size
        blocks.append(MockBlock(curr_block_payload, curr_offset))
        curr_offset += curr_block_size
    blocks.append(MockBlock(b"@" * (size - curr_offset), curr_offset))
    return blocks


@given(
    random=st.randoms(),
    dirty_blocks_count=st.integers(min_value=0, max_value=25),
    synced_blocks_slices=st.integers(min_value=0, max_value=25),
    synced_file_size=st.integers(min_value=0),
)
def test_get_merged_blocks(
    random, dirty_blocks_count, synced_file_size, synced_blocks_slices
):
    synced_blocks = _generate_contiguous_blocks(
        random, synced_blocks_slices, synced_file_size
    )
    synced_data = b"".join([x.data for x in synced_blocks])
    note("Synced blocks: %s" % synced_blocks)

    expected_data = synced_data
    dirty_blocks = []
    for i in range(dirty_blocks_count):
        dirty_block_size = random.randint(0, synced_file_size)
        dirty_block_offset = random.randint(0, synced_file_size)
        # Keep payload different for each block
        dirty_block_payload = chr(ord("a") + i).encode() * dirty_block_size
        dirty_blocks.append(MockBlock(dirty_block_payload, dirty_block_offset))
        expected_data = (
            expected_data[:dirty_block_offset]
            + dirty_block_payload
            + expected_data[dirty_block_offset + dirty_block_size:]
        )
    note("Dirty blocks: %s" % dirty_blocks)
    # Simulate truncate
    final_size = random.randint(0, len(expected_data))
    note("Truncate to: %s, expect: %r" % (final_size, expected_data))
    expected_data = expected_data[:final_size]

    merged_blocks = file.get_merged_blocks(synced_blocks, dirty_blocks, final_size)
    merged_data = b"".join(
        [block.data[start:end] for block, start, end in merged_blocks]
    )

    assert merged_data == expected_data


@given(
    random=st.randoms(),
    normalized_block_size=st.integers(min_value=1, max_value=10000),
    size=st.integers(min_value=0),
    slices=st.integers(min_value=0, max_value=25),
)
def test_get_normalized_blocks(random, normalized_block_size, size, slices):
    assume(size / normalized_block_size < 1000)
    blocks = _generate_contiguous_blocks(random, slices, size)
    note("Blocks: %s" % blocks)
    normalized_blocks = file.get_normalized_blocks(blocks, normalized_block_size)
    note("Normalized blocks: %s" % normalized_blocks)

    original_data = b"".join([x.data for x in blocks])

    normalized_data = []
    for i, normalized_block in enumerate(normalized_blocks):
        block_size = 0
        for block, offset, end in normalized_block:
            block_size += end - offset
            normalized_data.append(block.data[offset:end])
        if i < len(normalized_blocks) - 1:
            assert block_size == normalized_block_size
        else:
            assert block_size <= normalized_block_size
    normalized_data = b"".join(normalized_data)

    assert original_data == normalized_data


def test_get_normalized_blocks_like_middleage():
    blocks = []
    blocks.append(MockBlock(b"1" * 90, 0))
    blocks.append(MockBlock(b"2" * 100, 90))
    blocks.append(MockBlock(b"3" * 200, 190))
    blocks.append(MockBlock(b"4" * 100, 390))
    blocks.append(MockBlock(b"5" * 110, 490))
    blocks.append(MockBlock(b"6" * 100, 600))
    blocks.append(MockBlock(b"7" * 10, 700))

    results = [
        [(blocks[0], 0, 90, b"1" * 90), (blocks[1], 90, 100, b"2" * 10)],
        [(blocks[1], 100, 190, b"2" * 90), (blocks[2], 190, 200, b"3" * 10)],
        [(blocks[2], 200, 300, b"3" * 100)],
        [(blocks[2], 300, 390, b"3" * 90), (blocks[3], 390, 400, b"4" * 10)],
        [(blocks[3], 400, 490, b"4" * 90), (blocks[4], 490, 500, b"5" * 10)],
        [(blocks[4], 500, 600, b"5" * 100)],
        [(blocks[5], 600, 700, b"6" * 100)],
        [(blocks[6], 700, 710, b"7" * 10)],
    ]

    block_size = 100
    normalized_blocks = file.get_normalized_blocks(blocks, block_size)

    assert len(normalized_blocks) == 8

    for group_index, group in enumerate(results):
        for index, (base_block, offset, end, data) in enumerate(group):
            block, block_offset, block_end = normalized_blocks[group_index][index]
            if not block_offset and not block_end:
                block_offset = 0
                block_end = block.end - block.offset
            block_data = normalized_blocks[group_index][index][0].data
            block_data = block_data[block_offset:block_end]
            assert block_data == data
            block_offset += normalized_blocks[group_index][index][0].offset
            block_end += normalized_blocks[group_index][index][0].offset
            assert block == base_block
            assert block == normalized_blocks[group_index][index][0]
            assert block_offset == offset
            assert block_end == end
