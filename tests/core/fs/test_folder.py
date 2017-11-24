import trio
import pytest
from freezegun import freeze_time
from pendulum import datetime

from parsec.core.fs import *


@pytest.fixture
def root(fs):
    return fs._folder_entry_cls(
        access=fs._user_vlob_access_cls(b'<foo key>'),
        need_flush=False,
        need_sync=False,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=1,
        name='',
        parent=None,
        children_accesses={},
    )


@pytest.fixture
def foo(fs, root):
    foo = fs._folder_entry_cls(
        access=fs._vlob_access_cls('<foo id>', '<foo rts>', '<foo wts>', b'<foo key>'),
        need_flush=False,
        need_sync=True,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=0,
        name='foo',
        parent=root,
        children_accesses = {
            'child': fs._placeholder_access_cls('<child id>', b'<child key>'),
        },
    )
    root._children['foo'] = foo
    return foo


@pytest.fixture
def bar_txt(fs, foo):
    bar = fs._file_entry_cls(
        access=fs._vlob_access_cls('<bar id>', '<bar rts>', '<bar wts>', b'<bar key>'),
        need_flush=False,
        need_sync=True,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=0,
        name='bar.txt',
        parent=foo,
    )
    foo._children['bar.txt'] = bar
    return bar


@pytest.fixture
def new_txt(fs):
    return fs._file_entry_cls(
        access=fs._placeholder_access_cls('<new id>', b'<new key>'),
        need_flush=True,
        need_sync=True,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=0,
        name='new.txt',
        parent=None,
    )


@pytest.fixture
def spam_txt(fs, root):
    spam = fs._file_entry_cls(
        access=fs._vlob_access_cls('<spam id>', '<spam rts>', '<spam wts>', b'<spam key>'),
        need_flush=False,
        created=datetime(2017, 1, 1),
        updated=datetime(2017, 12, 31, 23, 59, 59),
        base_version=0,
        name='spam.txt',
        parent=root,
    )
    root._children['spam.txt'] = spam
    return spam


def create_entry(fs, name, parent=None, is_file=False, is_placeholder=False,
                 is_not_loaded=False, need_sync=False, need_flush=False):
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
    elif is_file:
        entry = fs._file_entry_cls(
            access=access,
            need_flush=need_flush,
            need_sync=need_sync,
            created=datetime(2017, 1, 1),
            updated=datetime(2017, 12, 31, 23, 59, 59),
            base_version=base_version,
            name=name,
            parent=parent,
        )
    else:
        entry = fs._folder_entry_cls(
            access=access,
            need_flush=need_flush,
            need_sync=need_sync,
            created=datetime(2017, 1, 1),
            updated=datetime(2017, 12, 31, 23, 59, 59),
            base_version=base_version,
            name=name,
            parent=root,
            children_accesses={},
        )
    if parent:
        parent._children[name] = entry
    return entry


def test_path(root, foo, bar_txt):
    assert root.path == ''  # hoooo big hack here !
    assert foo.path == '/foo'
    assert bar_txt.path == '/foo/bar.txt'
    with pytest.raises(AttributeError):
        foo.path = None


@pytest.mark.trio
async def test_attributes(foo, root):

    assert foo.parent is root
    with pytest.raises(AttributeError):
        foo.parent = None

    assert foo.name == 'foo'
    with pytest.raises(AttributeError):
        foo.name = None

    assert not foo.is_placeholder
    with pytest.raises(AttributeError):
        foo.is_placeholder = None

    assert foo.created == datetime(2017, 1, 1)
    with pytest.raises(AttributeError):
        foo.created = None

    assert foo.updated == datetime(2017, 12, 31, 23, 59, 59)
    with pytest.raises(AttributeError):
        foo.updated = None

    assert foo.base_version == 0
    with pytest.raises(AttributeError):
        foo.base_version = None

    assert set(foo.keys()) == {'child'}
    assert 'child' in foo
    assert 'dummy' not in foo


@pytest.mark.trio
async def test_fetch_child(fs, mocked_manifests_manager, foo):
    mocked_manifests_manager.fetch_from_local.return_value = {
        'format': 1,
        'type': 'local_file_manifest',
        'base_version': 0,
        'need_sync': True,
        'created': datetime(2017, 1, 1),
        'updated': datetime(2017, 12, 31, 23, 59, 59),
        'size': 0,
        'blocks': [],
        'dirty_blocks': [],
    }
    child = await foo.fetch_child('child')
    assert isinstance(child, fs._file_entry_cls)
    assert child.name == 'child'
    assert child.parent is foo


