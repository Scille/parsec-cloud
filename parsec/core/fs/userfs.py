# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import inspect
from typing import Dict
from uuid import UUID

from parsec.types import UserID
from parsec.event_bus import EventBus
from parsec.core.types import LocalDevice, FsPath
from parsec.core.local_storage import LocalStorage
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
from parsec.core.fs.workspacefs import WorkspaceFS
from parsec.core.remote_devices_manager import RemoteDevicesManager


class UserFS:
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

        self._local_folder_fs = LocalFolderFS(device, local_storage, event_bus)
        self._local_file_fs = LocalFileFS(device, local_storage, self._local_folder_fs, event_bus)
        self._remote_loader = RemoteLoader(backend_cmds, remote_devices_manager, local_storage)
        self._syncer = Syncer(
            device,
            backend_cmds,
            remote_devices_manager,
            self._local_folder_fs,
            self._local_file_fs,
            event_bus,
        )
        self._sharing = Sharing(
            device,
            backend_cmds,
            remote_devices_manager,
            self._local_folder_fs,
            self._syncer,
            self._remote_loader,
            event_bus,
        )

    def list_workspaces(self):
        user_manifest = self._local_folder_fs.get_user_manifest()
        return [w.name for w in user_manifest.workspaces]

    def get_workspace(self, workpace_name: str):
        workspace_entry = self._local_folder_fs._retrieve_workspace_entry(workpace_name)
        if not workspace_entry:
            raise FileNotFoundError(2, "No such file or directory", f"/{workpace_name}")
        return WorkspaceFS(
            workpace_name,
            device=self.device,
            local_storage=self.local_storage,
            backend_cmds=self.backend_cmds,
            event_bus=self.event_bus,
            _local_folder_fs=self._local_folder_fs,
            _local_file_fs=self._local_file_fs,
            _remote_loader=self._remote_loader,
            _syncer=self._syncer,
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

    async def stat(self) -> dict:
        cooked_path = FsPath("/")
        return await self._load_and_retry(self._local_folder_fs.stat, cooked_path)

    async def workspace_create(self, name: str) -> UUID:
        cooked_path = FsPath(f"/{name}")
        return await self._load_and_retry(self._local_folder_fs.workspace_create, cooked_path)

    async def workspace_rename(self, src: str, dst: str) -> None:
        cooked_src = FsPath(f"/{src}")
        cooked_dst = FsPath(f"/{dst}")
        await self._load_and_retry(self._local_folder_fs.workspace_rename, cooked_src, cooked_dst)

    async def full_sync(self) -> None:
        # TODO: do we really need this ? or should we optimize `sync(path='/')` ?
        await self._load_and_retry(self._syncer.full_sync)

    async def sync(self, recursive: bool = True) -> None:
        cooked_path = FsPath("/")
        await self._load_and_retry(self._syncer.sync, cooked_path, recursive)

    async def workspace_share(
        self,
        workspace_name: str,
        recipient: UserID,
        admin_right: bool = True,
        read_right: bool = True,
        write_right: bool = True,
    ) -> None:
        cooked_path = FsPath(f"/{workspace_name}")
        await self._load_and_retry(
            self._sharing.share,
            cooked_path,
            recipient,
            admin_right=admin_right,
            read_right=read_right,
            write_right=write_right,
        )

    async def workspace_get_permissions(self, workspace_name: str) -> Dict[UserID, Dict]:
        cooked_path = FsPath(f"/{workspace_name}")
        permissions = await self._load_and_retry(self._sharing.get_permissions, cooked_path)
        return permissions

    async def process_last_messages(self) -> None:
        await self._sharing.process_last_messages()
