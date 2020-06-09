# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import click
from pathlib import Path
from pendulum import Pendulum, parse as pendulum_parse

from parsec.utils import trio_run
from parsec.crypto import HashDigest
from parsec.cli_utils import cli_exception_handler, generate_not_available_cmd
from parsec.core import logged_core_factory
from parsec.core.types import DEFAULT_BLOCK_SIZE, FsPath
from parsec.core.cli.utils import core_config_and_device_options, core_config_options

try:
    from parsec.core.gui import run_gui as _run_gui

except ImportError as exc:
    run_gui = generate_not_available_cmd(exc)

else:

    @click.command(short_help="run parsec GUI")
    # Let the GUI handle the parsing of the url to display dialog on error
    @click.argument("url", required=False)
    @click.option("--diagnose", "-d", is_flag=True)
    @core_config_options
    def run_gui(config, url, diagnose, **kwargs):
        """
        Run parsec GUI
        """
        config = config.evolve(mountpoint_enabled=True)
        _run_gui(config, start_arg=url, diagnose=diagnose)


async def _run_mountpoint(config, device, timestamp: Pendulum = None):
    config = config.evolve(mountpoint_enabled=True)
    async with logged_core_factory(config, device):
        display_device = click.style(device.device_id, fg="yellow")
        mountpoint_display = click.style(str(config.mountpoint_base_dir.absolute()), fg="yellow")
        click.echo(f"{display_device}'s drive mounted at {mountpoint_display}")

        await trio.sleep_forever()


@click.command(short_help="run parsec mountpoint")
@core_config_and_device_options
@click.option("--mountpoint", "-m", type=click.Path(exists=False))
@click.option("--timestamp", "-t", type=lambda t: pendulum_parse(t, tz="local"))
def run_mountpoint(config, device, mountpoint, timestamp, **kwargs):
    """
    Expose device's parsec drive on the given mountpoint.
    """
    config = config.evolve(mountpoint_enabled=True)
    if mountpoint:
        config = config.evolve(mountpoint_base_dir=Path(mountpoint))
    with cli_exception_handler(config.debug):
        trio_run(_run_mountpoint, config, device, timestamp)


async def _import_file(workspace_fs, src, dest):
    try:
        await workspace_fs.touch(dest)
    except FileExistsError:
        pass
    with open(src, "rb") as fd:
        read_size = 0
        while True:
            chunk = fd.read(DEFAULT_BLOCK_SIZE)
            if not chunk:
                break
            await workspace_fs.write_bytes(dest, chunk, read_size)
            read_size += len(chunk)


def _local_access_digests(workspace_fs, src):
    digests = []
    with open(src, "rb") as fd:
        read_size = 0
        while True:
            chunk = fd.read(DEFAULT_BLOCK_SIZE)
            if not chunk:
                break
            digests.append(HashDigest.from_data(chunk))
            read_size += len(chunk)
    return digests


async def _update_file(workspace_fs, entry_id, path, absolute_path):
    remote_file_manifest = await workspace_fs.remote_loader.load_manifest(entry_id)
    local_access_digests = _local_access_digests(workspace_fs, path)
    remote_access_digests = [access.digest for access in remote_file_manifest.blocks]
    if local_access_digests != remote_access_digests:
        await _import_file(workspace_fs, path, str(absolute_path))
        print("update %s" % absolute_path)
        await workspace_fs.sync_by_id(entry_id, remote_changed=False, recursive=False)


async def _create_path(workspace_fs, is_dir, path, absolute_path):
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


async def _clear_directory(parent_absolute_path, path, workspace_fs, folder_manifest):
    local_children_keys = [p.name for p in await path.iterdir()]
    for name, entry_id in folder_manifest.children.items():
        if name not in local_children_keys:
            absolute_path = str(await (parent_absolute_path / name).absolute())
            print("delete %s" % absolute_path)
            if await workspace_fs.is_dir(absolute_path):
                await workspace_fs.rmtree(absolute_path)
            else:
                await workspace_fs.unlink(absolute_path)
            await workspace_fs.sync()


async def _get_or_create_directory(entry_id, workspace_fs, path, absolute_path):
    if entry_id:
        folder_manifest = await workspace_fs.remote_loader.load_manifest(entry_id)
    else:
        folder_manifest = await _create_path(workspace_fs, True, path, absolute_path)
    return folder_manifest


async def _upsert_file(entry_id, workspace_fs, path, absolute_path):
    if entry_id:
        await _update_file(workspace_fs, entry_id, path, absolute_path)
    else:
        await _create_path(workspace_fs, False, path, absolute_path)


async def _sync_directory_content(parent, source, workspace_fs, manifest):
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


def _parse_destination(core, destination):
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


async def _root_manifest_parent(core, path_destination, parent, workspace_fs, workspace_manifest):
    if not path_destination:
        root_manifest = workspace_manifest
    else:
        for p in trio.Path(path_destination).parts:
            parent = trio.Path(parent / p)
            entry_id = workspace_manifest.children.get(p)
            root_manifest = await _get_or_create_directory(
                entry_id, workspace_fs, parent, await parent.absolute()
            )
    return root_manifest, parent


async def _rsync(config, device, source, destination):
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
def rsync(config, device, source, destination, **kwargs):
    with cli_exception_handler(config.debug):
        trio_run(_rsync, config, device, source, destination)
