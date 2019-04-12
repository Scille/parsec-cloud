# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import math
from typing import Dict
from uuid import UUID

from parsec.types import UserID
from parsec.event_bus import EventBus
from parsec.core.types import LocalDevice, FsPath
from parsec.core.local_storage import LocalStorage
from parsec.core.backend_connection import BackendCmdsPool
from parsec.core.remote_devices_manager import RemoteDevicesManager
from parsec.core.fs.userfs import UserFS
from parsec.core.fs.exceptions import FSWorkspaceNotFoundError


class FS:
    def __init__(
        self,
        device: LocalDevice,
        local_storage: LocalStorage,
        backend_cmds: BackendCmdsPool,
        remote_devices_manager: RemoteDevicesManager,
        event_bus: EventBus,
    ):
        self.device = device
        self.local_storage = local_storage
        self.backend_cmds = backend_cmds
        self.event_bus = event_bus

        self._user_fs = UserFS(
            device, local_storage, backend_cmds, remote_devices_manager, event_bus
        )

        self._fds = {}

    def _put_fd(self, workspace, fd):
        self._fds[fd] = workspace
        return fd

    def _pop_fd(self, fd):
        return self._fds.pop(fd), fd

    def _get_fd(self, fd):
        return self._fds.get(fd), fd

    @property
    def _local_folder_fs(self):
        return self._user_fs._local_folder_fs

    @property
    def _syncer(self):
        return self._user_fs._syncer

    def _split_path(self, path):
        path = FsPath(path)
        if path.is_root():
            return None, None
        else:
            _, workspace_name, *parts = path.parts
            return workspace_name, FsPath("/" + "/".join(parts))

    def _get_workspace_entry_from_name(self, workspace_name):
        # Obviously broken if multiple workspaces have the same name :'(
        try:
            return next(
                w for w in self._user_fs.get_user_manifest().workspaces if w.name == workspace_name
            )
        except StopIteration:
            raise FileNotFoundError(2, "No such file or directory", f"/{workspace_name}")

    def _get_workspace(self, path):
        workspace_name, subpath = self._split_path(path)
        assert workspace_name
        workspace_entry = self._get_workspace_entry_from_name(workspace_name)
        try:
            workspace = self._user_fs.get_workspace(workspace_entry.access.id)
        except FSWorkspaceNotFoundError as exc:
            raise FileNotFoundError(2, "No such file or directory", f"/{workspace_name}") from exc

        return workspace, subpath

    def _iter_workspaces(self):
        um = self._user_fs.get_user_manifest()
        for w_entry in um.workspaces:
            yield self._user_fs.get_workspace(w_entry.access.id)

    async def stat(self, path: str) -> dict:
        workspace_name, subpath = self._split_path(path)
        if not workspace_name:
            um = self._user_fs.get_user_manifest()
            return {
                "type": "root",
                "id": self._user_fs.user_manifest_access.id,
                "is_folder": True,
                "created": um.created,
                "updated": um.updated,
                "base_version": um.base_version,
                "is_placeholder": um.is_placeholder,
                "need_sync": um.need_sync,
                # Only list workspace we still have access to
                "children": sorted(um.children.keys()),
            }

        else:
            workspace, _ = self._get_workspace(f"/{workspace_name}")
            return await workspace.stat(subpath)

    async def file_write(self, path: str, content: bytes, offset: int = 0) -> int:
        workspace, subpath = self._get_workspace(path)
        return await workspace.file_write(subpath, content, offset)

    async def file_truncate(self, path: str, length: int) -> None:
        workspace, subpath = self._get_workspace(path)
        return await workspace.file_truncate(subpath, length)

    async def file_read(self, path: str, size: int = math.inf, offset: int = 0) -> bytes:
        workspace, subpath = self._get_workspace(path)
        return await workspace.file_read(subpath, size, offset)

    async def file_fd_open(self, path: str, mode="rw") -> int:
        workspace, subpath = self._get_workspace(path)
        fd = await workspace.file_fd_open(subpath, mode)
        return self._put_fd(workspace, fd)

    async def file_fd_close(self, fd: int) -> None:
        workspace, fd = self._pop_fd(fd)
        await workspace.file_fd_close(fd)

    async def file_fd_seek(self, fd: int, offset: int) -> None:
        workspace, fd = self._get_fd(fd)
        await workspace.file_fd_seek(fd, offset)

    async def file_fd_truncate(self, fd: int, length: int) -> None:
        workspace, fd = self._get_fd(fd)
        await workspace.file_fd_truncate(fd, length)

    async def file_fd_write(self, fd: int, content: bytes, offset: int = None) -> int:
        workspace, fd = self._get_fd(fd)
        return await workspace.file_fd_write(fd, content, offset)

    async def file_fd_flush(self, fd: int) -> None:
        workspace, fd = self._get_fd(fd)
        await workspace.file_fd_flush(fd)

    async def file_fd_read(self, fd: int, size: int = -1, offset: int = None) -> bytes:
        workspace, fd = self._get_fd(fd)
        return await workspace.file_fd_read(fd, size, offset)

    async def touch(self, path: str) -> UUID:
        return await self.file_create(path)

    async def file_create(self, path: str) -> UUID:
        workspace, subpath = self._get_workspace(path)
        fd = await workspace.file_create(subpath)
        return self._put_fd(workspace, fd)

    async def mkdir(self, path: str) -> UUID:
        return await self.folder_create(path)

    async def folder_create(self, path: str) -> UUID:
        workspace, subpath = self._get_workspace(path)
        return await workspace.folder_create(subpath)

    async def workspace_create(self, path: str) -> UUID:
        cooked_path = FsPath(path)
        try:
            _, _ = self._get_workspace(path)
        except FileNotFoundError:
            pass
        else:
            # A workspace with this name already exists (shouldn't be a trouble,
            # except for legacy tests using oracle...)
            raise FileExistsError(17, "File exists", path)
        assert cooked_path.is_workspace()
        return await self._user_fs.workspace_create(cooked_path.workspace)

    async def workspace_rename(self, src: str, dst: str) -> None:
        cooked_src = FsPath(src)
        cooked_dst = FsPath(dst)
        assert cooked_src.is_workspace()
        assert cooked_dst.is_workspace()
        workspace_entry = self._get_workspace_entry_from_name(cooked_src.workspace)
        try:
            _, _ = self._get_workspace(dst)
        except FileNotFoundError:
            pass
        else:
            # A workspace with this name already exists (shouldn't be a trouble,
            # except for legacy tests using oracle...)
            raise FileExistsError(17, "File exists", dst)
        await self._user_fs.workspace_rename(workspace_entry.access.id, cooked_dst.workspace)

    async def move(self, src: str, dst: str, overwrite: bool = True) -> None:
        workspace, subpath_src = self._get_workspace(src)
        workspace_dst, subpath_dst = self._get_workspace(dst)
        assert workspace
        assert workspace.workspace_name == workspace_dst.workspace_name
        await workspace.move(subpath_src, subpath_dst)

    async def delete(self, path: str) -> None:
        workspace, subpath = self._get_workspace(path)
        await workspace.delete(subpath)

    async def sync(self, path: str, recursive: bool = True) -> None:
        workspace_name, subpath = self._split_path(path)
        if not workspace_name:
            if recursive:
                await self.full_sync()
            else:
                await self._user_fs.sync()

        else:
            workspace, _ = self._get_workspace(f"/{workspace_name}")
            stat = await workspace.stat("/")
            if stat["is_placeholder"]:
                await self._user_fs.sync()
            await workspace.sync(subpath, recursive)
            await self._user_fs.sync()

    # TODO: do we really need this ? or should we provide id manipulation at this level ?
    async def sync_by_id(self, entry_id: UUID) -> None:
        assert isinstance(entry_id, UUID)
        for workspace_name in [w["name"] for w in self._user_fs.stat()["workspaces"]]:
            workspace, _ = self._get_workspace(f"/{workspace_name}")
            try:
                return await workspace.sync_by_id(entry_id)
            except Exception:  # TODO: better exception
                pass

    # TODO: do we really need this ? or should we optimize `sync(path='/')` ?
    async def full_sync(self) -> None:
        await self._user_fs.sync()
        for workspace in self._iter_workspaces():
            await workspace.sync("/", recursive=True)
        await self._user_fs.sync()

    async def get_entry_path(self, id: UUID) -> FsPath:
        assert isinstance(id, UUID)
        for workspace in self._iter_workspaces():
            try:
                return await workspace.get_entry_path(id)
            except Exception:  # TODO: better exception
                pass

    async def share(
        self,
        path: str,
        recipient: str,
        admin_right: bool = True,
        read_right: bool = True,
        write_right: bool = True,
    ) -> None:
        cooked_path = FsPath(path)
        if not cooked_path.is_workspace():
            # Will fail with correct exception
            workspace_name = path
        else:
            workspace_name = cooked_path.workspace

        await self._user_fs.workspace_share(
            workspace_name,
            recipient=UserID(recipient),
            admin_right=admin_right,
            read_right=read_right,
            write_right=write_right,
        )

    async def get_permissions(self, path: str) -> Dict[UserID, Dict]:
        cooked_path = FsPath(path)
        assert cooked_path.is_workspace()
        return await self._user_fs.workspace_get_permissions(cooked_path.workspace)

    async def process_last_messages(self) -> None:
        await self._user_fs.process_last_messages()
