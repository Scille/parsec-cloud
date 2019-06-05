# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import PureWindowsPath
from contextlib import contextmanager
from winfspy import (
    NTStatusError,
    BaseFileSystemOperations,
    FILE_ATTRIBUTE,
    CREATE_FILE_CREATE_OPTIONS,
)
from winfspy.plumbing.winstuff import (
    dt_to_filetime,
    NTSTATUS,
    posix_to_ntstatus,
    SecurityDescriptor,
)

from parsec.core.types import FsPath
from parsec.crypto import CryptoError
from parsec.core.fs import FSInvalidFileDescriptor
from parsec.core.fs.sync_base import DEFAULT_BLOCK_SIZE
from parsec.core.backend_connection import BackendNotAvailable


MODES = {0: "r", 1: "r", 2: "rw", 3: "rw"}


@contextmanager
def translate_error(event_bus, path):
    try:
        yield

    except BackendNotAvailable as exc:
        event_bus.send("fs.access_backend_offline", msg=str(exc), path=path)
        raise NTStatusError(NTSTATUS.STATUS_NETWORK_UNREACHABLE) from exc

    except FSInvalidFileDescriptor as exc:
        raise NTStatusError(NTSTATUS.STATUS_SOME_NOT_MAPPED) from exc

    except OSError as exc:
        raise NTStatusError(posix_to_ntstatus(exc.errno)) from exc

    except CryptoError as exc:
        event_bus.send("fs.access_crypto_error", msg=str(exc), path=path)
        raise NTStatusError(NTSTATUS.STATUS_NETWORK_UNREACHABLE) from exc


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
        "index_number": stat["id"].int & 0xFFFFFFFF,  # uint64_t
    }

    if stat["type"] == "folder":
        attributes["file_attributes"] = FILE_ATTRIBUTE.FILE_ATTRIBUTE_DIRECTORY
        attributes["allocation_size"] = round_to_block_size(1)
        attributes["file_size"] = round_to_block_size(1)

    else:
        attributes["file_attributes"] = FILE_ATTRIBUTE.FILE_ATTRIBUTE_NORMAL
        attributes["allocation_size"] = round_to_block_size(stat["size"])
        attributes["file_size"] = stat["size"]

    return attributes


class OpenedFolder:
    def __init__(self, path):
        self.path = path
        self.deleted = False

    def is_root(self):
        return len(PureWindowsPath(self.path).parts) == 1


class OpenedFile:
    def __init__(self, path, fd):
        self.path = path
        self.fd = fd
        self.deleted = False


