# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID
import errno
from typing import Iterator
import pendulum

from parsec.core.types import FsPath

from parsec.core.fs.workspacefs.entry_transactions import from_errno
from parsec.core.fs.workspacefs.workspacefs import WorkspaceFS, AnyPath


class WorkspaceFSTimestamped(WorkspaceFS):
    def _throw_permission_error(*e, **ke):
        raise from_errno(errno.EACCES, "Not available for timestamped workspaces.")

    rename = _throw_permission_error
    mkdir = _throw_permission_error
    rmdir = _throw_permission_error
    touch = _throw_permission_error
    unlink = _throw_permission_error
    truncate = _throw_permission_error
    write_bytes = _throw_permission_error
    move = _throw_permission_error
    copytree = _throw_permission_error
    copyfile = _throw_permission_error
    rmtree = _throw_permission_error
    minimal_sync = _throw_permission_error
    sync_by_id = _throw_permission_error
    sync = _throw_permission_error

    def __init__(self, workspacefs: WorkspaceFS, timestamp: pendulum.Pendulum):
        self.workspace_id = workspacefs.workspace_id
        self.get_workspace_entry = workspacefs.get_workspace_entry
        self.device = workspacefs.device
        self.local_storage = workspacefs.local_storage
        self.backend_cmds = workspacefs.backend_cmds
        self.event_bus = workspacefs.event_bus
        self.remote_device_manager = workspacefs.remote_device_manager

        self.timestamp = timestamp

        # Legacy
        self._local_folder_fs = workspacefs._local_folder_fs
        self._syncer = workspacefs._syncer

        self.remote_loader = workspacefs.remote_loader
        self.file_transactions = workspacefs.file_transactions
        self.entry_transactions = workspacefs.entry_transactions

    # Information

    async def path_info(self, path: AnyPath) -> dict:
        return await self.entry_transactions.entry_info(FsPath(path), self.timestamp)

    async def path_id(self, path: AnyPath) -> UUID:
        info = await self.entry_transactions.entry_info(FsPath(path), self.timestamp)
        return info["id"]

    # Pathlib-like interface

    async def exists(self, path: AnyPath) -> bool:
        path = FsPath(path)
        try:
            if await self.entry_transactions.entry_info(path, self.timestamp):
                return True
        except FileNotFoundError:
            return False
        return False

    async def is_dir(self, path: AnyPath) -> bool:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(path, self.timestamp)
        return info["type"] == "folder"

    async def is_file(self, path: AnyPath) -> bool:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(FsPath(path), self.timestamp)
        return info["type"] == "file"

    async def iterdir(self, path: AnyPath) -> Iterator[FsPath]:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(path, self.timestamp)
        if "children" not in info:
            raise NotADirectoryError(str(path))
        for child in info["children"]:
            yield path / child

    async def read_bytes(self, path: AnyPath, size: int = -1, offset: int = 0) -> bytes:
        path = FsPath(path)
        _, fd = await self.entry_transactions.file_open(path, "r", self.timestamp)
        try:
            return await self.file_transactions.fd_read(fd, size, offset)
        finally:
            await self.file_transactions.fd_close(fd)
