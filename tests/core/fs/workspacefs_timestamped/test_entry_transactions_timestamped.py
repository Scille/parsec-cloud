# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.types import FsPath
from parsec.core.fs import FSLocalMissError


@pytest.mark.trio
async def test_root_entry_info(alice_workspace_t2, alice_workspace_t4):
    stat2 = await alice_workspace_t2.transactions.entry_info(FsPath("/"))
    assert stat2 == {
        "type": "folder",
        "id": alice_workspace_t4.transactions.workspace_id,
        "base_version": 1,
        "is_placeholder": False,
        "need_sync": False,
        "created": Pendulum(1999, 12, 31),
        "updated": Pendulum(1999, 12, 31),
        "children": ["foo"],
    }

    stat4 = await alice_workspace_t4.transactions.entry_info(FsPath("/"))
    assert stat4 == {
        "type": "folder",
        "id": alice_workspace_t4.transactions.workspace_id,
        "base_version": 2,
        "is_placeholder": False,
        "need_sync": False,
        "created": Pendulum(1999, 12, 31),
        "updated": Pendulum(2000, 1, 4),
        "children": ["files", "foo"],
    }


@pytest.mark.trio
async def test_existing_file_entry_versions(alice_workspace, alice):
    versions = await alice_workspace.transactions.entry_versions(FsPath("/files/renamed_content"))
    versions_list = list(versions.items())

    assert len(versions_list) == 6

    # Moved /files/content to /files/renamed_content on day 5, moved it again later
    assert versions_list[0][0][1:] == (3, Pendulum(2000, 1, 6), Pendulum(2000, 1, 7))
    assert versions_list[0][1] == (
        (alice.device_id, Pendulum(2000, 1, 5)),
        FsPath("/files/content"),
        FsPath("/files/renamed_again_content"),
    )

    # Created a new file with the same name on day 8
    assert versions_list[1][0][1:] == (1, Pendulum(2000, 1, 8), Pendulum(2000, 1, 8))
    assert versions_list[1][1] == ((alice.device_id, Pendulum(2000, 1, 8)), None, None)

    # Wrote on it. Moved it again on day 9 as we renamed /files to /moved_files
    assert versions_list[2][0][1:] == (2, Pendulum(2000, 1, 8), Pendulum(2000, 1, 9))
    assert versions_list[2][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        None,
        FsPath("/moved_files/renamed_content"),
    )

    # And moved back /moved_files to /files on day 11, /files/renamed_content is deleted on day 12
    assert versions_list[3][0][1:] == (2, Pendulum(2000, 1, 11), Pendulum(2000, 1, 12))
    assert versions_list[3][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        FsPath("/moved_files/renamed_content"),
        None,
    )

    # Created a file, again, but didn't write
    assert versions_list[4][0][1:] == (1, Pendulum(2000, 1, 13), Pendulum(2000, 1, 14))
    assert versions_list[4][1] == ((alice.device_id, Pendulum(2000, 1, 13)), None, None)

    # Used "touch" method again, but on a created file. Wrote on it. Didn't delete since then
    assert versions_list[5][0][1:3] == (2, Pendulum(2000, 1, 14))
    assert Pendulum.now().add(hours=-1) < versions_list[5][0][3] < Pendulum.now()
    assert versions_list[5][1] == ((alice.device_id, Pendulum(2000, 1, 14)), None, None)


@pytest.mark.trio
async def test_non_existing_file_entry_versions(alice_workspace, alice):
    versions = await alice_workspace.transactions.entry_versions(
        FsPath("/moved_files/renamed_content")
    )
    versions_list = list(versions.items())

    assert len(versions_list) == 2

    assert versions_list[0][0][1:] == (2, Pendulum(2000, 1, 9), Pendulum(2000, 1, 10))
    assert versions_list[0][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        FsPath("/files/renamed_content"),
        None,
    )

    assert versions_list[1][0][1:] == (2, Pendulum(2000, 1, 10), Pendulum(2000, 1, 11))
    assert versions_list[1][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        None,
        FsPath("/files/renamed_content"),
    )


