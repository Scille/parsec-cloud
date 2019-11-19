# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.fs.userfs import UserFS
from parsec.core.fs.workspacefs import WorkspaceFS
from parsec.core.types import FsPath
from parsec.core.types import EntryID


async def make_workspace_dir_inconsistent(workspace: WorkspaceFS, dir: FsPath):
    await workspace.mkdir(dir)
    await workspace.touch(dir / "foo.txt")
    rep_info = await workspace.transactions.entry_info(dir)
    rep_manifest = await workspace.local_storage.get_manifest(rep_info["id"])
    children = rep_manifest.children
    children["newfail.txt"] = EntryID("b9295787-d9aa-6cbd-be27-1ff83ac72fa6")
    rep_manifest.evolve(children=children)
    async with workspace.local_storage.lock_manifest(rep_info["id"]):
        await workspace.local_storage.set_manifest(rep_info["id"], rep_manifest)
    await workspace.sync()


async def create_inconsistent_workspace(user_fs: UserFS) -> WorkspaceFS:
    wid = await user_fs.workspace_create("w")
    workspace = user_fs.get_workspace(wid)
    await make_workspace_dir_inconsistent(workspace, FsPath("/rep"))
    return workspace
