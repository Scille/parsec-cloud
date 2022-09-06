# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import errno
from typing import Union
import pytest
from unittest.mock import ANY

from trio import open_nursery
from parsec._parsec import (
    LocalDevice,
    FileManifest,
    FolderManifest,
    UserManifest,
    WorkspaceManifest,
)

from parsec.api.protocol import DeviceID, RealmID, RealmRole
from parsec.api.data import EntryName
from parsec.core.types import EntryID, DEFAULT_BLOCK_SIZE
from parsec.core.fs import FsPath
from parsec.core.fs.exceptions import FSError, FSBackendOfflineError, FSLocalMissError
from parsec.core.fs.workspacefs.workspacefs import ReencryptionNeed, WorkspaceFS
from parsec.backend.block import BlockNotFoundError


@pytest.mark.trio
async def test_workspace_properties(alice_workspace):
    assert alice_workspace.get_workspace_name() == EntryName("w")
    assert alice_workspace.get_encryption_revision() == 1


@pytest.mark.trio
async def test_path_info(alice_workspace):
    info = await alice_workspace.path_info("/")
    assert {
        "base_version": ANY,
        "children": [EntryName("foo")],
        "created": ANY,
        "id": ANY,
        "is_placeholder": False,
        "need_sync": False,
        "type": "folder",
        "updated": ANY,
        "confinement_point": None,
    } == info

    info = await alice_workspace.path_info("/foo")
    assert {
        "base_version": ANY,
        "children": [EntryName("bar"), EntryName("baz")],
        "created": ANY,
        "id": ANY,
        "is_placeholder": False,
        "need_sync": False,
        "type": "folder",
        "updated": ANY,
        "confinement_point": None,
    } == info

    info = await alice_workspace.path_info("/foo/bar")
    assert {
        "base_version": ANY,
        "size": 0,
        "created": ANY,
        "id": ANY,
        "is_placeholder": False,
        "need_sync": False,
        "type": "file",
        "updated": ANY,
        "confinement_point": None,
    } == info


@pytest.mark.trio
async def test_get_user_roles(alice_workspace):
    assert await alice_workspace.get_user_roles() == {
        alice_workspace.device.user_id: RealmRole.OWNER
    }


@pytest.mark.trio
async def test_exists(alice_workspace):
    assert await alice_workspace.exists("/") is True
    assert await alice_workspace.exists("/foo") is True
    assert await alice_workspace.exists("/foo/bar") is True
    assert await alice_workspace.exists("/foo/baz") is True

    assert await alice_workspace.exists("/fiz") is False
    assert await alice_workspace.exists("/foo/fiz") is False
    assert await alice_workspace.exists("/fiz/foo") is False
    assert await alice_workspace.exists("/foo/bar/baz") is False


@pytest.mark.trio
async def test_is_dir(alice_workspace):
    assert await alice_workspace.is_dir("/") is True
    assert await alice_workspace.is_dir("/foo") is True
    assert await alice_workspace.is_dir("/foo/bar") is False

    with pytest.raises(FileNotFoundError):
        await alice_workspace.is_dir("/baz")


@pytest.mark.trio
async def test_is_file(alice_workspace):
    assert await alice_workspace.is_file("/") is False
    assert await alice_workspace.is_file("/foo") is False
    assert await alice_workspace.is_file("/foo/bar") is True

    with pytest.raises(FileNotFoundError):
        await alice_workspace.is_file("/baz")


@pytest.mark.trio
async def test_iterdir(alice_workspace):
    lst = [child async for child in alice_workspace.iterdir("/")]
    assert lst == [FsPath("/foo")]
    lst = [child async for child in alice_workspace.iterdir("/foo")]
    assert lst == [FsPath("/foo/bar"), FsPath("/foo/baz")]

    with pytest.raises(NotADirectoryError):
        async for child in alice_workspace.iterdir("/foo/bar"):
            assert False, child
    with pytest.raises(FileNotFoundError):
        async for child in alice_workspace.iterdir("/baz"):
            assert False, child


