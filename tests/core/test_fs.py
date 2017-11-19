import pytest
from trio.testing import trio_test
from unittest.mock import Mock

from parsec.core.fs import BaseFS, BaseFile, BaseFolder, FSInvalidPath
from parsec.core.manifest import (
    LocalFileManifest, LocalFolderManifest, LocalUserManifest,
    PlaceHolderEntry, SyncedEntry
)

from tests.common import alice


class MockedSyncedEntry(SyncedEntry):
    def __init__(self, id):
        super().__init__(id, '<%s-syncid>' % id, b'<key>', '<rts>', '<wts>')


class MockedPlaceHolderEntry(PlaceHolderEntry):
    def __init__(self, id):
        super().__init__(id, b'<key>')


def test_init_fs():
    manifests_manager = Mock()
    synchronizer = Mock()
    manifests_manager.fetch_user_manifest.return_value = LocalUserManifest()
    BaseFS(alice, manifests_manager, synchronizer)
    manifests_manager.fetch_user_manifest.assert_called_with()


def _fs_factory(local_user_manifest=None):
    manifests_manager = Mock()
    synchronizer = Mock()
    manifests_manager.fetch_user_manifest.return_value = local_user_manifest or LocalUserManifest()

    async def fetch_manifest(*args, **kwargs):
        return manifests_manager.a_fetch_manifest(*args, **kwargs)

    manifests_manager.fetch_manifest = fetch_manifest

    return BaseFS(alice, manifests_manager, synchronizer)


@trio_test
async def test_fs_fetch_root():
    local_user_manifest = LocalUserManifest(children={
        'foo': MockedSyncedEntry(id=10),
        'bar.txt': MockedSyncedEntry(id=20),
        'new': MockedPlaceHolderEntry(id=30)
    })
    fs = _fs_factory(local_user_manifest)
    root = await fs.fetch_path('/')
    assert isinstance(root, BaseFolder)
    assert root.keys() == {'foo', 'bar.txt', 'new'}


@trio_test
async def test_fs_fetch_file():
    fs = _fs_factory(LocalUserManifest(children={
        'foo': MockedSyncedEntry(id=10),
        'bar.txt': MockedSyncedEntry(id=20)
    }))
    fs._manifests_manager.a_fetch_manifest.side_effect = [
        LocalFolderManifest(children={
            'bam.txt': MockedPlaceHolderEntry(id=12),
            'bar.txt': MockedSyncedEntry(id=11)
        }),
        LocalFileManifest()
    ]
    file = await fs.fetch_path('/foo/bar.txt')
    assert isinstance(file, BaseFile)
    assert fs._manifests_manager.a_fetch_manifest.call_args_list == [
        ((MockedSyncedEntry(id=10), ), {}),
        ((MockedSyncedEntry(id=11), ), {})
    ]


@trio_test
async def test_fs_fetch_folder():
    fs = _fs_factory(LocalUserManifest(children={
        'foo': MockedPlaceHolderEntry(id=10),
        'bar.txt': MockedSyncedEntry(id=20)
    }))
    fs._manifests_manager.a_fetch_manifest.side_effect = [
        LocalFolderManifest(children={
            'bam.txt': MockedSyncedEntry(id=12),
            'bar': MockedPlaceHolderEntry(id=11)
        }),
        LocalFolderManifest()
    ]
    folder = await fs.fetch_path('/foo/bar')
    assert isinstance(folder, BaseFolder)
    assert fs._manifests_manager.a_fetch_manifest.call_args_list == [
        ((MockedPlaceHolderEntry(id=10), ), {}),
        ((MockedPlaceHolderEntry(id=11), ), {})
    ]


@pytest.mark.parametrize('badpath', ['', 'not/absolute'])
@trio_test
async def test_fs_fetch_invalid_path(badpath):
    fs = _fs_factory()
    with pytest.raises(FSInvalidPath):
        await fs.fetch_path(badpath)


@trio_test
async def test_fs_fetch_unknown_path():
    fs = _fs_factory(LocalUserManifest(children={
        'foo': MockedPlaceHolderEntry(id=10),
        'bar.txt': MockedSyncedEntry(id=20)
    }))
    fs._manifests_manager.a_fetch_manifest.side_effect = [
        LocalFolderManifest(children={
            'bam.txt': MockedSyncedEntry(id=12),
            'bar': MockedPlaceHolderEntry(id=11)
        })
    ]
    with pytest.raises(FSInvalidPath):
        await fs.fetch_path('/foo/bad.txt')


