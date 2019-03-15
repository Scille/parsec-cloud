# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from zlib import adler32
from pathlib import Path
from structlog import get_logger
from winfspy import FileSystem, enable_debug_log
from winfspy.plumbing.winstuff import filetime_now

from parsec.core.mountpoint.winfsp_operations import WinFSPOperations
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess
from parsec.core.mountpoint.exceptions import MountpointConfigurationError


__all__ = ("winfsp_mountpoint_runner",)


logger = get_logger()


def _bootstrap_mountpoint(mountpoint):
    # Mountpoint can be a drive letter, in such case nothing to do
    if str(mountpoint) == mountpoint.drive:
        return

    try:
        # On Windows, only mountpoint's parent must exists
        mountpoint.parent.mkdir(exist_ok=True, parents=True)
        if mountpoint.exists():
            raise MountpointConfigurationError(
                f"Mountpoint `{mountpoint.absolute()}` must not exists on windows"
            )

    except OSError as exc:
        # In case of hard crash, it's possible the mountpoint is still mounted
        # (but point to nothing). In such case access to the mountpoint end
        # up with an error :(
        raise MountpointConfigurationError(
            "Mountpoint is busy, has parsec prevously cleanly exited ?"
        ) from exc


def _teardown_mountpoint(mountpoint):
    pass


def _generate_volume_serial_number(device, workspace):
    return adler32(f"{device.organization_id}-{device.device_id}-{workspace}".encode())


async def winfsp_mountpoint_runner(
    workspace: str,
    mountpoint: Path,
    config: dict,
    fs,
    event_bus,
    *,
    task_status=trio.TASK_STATUS_IGNORED,
):
    """
    Raises:
        MountpointConfigurationError
    """
    portal = trio.BlockingTrioPortal()
    abs_mountpoint = str(mountpoint.absolute())
    fs_access = ThreadFSAccess(portal, fs)

    _bootstrap_mountpoint(mountpoint)

    if config.get("debug", False):
        enable_debug_log()

    volume_label = f"{fs.device.user_id}-{workspace}"[:31]
    volume_serial_number = _generate_volume_serial_number(fs.device, workspace)
    operations = WinFSPOperations(workspace, volume_label, fs_access)
    # See https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getvolumeinformationa  # noqa
    fs = FileSystem(
        str(abs_mountpoint),
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
        event_bus.send("mountpoint.starting", mountpoint=abs_mountpoint)
        fs.start()
        event_bus.send("mountpoint.started", mountpoint=abs_mountpoint)
        task_status.started(abs_mountpoint)

        await trio.sleep_forever()

    finally:
        fs.stop()
        event_bus.send("mountpoint.stopped", mountpoint=abs_mountpoint)