@pytest.mark.trio
async def test_listdir(alice_workspace):
    lst = await alice_workspace.listdir("/")
    assert lst == [FsPath("/foo")]
    lst = await alice_workspace.listdir("/foo")
    assert lst == [FsPath("/foo/bar"), FsPath("/foo/baz")]

    with pytest.raises(NotADirectoryError):
        await alice_workspace.listdir("/foo/bar")
    with pytest.raises(FileNotFoundError):
        await alice_workspace.listdir("/baz")


@pytest.mark.trio
async def test_rename(alice_workspace):
    await alice_workspace.rename("/foo", "/foz")
    await alice_workspace.rename("/foz/bar", "/foz/bal")
    assert await alice_workspace.is_file("/foz/bal")

    with pytest.raises(OSError) as context:
        await alice_workspace.rename("/foz/baz", "/baz")
    assert context.value.errno == errno.EXDEV
    with pytest.raises(FileNotFoundError):
        await alice_workspace.rename("/foo", "/fob")


@pytest.mark.trio
async def test_mkdir(alice_workspace):
    await alice_workspace.mkdir("/foz")
    assert await alice_workspace.is_dir("/foz")

    await alice_workspace.mkdir("/a/b/c/d", parents=True)
    lst = await alice_workspace.listdir("/a/b/c")
    assert lst == [FsPath("/a/b/c/d")]

    with pytest.raises(FileNotFoundError):
        await alice_workspace.mkdir("/x/y/z")

    await alice_workspace.mkdir("/foo", exist_ok=True)
    with pytest.raises(FileExistsError):
        await alice_workspace.mkdir("/foo")
    with pytest.raises(FileExistsError):
        await alice_workspace.mkdir("/foo/bar")


@pytest.mark.trio
async def test_rmdir(alice_workspace):
    await alice_workspace.mkdir("/foz")
    await alice_workspace.rmdir("/foz")
    lst = await alice_workspace.listdir("/")
    assert lst == [FsPath("/foo")]

    with pytest.raises(OSError) as context:
        await alice_workspace.rmdir("/foo")
    assert context.value.errno == errno.ENOTEMPTY

    with pytest.raises(NotADirectoryError):
        await alice_workspace.rmdir("/foo/bar")

    with pytest.raises(PermissionError):
        await alice_workspace.rmdir("/")


@pytest.mark.trio
async def test_touch(alice_workspace):
    await alice_workspace.touch("/bar")
    assert await alice_workspace.is_file("/bar")
    info = await alice_workspace.path_info("/bar")
    assert info["size"] == 0

    await alice_workspace.touch("/bar")
    assert await alice_workspace.is_file("/bar")
    info = await alice_workspace.path_info("/bar")
    assert info["size"] == 0

    with pytest.raises(FileExistsError):
        await alice_workspace.touch("/bar", exist_ok=False)

    await alice_workspace.touch("/foo")


@pytest.mark.trio
async def test_unlink(alice_workspace):
    await alice_workspace.unlink("/foo/bar")
    lst = await alice_workspace.listdir("/foo")
    assert lst == [FsPath("/foo/baz")]

    with pytest.raises(FileNotFoundError):
        await alice_workspace.unlink("/foo/bar")
    with pytest.raises(IsADirectoryError):
        await alice_workspace.unlink("/foo")
    # TODO: should this be a `IsADirectoryError`?
    with pytest.raises(PermissionError):
        await alice_workspace.unlink("/")


