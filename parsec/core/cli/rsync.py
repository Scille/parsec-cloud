# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import click

from parsec.utils import trio_run
from parsec.crypto import HashDigest
from parsec.api.data.manifest import WorkspaceManifest, FolderManifest

from parsec.core import logged_core_factory
from parsec.core.logged_core import LoggedCore
from parsec.core.config import CoreConfig
from parsec.api.data.entry import EntryID
from parsec.core.types import DEFAULT_BLOCK_SIZE, FsPath, local_device
from parsec.core.fs.workspacefs.workspacefs import AnyPath, WorkspaceFS
from parsec.core.cli.utils import core_config_and_device_options
from parsec.cli_utils import cli_exception_handler


async def _import_file(workspace_fs: WorkspaceFS, src: FsPath, dest: FsPath):
    try:
        await workspace_fs.touch(dest)
    except FileExistsError:
        pass
    f = await workspace_fs.open_file(dest, "w")
    await f.write(b"".join(_chunks_from_path(workspace_fs, src)))
    await f.close()


def _chunks_from_path(workspace_fs: WorkspaceFS, src: AnyPath):
    chunks = []
    with open(src, "rb") as fd:
        while True:
            chunk = fd.read(DEFAULT_BLOCK_SIZE)
            if not chunk:
                break
            chunks.append(chunk)
    return chunks


async def _update_file(
    workspace_fs: WorkspaceFS, entry_id: EntryID, path: AnyPath, absolute_path: AnyPath
):
    remote_file_manifest = await workspace_fs.remote_loader.load_manifest(entry_id)
    remote_access_digests = [access.digest for access in remote_file_manifest.blocks]
    offset = 0
    for idx, chunk in enumerate(_chunks_from_path(workspace_fs, path)):
        if HashDigest.from_data(chunk) != remote_access_digests[idx]:
            await workspace_fs.write_bytes(FsPath(str(absolute_path)), chunk, offset)
            print(f"update the block {idx} in {path}")
        offset += len(chunk)

    await workspace_fs.sync_by_id(entry_id, remote_changed=False, recursive=False)


async def _create_path(
    workspace_fs: WorkspaceFS, is_dir: bool, path: AnyPath, absolute_path: AnyPath
):
    print("create %s" % absolute_path)
    folder_manifest = None
    fs_path = FsPath(str(absolute_path))
    if is_dir:
        await workspace_fs.mkdir(fs_path)
        await workspace_fs.sync()
        rep_info = await workspace_fs.path_info(fs_path)
        folder_manifest = await workspace_fs.local_storage.get_manifest(rep_info["id"])
    else:
        await _import_file(workspace_fs, path, fs_path)
        await workspace_fs.sync()
    return folder_manifest


async def _clear_directory(
    parent_absolute_path: AnyPath,
    path: AnyPath,
    workspace_fs: WorkspaceFS,
    folder_manifest: FolderManifest,
):
    local_children_keys = [p.name for p in await path.iterdir()]
    for name, entry_id in folder_manifest.children.items():
        if name not in local_children_keys:
            absolute_path = FsPath(str(await (parent_absolute_path / name).absolute()))
            print("delete %s" % absolute_path)
            if await workspace_fs.is_dir(absolute_path):
                await workspace_fs.rmtree(absolute_path)
            else:
                await workspace_fs.unlink(absolute_path)
            await workspace_fs.sync()


async def _get_or_create_directory(
    entry_id: EntryID, workspace_fs: WorkspaceFS, path: AnyPath, absolute_path: AnyPath
):
    if entry_id:
        folder_manifest = await workspace_fs.remote_loader.load_manifest(entry_id)
    else:
        folder_manifest = await _create_path(workspace_fs, True, path, absolute_path)
    return folder_manifest


async def _upsert_file(
    entry_id: EntryID, workspace_fs: WorkspaceFS, path: AnyPath, absolute_path: AnyPath
):
    if entry_id:
        await _update_file(workspace_fs, entry_id, path, absolute_path)
    else:
        await _create_path(workspace_fs, False, path, absolute_path)


async def _sync_directory_content(
    parent: AnyPath, source: AnyPath, workspace_fs: WorkspaceFS, manifest: FolderManifest
):
    for path in await source.iterdir():
        name = path.name
        absolute_path = await (parent / name).absolute()
        entry_id = manifest.children.get(name)
        if await path.is_dir():
            folder_manifest = await _get_or_create_directory(
                entry_id, workspace_fs, path, absolute_path
            )
            await _sync_directory_content(absolute_path, path, workspace_fs, folder_manifest)
            if entry_id:
                await _clear_directory(absolute_path, path, workspace_fs, folder_manifest)
        else:
            await _upsert_file(entry_id, workspace_fs, path, absolute_path)


def _parse_destination(core: LoggedCore, destination: str):
    try:
        workspace_name, path = destination.split(":")
    except ValueError:
        workspace_name = destination
        path = None
    for workspace in core.user_fs.get_user_manifest().workspaces:
        if workspace.name == workspace_name:
            break
    else:
        raise SystemExit(f"Unknown workspace ({destination})")
    return workspace, path


async def _root_manifest_parent(
    core: LoggedCore,
    path_destination: str,
    parent: AnyPath,
    workspace_fs: WorkspaceFS,
    workspace_manifest: WorkspaceManifest,
):

    root_manifest = workspace_manifest
    if path_destination:
        for p in trio.Path(path_destination).parts:
            parent = trio.Path(parent / p)
            entry_id = root_manifest.children.get(p)
            root_manifest = await _get_or_create_directory(
                entry_id, workspace_fs, parent, await parent.absolute()
            )
    return root_manifest, parent


async def _rsync(
    config: CoreConfig, device: local_device.LocalDevice, source: str, destination: str
):
    async with logged_core_factory(config, device) as core:
        workspace, path = _parse_destination(core, destination)
        workspace_fs = core.user_fs.get_workspace(workspace.id)
        workspace_manifest = await workspace_fs.remote_loader.load_manifest(workspace.id)
        source = trio.Path(source)
        root_manifest, parent = await _root_manifest_parent(
            core, path, trio.Path("/"), workspace_fs, workspace_manifest
        )

        await _sync_directory_content(parent, source, workspace_fs, root_manifest)
        await _clear_directory(parent, source, workspace_fs, root_manifest)


@click.command(short_help="rsync to parsec")
@core_config_and_device_options
@click.argument("source")
@click.argument("destination")
def run_rsync(config, device, source, destination, **kwargs):
    with cli_exception_handler(config.debug):
        trio_run(_rsync, config, device, source, destination)
