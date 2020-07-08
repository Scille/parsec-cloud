# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from unittest import mock
from tests.common import AsyncMock
from parsec.core.cli import rsync
from parsec.crypto import HashDigest
from parsec.core.types import FsPath
from parsec.api.data.entry import EntryID


class ReadSideEffect:
    def __init__(self, data=""):
        self._data = data

    def __call__(self, length=0):  # That make my_read_side_effect a callable
        if not self._data:
            return ""
        if not length:
            length = len(self._data)
        r, self._data = self._data[:length], self._data[length:]
        return r


@pytest.fixture
@pytest.mark.trio
async def alice_workspace(alice_user_fs, running_backend):
    wid = await alice_user_fs.workspace_create("w")
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.mkdir("/foo")
    await workspace.touch("/foo/bar")
    await workspace.sync()
    return workspace


@pytest.mark.trio
async def test_import_file(alice_workspace):
    rsync._chunks_from_path = mock.Mock(return_value=[b"random", b"chunks"])
    f = await alice_workspace.open_file("/foo/bar")
    assert await f.read() == b""
    await rsync._import_file(alice_workspace, "/src_file", "/foo/bar")
    rsync._chunks_from_path.assert_called_once_with("/src_file")
    assert await f.read() == b"randomchunks"
    await f.close()


def test_chunks_from_path():
    with mock.patch("builtins.open", new_callable=mock.mock_open) as mo:
        mock_file = mo.return_value
        mock_file.read.side_effect = ReadSideEffect("chunk")

        res = rsync._chunks_from_path("src_file", 1)
        mo.assert_called_once_with("src_file", "rb")
        assert res == ["c", "h", "u", "n", "k"]

        mo.reset_mock()
        mock_file.read.side_effect = ReadSideEffect("chunk")

        res = rsync._chunks_from_path("src_file", 2)
        mo.assert_called_once_with("src_file", "rb")
        assert res == ["ch", "un", "k"]


@pytest.mark.trio
async def test_update_file(alice_workspace):
    block_mock1 = mock.Mock()
    block_mock1.digest = "block1"
    block_mock2 = mock.Mock()
    block_mock2.digest = "block2"

    manifest_mock = mock.Mock()
    manifest_mock.blocks = [block_mock1, block_mock2]

    load_manifest_mock = AsyncMock(spec=mock.Mock, side_effect=lambda x: manifest_mock)
    alice_workspace.remote_loader.load_manifest = load_manifest_mock

    write_bytes_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.write_bytes = write_bytes_mock

    sync_by_id_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.sync_by_id = sync_by_id_mock

    HashDigest.from_data = mock.Mock(side_effect=lambda x: x)
    rsync._chunks_from_path = mock.Mock(return_value=["block1", "block2"])

    entry_id = EntryID()
    await rsync._update_file(
        alice_workspace, entry_id, FsPath("/src_file"), FsPath("/parent/src_file")
    )
    rsync._chunks_from_path.assert_called_once_with(FsPath("/src_file"))
    load_manifest_mock.assert_called_once_with(entry_id)
    write_bytes_mock.assert_not_called()
    sync_by_id_mock.assert_called_once_with(entry_id, remote_changed=False, recursive=False)

    rsync._chunks_from_path = mock.Mock(return_value=["block1", "block3"])
    load_manifest_mock.reset_mock()
    sync_by_id_mock.reset_mock()

    await rsync._update_file(
        alice_workspace, entry_id, FsPath("/src_file"), FsPath("/parent/src_file")
    )
    rsync._chunks_from_path.assert_called_once_with(FsPath("/src_file"))
    load_manifest_mock.assert_called_once_with(entry_id)
    write_bytes_mock.assert_called_once_with(FsPath("/parent/src_file"), "block3", len("block1"))
    sync_by_id_mock.assert_called_once_with(entry_id, remote_changed=False, recursive=False)

    rsync._chunks_from_path = mock.Mock(return_value=["block3", "block4"])
    load_manifest_mock.reset_mock()
    sync_by_id_mock.reset_mock()
    write_bytes_mock.reset_mock()

    await rsync._update_file(
        alice_workspace, entry_id, FsPath("/src_file"), FsPath("/parent/src_file")
    )
    rsync._chunks_from_path.assert_called_once_with(FsPath("/src_file"))
    alice_workspace.remote_loader.load_manifest.assert_called_once_with(entry_id)
    write_bytes_mock.assert_has_calls(
        [
            mock.call(FsPath("/parent/src_file"), "block3", 0),
            mock.call(FsPath("/parent/src_file"), "block4", len("block3")),
        ]
    )
    sync_by_id_mock.assert_called_once_with(entry_id, remote_changed=False, recursive=False)


