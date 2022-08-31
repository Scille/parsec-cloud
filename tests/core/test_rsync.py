# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import trio
from unittest import mock
from parsec.api.data.manifest import WorkspaceManifest

from parsec.crypto import HashDigest
from parsec.api.data import EntryID, FolderManifest, EntryName
from parsec.core.cli import rsync
from parsec.core.fs import FsPath, UserFS, WorkspaceFS

from tests.common import AsyncMock


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
async def alice_workspace(alice_user_fs: UserFS, running_backend) -> WorkspaceFS:
    wid = await alice_user_fs.workspace_create(EntryName("w"))
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.mkdir("/foo")
    await workspace.touch("/foo/bar")
    await workspace.sync()
    return workspace


@pytest.mark.trio
async def test_import_file(alice_workspace: WorkspaceFS):
    with mock.patch(
        "parsec.core.cli.rsync._chunks_from_path",
        AsyncMock(spec=mock.Mock, side_effect=[[b"random", b"chunks"]]),
    ):
        f = await alice_workspace.open_file("/foo/bar", "wb+")
        assert await f.read() == b""
        await rsync._import_file(alice_workspace, "/src_file", "/foo/bar")
        rsync._chunks_from_path.assert_called_once_with("/src_file")
        assert await f.read() == b"randomchunks"
        await f.close()


@pytest.mark.trio
async def test_chunks_from_path():
    test = AsyncMock(spec=mock.Mock)

    with mock.patch("trio.open_file", AsyncMock(spec=mock.Mock, side_effect=[test])) as mo:

        test.read = AsyncMock(spec=mock.Mock, side_effect="chunk")
        res = await rsync._chunks_from_path("src_file", 1)
        mo.assert_called_once_with("src_file", "rb")
        test.read.assert_has_calls(
            [mock.call(1), mock.call(1), mock.call(1), mock.call(1), mock.call(1), mock.call(1)]
        )
        assert res == ["c", "h", "u", "n", "k"]
    test.reset_mock()

    with mock.patch("trio.open_file", AsyncMock(spec=mock.Mock, side_effect=[test])) as mo:

        test.read = AsyncMock(spec=mock.Mock, side_effect=["ch", "un", "k"])
        res = await rsync._chunks_from_path("src_file", 2)
        mo.assert_called_once_with("src_file", "rb")
        test.read.assert_has_calls([mock.call(2), mock.call(2), mock.call(2), mock.call(2)])
        assert res == ["ch", "un", "k"]


@pytest.mark.trio
async def test_update_file(alice_workspace: WorkspaceFS, monkeypatch):
    block_mock1 = mock.Mock()
    block_mock1.digest = b"block1"
    block_mock2 = mock.Mock()
    block_mock2.digest = b"block2"

    manifest_mock = mock.Mock(spec=FolderManifest)
    manifest_mock.blocks = [block_mock1, block_mock2]

    load_manifest_mock = AsyncMock(spec=mock.Mock, side_effect=lambda x: manifest_mock)
    alice_workspace.remote_loader.load_manifest = load_manifest_mock

    write_bytes_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.write_bytes = write_bytes_mock

    sync_by_id_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.sync_by_id = sync_by_id_mock
    monkeypatch.setattr(HashDigest, "from_data", mock.Mock(side_effect=lambda x: x))

    with mock.patch(
        "parsec.core.cli.rsync._chunks_from_path",
        AsyncMock(spec=mock.Mock, side_effect=[[b"block1", b"block2"]]),
    ):
        entry_id = EntryID.new()
        await rsync._update_file(
            alice_workspace, entry_id, FsPath("/src_file"), FsPath("/path_in_workspace")
        )
        rsync._chunks_from_path.assert_called_once_with(FsPath("/src_file"))
        load_manifest_mock.assert_called_once_with(entry_id)
        write_bytes_mock.assert_not_called()
        sync_by_id_mock.assert_called_once_with(entry_id, remote_changed=False, recursive=False)

    load_manifest_mock.reset_mock()
    sync_by_id_mock.reset_mock()

    with mock.patch(
        "parsec.core.cli.rsync._chunks_from_path",
        AsyncMock(spec=mock.Mock, side_effect=[[b"block1", b"block3"]]),
    ):

        await rsync._update_file(
            alice_workspace, entry_id, FsPath("/src_file"), FsPath("/path_in_workspace")
        )
        rsync._chunks_from_path.assert_called_once_with(FsPath("/src_file"))
        load_manifest_mock.assert_called_once_with(entry_id)
        write_bytes_mock.assert_called_once_with(
            FsPath("/path_in_workspace"), b"block3", len("block1")
        )
        sync_by_id_mock.assert_called_once_with(entry_id, remote_changed=False, recursive=False)

    load_manifest_mock.reset_mock()
    sync_by_id_mock.reset_mock()
    write_bytes_mock.reset_mock()

    with mock.patch(
        "parsec.core.cli.rsync._chunks_from_path",
        AsyncMock(spec=mock.Mock, side_effect=[[b"block3", b"block4"]]),
    ):
        await rsync._update_file(
            alice_workspace, entry_id, FsPath("/src_file"), FsPath("/path_in_workspace")
        )
        rsync._chunks_from_path.assert_called_once_with(FsPath("/src_file"))
        alice_workspace.remote_loader.load_manifest.assert_called_once_with(entry_id)
        write_bytes_mock.assert_has_calls(
            [
                mock.call(FsPath("/path_in_workspace"), b"block3", 0),
                mock.call(FsPath("/path_in_workspace"), b"block4", len("block3")),
            ]
        )
        sync_by_id_mock.assert_called_once_with(entry_id, remote_changed=False, recursive=False)