@pytest.mark.trio
async def test_fetch_unknown_child(foo):
    with pytest.raises(FSInvalidPath):
        await foo.fetch_child('dummy')


@pytest.mark.trio
async def test_delete_child(foo, bar_txt):
    with freeze_time('2017-07-07'):
        ret = await foo.delete_child('bar.txt')
    assert foo.updated == datetime(2017, 7, 7)
    assert foo.need_flush
    assert ret is bar_txt
    assert 'bar.txt' not in foo
    # deleted entry should be still usable to be inserted somewhere else
    assert bar_txt.parent is None


@pytest.mark.trio
async def test_delete_not_loaded_child(fs, foo, bar_txt):
    with freeze_time('2017-07-07'):
        ret = await foo.delete_child('child')
    assert foo.updated == datetime(2017, 7, 7)
    assert foo.need_flush
    assert isinstance(ret, fs._not_loaded_entry_cls)
    assert 'child' not in foo


@pytest.mark.trio
async def test_delete_unknown_child(foo):
    with pytest.raises(FSInvalidPath):
        await foo.delete_child('dummy')
    assert not foo.need_flush


@pytest.mark.trio
async def test_insert_child(foo, new_txt):
    with freeze_time('2017-07-07'):
        await foo.insert_child('new', new_txt)
    assert foo.updated == datetime(2017, 7, 7)
    assert foo.need_flush
    assert 'new' in foo
    assert new_txt.parent is foo


@pytest.mark.trio
async def test_insert_existing_child(foo, new_txt):
    with pytest.raises(FSInvalidPath):
        await foo.insert_child('child', new_txt)
    assert not foo.need_flush


@pytest.mark.trio
async def test_insert_entry_with_parent(foo, bar_txt):
    with pytest.raises(FSError):
        await foo.insert_child('new', bar_txt)
    assert not foo.need_flush


@pytest.mark.trio
async def test_create_folder(fs, foo):
    with freeze_time('2017-07-07'):
        new = await foo.create_folder('new')
    assert isinstance(new, fs._folder_entry_cls)

    assert 'new' in foo
    assert foo.updated == datetime(2017, 7, 7)
    assert foo.need_flush

    assert new.need_flush
    assert new.parent is foo
    assert new.name == 'new'
    assert new.updated == datetime(2017, 7, 7)
    assert new.created == datetime(2017, 7, 7)
    assert not new.keys()


@pytest.mark.trio
async def test_create_folder_existing_child(foo):
    with pytest.raises(FSInvalidPath):
        await foo.create_folder('child')
    assert not foo.need_flush


@pytest.mark.trio
async def test_create_file(fs, foo):
    with freeze_time('2017-07-07'):
        new = await foo.create_file('new')
    assert isinstance(new, fs._file_entry_cls)

    assert 'new' in foo
    assert foo.updated == datetime(2017, 7, 7)
    assert foo.need_flush

    assert new.parent is foo
    assert new.name == 'new'
    assert new.updated == datetime(2017, 7, 7)
    assert new.created == datetime(2017, 7, 7)
    assert new.size == 0


@pytest.mark.trio
async def test_create_file_existing_child(foo):
    with pytest.raises(FSInvalidPath):
        await foo.create_file('child')
    assert not foo.need_flush


@pytest.mark.trio
async def test_flush_not_needed(foo, mocked_manifests_manager):
    await foo.flush()
    assert not foo.need_flush
    mocked_manifests_manager.flush_on_local.assert_not_called()


@pytest.mark.trio
async def test_flush(foo, bar_txt, mocked_manifests_manager):
    with freeze_time('2017-07-07'):
        child = await foo.delete_child('child')
        await foo.insert_child('renamed', child)
    assert foo.need_flush
    await foo.flush()
    assert not foo.need_flush
    mocked_manifests_manager.fetch_from_local.assert_not_called()
    mocked_manifests_manager.fetch_from_backend.assert_not_called()
    mocked_manifests_manager.flush_on_local.assert_called_with(
        '<foo id>', b'<foo key>', {
            'format': 1,
            'type': 'local_folder_manifest',
            'base_version': 0,
            'need_sync': True,
            'created': datetime(2017, 1, 1),
            'updated': datetime(2017, 7, 7),
            'children': {
                'renamed': {
                    'type': 'placeholder',
                    'id': '<child id>',
                    'key': b'<child key>',
                },
                'bar.txt': {
                    'type': 'vlob',
                    'id': '<bar id>',
                    'rts': '<bar rts>',
                    'wts': '<bar wts>',
                    'key': b'<bar key>',
                },
            }
        }
    )