@pytest.mark.trio
async def test_create_path(alice_workspace):
    mkdir_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.mkdir = mkdir_mock

    sync_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.sync = sync_mock

    path_info_mock = AsyncMock(spec=mock.Mock, side_effect=lambda x: {"id": "mock_id"})
    alice_workspace.path_info = path_info_mock

    get_manifest_mock = AsyncMock(spec=mock.Mock, side_effect=lambda x: "mock_manifest")
    alice_workspace.local_storage.get_manifest = get_manifest_mock

    import_file_mock = AsyncMock(spec=mock.Mock)
    rsync._import_file = import_file_mock

    is_dir = True
    res = await rsync._create_path(alice_workspace, is_dir, FsPath("/test"), FsPath("/parent/test"))
    mkdir_mock.assert_called_once_with(FsPath("/parent/test"))
    sync_mock.assert_called_once_with()
    path_info_mock.assert_called_once_with(FsPath("/parent/test"))
    get_manifest_mock.assert_called_once_with("mock_id")
    import_file_mock.assert_not_called()
    assert res == "mock_manifest"

    mkdir_mock.reset_mock()
    sync_mock.reset_mock()
    path_info_mock.reset_mock()
    get_manifest_mock.reset_mock()
    import_file_mock.reset_mock()

    is_dir = False
    res = await rsync._create_path(alice_workspace, is_dir, FsPath("/test"), FsPath("/parent/test"))
    mkdir_mock.assert_not_called()
    path_info_mock.assert_not_called()
    get_manifest_mock.assert_not_called()
    import_file_mock.assert_called_once_with(
        alice_workspace, FsPath("/test"), FsPath("/parent/test")
    )
    sync_mock.assert_called_once_with()
    assert res is None


@pytest.mark.trio
async def test_clear_path(alice_workspace):
    is_dir_mock = AsyncMock(spec=mock.Mock, side_effect=lambda x: True)
    alice_workspace.is_dir = is_dir_mock

    rmtree_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.rmtree = rmtree_mock

    unlink_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.unlink = unlink_mock

    sync_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.sync = sync_mock

    path = FsPath("/test/toto")

    await rsync._clear_path(alice_workspace, path)
    is_dir_mock.assert_called_once_with(path)
    rmtree_mock.assert_called_once_with(path)
    unlink_mock.assert_not_called()
    sync_mock.assert_called_once_with()

    alice_workspace.is_dir.side_effect = lambda x: False
    is_dir_mock.reset_mock()
    rmtree_mock.reset_mock()
    sync_mock.reset_mock()

    await rsync._clear_path(alice_workspace, path)
    is_dir_mock.assert_called_once_with(path)
    rmtree_mock.assert_not_called()
    unlink_mock.assert_called_once_with(path)
    sync_mock.assert_called_once_with()


@pytest.mark.trio
async def test_clear_directory(alice_workspace):
    local_item1 = mock.Mock()
    local_item1.name = "item1"
    local_item2 = mock.Mock()
    local_item2.name = "item2"

    path = trio.Path("/test_directory")
    path.iterdir = AsyncMock(spec=mock.Mock, side_effect=lambda: [local_item1, local_item2])

    folder_manifest = mock.Mock()
    folder_manifest.children = {"item1": "id1", "item2": "id2", "item3": "id3"}

    clear_path_mock = AsyncMock(spec=mock.Mock())
    rsync._clear_path = clear_path_mock

    await rsync._clear_directory(
        trio.Path("/parent/test_directory"), path, alice_workspace, folder_manifest
    )
    clear_path_mock.assert_called_once_with(alice_workspace, FsPath("/parent/test_directory/item3"))

    clear_path_mock.reset_mock()
    folder_manifest.children["item4"] = "id4"

    await rsync._clear_directory(
        trio.Path("/parent/test_directory"), path, alice_workspace, folder_manifest
    )
    clear_path_mock.assert_has_calls(
        [
            mock.call(alice_workspace, FsPath("/parent/test_directory/item3")),
            mock.call(alice_workspace, FsPath("/parent/test_directory/item4")),
        ]
    )

    clear_path_mock.reset_mock()
    del folder_manifest.children["item3"]
    del folder_manifest.children["item4"]

    await rsync._clear_directory(
        trio.Path("/parent/test_directory"), path, alice_workspace, folder_manifest
    )
    clear_path_mock.assert_not_called()


