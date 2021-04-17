# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

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


async def _import_file(workspace_fs: WorkspaceFS, local_path: FsPath, dest: FsPath):
    dest_f = await workspace_fs.open_file(path=dest, mode="wb")
    async with dest_f:
        for chunk in await _chunks_from_path(local_path):
            await dest_f.write(chunk)


async def _chunks_from_path(src: AnyPath, size: int = DEFAULT_BLOCK_SIZE):
    chunks = []
    fd = await trio.open_file(src, "rb")

    async with fd:

        while True:
            chunk = await fd.read(size)
            if not chunk:
                break
            chunks.append(chunk)
    return chunks


async def _update_file(
    workspace_fs: WorkspaceFS, entry_id: EntryID, local_path: AnyPath, workspace_path: FsPath
):

    remote_file_manifest = await workspace_fs.remote_loader.load_manifest(entry_id)
    remote_access_digests = [access.digest for access in remote_file_manifest.blocks]
    offset = 0
    for idx, chunk in enumerate(await _chunks_from_path(local_path)):
        if HashDigest.from_data(chunk) != remote_access_digests[idx]:
            await workspace_fs.write_bytes(workspace_path, chunk, offset)
            print(f"update the block {idx} in {workspace_path}")
        offset += len(chunk)

    await workspace_fs.sync_by_id(entry_id, remote_changed=False, recursive=False)


async def _create_path(
    workspace_fs: WorkspaceFS, is_dir: bool, local_path: AnyPath, workspace_path: FsPath
):
    folder_manifest = None
    print(f"Create {workspace_path}")
    if is_dir:
        await workspace_fs.mkdir(workspace_path)
        await workspace_fs.sync()
        rep_info = await workspace_fs.path_info(workspace_path)
        folder_manifest = await workspace_fs.local_storage.get_manifest(rep_info["id"])
    else:
        await _import_file(workspace_fs, local_path, workspace_path)
        await workspace_fs.sync()
    return folder_manifest


async def _clear_path(workspace_fs: WorkspaceFS, workspace_path: FsPath):
    if await workspace_fs.is_dir(workspace_path):
        await workspace_fs.rmtree(workspace_path)
    else:
        await workspace_fs.unlink(workspace_path)
    await workspace_fs.sync()


async def _clear_directory(
    workspace_directory_path: FsPath,
    local_path: AnyPath,
    workspace_fs: WorkspaceFS,
    folder_manifest: FolderManifest,
):
    local_children_keys = [p.name for p in await local_path.iterdir()]
    for name, entry_id in folder_manifest.children.items():
        if name not in local_children_keys:
            absolute_path = FsPath(workspace_directory_path / name)
            print("delete %s" % absolute_path)
            await _clear_path(workspace_fs, absolute_path)


async def _get_or_create_directory(
    entry_id: EntryID, workspace_fs: WorkspaceFS, local_path: AnyPath, workspace_path: FsPath
):
    if entry_id:
        folder_manifest = await workspace_fs.remote_loader.load_manifest(entry_id)
    else:
        folder_manifest = await _create_path(workspace_fs, True, local_path, workspace_path)
    return folder_manifest


async def _upsert_file(
    entry_id: EntryID, workspace_fs: WorkspaceFS, local_path: AnyPath, workspace_path: FsPath
):
    if entry_id:
        await _update_file(workspace_fs, entry_id, local_path, workspace_path)
    else:
        await _create_path(workspace_fs, False, local_path, workspace_path)


async def _sync_directory(
    entry_id: EntryID, workspace_fs: WorkspaceFS, local_path: AnyPath, workspace_path: FsPath
):
    folder_manifest = await _get_or_create_directory(
        entry_id, workspace_fs, local_path, workspace_path
    )
    await _sync_directory_content(workspace_path, local_path, workspace_fs, folder_manifest)
    if entry_id:
        await _clear_directory(workspace_path, local_path, workspace_fs, folder_manifest)


async def _sync_directory_content(
    workspace_directory_path: FsPath,
    directory_local_path: AnyPath,
    workspace_fs: WorkspaceFS,
    manifest: FolderManifest,
):
    for local_path in await directory_local_path.iterdir():
        name = local_path.name
        workspace_path = FsPath(workspace_directory_path / name)
        entry_id = manifest.children.get(name)
        if await local_path.is_dir():
            await _sync_directory(entry_id, workspace_fs, local_path, workspace_path)
        else:
            await _upsert_file(entry_id, workspace_fs, local_path, workspace_path)


def _parse_destination(core: LoggedCore, destination: str):
    try:
        workspace_name, path = destination.split(":")
    except ValueError:
        workspace_name = destination
        path = None
    if path:
        try:
            path = FsPath(path)
        except ValueError:
            path = FsPath(f"/{path}")

    for workspace in core.user_fs.get_user_manifest().workspaces:
        if workspace.name == workspace_name:
            break
    else:
        raise SystemExit(f"Unknown workspace ({destination})")
    return workspace, path


async def _root_manifest_parent(
    path_destination: FsPath, workspace_fs: WorkspaceFS, workspace_manifest: WorkspaceManifest
):

    root_manifest = workspace_manifest
    parent = FsPath("/")
    if path_destination:
        for p in path_destination.parts:
            parent = FsPath(parent / p)
            entry_id = root_manifest.children.get(p)
            root_manifest = await _get_or_create_directory(entry_id, workspace_fs, parent, parent)

    return root_manifest, parent


async def _rsync(
    config: CoreConfig, device: local_device.LocalDevice, source: str, destination: str
):
    async with logged_core_factory(config, device) as core:
        workspace, destination_path = _parse_destination(core, destination)
        workspace_fs = core.user_fs.get_workspace(workspace.id)
        workspace_manifest = await workspace_fs.remote_loader.load_manifest(workspace.id)
        local_path = trio.Path(source)
        root_manifest, workspace_path = await _root_manifest_parent(
            destination_path, workspace_fs, workspace_manifest
        )

        await _sync_directory_content(workspace_path, local_path, workspace_fs, root_manifest)
        await _clear_directory(workspace_path, local_path, workspace_fs, root_manifest)


@click.command(short_help="rsync to parsec")
@core_config_and_device_options
@click.argument("source")
@click.argument("destination")
def run_rsync(config, device, source, destination, **kwargs):
    with cli_exception_handler(config.debug):
        trio_run(_rsync, config, device, source, destination)