@pytest.mark.trio
async def test_truncate(alice_workspace):
    await alice_workspace.write_bytes("/foo/bar", b"abcde")
    await alice_workspace.truncate("/foo/bar", 3)
    assert await alice_workspace.read_bytes("/foo/bar") == b"abc"

    await alice_workspace.truncate("/foo/bar", 13)
    assert await alice_workspace.read_bytes("/foo/bar") == b"abc" + b"\x00" * 10

    await alice_workspace.truncate("/foo/bar", 0)
    assert await alice_workspace.read_bytes("/foo/bar") == b""

    with pytest.raises(IsADirectoryError):
        await alice_workspace.truncate("/foo", 0)
    with pytest.raises(IsADirectoryError):
        await alice_workspace.truncate("/", 0)


@pytest.mark.trio
async def test_read_bytes(alice_workspace):
    assert await alice_workspace.read_bytes("/foo/bar") == b""

    await alice_workspace.write_bytes("/foo/bar", b"abcde")

    with pytest.raises(IsADirectoryError):
        await alice_workspace.read_bytes("/foo")
    with pytest.raises(IsADirectoryError):
        await alice_workspace.read_bytes("/")


@pytest.mark.trio
async def test_write_bytes(alice_workspace):
    # Pathlib mode (truncate=True)
    await alice_workspace.write_bytes("/foo/bar", b"abcde")
    assert await alice_workspace.read_bytes("/foo/bar") == b"abcde"

    # Clear the content of an existing file
    await alice_workspace.write_bytes("/foo/bar", b"")
    assert await alice_workspace.read_bytes("/foo/bar") == b""

    with pytest.raises(IsADirectoryError):
        await alice_workspace.read_bytes("/foo")
    with pytest.raises(IsADirectoryError):
        await alice_workspace.read_bytes("/")


@pytest.mark.trio
async def test_move(alice_workspace):
    await alice_workspace.move("/foo", "/foz")
    await alice_workspace.move("/foz/bar", "/foz/bal")
    assert await alice_workspace.is_file("/foz/bal")

    with pytest.raises(FileNotFoundError):
        await alice_workspace.move("/foo", "/fob")

    await alice_workspace.write_bytes("/foz/bal", b"abcde")
    await alice_workspace.write_bytes("/foz/baz", b"fghij")
    await alice_workspace.mkdir("/containfoz")
    await alice_workspace.move("/foz", "/containfoz")
    assert await alice_workspace.is_file("/containfoz/foz/bal")
    assert await alice_workspace.is_file("/containfoz/foz/baz")
    assert await alice_workspace.read_bytes("/containfoz/foz/bal") == b"abcde"
    assert await alice_workspace.read_bytes("/containfoz/foz/baz") == b"fghij"

    with pytest.raises(FileNotFoundError):
        await alice_workspace.move("/foz/baz", "/baz")

    await alice_workspace.move("/containfoz/foz/baz", "/baz")
    assert await alice_workspace.is_file("/baz")

    with pytest.raises(FileNotFoundError):
        await alice_workspace.move("/containfoz/foz/baz", "/biz")

    with pytest.raises(FileExistsError):
        await alice_workspace.move("/containfoz/foz/bal", "/baz")

    with pytest.raises(OSError):
        await alice_workspace.move("/containfoz", "/containfoz/foz")

    with pytest.raises(FileExistsError):
        await alice_workspace.move("/containfoz/foz", "/containfoz")


@pytest.mark.trio
async def test_copytree(alice_workspace):
    await alice_workspace.write_bytes("/foo/bar", b"a" * 9000 + b"b" * 40000)
    await alice_workspace.write_bytes("/foo/baz", b"a" * 40000 + b"b" * 9000)
    await alice_workspace.mkdir("/foo/dir")
    await alice_workspace.touch("/foo/dir/bar")
    await alice_workspace.write_bytes("/foo/dir/bar", b"a" * 5000 + b"b" * 6000)

    await alice_workspace.copytree("/foo", "/cfoo")
    assert await alice_workspace.read_bytes("/foo/bar") == b"a" * 9000 + b"b" * 40000
    assert await alice_workspace.read_bytes("/foo/baz") == b"a" * 40000 + b"b" * 9000
    assert await alice_workspace.read_bytes("/foo/dir/bar") == b"a" * 5000 + b"b" * 6000


