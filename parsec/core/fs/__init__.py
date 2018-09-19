import math
import inspect

from parsec.core.base import BaseAsyncComponent
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss, LocalFolderFS
from parsec.core.fs.local_file_fs import LocalFileFS, FSBlocksLocalMiss
from parsec.core.fs.syncer import Syncer
from parsec.core.fs.sharing import Sharing
from parsec.core.fs.remote_loader import RemoteLoader


class FS:
    # TODO: Legacy API
    @property
    def signal_ns(self):
        return self.event_bus

    def __init__(self, device, backend_cmds_sender, encryption_manager, event_bus):
        super().__init__()
        self.event_bus = event_bus
        self.device = device

        self._local_folder_fs = LocalFolderFS(device, event_bus)
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

            except FSBlocksLocalMiss as exc:
                for access in exc.accesses:
                    await self._remote_loader.load_block(access)

    async def stat(self, path):
        return await self._load_and_retry(self._local_folder_fs.stat, path)

    async def file_write(self, path, content, offset=0):
        fd = await self.file_fd_open(path)
        try:
            if offset:
                await self.file_fd_seek(fd, offset)
            await self.file_fd_write(fd, content)
        finally:
            await self.file_fd_close(fd)

    async def file_truncate(self, path, length):
        fd = await self.file_fd_open(path)
        try:
            await self.file_fd_truncate(fd, length)
        finally:
            await self.file_fd_close(fd)

    async def file_read(self, path, size=math.inf, offset=0):
        fd = await self.file_fd_open(path)
        try:
            if offset:
                await self.file_fd_seek(fd, offset)
            return await self.file_fd_read(fd, size)
        finally:
            await self.file_fd_close(fd)

    async def file_fd_open(self, path):
        access = await self._load_and_retry(self._local_folder_fs.get_access, path)
        return self._local_file_fs.open(access)

    async def file_fd_close(self, fd):
        self._local_file_fs.close(fd)

    async def file_fd_seek(self, fd, offset):
        self._local_file_fs.seek(fd, offset)

    async def file_fd_truncate(self, fd, length):
        self._local_file_fs.truncate(fd, length)

    async def file_fd_write(self, fd, content):
        self._local_file_fs.write(fd, content)

    async def file_fd_flush(self, fd):
        self._local_file_fs.flush(fd)

    async def file_fd_read(self, fd, size=-1):
        return await self._load_and_retry(self._local_file_fs.read, fd, size)

    async def file_create(self, path):
        await self._load_and_retry(self._local_folder_fs.touch, path)

    async def folder_create(self, path):
        await self._load_and_retry(self._local_folder_fs.mkdir, path)

    async def workspace_create(self, path):
        await self._load_and_retry(self._local_folder_fs.mkdir, path, workspace=True)

    async def move(self, src, dst):
        await self._load_and_retry(self._local_folder_fs.move, src, dst)

    async def delete(self, path):
        await self._load_and_retry(self._local_folder_fs.delete, path)

    async def sync(self, path, recursive=True):
        await self._load_and_retry(self._syncer.sync, path, recursive=recursive)

    # TODO: do we really need this ? or should we provide id manipulation at this level ?
    async def sync_by_id(self, entry_id):
        await self._load_and_retry(self._syncer.sync_by_id, entry_id)

    # TODO: do we really need this ? or should we optimize `sync(path='/')` ?
    async def full_sync(self):
        await self._load_and_retry(self._syncer.full_sync)

    async def share(self, path, recipient):
        await self._load_and_retry(self._sharing.share, path, recipient)

    async def process_last_messages(self):
        await self._sharing.process_last_messages()
