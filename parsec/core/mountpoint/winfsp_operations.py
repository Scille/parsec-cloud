# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from pathlib import PurePath
from pendulum import DateTime
from functools import partial, wraps
from contextlib import contextmanager
from typing import Optional, Union, Iterator
from trio import Cancelled, RunFinishedError
from structlog import get_logger
from winfspy import (
    NTStatusError,
    BaseFileSystemOperations,
    FILE_ATTRIBUTE,
    CREATE_FILE_CREATE_OPTIONS,
)
from winfspy.plumbing import dt_to_filetime, NTSTATUS, SecurityDescriptor

from parsec.api.data import EntryID
from parsec.core.core_events import CoreEvent
from parsec.core.fs import FsPath, FSLocalOperationError, FSRemoteOperationError
from parsec.core.fs.workspacefs.sync_transactions import DEFAULT_BLOCK_SIZE
from parsec.core.mountpoint.winify import winify_entry_name, unwinify_entry_name
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess, TrioDealockTimeoutError


logger = get_logger()

# Taken from https://docs.microsoft.com/en-us/windows/win32/fileio/file-access-rights-constants
FILE_READ_DATA = 1 << 0
FILE_WRITE_DATA = 1 << 1


def _winpath_to_parsec(path: str) -> FsPath:
    # Given / is not allowed, no need to check if path already contains it
    return FsPath(
        tuple(unwinify_entry_name(x) for x in path.replace("\\", "/").split("/") if x != "")
    )


def _parsec_to_winpath(path: FsPath) -> str:
    return "\\" + "\\".join(winify_entry_name(entry) for entry in path.parts)


class OpenedFolder:
    def __init__(self, path):
        self.path = path
        self.deleted = False

    def is_root(self):
        return self.path.is_root()


class OpenedFile:
    def __init__(self, path, fd):
        self.path = path
        self.fd = fd
        self.deleted = False


@contextmanager
def get_path_and_translate_error(
    fs_access: ThreadFSAccess,
    operation: str,
    file_context: Union[OpenedFile, OpenedFolder, str],
    mountpoint: PurePath,
    workspace_id: EntryID,
    timestamp: Optional[DateTime],
) -> Iterator[FsPath]:
    path: FsPath = FsPath("/<unkonwn>")
    try:
        if isinstance(file_context, (OpenedFile, OpenedFolder)):
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
            "The trio thread is unreachable, a deadlock might have occured",
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