@pytest.mark.trio
async def test_simple_sync(fs, mocked_manifests_manager):
    foo = create_entry(fs, 'foo', need_sync=True)
    with freeze_time('2017-07-07'):
        await foo.sync()
    assert not foo.need_sync
    assert foo.base_version == 2
    mocked_manifests_manager.flush_on_local.assert_not_called()
    mocked_manifests_manager.sync_with_backend.assert_called_with(
        '<foo id>', '<foo wts>', b'<foo key>', {
            'format': 1,
            'type': 'folder_manifest',
            'version': 2,
            'created': datetime(2017, 1, 1),
            'updated': datetime(2017, 12, 31, 23, 59, 59),
            'children': {
            }
        }
    )


@pytest.mark.trio
async def test_sync_with_children(fs, mocked_manifests_manager):
    foo = create_entry(fs, 'foo', need_sync=True)
    create_entry(fs, 'bar', parent=foo)
    create_entry(fs, 'spam.txt', is_file=True, parent=foo)
    # Note we don't need to load entry to sync the parent
    create_entry(fs, 'baz', is_not_loaded=True, parent=foo)
    with freeze_time('2017-07-07'):
        await foo.sync()
    assert not foo.need_sync
    assert foo.base_version == 2
    mocked_manifests_manager.flush_on_local.assert_not_called()
    mocked_manifests_manager.fetch_from_local.assert_not_called()
    assert mocked_manifests_manager.sync_with_backend.call_count == 1
    mocked_manifests_manager.sync_with_backend.assert_called_with(
        '<foo id>', '<foo wts>', b'<foo key>', {
            'format': 1,
            'type': 'folder_manifest',
            'version': 2,
            'created': datetime(2017, 1, 1),
            'updated': datetime(2017, 12, 31, 23, 59, 59),
            'children': {
                'bar': {
                    'id': '<bar id>',
                    'rts': '<bar rts>',
                    'wts': '<bar wts>',
                    'key': b'<bar key>',
                },
                'baz': {
                    'id': '<baz id>',
                    'rts': '<baz rts>',
                    'wts': '<baz wts>',
                    'key': b'<baz key>',
                },
                'spam.txt': {
                    'id': '<spam.txt id>',
                    'rts': '<spam.txt rts>',
                    'wts': '<spam.txt wts>',
                    'key': b'<spam.txt key>',
                }
            },
        }
    )


