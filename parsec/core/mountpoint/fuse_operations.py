# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import errno
import os
import re
from contextlib import contextmanager
from functools import partial
from pathlib import PurePath
from stat import S_IFDIR, S_IFREG, S_IRWXU
from typing import Any, Iterator, List

import trio
from fuse import FuseOSError, LoggingMixIn, Operations, fuse_exit, fuse_get_context
from structlog import get_logger

from parsec._parsec import CoreEvent, DateTime
from parsec.api.data import EntryID, EntryName
from parsec.core.fs import FSLocalOperationError, FsPath, FSRemoteOperationError
from parsec.core.fs.exceptions import FSReadOnlyError
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess, TrioDealockTimeoutError
from parsec.core.types import FileDescriptor

logger = get_logger()


# We are preventing the creation of file and folders starting with those prefixes
# It might not be the best solution but it does fix our problems for the moment.
# In particular:
#
# - .Trash-XXXX which is used to store removed files and directories
#   We don't really want this to be shared among users and our system already provides
#   backup capabilities.
BANNED_PREFIXES = (".Trash-",)


def is_banned(name: EntryName) -> bool:
    return any(name.str.startswith(prefix) for prefix in BANNED_PREFIXES)


@contextmanager
def get_path_and_translate_error(
    fs_access: ThreadFSAccess,
    operation: str,
    context: str | None,
    mountpoint: PurePath,
    workspace_id: EntryID,
    timestamp: DateTime | None,
) -> Iterator[FsPath]:
    path: FsPath = FsPath("/<unknown>")
    try:
        # The context argument might be None or "-" in some special cases
        # related to `release` and `releasedir` (when the file descriptor
        # is available but the corresponding path is not). In those cases,
        # we can simply ignore the path.
        if context not in (None, "-"):
            # FsPath conversion might raise an FSNameTooLongError so make
            # sure it runs within the try-except so it can be caught by the
            # FSLocalOperationError filter.
            assert context is not None
            path = FsPath(context)
        yield path

    except FuseOSError:
        raise

    except FSReadOnlyError as exc:
        fs_access.send_event(
            CoreEvent.MOUNTPOINT_READONLY,
            exc=exc,
            operation=operation,
            path=path,
            mountpoint=mountpoint,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )
        raise FuseOSError(exc.errno) from exc

    except FSLocalOperationError as exc:
        raise FuseOSError(exc.errno) from exc

    except FSRemoteOperationError as exc:
        fs_access.send_event(
            CoreEvent.MOUNTPOINT_REMOTE_ERROR,
            exc=exc,
            operation=operation,
            path=path,
            mountpoint=mountpoint,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )
        raise FuseOSError(exc.errno) from exc

    except TrioDealockTimeoutError as exc:
        # This exception is raised when the trio thread cannot be reached.
        # This is likely due to a deadlock, i.e the trio thread performing
        # a synchronous call to access the fuse file system (this can easily
        # happen in a third party library, e.g `QDesktopServices::openUrl`).
        # The fuse/winfsp kernel driver being involved in the deadlock, the
        # effects can easily propagate to the file explorer or the system
        # in general. This is why it is much better to break out of it with
        # and to return an error code indicating that the operation failed.
        logger.error(
            "The trio thread is unreachable, a deadlock might have occurred",
            operation=operation,
            path=str(path),
            mountpoint=str(mountpoint),
            workspace_id=workspace_id,
            timestamp=timestamp,
        )
        fs_access.send_event(
            CoreEvent.MOUNTPOINT_TRIO_DEADLOCK_ERROR,
            exc=exc,
            operation=operation,
            path=path,
            mountpoint=mountpoint,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )
        # Use EINVAL as error code, so it behaves the same as internal errors
        raise FuseOSError(errno.EINVAL) from exc

    except (trio.Cancelled, trio.RunFinishedError) as exc:
        # Might be raised by `self.fs_access` if the trio loop finishes in our back
        raise FuseOSError(errno.EACCES) from exc

    except Exception as exc:
        logger.exception(
            "Unhandled exception in fuse mountpoint",
            operation=operation,
            path=str(path),
            mountpoint=str(mountpoint),
            workspace_id=workspace_id,
            timestamp=timestamp,
        )
        fs_access.send_event(
            CoreEvent.MOUNTPOINT_UNHANDLED_ERROR,
            exc=exc,
            operation=operation,
            path=path,
            mountpoint=mountpoint,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )
        # Use EINVAL as fallback error code, since this is what fusepy does.
        raise FuseOSError(errno.EINVAL) from exc


