# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import json
from contextlib import contextmanager
from datetime import datetime
from functools import partial, wraps
from pathlib import PurePath
from typing import Any, Callable, Iterator, List, Tuple, TypeVar, Union, cast

from structlog import get_logger
from trio import Cancelled, RunFinishedError
from typing_extensions import Concatenate, ParamSpec
from winfspy import (
    CREATE_FILE_CREATE_OPTIONS,
    FILE_ATTRIBUTE,
    BaseFileSystemOperations,
    NTStatusError,
)
from winfspy.plumbing import NTSTATUS, SecurityDescriptor, dt_to_filetime

from parsec._parsec import CoreEvent, DateTime
from parsec.api.data import EntryID, EntryName
from parsec.core.fs import FSEndOfFileError, FSLocalOperationError, FsPath, FSRemoteOperationError
from parsec.core.fs.workspacefs.file_transactions import FileDescriptor
from parsec.core.fs.workspacefs.sync_transactions import DEFAULT_BLOCK_SIZE
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess, TrioDealockTimeoutError
from parsec.core.mountpoint.winify import unwinify_entry_name, winify_entry_name

logger = get_logger()

# Taken from https://docs.microsoft.com/en-us/windows/win32/fileio/file-access-rights-constants
FILE_READ_DATA = 1 << 0
FILE_WRITE_DATA = 1 << 1

# Used for the virtual file to retrieve entry info
# This is used as a way for external applications to get information
# on a file in a Parsec mountpoint by opening and reading the
# virtual file f"{file_path}{ENTRY_INFO_EXTENSION}".
ENTRY_INFO_EXTENSION = ".__parsec_entry_info__"


def is_entry_info_path(path: FsPath) -> bool:
    return path != FsPath("/") and path.name.str.endswith(ENTRY_INFO_EXTENSION)


def get_entry_info_initial_path(path: FsPath) -> FsPath:
    assert is_entry_info_path(path)

    entry_name = path.name.str.removesuffix(ENTRY_INFO_EXTENSION)
    return path.parent / entry_name


def get_stats_for_entry_info() -> dict[str, Any]:
    stats: dict[str, Any] = {
        "type": "file",
        # Arbitrary non-zero size
        "size": 1024,
        "created": DateTime.now(),
        "updated": DateTime.now(),
        "id": EntryID.new(),
    }
    return stats


def _winpath_to_parsec(path: str) -> FsPath:
    # Given / is not allowed, no need to check if path already contains it
    return FsPath(
        tuple(unwinify_entry_name(x) for x in path.replace("\\", "/").split("/") if x != "")
    )


class OpenedFolder:
    def __init__(self, path: FsPath) -> None:
        self.path = path
        self.deleted = False

    def is_root(self) -> bool:
        return self.path.is_root()


class OpenedFile:
    def __init__(self, path: FsPath, fd: FileDescriptor | None) -> None:
        self.path = path
        self.fd = fd
        self.deleted = False


class EntryInfo:
    class JSONEncoder(json.JSONEncoder):
        def default(self, obj: Any) -> Any:
            if isinstance(obj, EntryID):
                return obj.hex
            elif isinstance(obj, DateTime):
                return obj.to_rfc3339()
            elif isinstance(obj, EntryName):
                return obj.str
            return super().default(obj)

    def __init__(self, path: FsPath, entry_info: dict[str, Any]) -> None:
        self.path = path
        self._encoded = json.dumps(entry_info, cls=EntryInfo.JSONEncoder).encode()

    def read(self, offset: int, length: int) -> bytes:
        if offset >= len(self._encoded):
            raise FSEndOfFileError()
        return self._encoded[offset : offset + length]