@pytest.mark.trio
async def test_create_path(alice_workspace: WorkspaceFS):
    mkdir_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.mkdir = mkdir_mock

    sync_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.sync = sync_mock

    path_info_mock = AsyncMock(spec=mock.Mock, side_effect=lambda x: {"id": "mock_id"})
    alice_workspace.path_info = path_info_mock

    manifest_mock = mock.Mock(spec=FolderManifest)
    get_manifest_mock = AsyncMock(spec=mock.Mock, side_effect=lambda x: manifest_mock)
    alice_workspace.local_storage.get_manifest = get_manifest_mock

    import_file_mock = AsyncMock(spec=mock.Mock)

    with mock.patch("parsec.core.cli.rsync._import_file", import_file_mock):
        is_dir = True
        res = await rsync._create_path(
            alice_workspace, is_dir, FsPath("/test"), FsPath("/path_in_workspace/test")
        )
        mkdir_mock.assert_called_once_with(FsPath("/path_in_workspace/test"))
        sync_mock.assert_called_once_with()
        path_info_mock.assert_called_once_with(FsPath("/path_in_workspace/test"))
        get_manifest_mock.assert_called_once_with("mock_id")
        import_file_mock.assert_not_called()
        assert res is manifest_mock

    mkdir_mock.reset_mock()
    sync_mock.reset_mock()
    path_info_mock.reset_mock()
    get_manifest_mock.reset_mock()
    import_file_mock.reset_mock()

    with mock.patch("parsec.core.cli.rsync._import_file", import_file_mock):
        is_dir = False
        res = await rsync._create_path(
            alice_workspace, is_dir, FsPath("/test"), FsPath("/path_in_workspace/test")
        )
        mkdir_mock.assert_not_called()
        path_info_mock.assert_not_called()
        get_manifest_mock.assert_not_called()
        import_file_mock.assert_called_once_with(
            alice_workspace, FsPath("/test"), FsPath("/path_in_workspace/test")
        )
        sync_mock.assert_called_once_with()
        assert res is None


