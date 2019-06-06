# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from zlib import adler32
from pathlib import PurePath
from structlog import get_logger
from itertools import count
from winfspy import FileSystem, enable_debug_log
from winfspy.plumbing.winstuff import filetime_now

from parsec.core.mountpoint.exceptions import MountpointDriverCrash
from parsec.core.mountpoint.winfsp_operations import WinFSPOperations
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess


__all__ = ("winfsp_mountpoint_runner",)


logger = get_logger()


async def _bootstrap_mountpoint(base_mountpoint_path: PurePath, workspace_name) -> PurePath:
    # Mountpoint can be a drive letter, in such case nothing to do
    if str(base_mountpoint_path) == base_mountpoint_path.drive:
        return

    # Find a suitable path where to mount the workspace. The check we are doing
    # here are not atomic (and the mount operation is not itself atomic anyway),
    # hence there is still edgecases where the mount can crash due to concurrent
    # changes on the mountpoint path
    for tentative in count(1):
        if tentative == 1:
            dirname = workspace_name
        else:
            dirname = f"{workspace_name} ({tentative})"
        mountpoint_path = base_mountpoint_path / dirname

        try:
            # On Windows, only mountpoint's parent must exists
            trio_mountpoint_path = trio.Path(mountpoint_path)
            await trio_mountpoint_path.parent.mkdir(exist_ok=True, parents=True)
            if await trio_mountpoint_path.exists():
                continue
            return mountpoint_path

        except OSError:
            # In case of hard crash, it's possible the FUSE mountpoint is still
            # mounted (but points to nothing). In such case just mount in
            # another place
            continue


def _generate_volume_serial_number(device, workspace_id):
    return adler32(f"{device.organization_id}-{device.device_id}-{workspace_id}".encode())


async def winfsp_mountpoint_runner(
    workspace_fs,
    base_mountpoint_path: PurePath,
    config: dict,
    event_bus,
    *,
    task_status=trio.TASK_STATUS_IGNORED,
):
    """
    Raises:
        MountpointDriverCrash
    """
    device = workspace_fs.device
    workspace_name = workspace_fs.workspace_name
    portal = trio.BlockingTrioPortal()
    fs_access = ThreadFSAccess(portal, workspace_fs)

    mountpoint_path = await _bootstrap_mountpoint(base_mountpoint_path, workspace_name)

    if config.get("debug", False):
        enable_debug_log()

    volume_label = f"{device.user_id}-{workspace_name}"[:31]
    volume_serial_number = _generate_volume_serial_number(device, workspace_fs.workspace_id)
    operations = WinFSPOperations(event_bus, volume_label, fs_access)
    # See https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getvolumeinformationa  # noqa
    fs = FileSystem(
        str(mountpoint_path.absolute()),
        operations,
        sector_size=512,
        sectors_per_allocation_unit=1,
        volume_creation_time=filetime_now(),
        volume_serial_number=volume_serial_number,
        file_info_timeout=1000,
        case_sensitive_search=1,
        case_preserved_names=1,
        unicode_on_disk=1,
        persistent_acls=0,
        reparse_points=0,
        reparse_points_access_check=0,
        named_streams=0,
        read_only_volume=0,
        post_cleanup_when_modified_only=1,
        pass_query_directory_file_name=0,  # TODO: implement `operations.get_dir_info_by_name`
        device_control=0,
        um_file_context_is_user_context2=1,
        file_system_name="parsec-mnt",
        prefix="",
        # security_timeout_valid=1,
        # security_timeout=10000,
    )
    try:
        event_bus.send("mountpoint.starting", mountpoint=mountpoint_path)
        fs.start()
        event_bus.send("mountpoint.started", mountpoint=mountpoint_path)
        task_status.started(mountpoint_path)

        await trio.sleep_forever()

    except Exception as exc:
        raise MountpointDriverCrash(f"WinFSP has crashed on {mountpoint_path}: {exc}") from exc

    finally:
        fs.stop()
        event_bus.send("mountpoint.stopped", mountpoint=mountpoint_path)
