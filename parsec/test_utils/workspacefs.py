# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import string
from typing import cast

from parsec.api.data.entry import EntryName
from parsec.core.fs import FsPath, UserFS, WorkspaceFS, FSFileNotFoundError
from parsec.core.types import EntryID, LocalFolderManifest


async def make_workspace_dir_inconsistent(workspace: WorkspaceFS, dir: FsPath) -> None:
    """
    Create directory and make it inconsistent by adding an entry refering
    to an unknown EntryID.
    """

    await workspace.mkdir(dir)
    await workspace.touch(dir / "foo.txt")
    rep_info = await workspace.transactions.entry_info(dir)
    rep_manifest = await workspace.local_storage.get_manifest(cast(EntryID, rep_info["id"]))
    assert isinstance(rep_manifest, LocalFolderManifest)
    children = rep_manifest.children
    children[EntryName("newfail.txt")] = EntryID.from_hex("b9295787-d9aa-6cbd-be27-1ff83ac72fa6")
    rep_manifest = rep_manifest.evolve(children=children)
    async with workspace.local_storage.lock_manifest(cast(EntryID, rep_info["id"])):
        await workspace.local_storage.set_manifest(cast(EntryID, rep_info["id"]), rep_manifest)
    await workspace.sync()


async def make_workspace_dir_simple_versions(workspace: WorkspaceFS, dir: FsPath) -> None:
    await workspace.mkdir(dir)
    await workspace.sync()
    for i in range(0, 9):
        file_name = f"hello.txt"
        path = f"{dir}/{file_name}"
        num_chars = (i % 4 + 4) * 100 * 1024
        text = "".join(
            [
                string.ascii_letters[(i + 1) * (j + 1) % len(string.ascii_letters)]
                for j in range(num_chars)
            ]
        )
        try:
            await workspace.write_bytes(path, text.encode())
        except FSFileNotFoundError:
            await workspace.touch(path)
            await workspace.write_bytes(path, text.encode())
        await workspace.sync()


async def make_workspace_dir_complex_versions(workspace: WorkspaceFS, dir: FsPath) -> None:
    await workspace.mkdir(dir)
    await workspace.sync()
    # Create useless stuff before file creation.
    for i in range(50):
        await workspace.touch(dir / f"foo{i}.txt")
        await workspace.sync()
    await workspace.mkdir(dir / "foo")
    await workspace.sync()
    for i in range(50):
        await workspace.touch(dir / "foo" / f"foo{i}.txt")
        await workspace.sync()
    await workspace.mkdir(dir / "foo" / "foo")
    await workspace.sync()
    # Now write an interesting file when benchmarking GUI
    await workspace.touch(dir / "foo" / "foo" / "foo.txt")
    await workspace.sync()
    # Add files in the same directory
    for i in range(50):
        await workspace.touch(dir / "foo" / "foo" / f"foo{i}.txt")
        await workspace.sync()
    # Remove useless stuff after file creation.
    for i in range(50):
        await workspace.unlink(dir / "foo" / "foo" / f"foo{i}.txt")
        await workspace.unlink(dir / "foo" / f"foo{i}.txt")
        await workspace.unlink(dir / f"foo{i}.txt")
        await workspace.sync()


async def create_inconsistent_workspace(
    user_fs: UserFS, name: EntryName = EntryName("w")
) -> WorkspaceFS:
    wid = await user_fs.workspace_create(name)
    workspace = user_fs.get_workspace(wid)
    await make_workspace_dir_inconsistent(workspace, FsPath("/rep"))
    return workspace
