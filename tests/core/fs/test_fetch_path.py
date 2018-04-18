import trio
import pytest
from pendulum import datetime

from parsec.core.fs import *


@pytest.fixture
def populated_fs(fs):
    # /
    # /in_memory/
    # /in_memory/synced
    # /in_memory/placeholder
    # /synced_in_local
    # /placeholder_in_local
    # /in_backend

    fs.root = fs._folder_entry_cls(
        access=fs._user_vlob_access_cls(privkey=b"<user vlob key>"),
        user_id="alice",
        device_name="test",
    )

    in_memory = fs._folder_entry_cls(
        name="in_memory",
        parent=fs.root,
        access=fs._vlob_access_cls(
            id="<in_memory id>",
            rts="<in_memory rts>",
            wts="<in_memory wts>",
            key=b"<in_memory key>",
        ),
        user_id="alice",
        device_name="test",
    )

    in_memory._children = {
        "synced": fs._file_entry_cls(
            name="synced",
            parent=in_memory,
            access=fs._vlob_access_cls(
                id="<synced id>",
                rts="<synced rts>",
                wts="<synced wts>",
                key=b"<synced key>",
            ),
            user_id="alice",
            device_name="test",
        ),
        "placeholder": fs._file_entry_cls(
            name="placeholder",
            parent=in_memory,
            access=fs._placeholder_access_cls(
                id="<placeholder id>", key=b"<placeholder key>"
            ),
            user_id="alice",
            device_name="test",
        ),
    }

    fs.root._children = {
        "synced_in_local": fs._not_loaded_entry_cls(
            name="synced_in_local",
            parent=fs.root,
            access=fs._vlob_access_cls(
                id="<synced_in_local id>",
                rts="<synced_in_local rts>",
                wts="<synced_in_local wts>",
                key=b"<synced_in_local key>",
            ),
        ),
        "placeholder_in_local": fs._not_loaded_entry_cls(
            name="placeholder_in_local",
            parent=fs.root,
            access=fs._placeholder_access_cls(
                id="<placeholder_in_local id>", key=b"<placeholder_in_local key>"
            ),
        ),
        "in_backend": fs._not_loaded_entry_cls(
            name="in_backend",
            parent=fs.root,
            access=fs._vlob_access_cls(
                id="<in_backend id>",
                rts="<in_backend rts>",
                wts="<in_backend wts>",
                key=b"<in_backend key>",
            ),
        ),
        "in_memory": in_memory,
    }
    return fs


@pytest.fixture
def file_cls(fs):
    return fs._file_entry_cls


@pytest.fixture
def folder_cls(fs):
    return fs._folder_entry_cls


@pytest.mark.trio
async def test_get_path(file_cls, folder_cls):
    leaf = file_cls(
        access=None,
        name="bar.txt",
        parent=folder_cls(
            access=None,
            user_id="alice",
            device_name="test",
            name="spam",
            parent=folder_cls(
                access=None,
                user_id="alice",
                device_name="test",
                name="root",
                parent=None,
            ),
        ),
        user_id="alice",
        device_name="test",
    )
    assert leaf.path == "/root/spam/bar.txt"
    leaf._parent._parent = None
    leaf._name = "new.txt"
    assert leaf.path == "/spam/new.txt"


@pytest.mark.trio
async def test_lookup_in_memory(fs):
    populated_fs(fs)
    root = await fs.fetch_path("/")
    assert root is fs.root

    in_memory = await fs.fetch_path("/in_memory/")
    assert isinstance(in_memory, BaseFolderEntry)
    assert in_memory.parent is root

    synced = await fs.fetch_path("/in_memory/synced")
    assert isinstance(synced, BaseFileEntry)
    assert synced.parent is in_memory

    placeholder = await fs.fetch_path("/in_memory/placeholder")
    assert isinstance(placeholder, BaseFileEntry)
    assert placeholder.parent is in_memory

    # No call to manifest manager needed
    fs.manifests_manager.fetch_from_local.assert_not_called()
    fs.manifests_manager.fetch_from_backend.called_with.assert_not_called()


@pytest.mark.trio
async def test_lookup_in_local_storage(fs, mocked_manifests_manager):
    populated_fs(fs)
    fs.manifests_manager.fetch_from_local.return_value = {
        "format": 1,
        "type": "local_file_manifest",
        "user_id": "alice",
        "device_name": "test",
        "base_version": 2,
        "need_sync": True,
        "created": datetime(2017, 1, 1),
        "updated": datetime(2017, 12, 31, 23, 59, 59),
        "size": 0,
        "blocks": [],
        "dirty_blocks": [],
    }
    entry = await fs.fetch_path("/synced_in_local")
    assert fs.manifests_manager.fetch_from_local.called_with(
        "<synced_in_local id>", b"<synced_in_local key>"
    )
    fs.manifests_manager.fetch_from_backend.called_with.assert_not_called()
    assert isinstance(entry, BaseFileEntry)
    assert entry.parent is fs.root
    assert entry.name == "synced_in_local"
    assert entry.size == 0
    assert entry.base_version == 2
    assert entry.need_sync
    assert not entry.need_flush


@pytest.mark.trio
async def test_lookup_in_backend_storage(fs, mocked_manifests_manager):
    populated_fs(fs)
    fs.manifests_manager.fetch_from_local.return_value = None
    fs.manifests_manager.fetch_from_backend.return_value = {
        "format": 1,
        "type": "file_manifest",
        "user_id": "alice",
        "device_name": "test",
        "version": 2,
        "created": datetime(2017, 1, 1),
        "updated": datetime(2017, 12, 31, 23, 59, 59),
        "size": 0,
        "blocks": [],
    }
    entry = await fs.fetch_path("/synced_in_local")
    assert fs.manifests_manager.fetch_from_local.called_with(
        "<synced_in_local id>", b"<synced_in_local key>"
    )
    assert fs.manifests_manager.fetch_from_backend.called_with(
        "<synced_in_local id>", "<synced_in_local rts>", b"<synced_in_local key>"
    )
    assert isinstance(entry, BaseFileEntry)
    assert entry.parent is fs.root
    assert entry.name == "synced_in_local"
    assert entry.size == 0
    assert entry.base_version == 2
    assert not entry.need_sync
    assert not entry.need_flush


@pytest.mark.trio
async def test_concurrent_lookup(populated_fs, mocked_manifests_manager):
    fs = populated_fs
    fs.manifests_manager.fetch_from_local.return_value = None
    fs.manifests_manager.fetch_from_backend.return_value = {
        "format": 1,
        "type": "file_manifest",
        "user_id": "alice",
        "device_name": "test",
        "version": 2,
        "created": datetime(2017, 1, 1),
        "updated": datetime(2017, 12, 31, 23, 59, 59),
        "size": 0,
        "blocks": [],
    }

    async def lookup():
        await fs.fetch_path("/synced_in_local")
        await fs.fetch_path("/synced_in_local")

    async with trio.open_nursery() as nursery:
        for _ in range(3):
            nursery.start_soon(lookup)

    # Only the first access need to load the entry
    assert len(fs.manifests_manager.fetch_from_local.call_args_list) == 1
    assert len(fs.manifests_manager.fetch_from_backend.call_args_list) == 1
