import math
import inspect
from uuid import UUID

from parsec.core.fs.local_folder_fs import (
    FSManifestLocalMiss,
    FSMultiManifestLocalMiss,
    LocalFolderFS,
)
from parsec.core.fs.local_file_fs import LocalFileFS, FSBlocksLocalMiss
from parsec.core.fs.syncer import Syncer
from parsec.core.fs.sharing import Sharing
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.fs.types import Path


class FS:
    def __init__(
        self,
        device,
        backend_cmds_sender,
        encryption_manager,
        event_bus,
        allow_non_workpace_in_root=False,
    ):
        super().__init__()
        self.event_bus = event_bus
        self.device = device

        self._local_folder_fs = LocalFolderFS(device, event_bus, allow_non_workpace_in_root)
        self._local_file_fs = LocalFileFS(device, self._local_folder_fs, event_bus)
        self._remote_loader = RemoteLoader(backend_cmds_sender, encryption_manager, device.local_db)
        self._syncer = Syncer(
            device,
            backend_cmds_sender,
            encryption_manager,
            self._local_folder_fs,
            self._local_file_fs,
            event_bus,
        )
        self._sharing = Sharing(
            device,
            backend_cmds_sender,
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

    async def stat(self, path: str):
        cooked_path = Path(path)
        return await self._load_and_retry(self._local_folder_fs.stat, cooked_path)

    async def file_write(self, path: str, content: bytes, offset: int = 0):
        fd = await self.file_fd_open(path)
        try:
            if offset:
                await self.file_fd_seek(fd, offset)
            await self.file_fd_write(fd, content)
        finally:
            await self.file_fd_close(fd)

    async def file_truncate(self, path: str, length: int):
        fd = await self.file_fd_open(path)
        try:
            await self.file_fd_truncate(fd, length)
        finally:
            await self.file_fd_close(fd)

    async def file_read(self, path: str, size: int = math.inf, offset: int = 0):
        fd = await self.file_fd_open(path)
        try:
            if offset:
                await self.file_fd_seek(fd, offset)
            return await self.file_fd_read(fd, size)
        finally:
            await self.file_fd_close(fd)

    async def file_fd_open(self, path: str):
        cooked_path = Path(path)
        access = await self._load_and_retry(self._local_folder_fs.get_access, cooked_path)
        return self._local_file_fs.open(access)

    async def file_fd_close(self, fd: int):
        self._local_file_fs.close(fd)

    async def file_fd_seek(self, fd: int, offset: int):
        self._local_file_fs.seek(fd, offset)

    async def file_fd_truncate(self, fd: int, length: int):
        self._local_file_fs.truncate(fd, length)

    async def file_fd_write(self, fd: int, content: bytes):
        return self._local_file_fs.write(fd, content)

    async def file_fd_flush(self, fd: int):
        self._local_file_fs.flush(fd)

    async def file_fd_read(self, fd: int, size: int = -1):
        return await self._load_and_retry(self._local_file_fs.read, fd, size)

    async def touch(self, path: str):
        cooked_path = Path(path)
        await self._load_and_retry(self._local_folder_fs.touch, cooked_path)

    async def file_create(self, path: str):
        return await self.touch(path)

    async def mkdir(self, path: str):
        cooked_path = Path(path)
        await self._load_and_retry(self._local_folder_fs.mkdir, cooked_path)

    async def folder_create(self, path: str):
        return await self.mkdir(path)

    async def workspace_create(self, path: str):
        cooked_path = Path(path)
        await self._load_and_retry(self._local_folder_fs.workspace_create, cooked_path)

    async def workspace_rename(self, src: str, dst: str):
        cooked_src = Path(src)
        cooked_dst = Path(dst)
        await self._load_and_retry(self._local_folder_fs.workspace_rename, cooked_src, cooked_dst)

    async def move(self, src: str, dst: str):
        cooked_src = Path(src)
        cooked_dst = Path(dst)
        await self._load_and_retry(self._local_folder_fs.move, cooked_src, cooked_dst)

    async def copy(self, src: str, dst: str):
        cooked_src = Path(src)
        cooked_dst = Path(dst)
        await self._load_and_retry(self._local_folder_fs.copy, cooked_src, cooked_dst)

    async def delete(self, path: str):
        cooked_path = Path(path)
        await self._load_and_retry(self._local_folder_fs.delete, cooked_path)

    async def sync(self, path: str, recursive=True):
        cooked_path = Path(path)
        await self._load_and_retry(self._syncer.sync, cooked_path, recursive=recursive)

    # TODO: do we really need this ? or should we provide id manipulation at this level ?
    async def sync_by_id(self, entry_id: UUID):
        assert isinstance(entry_id, UUID)
        await self._load_and_retry(self._syncer.sync_by_id, entry_id)

    # TODO: do we really need this ? or should we optimize `sync(path='/')` ?
    async def full_sync(self):
        await self._load_and_retry(self._syncer.full_sync)

    async def get_entry_path(self, id: UUID):
        assert isinstance(id, UUID)
        path, _, _ = await self._load_and_retry(self._local_folder_fs.get_entry_path, id)
        return path

    async def share(self, path: str, recipient: str):
        cooked_path = Path(path)
        await self._load_and_retry(self._sharing.share, cooked_path, recipient)

    async def process_last_messages(self):
        await self._sharing.process_last_messages()
