# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import math
import inspect
from uuid import UUID

from parsec.core.types import FsPath
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss, FSMultiManifestLocalMiss
from parsec.core.fs.local_file_fs import FSBlocksLocalMiss


class WorkspaceFS:
    def __init__(
        self,
        workpace_name,
        device,
        local_storage,
        backend_cmds,
        event_bus,
        _local_folder_fs,
        _local_file_fs,
        _remote_loader,
        _syncer,
    ):
        self.workpace_name = workpace_name
        self.device = device
        self.local_storage = local_storage
        self.backend_cmds = backend_cmds
        self.event_bus = event_bus
        self._local_folder_fs = _local_folder_fs
        self._local_file_fs = _local_file_fs
        self._remote_loader = _remote_loader
        self._syncer = _syncer

    def _cook_path(self, relative_path=""):
        return FsPath(f"/{self.workpace_name}/{relative_path}")

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

    async def stat(self, path: FsPath) -> dict:
        path = self._cook_path(path)
        return await self._load_and_retry(self._local_folder_fs.stat, path)

    async def file_write(self, path: FsPath, content: bytes, offset: int = 0) -> int:
        fd = await self.file_fd_open(path, "rw")
        try:
            if offset:
                await self.file_fd_seek(fd, offset)
            return await self.file_fd_write(fd, content)
        finally:
            await self.file_fd_close(fd)

    async def file_truncate(self, path: FsPath, length: int) -> None:
        fd = await self.file_fd_open(path, "w")
        try:
            await self.file_fd_truncate(fd, length)
        finally:
            await self.file_fd_close(fd)

    async def file_read(self, path: FsPath, size: int = math.inf, offset: int = 0) -> bytes:
        fd = await self.file_fd_open(path, "r")
        try:
            if offset:
                await self.file_fd_seek(fd, offset)
            return await self.file_fd_read(fd, size)
        finally:
            await self.file_fd_close(fd)

    async def file_fd_open(self, path: FsPath, mode="rw") -> int:
        path = self._cook_path(path)
        access = await self._load_and_retry(self._local_folder_fs.get_access, path, mode)
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

    async def file_create(self, path: FsPath) -> UUID:
        path = self._cook_path(path)
        return await self._load_and_retry(self._local_folder_fs.touch, path)

    async def folder_create(self, path: FsPath) -> UUID:
        path = self._cook_path(path)
        return await self._load_and_retry(self._local_folder_fs.mkdir, path)

    async def move(self, src: FsPath, dst: FsPath, overwrite: bool = True) -> None:
        src = self._cook_path(src)
        dst = self._cook_path(dst)
        await self._load_and_retry(self._local_folder_fs.move, src, dst, overwrite)

    async def delete(self, path: FsPath) -> None:
        path = self._cook_path(path)
        await self._load_and_retry(self._local_folder_fs.delete, path)

    async def sync(self, path: FsPath, recursive: bool = True) -> None:
        path = self._cook_path(path)
        await self._load_and_retry(self._syncer.sync, path, recursive=recursive)

    # TODO: do we really need this ? or should we provide id manipulation at this level ?
    async def sync_by_id(self, entry_id: UUID) -> None:
        assert isinstance(entry_id, UUID)
        await self._load_and_retry(self._syncer.sync_by_id, entry_id)

    async def get_entry_path(self, id: UUID) -> FsPath:
        assert isinstance(id, UUID)
        path, _, _ = await self._load_and_retry(self._local_folder_fs.get_entry_path, id)
        return path