@pytest.mark.trio
async def test_get_or_create_directory(alice_workspace):

    load_manifest_mock = AsyncMock(spec=mock.Mock(), side_effect=lambda x: "load_manifest_mock")
    alice_workspace.remote_loader.load_manifest = load_manifest_mock

    _create_path_mock = AsyncMock(spec=mock.Mock(), side_effect=lambda *x: "_create_path_mock")
    rsync._create_path = _create_path_mock

    entry_id = EntryID()
    res = await rsync._get_or_create_directory(
        entry_id, alice_workspace, FsPath("/test_directory"), FsPath("/parent")
    )
    load_manifest_mock.assert_called_once_with(entry_id)
    _create_path_mock.assert_not_called()
    assert res == "load_manifest_mock"

    load_manifest_mock.reset_mock()
    res = await rsync._get_or_create_directory(
        None, alice_workspace, FsPath("/test_directory"), FsPath("/parent")
    )
    load_manifest_mock.assert_not_called()
    _create_path_mock.assert_called_once_with(
        alice_workspace, True, FsPath("/test_directory"), FsPath("/parent")
    )
    assert res == "_create_path_mock"


@pytest.mark.trio
async def test_upsert_file(alice_workspace):

    _update_file_mock = AsyncMock(spec=mock.Mock())
    rsync._update_file = _update_file_mock

    _create_path_mock = AsyncMock(spec=mock.Mock())
    rsync._create_path = _create_path_mock

    path = FsPath("/test")
    absolute_path = FsPath("/parent/test")
    entry_id = EntryID()

    await rsync._upsert_file(entry_id, alice_workspace, path, absolute_path)
    _update_file_mock.assert_called_once_with(alice_workspace, entry_id, path, absolute_path)
    _create_path_mock.assert_not_called()

    _update_file_mock.reset_mock()
    entry_id = None

    await rsync._upsert_file(None, alice_workspace, path, absolute_path)
    _update_file_mock.assert_not_called()
    _create_path_mock.assert_called_once_with(alice_workspace, False, path, absolute_path)


@pytest.mark.trio
async def test_sync_directory(alice_workspace):

    _get_or_create_directory_mock = AsyncMock(
        spec=mock.Mock(), side_effect=lambda *x: "folder_manifest_mock"
    )
    rsync._get_or_create_directory = _get_or_create_directory_mock

    _sync_directory_content_mock = AsyncMock(spec=mock.Mock())
    rsync._sync_directory_content = _sync_directory_content_mock

    _clear_directory_mock = AsyncMock(spec=mock.Mock())
    rsync._clear_directory = _clear_directory_mock

    entry_id = EntryID()
    path = FsPath("/test")
    absolute_path = FsPath("/parent/test")

    await rsync._sync_directory(entry_id, alice_workspace, path, absolute_path)
    _get_or_create_directory_mock.assert_called_once_with(
        entry_id, alice_workspace, path, absolute_path
    )
    _sync_directory_content_mock.assert_called_once_with(
        absolute_path, path, alice_workspace, "folder_manifest_mock"
    )
    _clear_directory_mock.assert_called_once_with(
        absolute_path, path, alice_workspace, "folder_manifest_mock"
    )

    _get_or_create_directory_mock.reset_mock()
    _sync_directory_content_mock.reset_mock()
    _clear_directory_mock.reset_mock()

    await rsync._sync_directory(None, alice_workspace, path, absolute_path)
    _get_or_create_directory_mock.assert_called_once_with(
        None, alice_workspace, path, absolute_path
    )
    _sync_directory_content_mock.assert_called_once_with(
        absolute_path, path, alice_workspace, "folder_manifest_mock"
    )
    _clear_directory_mock.assert_not_called()


