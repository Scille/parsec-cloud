# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import unicodedata
from zlib import adler32
from pathlib import Path
from structlog import get_logger
from itertools import count
from winfspy import FileSystem, enable_debug_log
from winfspy.plumbing.winstuff import filetime_now

from parsec.core.mountpoint.exceptions import MountpointDriverCrash
from parsec.core.mountpoint.winfsp_operations import WinFSPOperations, winify_entry_name
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess


__all__ = ("winfsp_mountpoint_runner",)


logger = get_logger()


async def cleanup_broken_links(path: trio.Path) -> None:
    def target():
        for child in sync_path.iterdir():
            try:
                if not child.exists():
                    child.unlink()
            except OSError:
                pass

    # This should be migrated to trio.Path API once issue #1308 is fixed.
    sync_path = Path(str(path))
    await trio.to_thread.run_sync(target)


async def is_path_available(path: trio.Path) -> bool:
    # The path already exists
    if await path.exists():
        return False

    # The path is a broken link
    try:
        await path.lstat()
        return True
    except OSError:
        pass

    # The path is available
    return True


async def _bootstrap_mountpoint(base_mountpoint_path: Path, workspace_name) -> Path:
    # Mountpoint can be a drive letter, in such case nothing to do
    if str(base_mountpoint_path) == base_mountpoint_path.drive:
        return

    # On Windows, only mountpoint's parent must exists
    trio_base_mountpoint_path = trio.Path(base_mountpoint_path)
    await trio_base_mountpoint_path.mkdir(exist_ok=True, parents=True)

    # Clean up broken links in base directory
    await cleanup_broken_links(trio_base_mountpoint_path)

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
        trio_mountpoint_path = trio_base_mountpoint_path / dirname

        try:
            # Ignore if the path is not available
            if not await is_path_available(trio_mountpoint_path):
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

    mountpoint_path = await _bootstrap_mountpoint(base_mountpoint_path, workspace_name)

    if config.get("debug", False):
        enable_debug_log()

    # Volume label is limited to 32 WCHAR characters, so force the label to
    # ascii to easily enforce the size.
    volume_label = (
        unicodedata.normalize("NFKD", f"{device.user_id}-{workspace_name}")
        .encode("ascii", "ignore")[:32]
        .decode("ascii")
    )
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

        # Because of reject_irp_prior_to_transact0, the mountpoint isn't ready yet
        # We have to add a bit of delay here, the tests would fail otherwise
        # 10 ms is more than enough, although a strict process would be nicer
        # Still, this is only temporary as avast is working on a fix at the moment
        # Another way to address this problem would be to migrate to python 3.8,
        # then use `os.stat` to differentiate between a started and a non-started
        # file syste.
        await trio.sleep(0.01)

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
