import os
import trio
import time
import threading
from pathlib import Path
from fuse import FUSE
from structlog import get_logger
from winfspy import FileSystem, enable_debug_log, filetime_now

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


async def winfsp_mountpoint_runner(
    fs, mountpoint: Path, config: dict, event_bus, *, task_status=trio.TASK_STATUS_IGNORED
):
    """
    Raises:
        MountpointConfigurationError
    """
    fuse_thread_started = threading.Event()
    fuse_thread_stopped = threading.Event()
    portal = trio.BlockingTrioPortal()
    abs_mountpoint = str(mountpoint.absolute())
    fs_access = ThreadFSAccess(portal, fs)

    _bootstrap_mountpoint(mountpoint)

    if config.get("debug", False):
        enable_debug_log()

    volume_label = f"parsec-{fs.device.user_name}"[:31]
    operations = WinFSPOperations(volume_label, fs_access)
    fs = FileSystem(
        abs_mountpoint,
        operations,
        sector_size=512,
        sectors_per_allocation_unit=1,
        volume_creation_time=filetime_now(),
        volume_serial_number=0,
        file_info_timeout=1000,
        case_sensitive_search=1,
        case_preserved_names=1,
        unicode_on_disk=1,
        persistent_acls=1,
        post_cleanup_when_modified_only=1,
        um_file_context_is_user_context2=1,
        file_system_name=mountpoint,
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