@pytest.mark.trio
async def test_sync_directory_content(alice_workspace):

    path_dir1 = trio.Path("/test_dir1")
    path_dir1.is_dir = AsyncMock(spec=mock.Mock(), side_effect=lambda: True)
    path_dir2 = trio.Path("/test_dir2")
    path_dir2.is_dir = AsyncMock(spec=mock.Mock(), side_effect=lambda: True)

    parent = trio.Path("/parent")
    source = trio.Path("/test")
    source.iterdir = AsyncMock(spec=mock.Mock(), side_effect=lambda: [path_dir1, path_dir2])

    mock_manifest = mock.Mock()
    mock_manifest.children = {"test_dir1": "id1"}

    _sync_directory_mock = AsyncMock(spec=mock.Mock())
    rsync._sync_directory = _sync_directory_mock

    _upsert_file_mock = AsyncMock(spec=mock.Mock())
    rsync._upsert_file = _upsert_file_mock

    await rsync._sync_directory_content(parent, source, alice_workspace, mock_manifest)
    _sync_directory_mock.assert_has_calls(
        [
            mock.call(
                "id1", alice_workspace, trio.Path("/test_dir1"), trio.Path("/parent/test_dir1")
            ),
            mock.call(
                None, alice_workspace, trio.Path("/test_dir2"), trio.Path("/parent/test_dir2")
            ),
        ]
    )
    _upsert_file_mock.assert_not_called()

    _sync_directory_mock.reset_mock()
    path_dir1.is_dir.side_effect = lambda: False
    path_dir2.is_dir.side_effect = lambda: False

    await rsync._sync_directory_content(parent, source, alice_workspace, mock_manifest)
    _upsert_file_mock.assert_has_calls(
        [
            mock.call(
                "id1", alice_workspace, trio.Path("/test_dir1"), trio.Path("/parent/test_dir1")
            ),
            mock.call(
                None, alice_workspace, trio.Path("/test_dir2"), trio.Path("/parent/test_dir2")
            ),
        ]
    )
    _sync_directory_mock.assert_not_called()

    _upsert_file_mock.reset_mock()
    path_dir1.is_dir.side_effect = lambda: True
    path_dir2.is_dir.side_effect = lambda: False

    await rsync._sync_directory_content(parent, source, alice_workspace, mock_manifest)
    _sync_directory_mock.assert_called_once_with(
        "id1", alice_workspace, trio.Path("/test_dir1"), trio.Path("/parent/test_dir1")
    )
    _upsert_file_mock.assert_called_once_with(
        None, alice_workspace, trio.Path("/test_dir2"), trio.Path("/parent/test_dir2")
    )


def test_parse_destination():
    mock_workspace1 = mock.Mock()
    mock_workspace1.name = "workspace1"
    mock_workspace2 = mock.Mock()
    mock_workspace2.name = "workspace2"

    workspaces_mock = mock.Mock()
    workspaces_mock.workspaces = [mock_workspace1, mock_workspace2]
    get_user_manifest_mock = mock.Mock(return_value=workspaces_mock)

    alice_core = mock.Mock()
    alice_core.user_fs.get_user_manifest = get_user_manifest_mock

    workspace, path = rsync._parse_destination(alice_core, "workspace1")
    assert workspace == mock_workspace1
    assert path is None

    workspace, path = rsync._parse_destination(alice_core, "workspace1:/test/save")
    assert workspace == mock_workspace1
    assert path == trio.Path("/test/save")

    workspace, path = rsync._parse_destination(alice_core, "workspace1")
    assert workspace == mock_workspace1
    assert path is None

    workspace, path = rsync._parse_destination(alice_core, "workspace2:/test/save2")
    assert workspace == mock_workspace2
    assert path == trio.Path("/test/save2")

    with pytest.raises(SystemExit):
        workspace, path = rsync._parse_destination(alice_core, "unknown_workspace")

    with pytest.raises(SystemExit):
        workspace, path = rsync._parse_destination(alice_core, "unknown_workspace:/test/save3")


@pytest.mark.trio
async def test_root_manifest_parent(alice_workspace):
    workspace_manifest = mock.Mock()

    _get_or_create_directory_mock = AsyncMock(spec=mock.Mock)
    rsync._get_or_create_directory = _get_or_create_directory_mock
    root_manifest, parent = await rsync._root_manifest_parent(
        None, alice_workspace, workspace_manifest
    )
    _get_or_create_directory_mock.assert_not_called()
    assert root_manifest == workspace_manifest
    assert parent == trio.Path("/")

    workspace_manifest.children = {"test": "id1"}
    workspace_test_save_manifest = mock.Mock()
    workspace_test_save_manifest.children = {}
    workspace_test_manifest = mock.Mock()
    _get_or_create_directory_mock.side_effect = [
        workspace_test_manifest,
        workspace_test_save_manifest,
    ]

    root_manifest, parent = await rsync._root_manifest_parent(
        trio.Path("test"), alice_workspace, workspace_manifest
    )
    assert root_manifest == workspace_test_manifest
    assert parent == trio.Path("/test")

    _get_or_create_directory_mock.side_effect = [
        workspace_test_manifest,
        workspace_test_save_manifest,
    ]
    root_manifest, parent = await rsync._root_manifest_parent(
        trio.Path("test/save"), alice_workspace, workspace_manifest
    )
    assert root_manifest == workspace_test_save_manifest
    assert parent == trio.Path("/test/save")