@pytest.mark.trio
async def test_copyfile(alice_workspace):
    await alice_workspace.write_bytes("/foo/bar", b"a" * 9000 + b"b" * 40000)
    await alice_workspace.copyfile("/foo/bar", "/copied")
    assert await alice_workspace.read_bytes("/copied") == b"a" * 9000 + b"b" * 40000


@pytest.mark.trio
async def test_rmtree(alice_workspace):
    await alice_workspace.mkdir("/foz")
    await alice_workspace.rmtree("/foz")
    lst = await alice_workspace.listdir("/")
    assert lst == [FsPath("/foo")]

    with pytest.raises(NotADirectoryError):
        await alice_workspace.rmdir("/foo/bar")

    await alice_workspace.mkdir("/foo/foz")
    await alice_workspace.touch("/foo/foz/faz")

    await alice_workspace.rmtree("/foo")
    lst = await alice_workspace.listdir("/")
    assert lst == []

    with pytest.raises(PermissionError):
        await alice_workspace.rmtree("/")


@pytest.mark.trio
async def test_dump(alice_workspace):
    baz_id = await alice_workspace.path_id("/foo/baz")
    async with alice_workspace.local_storage.lock_entry_id(baz_id):
        await alice_workspace.local_storage.clear_manifest(baz_id)
    assert {
        "base_version": 1,
        "children": {
            EntryName("foo"): {
                "base_version": 2,
                "children": {
                    EntryName("bar"): {
                        "base_version": 1,
                        "blocksize": 524_288,
                        "blocks": [],
                        "created": ANY,
                        "id": ANY,
                        "is_placeholder": False,
                        "need_sync": False,
                        "parent": ANY,
                        "size": 0,
                        "updated": ANY,
                    },
                    EntryName("baz"): {"id": ANY},
                },
                "created": ANY,
                "id": ANY,
                "is_placeholder": False,
                "need_sync": False,
                "parent": ANY,
                "updated": ANY,
                "local_confinement_points": [],
                "remote_confinement_points": [],
            }
        },
        "created": ANY,
        "id": ANY,
        "is_placeholder": False,
        "need_sync": False,
        "updated": ANY,
        "local_confinement_points": [],
        "remote_confinement_points": [],
        "speculative": False,
    } == await alice_workspace.dump()


@pytest.mark.trio
async def test_path_info_remote_loader_exceptions(
    monkeypatch,
    alice_workspace: WorkspaceFS,
    alice: LocalDevice,
):
    manifest, _ = await alice_workspace.transactions._get_manifest_from_path(FsPath("/foo/bar"))
    async with alice_workspace.local_storage.lock_entry_id(manifest.id):
        await alice_workspace.local_storage.clear_manifest(manifest.id)

    from parsec.core.backend_connection.authenticated import BackendAuthenticatedCmds

    vanilla_vlob_read = BackendAuthenticatedCmds.vlob_read

    async def _vlob_read_patched(transport, encryption_revision, vlob_id, version, timestamp):
        ret = await vanilla_vlob_read(transport, encryption_revision, vlob_id, version, timestamp)
        if vlob_id.hex == manifest.id.hex:
            modified_remote_manifest: Union[
                FileManifest, FolderManifest, UserManifest, WorkspaceManifest
            ] = manifest.base.evolve(**manifest_modifiers)
            workspace_entry = alice_workspace.get_workspace_entry()
            raw_modified_remote_manifest = modified_remote_manifest.dump_sign_and_encrypt(
                author_signkey=alice.signing_key, key=workspace_entry.key
            )
            ret["blob"] = raw_modified_remote_manifest
        return ret

    monkeypatch.setattr(
        "parsec.core.backend_connection.authenticated.BackendAuthenticatedCmds.vlob_read",
        _vlob_read_patched,
    )

    manifest_modifiers = {"id": EntryID.new()}
    with pytest.raises(FSError) as exc:
        await alice_workspace.path_info(FsPath("/foo/bar"))
    assert (
        f"Cannot decrypt vlob: Invalid entry ID: expected `{manifest.id.str}`, got `{manifest_modifiers['id'].str}`"
        in str(exc.value)
    )

    manifest_modifiers = {"version": 4}
    with pytest.raises(FSError) as exc:
        await alice_workspace.path_info(FsPath("/foo/bar"))
    assert "Invalid version: expected `1`, got `4`" in str(exc.value)

    manifest_modifiers = {"author": DeviceID("mallory@pc1")}
    with pytest.raises(FSError) as exc:
        await alice_workspace.path_info(FsPath("/foo/bar"))
    assert "Invalid author: expected `alice@dev1`, got `mallory@pc1`" in str(exc.value)


