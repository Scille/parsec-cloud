# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import unicodedata
from zlib import adler32
from pathlib import Path
from structlog import get_logger
from winfspy import FileSystem, enable_debug_log
from winfspy.plumbing.winstuff import filetime_now

from parsec.core.mountpoint.exceptions import MountpointDriverCrash
from parsec.core.mountpoint.winfsp_operations import WinFSPOperations, winify_entry_name
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess


__all__ = ("winfsp_mountpoint_runner",)


logger = get_logger()


async def _find_next_available_drive(starting_from="G") -> Path:
    letters = map(chr, range(ord(starting_from), ord("Z") + 1))
    drives = [Path(f"{letter}:\\") for letter in letters]
    for drive in drives:
        try:
            if not await trio.to_thread.run_sync(drive.exists):
                return drive
        # Might fail for some unrelated reasons
        except OSError:
            continue


def _generate_volume_serial_number(device, workspace_id):
    return adler32(f"{device.organization_id}-{device.device_id}-{workspace_id}".encode())


async def _wait_for_winfsp_ready(mountpoint_path, timeout=1.0):
    trio_mountpoint_path = trio.Path(mountpoint_path)

    # Polling for `timeout` seconds until winfsp is ready
    with trio.fail_after(timeout):
        while True:
            try:
                if await trio_mountpoint_path.exists():
                    return
                await trio.sleep(0.01)
            # Looks like a revoked workspace has been mounted
            except PermissionError:
                return
            # Might be another OSError like errno 113 (No route to host)
            except OSError:
                return


async def winfsp_mountpoint_runner(
    workspace_fs,
    base_mountpoint_path: Path,
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
    workspace_name = winify_entry_name(workspace_fs.get_workspace_name())
    trio_token = trio.hazmat.current_trio_token()
    fs_access = ThreadFSAccess(trio_token, workspace_fs)

    mountpoint_path = await _find_next_available_drive()

    if config.get("debug", False):
        enable_debug_log()

    # Volume label is limited to 32 WCHAR characters, so force the label to
    # ascii to easily enforce the size.
    volume_label = (
        unicodedata.normalize("NFKD", f"{workspace_name.capitalize()}")
        .encode("ascii", "ignore")[:32]
        .decode("ascii")
    )
    volume_serial_number = _generate_volume_serial_number(device, workspace_fs.workspace_id)
    operations = WinFSPOperations(event_bus, volume_label, fs_access)
    # See https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getvolumeinformationa  # noqa
    fs = FileSystem(
        mountpoint_path.drive,
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
        device_control=0,
        um_file_context_is_user_context2=1,
        file_system_name="Parsec",
        prefix="",
        # The minimum value for IRP timeout is 1 minute (default is 5)
        irp_timeout=60000,
        # Work around the avast/winfsp incompatibility
        reject_irp_prior_to_transact0=True,
        # security_timeout_valid=1,
        # security_timeout=10000,
    )
    try:
        event_bus.send("mountpoint.starting", mountpoint=mountpoint_path)

        # Run fs start in a thread, as a cancellable operation
        # This is because fs.start() might get stuck for while in case of an IRP timeout
        await trio.to_thread.run_sync(fs.start, cancellable=True)
        await _wait_for_winfsp_ready(mountpoint_path)

        event_bus.send("mountpoint.started", mountpoint=mountpoint_path)
        task_status.started(mountpoint_path)

        await trio.sleep_forever()

    except Exception as exc:
        raise MountpointDriverCrash(f"WinFSP has crashed on {mountpoint_path}: {exc}") from exc

    finally:
        # Must run in thread given this call will wait for any winfsp operation
        # to finish so blocking the trio loop can produce a dead lock...
        with trio.CancelScope(shield=True):
            await trio.to_thread.run_sync(fs.stop)
        event_bus.send("mountpoint.stopped", mountpoint=mountpoint_path)
