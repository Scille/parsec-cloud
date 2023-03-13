# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, List, Tuple, Union

import click
import trio

from parsec._parsec import EntryID, EntryName, HashDigest
from parsec.api.data.manifest import FolderManifest, WorkspaceEntry, WorkspaceManifest
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options
from parsec.core.config import CoreConfig
from parsec.core.fs import FsPath, WorkspaceFS
from parsec.core.logged_core import LoggedCore
from parsec.core.types import DEFAULT_BLOCK_SIZE, LocalDevice
from parsec.utils import trio_run


async def _import_file(workspace_fs: WorkspaceFS, local_path: trio.Path, dest: FsPath) -> None:
    dest_f = await workspace_fs.open_file(path=dest, mode="wb")
    async with dest_f:
        for chunk in await _chunks_from_path(local_path):
            await dest_f.write(chunk)


async def _chunks_from_path(file_src: trio.Path, size: int = DEFAULT_BLOCK_SIZE) -> List[bytes]:
    chunks = []
    fd = await file_src.open("rb")

    async with fd:

        while True:
            chunk = await fd.read(size)
            if not chunk:
                break
            chunks.append(chunk)
    return chunks


async def _update_file(
    workspace_fs: WorkspaceFS, entry_id: EntryID, local_path: trio.Path, workspace_path: FsPath
) -> None:
    remote_file_manifest = await workspace_fs.remote_loader.load_manifest(entry_id)
    assert "blocks" in remote_file_manifest.__dict__
    # Mypy give errors that some elements of the union don't have the attribute `blocks` (that correct)
    # But we can't make an assertion on the type because we use mock on some test that would fail that assert
    # So we at least check that the object returned by `load_manifest` has a `blocks` attribute.
    remote_access_digests = [access.digest for access in remote_file_manifest.blocks]  # type: ignore[union-attr]
    offset = 0
    for idx, chunk in enumerate(await _chunks_from_path(local_path)):
        if HashDigest.from_data(chunk) != remote_access_digests[idx]:
            # `write_bytes` is currently defined with 2 arguments: a path and a bytearray.
            # We don't have an equivalent to what is written below with an offset and Mypy don't like it.
            await workspace_fs.write_bytes(workspace_path, chunk, offset)  # type: ignore[call-arg]
            print(f"update the block {idx} in {workspace_path}")
        offset += len(chunk)

    await workspace_fs.sync_by_id(entry_id, remote_changed=False, recursive=False)


async def _create_path(
    workspace_fs: WorkspaceFS, is_dir: bool, local_path: trio.Path, workspace_path: FsPath
) -> FolderManifest | None:
    print(f"Create {workspace_path}")
    if is_dir:
        await workspace_fs.mkdir(workspace_path)
        await workspace_fs.sync()
        rep_info = await workspace_fs.path_info(workspace_path)
        folder_manifest_id = rep_info["id"]
        assert isinstance(folder_manifest_id, EntryID)
        folder_manifest = await workspace_fs.local_storage.get_manifest(folder_manifest_id)
        assert isinstance(folder_manifest, FolderManifest)
        return folder_manifest
    else:
        await _import_file(workspace_fs, local_path, workspace_path)
        await workspace_fs.sync()
    return None


async def _clear_path(workspace_fs: WorkspaceFS, workspace_path: FsPath) -> None:
    if await workspace_fs.is_dir(workspace_path):
        await workspace_fs.rmtree(workspace_path)
    else:
        await workspace_fs.unlink(workspace_path)
    await workspace_fs.sync()


async def _clear_directory(
    workspace_directory_path: FsPath,
    local_path: trio.Path,
    workspace_fs: WorkspaceFS,
    folder_manifest: FolderManifest | WorkspaceManifest,
) -> None:
    local_children_keys = [p.name for p in await local_path.iterdir()]
    for name in folder_manifest.children.keys():
        if name.str not in local_children_keys:
            absolute_path = FsPath(workspace_directory_path / name)
            print("delete %s" % absolute_path)
            await _clear_path(workspace_fs, absolute_path)


async def _get_or_create_directory(
    entry_id: EntryID | None,
    workspace_fs: WorkspaceFS,
    local_path: trio.Path,
    workspace_path: FsPath,
) -> FolderManifest | None:
    if entry_id:
        manifest = await workspace_fs.remote_loader.load_manifest(entry_id)
        assert manifest is None or isinstance(
            manifest, FolderManifest
        ), f"Not a folder manifest: {type(manifest).__name__}"
        folder_manifest: FolderManifest | None = manifest
    else:
        folder_manifest = await _create_path(workspace_fs, True, local_path, workspace_path)
    return folder_manifest


