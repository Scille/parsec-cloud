# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from pendulum import datetime

from parsec.api.data import EntryName
from parsec.core.fs import FsPath
from parsec.core.fs.exceptions import FSLocalMissError


@pytest.mark.trio
async def test_root_entry_info(alice_workspace_t2, alice_workspace_t4):
    stat2 = await alice_workspace_t2.transactions.entry_info(FsPath("/"))
    assert stat2 == {
        "type": "folder",
        "id": alice_workspace_t4.transactions.workspace_id,
        "base_version": 1,
        "is_placeholder": False,
        "need_sync": False,
        "created": datetime(1999, 12, 31),
        "updated": datetime(1999, 12, 31),
        "children": [EntryName("foo")],
        "confinement_point": None,
    }

    stat4 = await alice_workspace_t4.transactions.entry_info(FsPath("/"))
    assert stat4 == {
        "type": "folder",
        "id": alice_workspace_t4.transactions.workspace_id,
        "base_version": 2,
        "is_placeholder": False,
        "need_sync": False,
        "created": datetime(1999, 12, 31),
        "updated": datetime(2000, 1, 4),
        "children": [EntryName("files"), EntryName("foo")],
        "confinement_point": None,
    }


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
    alice_workspace_t4.transactions.local_storage._cache.clear()
    with pytest.raises(FSLocalMissError):
        await alice_workspace_t4.transactions.local_storage.get_manifest(entry_id)
    await alice_workspace_t4.transactions.entry_info(FsPath("/"))


@pytest.mark.trio
async def test_access_unknown_entry(alice_workspace_t4):
    with pytest.raises(FileNotFoundError):
        await alice_workspace_t4.transactions.entry_info(FsPath("/dummy"))