@trio_test
async def test_fs_fetch_unknown_root():
    fs = _fs_factory(LocalUserManifest(children={
        'foo': MockedPlaceHolderEntry(id=10),
        'bar.txt': MockedSyncedEntry(id=20)
    }))
    with pytest.raises(FSInvalidPath):
        await fs.fetch_path('/bad.txt')


def test_fs_create_folder():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest()
    fs._manifests_manager.create_placeholder_folder.return_value = (
        entry, manifest)
    folder = fs.create_folder()
    assert isinstance(folder, BaseFolder)
    assert folder.id == entry.id
    assert folder.created == manifest.created
    assert fs._manifests_manager.create_placeholder_folder.call_args_list == [((), {})]


def test_fs_create_file():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFileManifest()
    fs._manifests_manager.create_placeholder_file.return_value = (
        entry, manifest)
    file = fs.create_file()
    assert isinstance(file, BaseFile)
    assert file.id == entry.id
    assert file.created == manifest.created
    assert fs._manifests_manager.create_placeholder_file.call_args_list == [((), {})]


def test_fs_folder_add_file_child():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest()
    folder = fs._folder_cls(entry, manifest, need_flush=False)

    new_entry = PlaceHolderEntry()
    new_manifest = LocalFileManifest()
    fs._manifests_manager.create_placeholder_file.return_value = (
        new_entry, new_manifest
    )

    new_file = folder.create_file('spam.txt')

    assert isinstance(new_file, BaseFile)
    assert new_file.id == new_entry.id
    assert new_file.created == new_manifest.created
    assert fs._manifests_manager.create_placeholder_file.call_args_list == [((), {})]
    assert set(folder.keys()) == {'spam.txt'}


def test_fs_folder_add_already_exists_file_child():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest(children={
        'bam.txt': MockedSyncedEntry(id=12),
        'bar': MockedPlaceHolderEntry(id=11)
    })
    folder = fs._folder_cls(entry, manifest, need_flush=False)

    with pytest.raises(FSInvalidPath):
        folder.create_file('bam.txt')


def test_fs_folder_add_folder_child():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest()
    folder = fs._folder_cls(entry, manifest, need_flush=False)

    new_entry = PlaceHolderEntry()
    new_manifest = LocalFolderManifest()
    fs._manifests_manager.create_placeholder_folder.return_value = (
        new_entry, new_manifest
    )

    new_file = folder.create_folder('spam')

    assert isinstance(new_file, BaseFolder)
    assert new_file.id == new_entry.id
    assert new_file.created == new_manifest.created
    assert fs._manifests_manager.create_placeholder_folder.call_args_list == [((), {})]
    assert set(folder.keys()) == {'spam'}


def test_fs_folder_add_already_exists_folder_child():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest(children={
        'bam.txt': MockedSyncedEntry(id=12),
        'bar': MockedPlaceHolderEntry(id=11)
    })
    folder = fs._folder_cls(entry, manifest, need_flush=False)

    with pytest.raises(FSInvalidPath):
        folder.create_folder('bar')


@trio_test
async def test_fs_folder_fetch_child():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest(children={
        'bam.txt': MockedSyncedEntry(id=12),
        'bar': MockedPlaceHolderEntry(id=11)
    })
    folder = fs._folder_cls(entry, manifest, need_flush=False)

    fs._manifests_manager.a_fetch_manifest.return_value = LocalFolderManifest()

    res = await folder.fetch_child('bar')
    assert isinstance(res, BaseFolder)
    assert res.id == 11

    assert fs._manifests_manager.a_fetch_manifest.call_args_list == [
        ((MockedPlaceHolderEntry(id=11), ), {})]


@trio_test
async def test_fs_folder_fetch_unknown_child():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest(children={
        'bam.txt': MockedSyncedEntry(id=12),
        'bar': MockedPlaceHolderEntry(id=11)
    })
    folder = fs._folder_cls(entry, manifest, need_flush=False)

    with pytest.raises(FSInvalidPath):
        await folder.fetch_child('foo')


def test_fs_folder_delete_child():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest(children={
        'bam.txt': MockedSyncedEntry(id=12),
        'bar': MockedPlaceHolderEntry(id=11)
    })
    folder = fs._folder_cls(entry, manifest, need_flush=False)

    folder.delete_child('bar')

    assert set(folder.keys()) == {'bam.txt'}


def test_fs_folder_delete_unknown_child():
    fs = _fs_factory()
    entry = PlaceHolderEntry()
    manifest = LocalFolderManifest(children={
        'bam.txt': MockedSyncedEntry(id=12),
        'bar': MockedPlaceHolderEntry(id=11)
    })
    folder = fs._folder_cls(entry, manifest, need_flush=False)

    with pytest.raises(FSInvalidPath):
        folder.delete_child('foo')