def round_to_block_size(size, block_size=DEFAULT_BLOCK_SIZE):
    if size % block_size == 0:
        return size
    return ((size // block_size) + 1) * block_size


def stat_to_file_attributes(stat):
    # TODO: consider using FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS/FILE_ATTRIBUTE_RECALL_ON_OPEN ?
    # (see https://docs.microsoft.com/en-us/windows/desktop/fileio/file-attribute-constants)
    if stat["type"] == "folder":
        return FILE_ATTRIBUTE.FILE_ATTRIBUTE_DIRECTORY
    else:
        return FILE_ATTRIBUTE.FILE_ATTRIBUTE_NORMAL


def stat_to_winfsp_attributes(stat):
    created = dt_to_filetime(stat["created"])
    updated = dt_to_filetime(stat["updated"])
    attributes = {
        "creation_time": created,
        "last_access_time": updated,
        "last_write_time": updated,
        "change_time": updated,
        "index_number": stat["id"].uuid.int & 0xFFFFFFFF,  # uint64_t
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


def handle_error(func):
    """A decorator to handle error in wfspy operations"""
    operation = func.__name__

    @wraps(func)
    def wrapper(self, arg, *args, **kwargs):
        with self._get_path_and_translate_error(operation=operation, file_context=arg):
            return func.__get__(self)(arg, *args, **kwargs)

    return wrapper


class WinFSPOperations(BaseFileSystemOperations):
    def __init__(
        self,
        fs_access: ThreadFSAccess,
        volume_label: str,
        mountpoint: PurePath,
        workspace_id: EntryID,
        timestamp: Optional[DateTime],
    ):
        super().__init__()
        # see https://docs.microsoft.com/fr-fr/windows/desktop/SecAuthZ/security-descriptor-string-format  # noqa
        self._security_descriptor = SecurityDescriptor.from_string(
            # "O:BAG:BAD:P(A;;FA;;;SY)(A;;FA;;;BA)(A;;FA;;;WD)"
            "O:BAG:BAD:NO_ACCESS_CONTROL"
        )
        self.fs_access = fs_access

        # We have currently no way of easily getting the size of workspace
        # Also, the total size of a workspace is not limited
        # For the moment let's settle on 0 MB used for 1 TB available
        self._volume_info = {
            "total_size": 1 * 1024 ** 4,  # 1 TB
            "free_size": 1 * 1024 ** 4,  # 1 TB
            "volume_label": volume_label,
        }

        self._get_path_and_translate_error = partial(
            get_path_and_translate_error,
            fs_access=self.fs_access,
            mountpoint=mountpoint,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )

    def get_volume_info(self):
        return self._volume_info

    def set_volume_label(self, volume_label):
        self._volume_info["volume_label"] = volume_label

    @handle_error
    def get_security_by_name(self, file_name):
        file_name = _winpath_to_parsec(file_name)
        stat = self.fs_access.entry_info(file_name)
        return (
            stat_to_file_attributes(stat),
            self._security_descriptor.handle,
            self._security_descriptor.size,
        )

    @handle_error
    def create(
        self,
        file_name,
        create_options,
        granted_access,
        file_attributes,
        security_descriptor,
        allocation_size,
    ):
        # `granted_access` is already handle by winfsp
        # `allocation_size` useless for us
        # `security_descriptor` is not supported yet
        file_name = _winpath_to_parsec(file_name)

        if create_options & CREATE_FILE_CREATE_OPTIONS.FILE_DIRECTORY_FILE:
            self.fs_access.folder_create(file_name)
            return OpenedFolder(file_name)

        else:
            _, fd = self.fs_access.file_create(file_name, open=True)
            return OpenedFile(file_name, fd)

    @handle_error
    def get_security(self, file_context):
        return self._security_descriptor.handle, self._security_descriptor.size

    @handle_error
    def set_security(self, file_context, security_information, modification_descriptor):
        # TODO
        pass

    @handle_error
    def rename(self, file_context, file_name, new_file_name, replace_if_exists):
        file_name = _winpath_to_parsec(file_name)
        new_file_name = _winpath_to_parsec(new_file_name)
        self.fs_access.entry_rename(file_name, new_file_name, overwrite=replace_if_exists)

    @handle_error
    def open(self, file_name, create_options, granted_access):
        # `granted_access` is already handle by winfsp
        file_name = _winpath_to_parsec(file_name)
        write_mode = bool(granted_access & FILE_WRITE_DATA)
        try:
            _, fd = self.fs_access.file_open(file_name, write_mode=write_mode)
            return OpenedFile(file_name, fd)
        except IsADirectoryError:
            return OpenedFolder(file_name)

    @handle_error
    def close(self, file_context):
        # The file might be deleted at this point. This is fine though as the
        # file descriptor can still be used after a deletion (posix style)
        if isinstance(file_context, OpenedFile):
            self.fs_access.fd_close(file_context.fd)

    @handle_error
    def get_file_info(self, file_context):
        stat = self.fs_access.entry_info(file_context.path)
        return stat_to_winfsp_attributes(stat)

    @handle_error
    def set_basic_info(
        self,
        file_context,
        file_attributes,
        creation_time,
        last_access_time,
        last_write_time,
        change_time,
        file_info,
    ) -> dict:
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
    def set_file_size(self, file_context, new_size, set_allocation_size):
        self.fs_access.fd_resize(file_context.fd, new_size, truncate_only=set_allocation_size)

    @handle_error
    def can_delete(self, file_context, file_name: str) -> None:
        self.fs_access.check_write_rights(file_context.path)
        stat = self.fs_access.entry_info(file_context.path)
        if stat["type"] == "file":
            return
        if file_context.is_root():
            # Cannot remove root mountpoint !
            raise NTStatusError(NTSTATUS.STATUS_RESOURCEMANAGER_READ_ONLY)
        if stat["children"]:
            raise NTStatusError(NTSTATUS.STATUS_DIRECTORY_NOT_EMPTY)

    @handle_error
    def read_directory(self, file_context, marker):
        entries = []
        stat = self.fs_access.entry_info(file_context.path)

        if stat["type"] == "file":
            raise NTStatusError(NTSTATUS.STATUS_NOT_A_DIRECTORY)

        # NOTE: The "." and ".." directories should ONLY be included
        # if the queried directory is not root

        print(
            f"READ DIR marker: {marker}, file ctx: {file_context.path!r} (is_root: {file_context.path.is_root()})"
        )
        print(f"READ DIR stats: {stat!r}")

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

        print(f"READ DIR entries: {[e['file_name'] for e in entries]!r}")
        return entries

    @handle_error
    def get_dir_info_by_name(self, file_context, file_name):
        child_name = unwinify_entry_name(file_name)
        stat = self.fs_access.entry_info(file_context.path / child_name)
        entry = {"file_name": file_name, **stat_to_winfsp_attributes(stat)}
        return entry

    @handle_error
    def read(self, file_context, offset, length):
        buffer = self.fs_access.fd_read(file_context.fd, length, offset, raise_eof=True)
        return buffer

    @handle_error
    def write(self, file_context, buffer, offset, write_to_end_of_file, constrained_io):
        # Adapt offset
        if write_to_end_of_file:
            offset = -1
        # LocalStorage.set only wants bytes or bytearray, not a cffi buffer
        buffer = bytes(buffer)
        # Atomic write
        return self.fs_access.fd_write(file_context.fd, buffer, offset, constrained_io)

    @handle_error
    def flush(self, file_context) -> None:
        self.fs_access.fd_flush(file_context.fd)

    @handle_error
    def cleanup(self, file_context, file_name, flags) -> None:
        # Cleanup operation is causal but close is not, so it's important
        # to delete file and folder here in order to make sure the file/folder
        # is actually deleted by the time the API call returns.
        FspCleanupDelete = 0x1
        if flags & FspCleanupDelete:
            # The file name is only provided for a delete operation, it is `None` otherwise
            file_name = _winpath_to_parsec(file_name)
            if isinstance(file_context, OpenedFile):
                self.fs_access.file_delete(file_name)
            else:
                self.fs_access.folder_delete(file_name)

    @handle_error
    def overwrite(
        self, file_context, file_attributes, replace_file_attributes: bool, allocation_size: int
    ) -> None:
        self.fs_access.fd_resize(file_context.fd, allocation_size, truncate_only=True)