@pytest.mark.trio
async def test_sync_with_placeholder_children(fs, mocked_manifests_manager):
    foo = create_entry(fs, 'foo', need_sync=True)

    # Folder with placeholders, those shouldn't be synced
    bar = create_entry(fs, 'bar', is_placeholder=True, parent=foo)
    create_entry(fs, 'a', is_placeholder=True, parent=bar)
    create_entry(fs, 'b', is_file=True, is_placeholder=True, parent=bar)
    create_entry(fs, 'c', is_not_loaded=True, is_placeholder=True, parent=bar)

    # Regular empty file
    create_entry(fs, 'spam.txt', is_file=True, is_placeholder=True, parent=foo)

    # This entry must be loaded to know it type before being able to synchronize it
    create_entry(fs, 'mysterious', is_not_loaded=True, is_placeholder=True, parent=foo)
    mocked_manifests_manager.fetch_from_local.side_effect = [
        {  # Dirty file, again we shouldn't sync the new blocks
            'format': 1,
            'type': 'local_file_manifest',
            'base_version': 0,
            'need_sync': True,
            'created': datetime(2017, 1, 1),
            'updated': datetime(2017, 12, 31, 23, 59, 59),
            'size': 42,
            'blocks': [
                {
                    'id': '<block id>',
                    'key': '<block key>',
                    'offset': 0,
                    'size': 20,
                },
            ],
            'dirty_blocks': [
                {
                    'id': '<dirty block id>',
                    'key': '<dirty block key>',
                    'offset': 20,
                    'size': 22,
                },
            ],
        }
    ]

    # Each placeholder will trigger a vlob creation in backend
    mocked_manifests_manager.sync_new_entry_with_backend.side_effect = [
        ('<id 1>', '<rts 1>', '<wts 1>'),
        ('<id 2>', '<rts 2>', '<wts 2>'),
        ('<id 3>', '<rts 3>', '<wts 3>'),
    ]

    with freeze_time('2017-07-07'):
        await foo.sync()
    assert not foo.need_sync
    assert foo.base_version == 2
    mocked_manifests_manager.fetch_from_local.assert_called_with(
        '<mysterious id>', b'<mysterious key>')
    assert mocked_manifests_manager.sync_new_entry_with_backend.call_args_list == [
        [
            (b'<bar key>', {
                'format': 1,
                'type': 'folder_manifest',
                'version': 1,
                'created': datetime(2017, 1, 1),
                'updated': datetime(2017, 1, 1),
                'children': {}
            }),
            {}
        ],
        [
            (b'<spam.txt key>', {
                'format': 1,
                'type': 'file_manifest',
                'version': 1,
                'created': datetime(2017, 1, 1),
                'updated': datetime(2017, 1, 1),
                'blocks': {},
                'size': 0
            }),
            {}
        ],
        [
            (b'<mysterious key>', {
                'format': 1,
                'type': 'file_manifest',
                'version': 1,
                'created': datetime(2017, 1, 1),
                'updated': datetime(2017, 1, 1),
                'blocks': {},
                'size': 0
            }),
            {}
        ],
    ]

    # TODO: also check local storage flushes

    assert mocked_manifests_manager.sync_with_backend.call_count == 1
    mocked_manifests_manager.sync_with_backend.assert_called_with(
        '<foo id>', '<foo wts>', b'<foo key>', {
            'format': 1,
            'type': 'folder_manifest',
            'version': 2,
            'created': datetime(2017, 1, 1),
            'updated': datetime(2017, 12, 31, 23, 59, 59),
            'children': {
                'bar': {
                    'id': '<id 1>',
                    'rts': '<rts 1>',
                    'wts': '<wts 1>',
                    'key': b'<bar key>',
                },
                'spam.txt': {
                    'id': '<id 2>',
                    'rts': '<rts 2>',
                    'wts': '<wts 2>',
                    'key': b'<spam.txt key>',
                },
                'mysterious': {
                    'id': '<id 3>',
                    'rts': '<rts 3>',
                    'wts': '<wts 3>',
                    'key': b'<mysterious key>',
                },
            },
        }
    )


@pytest.mark.trio
async def test_sync_with_concurrent_updates(fs, mocked_manifests_manager):
    foo = create_entry(fs, 'foo', need_sync=True)
    create_entry(fs, 'a', is_file=True, is_placeholder=True, parent=foo)
    create_entry(fs, 'b', is_file=True, is_placeholder=True, parent=foo)
    create_entry(fs, 'c', parent=foo)

    # Each placeholder will trigger a vlob creation in backend
    mocked_manifests_manager.sync_new_entry_with_backend.side_effect = [
        ('<id 1>', '<rts 1>', '<wts 1>'),
        ('<id 2>', '<rts 2>', '<wts 2>'),
    ]

    async def _sync_with_backend(*args, **kwargs):
        # During the time the manifest is uploaded to the backend,
        # we can make modifications
        await foo.delete_child('a')
        b = await foo.delete_child('b')
        await foo.delete_child('c')
        await foo.insert_child('c', b)

    mocked_manifests_manager.sync_with_backend.side_effect = _sync_with_backend

    with freeze_time('2017-07-07'):
        await foo.sync()
    # In the end, folder should have kept it changes
    assert set(foo.keys()) == {'c'}
    new_c = await foo.fetch_child('c')
    assert isinstance(new_c, fs._file_entry_cls)
    assert isinstance(new_c._access, fs._vlob_access_cls)
    assert new_c._access.id == '<id 2>'


@pytest.mark.xfail
@pytest.mark.trio
async def test_sync_outdated_version(fs, mocked_manifests_manager):
    raise NotImplementedError()