@contextmanager
def get_path_and_translate_error(
    fs_access: ThreadFSAccess,
    operation: str,
    file_context: Union[OpenedFile, OpenedFolder, EntryInfo, str],
    mountpoint: PurePath,
    workspace_id: EntryID,
    timestamp: DateTime | None,
) -> Iterator[FsPath]:
    path: FsPath = FsPath("/<unknown>")
    try:
        if isinstance(file_context, (OpenedFile, OpenedFolder, EntryInfo)):
            path = file_context.path
        else:
            # FsPath conversion might raise an FSNameTooLongError so make
            # sure it runs within the try-except so it can be caught by the
            # FSLocalOperationError filter.
            path = _winpath_to_parsec(file_context)
        yield path

    except NTStatusError:
        raise

    except FSLocalOperationError as exc:
        raise NTStatusError(exc.ntstatus) from exc

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
        raise NTStatusError(exc.ntstatus) from exc

    except (Cancelled, RunFinishedError) as exc:
        # WinFSP teardown operation doesn't make sure no concurrent operation
        # are running
        raise NTStatusError(NTSTATUS.STATUS_NO_SUCH_DEVICE) from exc

    except TrioDealockTimeoutError as exc:
        # See the similar clause in `fuse_operations` for a detailed explanation
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
        raise NTStatusError(NTSTATUS.STATUS_INTERNAL_ERROR) from exc

    except Exception as exc:
        logger.exception(
            "Unhandled exception in winfsp mountpoint",
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
        raise NTStatusError(NTSTATUS.STATUS_INTERNAL_ERROR) from exc


def round_to_block_size(size: int, block_size: int = DEFAULT_BLOCK_SIZE) -> int:
    if size % block_size == 0:
        return size
    return ((size // block_size) + 1) * block_size


def stat_to_file_attributes(stat: dict[str, str]) -> FILE_ATTRIBUTE:
    # TODO: consider using FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS/FILE_ATTRIBUTE_RECALL_ON_OPEN ?
    # (see https://docs.microsoft.com/en-us/windows/desktop/fileio/file-attribute-constants)
    if stat["type"] == "folder":
        return FILE_ATTRIBUTE.FILE_ATTRIBUTE_DIRECTORY
    else:
        return FILE_ATTRIBUTE.FILE_ATTRIBUTE_NORMAL


def stat_to_winfsp_attributes(stat: dict[str, Any]) -> dict[str, Any]:
    created = dt_to_filetime(datetime.fromtimestamp(stat["created"].timestamp()))
    updated = dt_to_filetime(datetime.fromtimestamp(stat["updated"].timestamp()))
    attributes = {
        "creation_time": created,
        "last_access_time": updated,
        "last_write_time": updated,
        "change_time": updated,
        "index_number": stat["id"].int & 0xFFFFFFFF,  # uint64_t
    }

    if stat["type"] == "folder":
        attributes["file_attributes"] = FILE_ATTRIBUTE.FILE_ATTRIBUTE_DIRECTORY
        attributes["allocation_size"] = 0
        attributes["file_size"] = 0

    else:
        # FILE_ATTRIBUTE_ARCHIVE is a good default attribute
        # This way, we don't need to deal with the weird semantics of
        # FILE_ATTRIBUTE_NORMAL which means "no other attributes is set"
        # Also, this is what the winfsp memfs does.
        attributes["file_attributes"] = FILE_ATTRIBUTE.FILE_ATTRIBUTE_ARCHIVE
        attributes["allocation_size"] = round_to_block_size(stat["size"])
        attributes["file_size"] = stat["size"]

    # Disable content indexing for all files and directories
    attributes["file_attributes"] |= FILE_ATTRIBUTE.FILE_ATTRIBUTE_NOT_CONTENT_INDEXED

    return attributes


FileContext = Union[OpenedFile, OpenedFolder, EntryInfo, str]
R = TypeVar("R")
P = ParamSpec("P")
T = TypeVar("T", bound=BaseFileSystemOperations)
U = TypeVar("U", bound=FileContext)


def handle_error(func: Callable[Concatenate[T, U, P], R]) -> Callable[Concatenate[T, U, P], R]:
    """A decorator to handle error in wfspy operations"""
    operation = func.__name__

    @wraps(func)
    def wrapper(self: T, arg: U, *args: P.args, **kwargs: P.kwargs) -> R:
        with self._get_path_and_translate_error(operation=operation, file_context=arg):
            return func.__get__(self)(arg, *args, **kwargs)

    return wrapper


# `BaseFileSystemOperations` is resolved as `Any` on non-windows platform.
# We can't derive from `Any` type (misc).
class WinFSPOperations(BaseFileSystemOperations):  # type: ignore[misc]
    def __init__(
        self,
        fs_access: ThreadFSAccess,
        volume_label: str,
        mountpoint: PurePath,
        workspace_id: EntryID,
        timestamp: DateTime | None,
    ) -> None:
        super().__init__()
        # see https://docs.microsoft.com/fr-fr/windows/desktop/SecAuthZ/security-descriptor-string-format
        self._security_descriptor = SecurityDescriptor.from_string(
            # "O:BAG:BAD:P(A;;FA;;;SY)(A;;FA;;;BA)(A;;FA;;;WD)"
            "O:BAG:BAD:NO_ACCESS_CONTROL"
        )
        self.fs_access = fs_access

        # We have currently no way of easily getting the size of workspace
        # Also, the total size of a workspace is not limited
        # For the moment let's settle on 0 MB used for 1 TB available
        self._volume_info = {
            "total_size": 1 * 1024**4,  # 1 TB
            "free_size": 1 * 1024**4,  # 1 TB
            "volume_label": volume_label,
        }

        self._get_path_and_translate_error = partial(
            get_path_and_translate_error,
            fs_access=self.fs_access,
            mountpoint=mountpoint,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )

    def get_volume_info(self) -> dict[str, Any]:
        return self._volume_info

    def set_volume_label(self, volume_label: str) -> None:
        self._volume_info["volume_label"] = volume_label

    @handle_error
    def get_security_by_name(self, file_name: str) -> Tuple[dict[str, str], Any, int]:
        parsec_file_name = _winpath_to_parsec(file_name)
        is_entry_info = is_entry_info_path(parsec_file_name)
        if is_entry_info:
            parsec_file_name = get_entry_info_initial_path(parsec_file_name)
        stat = (
            self.fs_access.entry_info(parsec_file_name)
            if not is_entry_info
            else get_stats_for_entry_info()
        )
        return (
            stat_to_file_attributes(stat),
            self._security_descriptor.handle,
            self._security_descriptor.size,
        )

    @handle_error
    def create(
        self,
        file_name: str,
        create_options: Any,
        granted_access: Any,
        file_attributes: Any,
        security_descriptor: Any,
        allocation_size: Any,
    ) -> FileContext:
        # `granted_access` is already handle by winfsp
        # `allocation_size` useless for us
        # `security_descriptor` is not supported yet
        parsec_file_name = _winpath_to_parsec(file_name)
        if create_options & CREATE_FILE_CREATE_OPTIONS.FILE_DIRECTORY_FILE:
            self.fs_access.folder_create(parsec_file_name)
            return OpenedFolder(parsec_file_name)
        else:
            _, fd = self.fs_access.file_create(parsec_file_name, open=True)
            return OpenedFile(parsec_file_name, fd)

    @handle_error
    def get_security(self, file_context: FileContext) -> Tuple[Any, int]:
        return self._security_descriptor.handle, self._security_descriptor.size

    @handle_error
    def set_security(
        self, file_context: FileContext, security_information: Any, modification_descriptor: Any
    ) -> None:
        # TODO
        pass

    @handle_error
    def rename(
        self, file_context: FileContext, file_name: str, new_file_name: str, replace_if_exists: bool
    ) -> None:
        parsec_file_name = _winpath_to_parsec(file_name)
        parsec_new_file_name = _winpath_to_parsec(new_file_name)
        self.fs_access.entry_rename(
            parsec_file_name, parsec_new_file_name, overwrite=replace_if_exists
        )

    @handle_error
    def open(
        self, file_name: str, create_options: Any, granted_access: Any
    ) -> Union[OpenedFile, OpenedFolder, EntryInfo]:
        # `granted_access` is already handle by winfsp
        parsec_file_name = _winpath_to_parsec(file_name)
        write_mode = bool(granted_access & FILE_WRITE_DATA)

        if not write_mode and is_entry_info_path(parsec_file_name):
            parsec_file_name = get_entry_info_initial_path(parsec_file_name)
            entry_info = self.fs_access.entry_info(parsec_file_name)
            return EntryInfo(parsec_file_name, entry_info)

        try:
            _, fd = self.fs_access.file_open(parsec_file_name, write_mode=write_mode)
            return OpenedFile(parsec_file_name, fd)
        except IsADirectoryError:
            return OpenedFolder(parsec_file_name)

    @handle_error
    def close(self, file_context: FileContext) -> None:
        # The file might be deleted at this point. This is fine though as the
        # file descriptor can still be used after a deletion (posix style)
        if isinstance(file_context, (OpenedFile)) and file_context.fd is not None:
            self.fs_access.fd_close(file_context.fd)

    @handle_error
    def get_file_info(self, file_context: FileContext) -> dict[str, Any]:
        assert isinstance(file_context, (OpenedFile, OpenedFolder, EntryInfo))
        stat = (
            self.fs_access.entry_info(file_context.path)
            if not isinstance(file_context, EntryInfo)
            else get_stats_for_entry_info()
        )
        return stat_to_winfsp_attributes(stat)

    @handle_error
    def set_basic_info(
        self,
        file_context: FileContext,
        file_attributes: Any,
        creation_time: Any,
        last_access_time: Any,
        last_write_time: Any,
        change_time: Any,
        file_info: Any,
    ) -> Any:
        # TODO

        # file_obj = file_context.file_obj
        # if file_attributes != FILE_ATTRIBUTE.INVALID_FILE_ATTRIBUTES:
        #     file_obj.file_attributes = file_attributes
        # if creation_time:
        #     file_obj.creation_time = creation_time
        # if last_access_time:
        #     file_obj.last_access_time = last_access_time
        # if last_write_time:
        #     file_obj.last_write_time = last_write_time
        # if change_time:
        #     file_obj.change_time = change_time

        return self.get_file_info(file_context)

    @handle_error
    def set_file_size(
        self, file_context: FileContext, new_size: int, set_allocation_size: bool
    ) -> None:
        assert isinstance(file_context, OpenedFile)
        assert file_context.fd is not None
        self.fs_access.fd_resize(file_context.fd, new_size, truncate_only=set_allocation_size)

    @handle_error
    def can_delete(self, file_context: FileContext, file_name: str) -> None:
        assert isinstance(file_context, (OpenedFile, OpenedFolder))
        self.fs_access.check_write_rights(file_context.path)
        stat = self.fs_access.entry_info(file_context.path)
        if stat["type"] == "file":
            return
        if cast(OpenedFolder, file_context).is_root():
            # Cannot remove root mountpoint !
            raise NTStatusError(NTSTATUS.STATUS_RESOURCEMANAGER_READ_ONLY)
        if stat["children"]:
            raise NTStatusError(NTSTATUS.STATUS_DIRECTORY_NOT_EMPTY)

    @handle_error
    def read_directory(self, file_context: FileContext, marker: str | None) -> List[dict[str, Any]]:
        assert isinstance(file_context, OpenedFolder)
        entries = []
        stat = self.fs_access.entry_info(file_context.path)

        if stat["type"] == "file":
            raise NTStatusError(NTSTATUS.STATUS_NOT_A_DIRECTORY)

        # NOTE: The "." and ".." directories should ONLY be included
        # if the queried directory is not root

        # Current directory
        if marker is None and not file_context.path.is_root():
            entry = {"file_name": ".", **stat_to_winfsp_attributes(stat)}
            entries.append(entry)
        elif marker == ".":
            marker = None

        # Parent directory
        if marker is None and not file_context.path.is_root():
            parent_stat = self.fs_access.entry_info(file_context.path.parent)
            entry = {"file_name": "..", **stat_to_winfsp_attributes(parent_stat)}
            entries.append(entry)
        elif marker == "..":
            marker = None

        # NOTE: we *do not* rely on alphabetically sorting to compare the
        # marker given `..` is always the first element event if we could
        # have children name before it (`.-foo` for instance)
        iter_children_names = iter(stat["children"])
        if marker is not None:
            for child_name in iter_children_names:
                if child_name.str == marker:
                    break

        # All remaining children are located after the marker
        for child_name in iter_children_names:
            name = winify_entry_name(child_name)
            child_stat = self.fs_access.entry_info(file_context.path / child_name)
            entry = {"file_name": name, **stat_to_winfsp_attributes(child_stat)}
            entries.append(entry)

        return entries

    @handle_error
    def get_dir_info_by_name(self, file_context: FileContext, file_name: str) -> dict[str, Any]:
        assert isinstance(file_context, OpenedFolder)
        child_name = unwinify_entry_name(file_name)
        stat = self.fs_access.entry_info(file_context.path / child_name)
        entry = {"file_name": file_name, **stat_to_winfsp_attributes(stat)}
        return entry

    @handle_error
    def read(self, file_context: FileContext, offset: int, length: int) -> bytes:
        if isinstance(file_context, EntryInfo):
            return file_context.read(offset, length)
        assert isinstance(file_context, OpenedFile)
        assert file_context.fd is not None
        buffer = self.fs_access.fd_read(file_context.fd, length, offset, raise_eof=True)
        return buffer

    @handle_error
    def write(
        self,
        file_context: FileContext,
        buffer: bytes,
        offset: int,
        write_to_end_of_file: bool,
        constrained_io: bool,
    ) -> int:
        assert isinstance(file_context, OpenedFile)
        assert file_context.fd is not None

        # Adapt offset
        if write_to_end_of_file:
            offset = -1
        # LocalStorage.set only wants bytes or bytearray, not a cffi buffer
        buffer = bytes(buffer)
        # Atomic write
        return self.fs_access.fd_write(file_context.fd, buffer, offset, constrained_io)

    @handle_error
    def flush(self, file_context: FileContext) -> None:
        assert isinstance(file_context, OpenedFile)
        assert file_context.fd is not None

        self.fs_access.fd_flush(file_context.fd)

    @handle_error
    def cleanup(self, file_context: FileContext, file_name: str, flags: Any) -> None:
        # Cleanup operation is causal but close is not, so it's important
        # to delete file and folder here in order to make sure the file/folder
        # is actually deleted by the time the API call returns.
        FspCleanupDelete = 0x1
        if flags & FspCleanupDelete:
            # The file name is only provided for a delete operation, it is `None` otherwise
            parsec_file_name = _winpath_to_parsec(file_name)
            if isinstance(file_context, OpenedFile):
                self.fs_access.file_delete(parsec_file_name)
            else:
                self.fs_access.folder_delete(parsec_file_name)

    @handle_error
    def overwrite(
        self,
        file_context: FileContext,
        file_attributes: Any,
        replace_file_attributes: bool,
        allocation_size: int,
    ) -> None:
        assert isinstance(file_context, OpenedFile)
        assert file_context.fd is not None

        self.fs_access.fd_resize(file_context.fd, allocation_size, truncate_only=True)