@pytest.mark.trio
async def test_clear_path(alice_workspace: UserFS):
    is_dir_mock = AsyncMock(spec=mock.Mock, side_effect=lambda x: True)
    alice_workspace.is_dir = is_dir_mock

    rmtree_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.rmtree = rmtree_mock

    unlink_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.unlink = unlink_mock

    sync_mock = AsyncMock(spec=mock.Mock)
    alice_workspace.sync = sync_mock

    path = FsPath("/path_in_workspace/test")

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
async def test_clear_directory(alice_workspace: UserFS):
    local_item1 = mock.Mock()
    local_item1.name = "item1"
    local_item2 = mock.Mock()
    local_item2.name = "item2"

    path = trio.Path("/test_directory")
    path.iterdir = AsyncMock(spec=mock.Mock, side_effect=lambda: [local_item1, local_item2])

    folder_manifest = mock.Mock()
    folder_manifest.children = {"item1": "id1", "item2": "id2", "item3": "id3"}

    clear_path_mock = AsyncMock(spec=mock.Mock())
    with mock.patch("parsec.core.cli.rsync._clear_path", clear_path_mock):
        await rsync._clear_directory(
            FsPath("/path_in_workspace"), path, alice_workspace, folder_manifest
        )
        clear_path_mock.assert_called_once_with(alice_workspace, FsPath("/path_in_workspace/item3"))

    clear_path_mock.reset_mock()
    folder_manifest.children["item4"] = "id4"
    with mock.patch("parsec.core.cli.rsync._clear_path", clear_path_mock):
        await rsync._clear_directory(
            FsPath("/path_in_workspace"), path, alice_workspace, folder_manifest
        )
        clear_path_mock.assert_has_calls(
            [
                mock.call(alice_workspace, FsPath("/path_in_workspace/item3")),
                mock.call(alice_workspace, FsPath("/path_in_workspace/item4")),
            ]
        )

    clear_path_mock.reset_mock()
    del folder_manifest.children["item3"]
    del folder_manifest.children["item4"]

    with mock.patch("parsec.core.cli.rsync._clear_path", clear_path_mock):
        await rsync._clear_directory(
            FsPath("/path_in_workspace"), path, alice_workspace, folder_manifest
        )
        clear_path_mock.assert_not_called()


@pytest.mark.trio
async def test_get_or_create_directory(alice_workspace: UserFS):
    manifest1 = mock.Mock(spec=FolderManifest)
    manifest2 = mock.Mock(spec=FolderManifest)

    load_manifest_mock = AsyncMock(spec=mock.Mock(), side_effect=lambda x: manifest1)
    alice_workspace.remote_loader.load_manifest = load_manifest_mock

    _create_path_mock = AsyncMock(spec=mock.Mock(), side_effect=lambda *x: manifest2)
    with mock.patch("parsec.core.cli.rsync._create_path", _create_path_mock):
        entry_id = EntryID.new()
        res = await rsync._get_or_create_directory(
            entry_id, alice_workspace, FsPath("/test_directory"), FsPath("/path_in_workspace")
        )
        load_manifest_mock.assert_called_once_with(entry_id)
        _create_path_mock.assert_not_called()
        assert res is manifest1

    load_manifest_mock.reset_mock()

    with mock.patch("parsec.core.cli.rsync._create_path", _create_path_mock):
        res = await rsync._get_or_create_directory(
            None, alice_workspace, FsPath("/test_directory"), FsPath("/path_in_workspace")
        )
        load_manifest_mock.assert_not_called()
        _create_path_mock.assert_called_once_with(
            alice_workspace, True, FsPath("/test_directory"), FsPath("/path_in_workspace")
        )
        assert res is manifest2


@pytest.mark.trio
async def test_upsert_file(alice_workspace: UserFS):

    _update_file_mock = AsyncMock(spec=mock.Mock())
    _create_path_mock = AsyncMock(spec=mock.Mock())

    path = FsPath("/test")
    workspace_path = FsPath("/path_in_workspace")
    entry_id = EntryID.new()

    with mock.patch("parsec.core.cli.rsync._create_path", _create_path_mock):
        with mock.patch("parsec.core.cli.rsync._update_file", _update_file_mock):
            await rsync._upsert_file(entry_id, alice_workspace, path, workspace_path)
            _update_file_mock.assert_called_once_with(
                alice_workspace, entry_id, path, workspace_path
            )
            _create_path_mock.assert_not_called()

    _update_file_mock.reset_mock()
    entry_id = None

    with mock.patch("parsec.core.cli.rsync._create_path", _create_path_mock):
        with mock.patch("parsec.core.cli.rsync._update_file", _update_file_mock):
            await rsync._upsert_file(None, alice_workspace, path, workspace_path)
            _update_file_mock.assert_not_called()
            _create_path_mock.assert_called_once_with(alice_workspace, False, path, workspace_path)


