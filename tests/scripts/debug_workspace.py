#! /usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


import os
import shutil

import trio
from tqdm import tqdm
from humanize import naturalsize

from parsec.logging import configure_logging
from parsec.core.logged_core import logged_core_factory
from parsec.core.types import FsPath
from parsec.core.config import get_default_config_dir, load_config
from parsec.core.local_device import list_available_devices, load_device_with_password
from parsec.test_utils import (
    make_workspace_dir_inconsistent as make_workspace_dir_inconsistent_helper,
)


LOG_LEVEL = "WARNING"
DEVICE_ID = "alice@laptop"
PASSWORD = "test"


async def make_workspace_dir_inconsistent(device, workspace, corrupted_path):
    await make_workspace_dir_inconsistent_helper(workspace, FsPath(corrupted_path))
    print(f"{device.device_id} | {workspace.get_workspace_name()} | {corrupted_path}")


async def benchmark_file_writing(device, workspace):
    # Touch file
    path = "/foo"
    block_size = 512 * 1024
    file_size = 64 * 1024 * 1024
    await workspace.touch(path)
    info = await workspace.path_info(path)

    # Log
    print(f"{device.device_id} | {workspace.get_workspace_name()} | {path}")

    # Write file
    start = info["size"] // block_size
    stop = file_size // block_size
    for i in tqdm(range(start, stop)):
        await workspace.write_bytes(path, b"\x00" * block_size, offset=i * block_size)

    # Sync
    await workspace.sync()

    # Serialize
    manifest = await workspace.local_storage.get_manifest(info["id"])
    raw = manifest.dump()

    # Printout
    print(f"File size: {naturalsize(file_size)}")
    print(f"Manifest size: {naturalsize(len(raw))}")

    # Let sync monitor finish
    await trio.sleep(2)


async def test_source_after_cross_directory_move(workspace, base):
    await workspace.mkdir("/a")
    await workspace.touch("/a/myfile1.txt")
    await workspace.touch("/a/myfile2.txt")
    await workspace.touch("/a/myfile3.txt")
    await workspace.write_bytes("/a/myfile1.txt", b"Hello world 1!\n")
    await workspace.write_bytes("/a/myfile2.txt", b"Hello world 2!\n")
    await workspace.write_bytes("/a/myfile3.txt", b"Hello world 3!\n")
    await workspace.mkdir("/b")
    await workspace.sync()

    info1_before = await workspace.path_info("/a/myfile1.txt")
    info2_before = await workspace.path_info("/a/myfile2.txt")
    info3_before = await workspace.path_info("/a/myfile3.txt")

    # Move 1
    await workspace.move("/a/myfile1.txt", "/b")

    # Move 2
    src = str(base / "a" / "myfile2.txt")
    dst = str(base / "b")
    await trio.run_process(f"mv {src} {dst}".split())

    # Move 3
    src = str(base / "a" / "myfile3.txt")
    dst = str(base / "b")
    try:
        await trio.to_thread.run_sync(shutil.move, src, dst)
    except OSError:
        pass

    info1_after = await workspace.path_info("/b/myfile1.txt")
    info2_after = await workspace.path_info("/b/myfile2.txt")
    info3_after = await workspace.path_info("/b/myfile3.txt")

    manifest1 = await workspace.local_storage.get_manifest(info1_after["id"])
    manifest2 = await workspace.local_storage.get_manifest(info2_after["id"])
    manifest3 = await workspace.local_storage.get_manifest(info3_after["id"])

    assert manifest1.source == info1_before["id"]
    assert manifest2.source == info2_before["id"]
    assert manifest3.source == info3_before["id"]


async def test_source_after_file_writing(workspace, base):
    await workspace.touch("/a.txt")
    await workspace.write_bytes("/a.txt", b"foo")
    await workspace.sync()

    info_before = await workspace.path_info("/a.txt")

    code = f"""\
from gi.repository import Gio
f = Gio.File.new_for_path("{base}/a.txt")
f.replace_contents(b"bar", None, False, 0, None)
"""
    await trio.run_process(["/usr/bin/python3.6", "-c", code])
    assert await workspace.read_bytes("/a.txt") == b"bar"
    info_after = await workspace.path_info("/a.txt")

    assert info_before["id"] != info_after["id"]
    manifest = await workspace.local_storage.get_manifest(info_after["id"])
    assert manifest.source == info_before["id"]


async def main():

    # Config
    configure_logging(LOG_LEVEL)
    config_dir = get_default_config_dir(os.environ)
    config = load_config(config_dir)
    config = config.evolve(mountpoint_enabled=True)
    devices = list_available_devices(config_dir)
    key_file = next(key_file for _, device_id, _, key_file in devices if device_id == DEVICE_ID)
    device = load_device_with_password(key_file, PASSWORD)

    # Log in
    async with logged_core_factory(config, device) as core:

        # Get workspace
        user_manifest = core.user_fs.get_user_manifest()
        workspace_entry = user_manifest.workspaces[0]
        workspace_id = workspace_entry.id
        workspace = core.user_fs.get_workspace(workspace_id)

        # Mount workspace
        await core.mountpoint_manager.mount_workspace(workspace_id)
        base = core.mountpoint_manager.get_path_in_mountpoint(workspace_id, FsPath("/"))

        await test_source_after_file_writing(workspace, base)
        # await test_source_after_cross_directory_move(workspace, base)
        # await make_workspace_dir_inconsistent(device, workspace, "/bar")
        # await benchmark_file_writing(device, workspace)


if __name__ == "__main__":
    trio.run(main)