@pytest.mark.trio
async def test_get_reencryption_need(alice_workspace, running_backend, monkeypatch):
    expected = ReencryptionNeed(user_revoked=(), role_revoked=())
    assert await alice_workspace.get_reencryption_need() == expected

    with running_backend.offline():
        with pytest.raises(FSBackendOfflineError):
            await alice_workspace.get_reencryption_need()

    # Reproduce a backend offline after the certificates have been retrieved (see issue #1335)
    reply = await alice_workspace.remote_loader.backend_cmds.realm_get_role_certificates(
        RealmID(alice_workspace.workspace_id.uuid)
    )
    original = alice_workspace.remote_loader.backend_cmds.realm_get_role_certificates

    async def mockup(*args):
        if args == (alice_workspace.workspace_id,):
            return reply
        return await original(*args)

    monkeypatch.setattr(
        alice_workspace.remote_loader.backend_cmds, "realm_get_role_certificates", mockup
    )

    with running_backend.offline():
        with pytest.raises(FSBackendOfflineError):
            await alice_workspace.get_reencryption_need()


@pytest.mark.trio
async def test_backend_block_data_online(
    alice_user_fs, alice2_user_fs, running_backend, monkeypatch
):
    def get_blocks_size(blocks):
        size = 0
        for block in blocks:
            size += block.size
        return size

    def check_size_integrity(file_size, proper_blocks_size, pending_chunks_size):
        assert file_size == TAZ_V2_BLOCKS * DEFAULT_BLOCK_SIZE
        assert proper_blocks_size == TAZ_V2_BLOCKS * DEFAULT_BLOCK_SIZE
        assert pending_chunks_size == 0

    wid = await alice_user_fs.workspace_create(EntryName("w"))
    await alice_user_fs.sync()
    await alice2_user_fs.sync()

    alice_workspace = alice_user_fs.get_workspace(wid)
    alice2_workspace = alice2_user_fs.get_workspace(wid)

    fspath = "/taz"

    await alice_workspace.touch(fspath)
    # Placeholder sync always starts by a "minimal sync" (i.e. an empty manifest is synchronized so that
    # parent folder don't have to wait for the full sync before syncing itself).
    # Here we force the minimal sync so to avoid this corner-case in the rest of the test.
    await alice_workspace.sync()
    await alice2_workspace.sync()

    # Fill the file with multiple blocks worth of data so that parallel upload will occurs
    TAZ_V2_BLOCKS = 6
    data_list = []
    data = b""
    for i in range(6):
        data_list.append((bytes(chr(ord("a") + i), "utf-8")) * DEFAULT_BLOCK_SIZE)
        data += data_list[i]

    await alice_workspace.write_bytes(fspath, data)

    info = await alice_workspace.path_info(fspath)

    assert info["base_version"] == 1
    assert info["is_placeholder"] is False
    assert info["need_sync"] is True

    (
        local_and_remote_blocks,
        local_blocks,
        remote_blocks,
        file_size,
        proper_blocks_size,
        pending_chunks_size,
    ) = await alice_workspace.get_blocks_by_type(fspath)
    assert len(remote_blocks) == 0
    assert len(local_blocks) == TAZ_V2_BLOCKS
    assert len(local_and_remote_blocks) == 0
    assert get_blocks_size(local_blocks) == TAZ_V2_BLOCKS * DEFAULT_BLOCK_SIZE
    check_size_integrity(file_size, proper_blocks_size, pending_chunks_size)

    data_dict = {}
    for block in local_blocks:
        data_dict[block.id] = await alice_workspace.local_storage.chunk_storage.get_chunk(block.id)

    await alice_workspace.sync()
    (
        local_and_remote_blocks,
        local_blocks,
        remote_blocks,
        file_size,
        proper_blocks_size,
        pending_chunks_size,
    ) = await alice_workspace.get_blocks_by_type(fspath)
    assert len(remote_blocks) == 0
    assert len(local_blocks) == 0
    assert len(local_and_remote_blocks) == TAZ_V2_BLOCKS
    assert get_blocks_size(local_and_remote_blocks) == TAZ_V2_BLOCKS * DEFAULT_BLOCK_SIZE

    (
        local_and_remote_blocks,
        local_blocks,
        remote_blocks,
        file_size,
        proper_blocks_size,
        pending_chunks_size,
    ) = await alice_workspace.get_blocks_by_type(fspath, DEFAULT_BLOCK_SIZE)
    assert len(remote_blocks) == 0
    assert len(local_blocks) == 0
    assert len(local_and_remote_blocks) == 1
    assert get_blocks_size(local_and_remote_blocks) == DEFAULT_BLOCK_SIZE
    check_size_integrity(file_size, proper_blocks_size, pending_chunks_size)

    # Check the blocks to download and the size of the total manifest
    (
        local_and_remote_blocks,
        local_blocks,
        remote_blocks,
        file_size,
        proper_blocks_size,
        pending_chunks_size,
    ) = await alice2_workspace.get_blocks_by_type(fspath)
    assert len(remote_blocks) == TAZ_V2_BLOCKS
    assert len(local_blocks) == 0
    assert len(local_and_remote_blocks) == 0
    assert get_blocks_size(remote_blocks) == TAZ_V2_BLOCKS * DEFAULT_BLOCK_SIZE
    check_size_integrity(file_size, proper_blocks_size, pending_chunks_size)

    for i in range(6):
        try:
            await alice2_workspace.local_storage.block_storage.get_chunk(remote_blocks[i].id)
        except FSLocalMissError:
            continue
        assert False

    (
        local_and_remote_blocks,
        local_blocks,
        remote_blocks,
        file_size,
        proper_blocks_size,
        pending_chunks_size,
    ) = await alice2_workspace.get_blocks_by_type(fspath, DEFAULT_BLOCK_SIZE)
    assert len(remote_blocks) == 1
    assert len(local_blocks) == 0
    assert len(local_and_remote_blocks) == 0
    assert get_blocks_size(remote_blocks) == DEFAULT_BLOCK_SIZE
    check_size_integrity(file_size, proper_blocks_size, pending_chunks_size)

    (
        local_and_remote_blocks,
        local_blocks,
        remote_blocks,
        file_size,
        proper_blocks_size,
        pending_chunks_size,
    ) = await alice2_workspace.get_blocks_by_type(fspath, DEFAULT_BLOCK_SIZE * TAZ_V2_BLOCKS)
    # load one block
    block = remote_blocks[3]
    await alice2_workspace.load_block(block)
    assert (
        await alice2_workspace.local_storage.block_storage.get_chunk(block.id)
        == data_dict[block.id]
    )

    (
        local_and_remote_blocks,
        local_blocks,
        remote_blocks,
        file_size,
        proper_blocks_size,
        pending_chunks_size,
    ) = await alice2_workspace.get_blocks_by_type(fspath)
    assert len(remote_blocks) == TAZ_V2_BLOCKS - 1
    assert len(local_blocks) == 0
    assert len(local_and_remote_blocks) == 1
    assert get_blocks_size(remote_blocks) == (TAZ_V2_BLOCKS - 1) * DEFAULT_BLOCK_SIZE
    assert get_blocks_size(local_and_remote_blocks) == DEFAULT_BLOCK_SIZE
    check_size_integrity(file_size, proper_blocks_size, pending_chunks_size)

    # load the rest
    async with open_nursery() as nursery:
        async with await alice2_workspace.receive_load_blocks(
            remote_blocks, nursery
        ) as receive_channel:
            async for value in receive_channel:
                assert value

    (
        local_and_remote_blocks,
        local_blocks,
        remote_blocks,
        file_size,
        proper_blocks_size,
        pending_chunks_size,
    ) = await alice2_workspace.get_blocks_by_type(fspath)
    assert len(remote_blocks) == 0
    assert len(local_blocks) == 0
    assert len(local_and_remote_blocks) == TAZ_V2_BLOCKS
    assert get_blocks_size(local_and_remote_blocks) == TAZ_V2_BLOCKS * DEFAULT_BLOCK_SIZE
    check_size_integrity(file_size, proper_blocks_size, pending_chunks_size)