@pytest.mark.trio
async def test_sync_directory(alice_workspace: UserFS):

    _get_or_create_directory_mock = AsyncMock(
        spec=mock.Mock(), side_effect=lambda *x: "folder_manifest_mock"
    )
    _sync_directory_content_mock = AsyncMock(spec=mock.Mock())
    _clear_directory_mock = AsyncMock(spec=mock.Mock())

    entry_id = EntryID.new()
    path = FsPath("/test")
    workspace_path = FsPath("/path_in_workspace")

    with mock.patch(
        "parsec.core.cli.rsync._get_or_create_directory", _get_or_create_directory_mock
    ):
        with mock.patch(
            "parsec.core.cli.rsync._sync_directory_content", _sync_directory_content_mock
        ):
            with mock.patch("parsec.core.cli.rsync._clear_directory", _clear_directory_mock):
                await rsync._sync_directory(entry_id, alice_workspace, path, workspace_path)
                _get_or_create_directory_mock.assert_called_once_with(
                    entry_id, alice_workspace, path, workspace_path
                )
                _sync_directory_content_mock.assert_called_once_with(
                    workspace_path, path, alice_workspace, "folder_manifest_mock"
                )
                _clear_directory_mock.assert_called_once_with(
                    workspace_path, path, alice_workspace, "folder_manifest_mock"
                )

    _get_or_create_directory_mock.reset_mock()
    _sync_directory_content_mock.reset_mock()
    _clear_directory_mock.reset_mock()

    with mock.patch(
        "parsec.core.cli.rsync._get_or_create_directory", _get_or_create_directory_mock
    ):
        with mock.patch(
            "parsec.core.cli.rsync._sync_directory_content", _sync_directory_content_mock
        ):
            with mock.patch("parsec.core.cli.rsync._clear_directory", _clear_directory_mock):
                await rsync._sync_directory(None, alice_workspace, path, workspace_path)
                _get_or_create_directory_mock.assert_called_once_with(
                    None, alice_workspace, path, workspace_path
                )
                _sync_directory_content_mock.assert_called_once_with(
                    workspace_path, path, alice_workspace, "folder_manifest_mock"
                )
                _clear_directory_mock.assert_not_called()