async def _upsert_file(
    entry_id: EntryID | None,
    workspace_fs: WorkspaceFS,
    local_path: trio.Path,
    workspace_path: FsPath,
) -> None:
    if entry_id:
        await _update_file(workspace_fs, entry_id, local_path, workspace_path)
    else:
        await _create_path(workspace_fs, False, local_path, workspace_path)


async def _sync_directory(
    entry_id: EntryID | None,
    workspace_fs: WorkspaceFS,
    local_path: trio.Path,
    workspace_path: FsPath,
) -> None:
    folder_manifest = await _get_or_create_directory(
        entry_id, workspace_fs, local_path, workspace_path
    )
    assert folder_manifest is not None, "Missing folder manifest"
    await _sync_directory_content(workspace_path, local_path, workspace_fs, folder_manifest)
    if entry_id:
        await _clear_directory(workspace_path, local_path, workspace_fs, folder_manifest)


async def _sync_directory_content(
    workspace_directory_path: FsPath,
    directory_local_path: trio.Path,
    workspace_fs: WorkspaceFS,
    manifest: FolderManifest | WorkspaceManifest,
) -> None:
    for local_path in await directory_local_path.iterdir():
        name = EntryName(local_path.name)
        workspace_path = FsPath(workspace_directory_path / name)
        entry_id = manifest.children.get(name)
        if await local_path.is_dir():
            await _sync_directory(entry_id, workspace_fs, local_path, workspace_path)
        else:
            await _upsert_file(entry_id, workspace_fs, local_path, workspace_path)


def _parse_destination(core: LoggedCore, destination: str) -> Tuple[WorkspaceEntry, FsPath | None]:
    try:
        workspace_name, path = destination.split(":")
    except ValueError:
        workspace_name = destination
        path = None
    if path:
        try:
            fs_path = FsPath(path)
        except ValueError:
            fs_path = FsPath(f"/{path}")
    else:
        fs_path = None

    for workspace in core.user_fs.get_user_manifest().workspaces:
        if workspace.name.str == workspace_name:
            break
    else:
        raise SystemExit(f"Unknown workspace ({destination})")
    return workspace, fs_path


async def _root_manifest_parent(
    path_destination: FsPath, workspace_fs: WorkspaceFS, workspace_manifest: WorkspaceManifest
) -> Tuple[Union[WorkspaceManifest, FolderManifest], FsPath]:

    root_manifest: WorkspaceManifest | FolderManifest = workspace_manifest
    parent = trio.Path("/")
    fs_parent = FsPath("/")
    if path_destination:
        for p in path_destination.parts:
            parent = parent / p.str
            fs_parent = fs_parent / p
            entry_id = root_manifest.children.get(p)
            manifest = await _get_or_create_directory(entry_id, workspace_fs, parent, fs_parent)
            assert isinstance(manifest, FolderManifest)
            root_manifest = manifest

    return root_manifest, fs_parent


async def _rsync(config: CoreConfig, device: LocalDevice, source: str, destination: str) -> None:
    async with logged_core_factory(config, device) as core:
        workspace, destination_path = _parse_destination(core, destination)
        assert destination_path is not None

        workspace_fs = core.user_fs.get_workspace(workspace.id)
        workspace_manifest = await workspace_fs.remote_loader.load_manifest(workspace.id)
        assert isinstance(workspace_manifest, WorkspaceManifest)

        local_path = trio.Path(source)
        root_manifest, workspace_path = await _root_manifest_parent(
            destination_path, workspace_fs, workspace_manifest
        )

        await _sync_directory_content(workspace_path, local_path, workspace_fs, root_manifest)
        await _clear_directory(workspace_path, local_path, workspace_fs, root_manifest)


@click.command(short_help="rsync to parsec")
@click.argument("source")
@click.argument("destination")
@core_config_and_device_options
@cli_command_base_options
def run_rsync(
    config: CoreConfig, device: LocalDevice, source: str, destination: str, **kwargs: Any
) -> None:
    with cli_exception_handler(config.debug):
        trio_run(_rsync, config, device, source, destination)