class WinFSPOperations(BaseFileSystemOperations):
    def __init__(self, volume_label, fs_access):
        super().__init__()
        if len(volume_label) > 31:
            raise ValueError("`volume_label` must be 31 characters long max")

        # see https://docs.microsoft.com/fr-fr/windows/desktop/SecAuthZ/security-descriptor-string-format  # noqa
        self._security_descriptor = SecurityDescriptor(
            # "O:BAG:BAD:P(A;;FA;;;SY)(A;;FA;;;BA)(A;;FA;;;WD)"
            "O:BAG:BAD:NO_ACCESS_CONTROL"
        )
        self.fs_access = fs_access

        max_file_nodes = 1024
        max_file_size = 16 * 1024 * 1024
        file_nodes = 1
        self._volume_info = {
            "total_size": max_file_nodes * max_file_size,
            "free_size": (max_file_nodes - file_nodes) * max_file_size,
            "volume_label": volume_label,
        }

    def get_volume_info(self):
        return self._volume_info

    def set_volume_label(self, volume_label):
        self._volume_info["volume_label"] = volume_label

    def get_security_by_name(self, file_name):
        file_name = FsPath(file_name)
        with translate_error(self.event_bus, file_name):
            stat = self.fs_access.entry_info(file_name)

        return (
            stat_to_file_attributes(stat),
            self._security_descriptor.handle,
            self._security_descriptor.size,
        )

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
        file_name = FsPath(file_name)

        with translate_error(self.event_bus, file_name):
            if create_options & CREATE_FILE_CREATE_OPTIONS.FILE_DIRECTORY_FILE:
                self.fs_access.folder_create(file_name)
                return OpenedFolder(file_name)

            else:
                _, fd = self.fs_access.file_create(file_name, open=True)
                return OpenedFile(file_name, fd)

    def get_security(self, file_context):
        return self._security_descriptor.handle, self._security_descriptor.size

    def set_security(self, file_context, security_information, modification_descriptor):
        # TODO
        pass

    def rename(self, file_context, file_name, new_file_name, replace_if_exists):
        file_name = FsPath(file_name)
        new_file_name = FsPath(new_file_name)
        with translate_error(self.event_bus, file_name):
            self.fs_access.entry_rename(file_name, new_file_name, overwrite=False)

    def open(self, file_name, create_options, granted_access):
        file_name = FsPath(file_name)
        mode = MODES[granted_access % 4]
        # `granted_access` is already handle by winfsp
        with translate_error(self.event_bus, file_name):
            try:
                _, fd = self.fs_access.file_open(file_name, mode=mode)
                return OpenedFile(file_name, fd)

            except IsADirectoryError:
                return OpenedFolder(file_name)

    def close(self, file_context):
        if isinstance(file_context, OpenedFile):
            with translate_error(self.event_bus, file_context.path):
                self.fs_access.fd_close(file_context.fd)

        if file_context.deleted:
            if isinstance(file_context, OpenedFile):
                with translate_error(self.event_bus, file_context.path):
                    self.fs_access.file_delete(file_context.path)
            else:
                with translate_error(self.event_bus, file_context.path):
                    self.fs_access.folder_delete(file_context.path)

    def get_file_info(self, file_context):
        with translate_error(self.event_bus, file_context.path):
            stat = self.fs_access.entry_info(file_context.path)

        return stat_to_winfsp_attributes(stat)

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

    def set_file_size(self, file_context, new_size, set_allocation_size):

        if not set_allocation_size:
            with translate_error(self.event_bus, file_context.path):
                self.fs_access.fd_resize(file_context.fd, new_size)

    def can_delete(self, file_context, file_name: str) -> None:
        with translate_error(self.event_bus, file_context.path):
            stat = self.fs_access.entry_info(file_context.path)
            if stat["type"] == "file":
                return
            if file_context.is_root():
                # Cannot remove root mountpoint !
                raise NTStatusError(NTSTATUS.STATUS_RESOURCEMANAGER_READ_ONLY)
            if stat["children"]:
                raise NTStatusError(NTSTATUS.STATUS_DIRECTORY_NOT_EMPTY)

    def read_directory(self, file_context, marker):
        with translate_error(self.event_bus, file_context.path):
            stat = self.fs_access.entry_info(file_context.path)

            if stat["type"] == "file":
                raise NTStatusError(NTSTATUS.STATUS_NOT_A_DIRECTORY)

            entries = []

            for child_name in stat["children"]:
                if marker is not None and child_name < marker:
                    continue
                child_stat = self.fs_access.entry_info(file_context.path / child_name)
                entries.append({"file_name": child_name, **stat_to_winfsp_attributes(child_stat)})

            if not file_context.path.is_root():
                entries.append({"file_name": ".."})

        return entries

    def read(self, file_context, offset, length):
        with translate_error(self.event_bus, file_context.path):
            buffer = self.fs_access.fd_read(file_context.fd, length, offset)
            return buffer

    def write(self, file_context, buffer, offset, write_to_end_of_file, constrained_io):
        # `constrained_io` seems too complicated to implement for us
        with translate_error(self.event_bus, file_context.path):
            if write_to_end_of_file:
                offset = -1
            # LocalStorage.set only wants bytes or bytearray...
            buffer = bytes(buffer)
            ret = self.fs_access.fd_write(file_context.fd, buffer, offset)
            return ret

    def flush(self, file_context) -> None:
        with translate_error(self.event_bus, file_context.path):
            self.fs_access.fd_flush(file_context.fd)

    def cleanup(self, file_context, file_name, flags) -> None:
        # FspCleanupDelete
        file_context.deleted = flags & 1
