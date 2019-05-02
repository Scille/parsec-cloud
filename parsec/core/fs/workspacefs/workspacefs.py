# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import inspect
from typing import Union, Iterator
from uuid import UUID

from parsec.core.types import FsPath, AccessID
from parsec.core.fs.workspacefs.file_transactions import FileTransactions
from parsec.core.fs.workspacefs.entry_transactions import EntryTransactions

# Legacy
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss, FSMultiManifestLocalMiss


AnyPath = Union[FsPath, str]


class WorkspaceFS:
    def __init__(
        self,
        workspace_id,
        get_workspace_entry,
        device,
        local_storage,
        backend_cmds,
        event_bus,
        _local_folder_fs,
        _remote_loader,
        _syncer,
    ):
        self.workspace_id = workspace_id
        self.get_workspace_entry = get_workspace_entry
        self.device = device
        self.local_storage = local_storage
        self.backend_cmds = backend_cmds
        self.event_bus = event_bus
        self._local_folder_fs = _local_folder_fs
        self._remote_loader = _remote_loader
        self._syncer = _syncer
        self._user_roles_cache_access = None

        self.file_transactions = FileTransactions(
            self.workspace_id, local_storage, self._remote_loader, event_bus
        )
        self.entry_transactions = EntryTransactions(
            self.workspace_id,
            self.get_workspace_entry,
            self.device,
            local_storage,
            self._remote_loader,
            event_bus,
        )

    # Information

    async def path_info(self, path: AnyPath) -> dict:
        return await self.entry_transactions.entry_info(FsPath(path))

    # Pathlib-like interface

    async def is_dir(self, path: AnyPath) -> bool:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(path)
        return info["type"] == "folder"

    async def is_file(self, path: AnyPath) -> bool:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(FsPath(path))
        return info["type"] == "file"

    async def iterdir(self, path: AnyPath) -> Iterator[FsPath]:
        path = FsPath(path)
        info = await self.entry_transactions.entry_info(path)
        if "children" not in info:
            raise NotADirectoryError(str(path))
        for child in info["children"]:
            yield path / child

    async def listdir(self, path: AnyPath) -> Iterator[FsPath]:
        return [child async for child in self.iterdir(path)]

    async def rename(self, source: AnyPath, destination: AnyPath, overwrite: bool = True) -> None:
        source = FsPath(source)
        destination = FsPath(destination)
        await self.entry_transactions.entry_rename(source, destination, overwrite=overwrite)

    async def mkdir(self, path: AnyPath, parents: bool = False, exist_ok: bool = False) -> None:
        path = FsPath(path)
        try:
            await self.entry_transactions.folder_create(path)
        except FileNotFoundError:
            if not parents or path.parent == path:
                raise
            await self.mkdir(path.parent, parents=True, exist_ok=True)
            await self.mkdir(path, parents=False, exist_ok=exist_ok)
        except FileExistsError:
            if not exist_ok or not await self.is_dir(path):
                raise

    async def rmdir(self, path: AnyPath) -> None:
        path = FsPath(path)
        await self.entry_transactions.folder_delete(path)

    async def touch(self, path: AnyPath, exist_ok: bool = True) -> None:
        path = FsPath(path)
        try:
            await self.entry_transactions.file_create(path, open=False)
        except FileExistsError:
            if not exist_ok or not await self.is_file(path):
                raise

    async def unlink(self, path: AnyPath) -> None:
        path = FsPath(path)
        await self.entry_transactions.file_delete(path)

    async def truncate(self, path: AnyPath, length: int) -> None:
        path = FsPath(path)
        _, fd = await self.entry_transactions.file_open(path, "w")
        try:
            return await self.file_transactions.fd_resize(fd, length)
        finally:
            await self.file_transactions.fd_close(fd)

    async def read_bytes(self, path: AnyPath, size: int = -1, offset: int = 0) -> bytes:
        path = FsPath(path)
        _, fd = await self.entry_transactions.file_open(path, "r")
        try:
            return await self.file_transactions.fd_read(fd, size, offset)
        finally:
            await self.file_transactions.fd_close(fd)

    async def write_bytes(self, path: AnyPath, data: bytes, offset: int = 0) -> int:
        path = FsPath(path)
        _, fd = await self.entry_transactions.file_open(path, "w")
        try:
            return await self.file_transactions.fd_write(fd, data, offset)
        finally:
            await self.file_transactions.fd_close(fd)

    # Shutil-like interface

    async def move(self, source: AnyPath, destination: AnyPath):
        source = FsPath(source)
        destination = FsPath(destination)
        if source.parent == destination.parent:
            return await self.rename(source, destination)
        # TODO - reference implementation:
        # https://github.com/python/cpython/blob/3.7/Lib/shutil.py#L525
        raise NotImplementedError

    async def rmtree(self, path: AnyPath):
        path = FsPath(path)
        async for child in self.iterdir(path):
            if await self.is_dir(child):
                await self.rmtree(child)
            else:
                await self.unlink(child)
        await self.rmdir(path)

    # Left to migrate

    def _cook_path(self, relative_path=""):
        workspace_name = self.get_workspace_entry().name
        return FsPath(f"/{workspace_name}/{relative_path}")

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

    async def sync(self, path: FsPath, recursive: bool = True) -> None:
        path = self._cook_path(path)
        await self._load_and_retry(self._syncer.sync, path, recursive=recursive)

    # TODO: do we really need this ? or should we provide id manipulation at this level ?
    async def sync_by_id(self, entry_id: AccessID) -> None:
        assert isinstance(entry_id, UUID)
        await self._load_and_retry(self._syncer.sync_by_id, entry_id)

    async def get_entry_path(self, id: AccessID) -> FsPath:
        assert isinstance(id, UUID)
        path, _, _ = await self._load_and_retry(self._local_folder_fs.get_entry_path, id)
        return path