@pytest.mark.trio
async def test_backend_block_upload_error_during_sync(
    alice_user_fs, alice2_user_fs, running_backend, monkeypatch
):
    wid = await alice_user_fs.workspace_create(EntryName("w"))
    await alice_user_fs.sync()
    await alice2_user_fs.sync()

    alice_workspace = alice_user_fs.get_workspace(wid)
    alice2_workspace = alice2_user_fs.get_workspace(wid)

    fspath = "/taz"

    await alice_workspace.touch(fspath)
    # Placeholder sync always starts by a "minimal sync" (i.e. an empty manifest is synchronized so that
    # parent folder don't have to wait for the full sync before syncing itself).
    # Here we force the minimal sync so to avoid this corner-case in the rest of the test.
    await alice_workspace.sync()

    # Fill the file with multiple blocks worth of data so that parallel upload will occurs
    TAZ_V2_BLOCKS = 6
    await alice_workspace.write_bytes(fspath, b"a" * TAZ_V2_BLOCKS * DEFAULT_BLOCK_SIZE)

    info = await alice_workspace.path_info(fspath)

    assert info["base_version"] == 1
    assert info["is_placeholder"] is False
    assert info["need_sync"] is True

    vanilla_backend_block_create = running_backend.backend.block.create

    blocks_create_before_crash = TAZ_V2_BLOCKS // 2

    async def mock_backend_block_create(*args, **kwargs):
        nonlocal blocks_create_before_crash
        if blocks_create_before_crash == 0:
            raise BlockNotFoundError()
        blocks_create_before_crash -= 1
        return await vanilla_backend_block_create(*args, **kwargs)

    monkeypatch.setattr(running_backend.backend.block, "create", mock_backend_block_create)

    with pytest.raises(FSError):
        await alice_workspace.sync()

    # File sync has failed, so it info should be the same
    info = await alice_workspace.path_info(fspath)
    assert info["base_version"] == 1
    assert info["need_sync"] is True
    # Other device doesn't see the modifications that have failed to be synced
    await alice2_workspace.sync()
    info = await alice2_workspace.path_info(fspath)
    assert info["base_version"] == 1
    assert info["need_sync"] is False
    assert info["size"] == 0

    monkeypatch.setattr(running_backend.backend.block, "create", vanilla_backend_block_create)

    await alice2_workspace.sync()
    assert await alice2_workspace.read_bytes(fspath) == bytearray()

    await alice_workspace.sync()

    info = await alice_workspace.path_info(fspath)
    assert info["base_version"] == 2
    assert info["need_sync"] is False

    await alice2_workspace.sync()
    assert await alice2_workspace.read_bytes(fspath) == b"a" * TAZ_V2_BLOCKS * DEFAULT_BLOCK_SIZE