@pytest.mark.trio
async def test_existing_directory_entry_versions(alice_workspace, alice):
    versions = await alice_workspace.transactions.entry_versions(FsPath("/files"))
    versions_list = list(versions.items())

    assert len(versions_list) == 8

    assert versions_list[0][0][1:] == (1, Pendulum(2000, 1, 4), Pendulum(2000, 1, 4))
    assert versions_list[0][1] == ((alice.device_id, Pendulum(2000, 1, 4)), None, None)

    assert versions_list[1][0][1:] == (2, Pendulum(2000, 1, 4), Pendulum(2000, 1, 6))
    assert versions_list[1][1] == ((alice.device_id, Pendulum(2000, 1, 4)), None, None)

    assert versions_list[2][0][1:] == (3, Pendulum(2000, 1, 6), Pendulum(2000, 1, 7))
    assert versions_list[2][1] == ((alice.device_id, Pendulum(2000, 1, 6)), None, None)

    assert versions_list[3][0][1:] == (4, Pendulum(2000, 1, 7), Pendulum(2000, 1, 8))
    assert versions_list[3][1] == ((alice.device_id, Pendulum(2000, 1, 7)), None, None)

    assert versions_list[4][0][1:] == (5, Pendulum(2000, 1, 8), Pendulum(2000, 1, 9))
    assert versions_list[4][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        None,
        FsPath("/moved_files"),
    )

    assert versions_list[5][0][1:] == (6, Pendulum(2000, 1, 11), Pendulum(2000, 1, 12))
    assert versions_list[5][1] == (
        (alice.device_id, Pendulum(2000, 1, 10)),
        FsPath("/moved_files"),
        None,
    )

    assert versions_list[6][0][1:] == (7, Pendulum(2000, 1, 12), Pendulum(2000, 1, 13))
    assert versions_list[6][1] == ((alice.device_id, Pendulum(2000, 1, 12)), None, None)

    assert versions_list[7][0][1:3] == (8, Pendulum(2000, 1, 13))
    assert Pendulum.now().add(hours=-1) < versions_list[7][0][3] < Pendulum.now()
    assert versions_list[7][1] == ((alice.device_id, Pendulum(2000, 1, 13)), None, None)


@pytest.mark.trio
async def test_non_existing_directory_entry_versions(alice_workspace, alice):
    versions = await alice_workspace.transactions.entry_versions(FsPath("/moved_files"))
    versions_list = list(versions.items())

    assert len(versions_list) == 2

    assert versions_list[0][0][1:] == (5, Pendulum(2000, 1, 9), Pendulum(2000, 1, 10))
    assert versions_list[0][1] == ((alice.device_id, Pendulum(2000, 1, 8)), FsPath("/files"), None)

    assert versions_list[1][0][1:] == (6, Pendulum(2000, 1, 10), Pendulum(2000, 1, 11))
    assert versions_list[1][1] == ((alice.device_id, Pendulum(2000, 1, 10)), None, FsPath("/files"))


@pytest.mark.trio
async def test_file_create(alice_workspace_t4):
    with pytest.raises(PermissionError):
        access_id, fd = await alice_workspace_t4.transactions.file_create(FsPath("/foo.txt"))


@pytest.mark.trio
async def test_file_delete(alice_workspace_t4):
    with pytest.raises(PermissionError):
        await alice_workspace_t4.transactions.file_delete(FsPath("/foo/bar"))


@pytest.mark.trio
async def test_folder_delete(alice_workspace_t4):
    with pytest.raises(PermissionError):
        await alice_workspace_t4.transactions.folder_delete(FsPath("/foo"))


@pytest.mark.trio
async def test_rename(alice_workspace_t4):
    with pytest.raises(PermissionError):
        await alice_workspace_t4.transactions.entry_rename(FsPath("/foo"), FsPath("/foo2"))


@pytest.mark.trio
async def test_access_not_loaded_entry(alice_workspace_t4):
    entry_id = alice_workspace_t4.transactions.get_workspace_entry().id
    async with alice_workspace_t4.transactions.local_storage.lock_entry_id(entry_id):
        await alice_workspace_t4.transactions.local_storage.clear_manifest(entry_id)
    with pytest.raises(FSLocalMissError):
        await alice_workspace_t4.transactions.local_storage.get_manifest(entry_id)
    async with alice_workspace_t4.transactions.local_storage.lock_entry_id(entry_id):
        await alice_workspace_t4.transactions.local_storage.clear_manifest(entry_id)
    await alice_workspace_t4.transactions.entry_info(FsPath("/"))


@pytest.mark.trio
async def test_access_unknown_entry(alice_workspace_t4):
    with pytest.raises(FileNotFoundError):
        await alice_workspace_t4.transactions.entry_info(FsPath("/dummy"))