# We can't derive from any (because of unresolved imports)
class FuseOperations(LoggingMixIn, Operations):  # type: ignore[no-any-unimported, misc]
    def __init__(
        self,
        fs_access: ThreadFSAccess,
        mountpoint: PurePath,
        workspace_id: EntryID,
        timestamp: DateTime | None,
    ) -> None:
        super().__init__()
        self.fs_access = fs_access
        self.fds: dict[str, Any] = {}
        self._need_exit = False
        self._get_path_and_translate_error = partial(
            get_path_and_translate_error,
            fs_access=self.fs_access,
            mountpoint=mountpoint,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )

    def __call__(self, operation: str, context: str | None, *args: Any, **kwargs: Any) -> Any:
        with self._get_path_and_translate_error(operation=operation, context=context) as path:
            return super().__call__(operation, path, *args, **kwargs)

    def schedule_exit(self) -> None:
        # TODO: Currently call fuse_exit from a non fuse thread is not possible
        # (see https://github.com/fusepy/fusepy/issues/116).
        self._need_exit = True

    def init(self, path: FsPath) -> None:
        pass

    def statfs(self, path: FsPath) -> dict[str, int]:
        # We have currently no way of easily getting the size of workspace
        # Also, the total size of a workspace is not limited
        # For the moment let's settle on 0 MB used for 1 TB available
        return {
            "f_bsize": 512 * 1024,  # 512 KB, i.e the default block size
            "f_frsize": 512 * 1024,  # 512 KB, i.e the default block size
            "f_blocks": 2 * 1024**2,  # 2 MBlocks is 1 TB
            "f_bfree": 2 * 1024**2,  # 2 MBlocks is 1 TB
            "f_bavail": 2 * 1024**2,  # 2 MBlocks is 1 TB
            "f_namemax": 255,  # 255 bytes as maximum length for filenames
        }

    def getattr(self, path: FsPath, fh: int | None = None) -> dict[str, Any]:
        if self._need_exit:
            fuse_exit()

        stat = self.fs_access.entry_info(path)

        fuse_stat = {}
        # Set it to 777 access
        fuse_stat["st_mode"] = 0
        if stat["type"] == "folder":
            fuse_stat["st_mode"] |= S_IFDIR
            fuse_stat["st_size"] = 4096  # Because why not ?
            fuse_stat["st_nlink"] = 2
        else:
            fuse_stat["st_mode"] |= S_IFREG
            fuse_stat["st_size"] = stat["size"]
            fuse_stat["st_nlink"] = 1
        fuse_stat["st_blocks"] = fuse_stat["st_size"] // 512
        if fuse_stat["st_size"] % 512:
            fuse_stat["st_blocks"] += 1
        fuse_stat["st_mode"] |= S_IRWXU
        fuse_stat["st_ctime"] = stat["created"].timestamp()  # TODO change to local timezone
        fuse_stat["st_mtime"] = stat["updated"].timestamp()
        fuse_stat["st_atime"] = stat["updated"].timestamp()  # TODO not supported ?
        uid, gid, _ = fuse_get_context()
        fuse_stat["st_uid"] = uid
        fuse_stat["st_gid"] = gid
        return fuse_stat

    def chmod(self, path: FsPath, mode: int) -> None:
        # TODO: silently ignored for the moment
        return

    def chown(self, path: FsPath, uid: int, gid: int) -> None:
        # TODO: silently ignored for the moment
        return

    def readdir(self, path: FsPath, fh: int) -> List[str]:
        stat = self.fs_access.entry_info(path)

        if stat["type"] == "file":
            raise FuseOSError(errno.ENOTDIR)

        return [".", ".."] + list(c.str for c in stat["children"])

    def create(self, path: FsPath, mode: int) -> FileDescriptor | None:
        if is_banned(path.name):
            raise FuseOSError(errno.EACCES)
        _, fd = self.fs_access.file_create(path, open=True)
        return fd

    def open(self, path: FsPath, flags: int = 0) -> FileDescriptor:
        # Filter file status and file creation flags
        write_mode = (flags % 4) in (os.O_WRONLY, os.O_RDWR)
        _, fd = self.fs_access.file_open(path, write_mode=write_mode)
        return fd

    def release(self, path: FsPath, fh: FileDescriptor) -> None:
        self.fs_access.fd_close(fh)

    def read(self, path: FsPath, size: int, offset: int, fh: FileDescriptor) -> bytes:
        # Atomic read
        ret = self.fs_access.fd_read(fh, size, offset, raise_eof=False)
        # Fuse wants bytes but fd_read returns a bytearray
        return bytes(ret)

    def write(self, path: FsPath, data: bytes, offset: int, fh: FileDescriptor) -> int:
        return self.fs_access.fd_write(fh, data, offset)

    def truncate(self, path: FsPath, length: int, fh: FileDescriptor | None = None) -> None:
        if fh:
            self.fs_access.fd_resize(fh, length)
        else:
            self.fs_access.file_resize(path, length)

    def unlink(self, path: FsPath) -> None:
        self.fs_access.file_delete(path)

    def mkdir(self, path: FsPath, mode: int) -> int:
        if is_banned(path.name):
            raise FuseOSError(errno.EACCES)
        self.fs_access.folder_create(path)
        return 0

    def rmdir(self, path: FsPath) -> int:
        self.fs_access.folder_delete(path)
        return 0

    def rename(self, path: FsPath, destination: FsPath) -> int:
        destination = FsPath(destination)
        if not path.parent.is_root() and re.match(r".*\.sb-\w*-\w*", str(path.parent.name)):
            # This shouldn't happen with the defer_permission option in the FUSE function,
            # but if there ever is a permission error while saving with Apple software,
            # this is a fallback to avoid any unintended behavior related to this format
            # of temporary files. See https://github.com/Scille/parsec-cloud/pull/2211
            self.fs_access.workspace_move(path, destination)
        else:
            self.fs_access.entry_rename(path, destination, overwrite=True)
        return 0

    def flush(self, path: FsPath, fh: FileDescriptor) -> int:
        self.fs_access.fd_flush(fh)
        return 0

    def fsync(self, path: FsPath, datasync: Any, fh: FileDescriptor) -> int:
        self.fs_access.fd_flush(fh)
        return 0

    def fsyncdir(self, path: FsPath, datasync: Any, fh: FileDescriptor) -> int:
        return 0  # TODO
