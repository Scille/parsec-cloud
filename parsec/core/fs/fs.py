# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import math
import inspect
from uuid import UUID

from parsec.event_bus import EventBus
from parsec.core.types import LocalDevice, FsPath
from parsec.core.local_db import LocalDB
from parsec.core.backend_connection import BackendCmdsPool
from parsec.core.fs.local_folder_fs import (
    FSManifestLocalMiss,
    FSMultiManifestLocalMiss,
    LocalFolderFS,
)
from parsec.core.fs.local_file_fs import LocalFileFS, FSBlocksLocalMiss
from parsec.core.fs.syncer import Syncer
from parsec.core.fs.sharing import Sharing
from parsec.core.fs.remote_loader import RemoteLoader


class FS:
    def __init__(
        self,
        device: LocalDevice,
        local_db: LocalDB,
        backend_cmds: BackendCmdsPool,
        encryption_manager,
        event_bus: EventBus,
    ):
        self.device = device
        self.local_db = local_db
        self.backend_cmds = backend_cmds
        self.event_bus = event_bus

        self._local_folder_fs = LocalFolderFS(device, local_db, event_bus)
        self._local_file_fs = LocalFileFS(device, local_db, self._local_folder_fs, event_bus)
        self._remote_loader = RemoteLoader(backend_cmds, encryption_manager, local_db)
        self._syncer = Syncer(
            device,
            backend_cmds,
            encryption_manager,
            self._local_folder_fs,
            self._local_file_fs,
            event_bus,
        )
        self._sharing = Sharing(
            device,
            backend_cmds,
            encryption_manager,
            self._local_folder_fs,
            self._syncer,
            self._remote_loader,
            event_bus,
        )

    async def _load_and_retry(self, fn, *args, **kwargs):
        while True:
            try:
                if inspect.iscoroutinefunction(fn):
                    return await fn(*args, **kwargs)
                else:
                    return fn(*args, **kwargs)

            except FSManifestLocalMiss as exc:
                await self._remote_loader.load_manifest(exc.access)

            except FSMultiManifestLocalMiss as exc:
                for access in exc.accesses:
                    await self._remote_loader.load_manifest(access)

            except FSBlocksLocalMiss as exc:
                for access in exc.accesses:
                    await self._remote_loader.load_block(access)

    async def stat(self, path: str) -> dict:
        cooked_path = FsPath(path)
        return await self._load_and_retry(self._local_folder_fs.stat, cooked_path)

    async def file_write(self, path: str, content: bytes, offset: int = 0) -> int:
        fd = await self.file_fd_open(path)
        try:
            if offset:
                await self.file_fd_seek(fd, offset)
            return await self.file_fd_write(fd, content)
        finally:
            await self.file_fd_close(fd)

    async def file_truncate(self, path: str, length: int) -> None:
        fd = await self.file_fd_open(path)
        try:
            await self.file_fd_truncate(fd, length)
        finally:
            await self.file_fd_close(fd)

    async def file_read(self, path: str, size: int = math.inf, offset: int = 0) -> bytes:
        fd = await self.file_fd_open(path)
        try:
            if offset:
                await self.file_fd_seek(fd, offset)
            return await self.file_fd_read(fd, size)
        finally:
            await self.file_fd_close(fd)

    async def file_fd_open(self, path: str) -> int:
        cooked_path = FsPath(path)
        access = await self._load_and_retry(self._local_folder_fs.get_access, cooked_path)
        return self._local_file_fs.open(access)

    async def file_fd_close(self, fd: int) -> None:
        self._local_file_fs.close(fd)

    async def file_fd_seek(self, fd: int, offset: int) -> None:
        self._local_file_fs.seek(fd, offset)

    async def file_fd_truncate(self, fd: int, length: int) -> None:
        self._local_file_fs.truncate(fd, length)

    async def file_fd_write(self, fd: int, content: bytes, offset: int = None) -> int:
        return self._local_file_fs.write(fd, content, offset)

    async def file_fd_flush(self, fd: int) -> None:
        self._local_file_fs.flush(fd)

    async def file_fd_read(self, fd: int, size: int = -1, offset: int = None) -> bytes:
        return await self._load_and_retry(self._local_file_fs.read, fd, size, offset)

    async def touch(self, path: str) -> None:
        cooked_path = FsPath(path)
        await self._load_and_retry(self._local_folder_fs.touch, cooked_path)

    async def file_create(self, path: str) -> None:
        await self.touch(path)

    async def mkdir(self, path: str) -> None:
        cooked_path = FsPath(path)
        await self._load_and_retry(self._local_folder_fs.mkdir, cooked_path)

    async def folder_create(self, path: str) -> None:
        await self.mkdir(path)

    async def workspace_create(self, path: str) -> None:
        cooked_path = FsPath(path)
        await self._load_and_retry(self._local_folder_fs.workspace_create, cooked_path)

    async def workspace_rename(self, src: str, dst: str) -> None:
        cooked_src = FsPath(src)
        cooked_dst = FsPath(dst)
        await self._load_and_retry(self._local_folder_fs.workspace_rename, cooked_src, cooked_dst)

    async def move(self, src: str, dst: str, overwrite: bool = True) -> None:
        cooked_src = FsPath(src)
        cooked_dst = FsPath(dst)
        await self._load_and_retry(self._local_folder_fs.move, cooked_src, cooked_dst, overwrite)

    async def delete(self, path: str) -> None:
        cooked_path = FsPath(path)
        await self._load_and_retry(self._local_folder_fs.delete, cooked_path)

    async def sync(self, path: str, recursive=True) -> None:
        cooked_path = FsPath(path)
        await self._load_and_retry(self._syncer.sync, cooked_path, recursive=recursive)

    # TODO: do we really need this ? or should we provide id manipulation at this level ?
    async def sync_by_id(self, entry_id: UUID) -> None:
        assert isinstance(entry_id, UUID)
        await self._load_and_retry(self._syncer.sync_by_id, entry_id)

    # TODO: do we really need this ? or should we optimize `sync(path='/')` ?
    async def full_sync(self) -> None:
        await self._load_and_retry(self._syncer.full_sync)

    async def get_entry_path(self, id: UUID) -> FsPath:
        assert isinstance(id, UUID)
        path, _, _ = await self._load_and_retry(self._local_folder_fs.get_entry_path, id)
        return path

    async def share(self, path: str, recipient: str) -> None:
        cooked_path = FsPath(path)
        await self._load_and_retry(self._sharing.share, cooked_path, recipient)

    async def process_last_messages(self) -> None:
        await self._sharing.process_last_messages()
