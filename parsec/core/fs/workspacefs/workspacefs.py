# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import inspect
from uuid import UUID
import errno
from typing import Union, Iterator, Dict

from parsec.types import UserID
from parsec.core.types import FsPath, EntryID, LocalDevice, WorkspaceRole, Manifest
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendCmdsNotAllowed,
    BackendConnectionError,
)

from parsec.core.local_storage import LocalStorageMissingError

from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.fs.workspacefs.file_transactions import FileTransactions
from parsec.core.fs.workspacefs.entry_transactions import EntryTransactions
from parsec.core.fs.workspacefs.sync_transactions import SyncTransactions

from parsec.core.fs.utils import is_file_manifest

from parsec.core.fs.exceptions import (
    FSError,
    FSBackendOfflineError,
    FSRemoteManifestNotFound,
    FSRemoteSyncError,
    FSNoSynchronizationRequired,
)

# Legacy
from parsec.core.fs.local_folder_fs import (
    FSManifestLocalMiss,
    FSMultiManifestLocalMiss,
    FSEntryNotFound,
)


AnyPath = Union[FsPath, str]


class WorkspaceFS:
    def __init__(
        self,
        workspace_id: EntryID,
        get_workspace_entry,
        device: LocalDevice,
        local_storage,
        backend_cmds,
        event_bus,
        remote_device_manager,
        _local_folder_fs,
        _syncer,
    ):
        self.workspace_id = workspace_id
        self.get_workspace_entry = get_workspace_entry
        self.device = device
        self.local_storage = local_storage
        self.backend_cmds = backend_cmds
        self.event_bus = event_bus
        self.remote_device_manager = remote_device_manager

        # Legacy
        self._local_folder_fs = _local_folder_fs
        self._syncer = _syncer

        self.remote_loader = RemoteLoader(
            self.device,
            self.workspace_id,
            # TODO: key is subject to change, we should use get_workspace_entry() instead
            self.workspace_key,
            self.backend_cmds,
            self.remote_device_manager,
            self.local_storage,
        )
        self.file_transactions = FileTransactions(
            self.workspace_id, self.local_storage, self.remote_loader, self.event_bus
        )
        self.entry_transactions = EntryTransactions(
            self.workspace_id,
            self.get_workspace_entry,
            self.device,
            self.local_storage,
            self.remote_loader,
            self.event_bus,
        )
        self.sync_transactions = SyncTransactions(
            self.workspace_id, self.local_storage, self.remote_loader, self.event_bus
        )

    def __repr__(self):
        return f"<{type(self).__name__}(id={self.workspace_id!r}, name={self.workspace_name!r})>"

    @property
    def workspace_name(self) -> str:
        return self.get_workspace_entry().name

    @property
    def workspace_key(self) -> str:
        return self.get_workspace_entry().key

    # Information

    async def path_info(self, path: AnyPath) -> dict:
        return await self.entry_transactions.entry_info(FsPath(path))

    async def path_id(self, path: AnyPath) -> UUID:
        info = await self.entry_transactions.entry_info(FsPath(path))
        return info["id"]

    async def get_user_roles(self) -> Dict[UserID, WorkspaceRole]:
        """
        Raises:
            FSError
            FSWorkspaceNotFoundError
            FSBackendOfflineError
        """
        try:
            return await self.backend_cmds.realm_get_roles(self.workspace_id)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsNotAllowed:
            # Seems we lost all the access roles
            return {}

        except BackendConnectionError as exc:
            raise FSError(f"Cannot retrieve workspace per-user roles: {exc}") from exc

    # Pathlib-like interface

    async def exists(self, path: AnyPath) -> bool:
        path = FsPath(path)
        try:
            if await self.entry_transactions.entry_info(path):
                return True
        except FileNotFoundError:
            return False
        return False

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
        """
        Raises:
            FileExistsError
        """
        source = FsPath(source)
        destination = FsPath(destination)
        real_destination = destination
        if _destinsrc(source, destination):
            raise OSError(errno.EINVAL)
        try:
            if await self.is_dir(destination):
                real_destination = destination.joinpath(source.name)
                if await self.exists(real_destination):
                    raise FileExistsError
        # At this point, real_destination is the target either representing :
        # - the destination path if it didn't already exist,
        # - a new entry with the same name as source, but inside the destination directory
        except FileNotFoundError:
            pass
        if source.parent == real_destination.parent:
            return await self.rename(source, real_destination)
        elif await self.is_dir(source):
            await self.copytree(source, real_destination)
            await self.rmtree(source)
            return
        elif await self.is_file(source):
            await self.copyfile(source, real_destination)
            await self.unlink(source)
            return
        raise NotImplementedError

    async def copytree(self, source_path: AnyPath, target_path: AnyPath):
        source_path = FsPath(source_path)
        target_path = FsPath(target_path)
        source_files = await self.listdir(source_path)
        await self.mkdir(target_path)
        for source_file in source_files:
            target_file = target_path.joinpath(source_file.name)
            if await self.is_dir(source_file):
                await self.copytree(source_file, target_file)
            elif await self.is_file(source_file):
                await self.copyfile(source_file, target_file)

    async def copyfile(
        self, source_path: AnyPath, target_path: AnyPath, length=16 * 1024, exist_ok: bool = False
    ):
        await self.touch(target_path, exist_ok=exist_ok)
        offset = 0
        while True:
            buff = await self.read_bytes(source_path, length, offset * length)
            if not buff:
                break
            await self.write_bytes(target_path, buff, offset * length)
            offset += 1

    async def rmtree(self, path: AnyPath):
        path = FsPath(path)
        async for child in self.iterdir(path):
            if await self.is_dir(child):
                await self.rmtree(child)
            else:
                await self.unlink(child)
        await self.rmdir(path)

    # Sync interface

    async def minimal_sync(self, entry_id: EntryID) -> None:
        """Raises: FSBackendOfflineError"""
        # Get a minimal manifest to upload
        try:
            remote_manifest = await self.sync_transactions.get_minimal_remote_manifest(entry_id)
        # Not available locally so noting to synchronize
        except LocalStorageMissingError:
            return

        # No miminal manifest to upload, the entry is not a placeholder
        if remote_manifest is None:
            return

        # Legacy
        if is_file_manifest(remote_manifest):
            await self._legacy_minimal_file_sync(entry_id)
            return

        # Upload the miminal manifest
        try:
            await self.remote_loader.upload_manifest(entry_id, remote_manifest)
        # The upload has failed: download the latest remote manifest
        except FSRemoteSyncError:
            remote_manifest = await self.remote_loader.load_remote_manifest(entry_id)

        # Register the manifest to unset the placeholder tag
        await self.sync_transactions.synchronization_step(entry_id, remote_manifest)

    async def _sync_by_id(self, entry_id: EntryID, remote_changed: bool = True) -> Manifest:
        """Raises: FSBackendOfflineError"""
        # Get the current remote manifest if it has changed
        remote_manifest = None
        if remote_changed:
            try:
                remote_manifest = await self.remote_loader.load_remote_manifest(entry_id)
            except FSRemoteManifestNotFound:
                pass

        # Check type
        if self._use_legacy_sync(entry_id, remote_manifest):
            return await self._legacy_file_sync(entry_id)

        # Loop over sync transactions
        while True:

            # Perform the transaction
            try:
                new_remote_manifest = await self.sync_transactions.synchronization_step(
                    entry_id, remote_manifest
                )
            # The manifest doesn't exist locally
            except LocalStorageMissingError:
                raise FSNoSynchronizationRequired(entry_id)

            # No new manifest to upload, the entry is synced!
            if new_remote_manifest is None:
                return remote_manifest or self.local_storage.get_base_manifest(entry_id)

            # Synchronize placeholder children
            for child in self.sync_transactions.get_placeholder_children(new_remote_manifest):
                await self.minimal_sync(child)

            # Upload the new manifest containing the latest changes
            try:
                await self.remote_loader.upload_manifest(entry_id, new_remote_manifest)
            # The upload has failed: download the latest remote manifest
            except FSRemoteSyncError:
                remote_manifest = await self.remote_loader.load_remote_manifest(entry_id)
            # The upload has succeed: loop to acknowledge this new version
            else:
                remote_manifest = new_remote_manifest

    async def sync_by_id(
        self, entry_id: EntryID, remote_changed: bool = True, recursive: bool = True
    ):
        # Sync parent first
        try:
            manifest = await self._sync_by_id(entry_id, remote_changed=remote_changed)

        # Nothing to synchronize if the manifest does not exist locally
        except FSNoSynchronizationRequired:
            return

        # Non-recursive
        if not recursive or is_file_manifest(manifest):
            return

        # Synchronize children
        for name, entry_id in manifest.children.items():
            await self.sync_by_id(entry_id, remote_changed=remote_changed, recursive=True)

    async def sync(
        self, path: AnyPath, remote_changed: bool = True, recursive: bool = True
    ) -> None:
        path = FsPath(path)
        entry_id, _ = await self.entry_transactions._get_entry(path)
        # TODO: Maybe the path itself is not synchronized with the remote
        # Should we do something about it?
        await self.sync_by_id(entry_id, remote_changed=remote_changed, recursive=recursive)

    # Temporary methods

    async def _load_and_retry(self, fn, *args, **kwargs):
        while True:
            try:
                if inspect.iscoroutinefunction(fn):
                    return await fn(*args, **kwargs)
                else:
                    return fn(*args, **kwargs)

            except FSManifestLocalMiss as exc:
                await self.remote_loader.load_manifest(exc.access)

            except FSMultiManifestLocalMiss as exc:
                for access in exc.accesses:
                    await self.remote_loader.load_manifest(access)

    def _use_legacy_sync(self, entry_id, remote_manifest):
        if is_file_manifest(remote_manifest):
            return True
        try:
            local_manifest = self.local_storage.get_manifest(entry_id)
        except LocalStorageMissingError:
            return False
        return is_file_manifest(local_manifest)

    async def _legacy_minimal_file_sync(self, entry_id):
        # Get info
        try:
            path, access, manifest = await self._load_and_retry(
                self._syncer.local_folder_fs.get_entry_path, entry_id
            )
        except FSEntryNotFound:
            return
        # Minimal sync
        if manifest.is_placeholder:
            await self._load_and_retry(self._syncer._minimal_sync_file, path, access, manifest)

    async def _legacy_file_sync(self, entry_id):
        # Minimal sync
        await self._legacy_minimal_file_sync(entry_id)
        # Get info
        try:
            path, access, manifest = await self._load_and_retry(
                self._syncer.local_folder_fs.get_entry_path, entry_id
            )
        except FSEntryNotFound:
            raise FSNoSynchronizationRequired(entry_id)
        # Actual sync
        await self._load_and_retry(self._syncer._sync_file, path, access, manifest)
        # Return the manifest
        return manifest