@pytest.mark.trio
async def test_sync_directory_content(alice_workspace: UserFS):

    path_dir1 = trio.Path("/test_dir1")
    path_dir1.is_dir = AsyncMock(spec=mock.Mock(), side_effect=lambda: True)
    path_dir2 = trio.Path("/test_dir2")
    path_dir2.is_dir = AsyncMock(spec=mock.Mock(), side_effect=lambda: True)

    workspace_path = FsPath("/path_in_workspace")
    source = trio.Path("/test")
    source.iterdir = AsyncMock(spec=mock.Mock(), side_effect=lambda: [path_dir1, path_dir2])

    mock_manifest = mock.Mock()
    mock_manifest.children = {"test_dir1": "id1"}

    _sync_directory_mock = AsyncMock(spec=mock.Mock())
    _upsert_file_mock = AsyncMock(spec=mock.Mock())

    with mock.patch("parsec.core.cli.rsync._sync_directory", _sync_directory_mock):
        with mock.patch("parsec.core.cli.rsync._upsert_file", _upsert_file_mock):
            await rsync._sync_directory_content(
                workspace_path, source, alice_workspace, mock_manifest
            )
            _sync_directory_mock.assert_has_calls(
                [
                    mock.call(
                        "id1",
                        alice_workspace,
                        trio.Path("/test_dir1"),
                        FsPath("/path_in_workspace/test_dir1"),
                    ),
                    mock.call(
                        None,
                        alice_workspace,
                        trio.Path("/test_dir2"),
                        FsPath("/path_in_workspace/test_dir2"),
                    ),
                ]
            )
            _upsert_file_mock.assert_not_called()

    _sync_directory_mock.reset_mock()
    path_dir1.is_dir.side_effect = lambda: False
    path_dir2.is_dir.side_effect = lambda: False

    with mock.patch("parsec.core.cli.rsync._sync_directory", _sync_directory_mock):
        with mock.patch("parsec.core.cli.rsync._upsert_file", _upsert_file_mock):
            await rsync._sync_directory_content(
                workspace_path, source, alice_workspace, mock_manifest
            )
            _upsert_file_mock.assert_has_calls(
                [
                    mock.call(
                        "id1",
                        alice_workspace,
                        trio.Path("/test_dir1"),
                        FsPath("/path_in_workspace/test_dir1"),
                    ),
                    mock.call(
                        None,
                        alice_workspace,
                        trio.Path("/test_dir2"),
                        FsPath("/path_in_workspace/test_dir2"),
                    ),
                ]
            )
            _sync_directory_mock.assert_not_called()

    _upsert_file_mock.reset_mock()
    path_dir1.is_dir.side_effect = lambda: True
    path_dir2.is_dir.side_effect = lambda: False

    with mock.patch("parsec.core.cli.rsync._sync_directory", _sync_directory_mock):
        with mock.patch("parsec.core.cli.rsync._upsert_file", _upsert_file_mock):
            await rsync._sync_directory_content(
                workspace_path, source, alice_workspace, mock_manifest
            )
            _sync_directory_mock.assert_called_once_with(
                "id1",
                alice_workspace,
                trio.Path("/test_dir1"),
                FsPath("/path_in_workspace/test_dir1"),
            )
            _upsert_file_mock.assert_called_once_with(
                None,
                alice_workspace,
                trio.Path("/test_dir2"),
                FsPath("/path_in_workspace/test_dir2"),
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
    assert path == FsPath("/test/save")

    workspace, path = rsync._parse_destination(alice_core, "workspace1")
    assert workspace == mock_workspace1
    assert path is None

    workspace, path = rsync._parse_destination(alice_core, "workspace2:/test/save2")
    assert workspace == mock_workspace2
    assert path == FsPath("/test/save2")

    with pytest.raises(SystemExit):
        workspace, path = rsync._parse_destination(alice_core, "unknown_workspace")

    with pytest.raises(SystemExit):
        workspace, path = rsync._parse_destination(alice_core, "unknown_workspace:/test/save3")


@pytest.mark.trio
async def test_root_manifest_parent(alice_workspace: UserFS):
    workspace_manifest = mock.Mock(spec=WorkspaceManifest)

    _get_or_create_directory_mock = AsyncMock(spec=mock.Mock)
    with mock.patch(
        "parsec.core.cli.rsync._get_or_create_directory", _get_or_create_directory_mock
    ):

        root_manifest, parent = await rsync._root_manifest_parent(
            None, alice_workspace, workspace_manifest
        )
        _get_or_create_directory_mock.assert_not_called()
        assert root_manifest is workspace_manifest
        assert parent == FsPath("/")

    workspace_manifest.children = {"test": "id1"}
    workspace_test_save_manifest = mock.Mock(spec=FolderManifest)
    workspace_test_save_manifest.children = {}
    workspace_test_manifest = mock.Mock(spec=FolderManifest)
    _get_or_create_directory_mock.side_effect = [
        workspace_test_manifest,
        workspace_test_save_manifest,
    ]

    with mock.patch(
        "parsec.core.cli.rsync._get_or_create_directory", _get_or_create_directory_mock
    ):
        root_manifest, parent = await rsync._root_manifest_parent(
            FsPath("/path_in_workspace"), alice_workspace, workspace_manifest
        )
        assert root_manifest == workspace_test_manifest
        assert parent == FsPath("/path_in_workspace")

    _get_or_create_directory_mock.side_effect = [
        workspace_test_manifest,
        workspace_test_save_manifest,
    ]
    with mock.patch(
        "parsec.core.cli.rsync._get_or_create_directory", _get_or_create_directory_mock
    ):
        root_manifest, parent = await rsync._root_manifest_parent(
            FsPath("/path_in_workspace/save"), alice_workspace, workspace_manifest
        )
        assert root_manifest == workspace_test_save_manifest
        assert parent == FsPath("/path_in_workspace/save")
