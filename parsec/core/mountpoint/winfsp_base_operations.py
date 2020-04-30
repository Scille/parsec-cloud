# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import functools
from winfspy import NTStatusError, BaseFileSystemOperations, FILE_ATTRIBUTE
from winfspy.plumbing.winstuff import filetime_now, NTSTATUS, SecurityDescriptor
from parsec.core.mountpoint.winfsp_operations import OpenedFile, OpenedFolder, FILE_WRITE_DATA


def split_file_name(arg):
    assert arg.startswith("\\")
    if arg == "\\":
        return None, arg
    name_list = arg.split("\\")
    name = name_list.pop(1)
    return name, "\\".join(name_list) or "\\"


def forward_operation(func):
    @functools.wraps(func)
    def wrapper(self, *args, **kwargs):
        return self._forward_operation(func, *args, **kwargs)

    return wrapper


class WinfspBaseOperations(BaseFileSystemOperations):
    def __init__(self, event_bus, volume_label):
        super().__init__()

        self.event_bus = event_bus
        self.operations_mapping = {}

        self._timestamp = filetime_now()
        self._root_folder = object()
        self._root_attributes = FILE_ATTRIBUTE.FILE_ATTRIBUTE_DIRECTORY
        self._root_attributes |= FILE_ATTRIBUTE.FILE_ATTRIBUTE_READONLY
        self._root_stat = {
            "creation_time": self._timestamp,
            "last_access_time": self._timestamp,
            "last_write_time": self._timestamp,
            "change_time": self._timestamp,
            "index_number": 1,
            "file_attributes": self._root_attributes,
            "allocation_size": 0,
            "file_size": 0,
        }

        # see https://docs.microsoft.com/fr-fr/windows/desktop/SecAuthZ/security-descriptor-string-format  # noqa
        self._security_descriptor = SecurityDescriptor.from_string(
            # "O:BAG:BAD:P(A;;FA;;;SY)(A;;FA;;;BA)(A;;FA;;;WD)"
            "O:BAG:BAD:NO_ACCESS_CONTROL"
        )

        max_file_nodes = 1024
        max_file_size = 16 * 1024 * 1024
        file_nodes = 1
        self._volume_info = {
            "total_size": max_file_nodes * max_file_size,
            "free_size": (max_file_nodes - file_nodes) * max_file_size,
            "volume_label": volume_label,
        }

    # Helpers

    def _forward_operation(self, func, arg, *args):

        print(">>>", func.__name__, arg, args)
        try:
            result = self.__forward_operation(func, arg, *args)
        except Exception as exc:
            print("<<<", func.__name__, exc)
            raise
        else:
            print("<<<", func.__name__, result)
            return result

    def __forward_operation(self, func, arg, *args):
        # Operation name
        operation = func.__name__

        # Operation on root
        if arg in ("\\", self._root_folder):
            return func.__get__(self)(arg, *args)

        # Operation from an open file or folder
        if isinstance(arg, (OpenedFile, OpenedFolder)):
            operations = arg.operations
        # Operation on a path
        else:
            assert isinstance(arg, str)
            name, arg = split_file_name(arg)
            if name not in self.operations_mapping:
                status_mapping = {"create": NTSTATUS.STATUS_MEDIA_WRITE_PROTECTED}
                status = status_mapping.get(operation, NTSTATUS.STATUS_OBJECT_NAME_NOT_FOUND)
                raise NTStatusError(status)
            operations = self.operations_mapping[name]

        # Special case of cleanup and can_delete operation
        if operation in ("cleanup", "can_delete"):
            file_name, *more = args
            _, file_name = split_file_name(file_name)
            args = file_name, *more

        # Special case of rename operation
        if operation == "rename":
            # Split source and destination file names
            file_name, new_file_name, replace_if_exists = args
            source, file_name = split_file_name(file_name)
            dest, new_file_name = split_file_name(new_file_name)

            # Detect cross workspace renaming
            if source != dest:
                raise NTStatusError(NTSTATUS.STATUS_NOT_SAME_DEVICE)

            # New arguments
            args = (file_name, new_file_name, replace_if_exists)

        # Delegate operation to a suboperations class
        method = getattr(operations, operation)
        return method(arg, *args)

    # Operations performed in the root context

    def get_volume_info(self):
        return self._volume_info

    def set_volume_label(self, volume_label):
        self._volume_info["volume_label"] = volume_label

    @forward_operation
    def get_security_by_name(self, file_name):
        return (
            self._root_attributes,
            self._security_descriptor.handle,
            self._security_descriptor.size,
        )

    @forward_operation
    def create(
        self,
        file_name,
        create_options,
        granted_access,
        file_attributes,
        security_descriptor,
        allocation_size,
    ):
        raise NTStatusError(NTSTATUS.STATUS_MEDIA_WRITE_PROTECTED)

    @forward_operation
    def get_security(self, file_context):
        return self._security_descriptor.handle, self._security_descriptor.size

    @forward_operation
    def set_security(self, file_context, security_information, modification_descriptor):
        pass

    @forward_operation
    def rename(self, file_context, file_name, new_file_name, replace_if_exists):
        raise NTStatusError(NTSTATUS.STATUS_MEDIA_WRITE_PROTECTED)

    @forward_operation
    def open(self, file_name, create_options, granted_access):
        if granted_access & FILE_WRITE_DATA:
            raise NTStatusError(NTSTATUS.STATUS_MEDIA_WRITE_PROTECTED)
        return self._root_folder

    @forward_operation
    def close(self, file_context):
        assert file_context == self._root_folder

    @forward_operation
    def get_file_info(self, file_context):
        assert file_context == self._root_folder
        return self._root_stat

    @forward_operation
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
        assert file_context == self._root_folder
        raise NTStatusError(NTSTATUS.STATUS_MEDIA_WRITE_PROTECTED)

    @forward_operation
    def set_file_size(self, file_context, new_size, set_allocation_size):
        assert False

    @forward_operation
    def can_delete(self, file_context, file_name: str) -> None:
        assert file_context == self._root_folder
        raise NTStatusError(NTSTATUS.STATUS_MEDIA_WRITE_PROTECTED)

    @forward_operation
    def read_directory(self, file_context, marker):
        assert file_context == self._root_folder

        # NOTE: The "." and ".." directories should ONLY be included
        # if the queried directory is not root
        entries = []
        for name, operations in self.operations_mapping.items():
            if marker is not None:
                marker = None if marker == name else marker
                continue
            try:
                stat = operations.get_file_info("\\")
            except NTStatusError:
                continue
            entry = {"file_name": name, **stat}
            entries.append(entry)

        return entries

    @forward_operation
    def get_dir_info_by_name(self, file_context, file_name):
        assert file_context == self._root_folder
        operations = self.operations_mapping.get(file_name)
        if not operations:
            raise NTStatusError(NTSTATUS.STATUS_OBJECT_NAME_NOT_FOUND)
        stat = operations.get_file_info("\\")
        entry = {"file_name": file_name, **stat}
        return entry

    @forward_operation
    def read(self, file_context, offset, length):
        assert False

    @forward_operation
    def write(self, file_context, buffer, offset, write_to_end_of_file, constrained_io):
        assert False

    @forward_operation
    def flush(self, file_context) -> None:
        assert False

    @forward_operation
    def cleanup(self, file_context, file_name, flags) -> None:
        FspCleanupDelete = 0x1
        if flags & FspCleanupDelete:
            raise NTStatusError(NTSTATUS.STATUS_MEDIA_WRITE_PROTECTED)

    @forward_operation
    def overwrite(
        self, file_context, file_attributes, replace_file_attributes: bool, allocation_size: int
    ) -> None:
        assert False
