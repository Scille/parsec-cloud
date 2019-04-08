# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from uuid import UUID
import pendulum

from parsec.crypto import (
    encrypt_signed_msg_with_secret_key,
    decrypt_raw_with_secret_key,
    encrypt_raw_with_secret_key,
    decrypt_and_verify_signed_msg_with_secret_key,
)
from parsec.core.backend_connection import BackendCmdsBadResponse
from parsec.core.types import (
    FsPath,
    Access,
    LocalFolderManifest,
    LocalFileManifest,
    LocalManifest,
    remote_manifest_serializer,
)
from parsec.core.fs.utils import is_file_manifest, is_folderish_manifest, is_placeholder_manifest
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss, FSEntryNotFound


class SyncConcurrencyError(Exception):
    pass


DEFAULT_BLOCK_SIZE = 2 ** 16  # 64Kio


class BaseSyncer:
    def __init__(
        self,
        device,
        backend_cmds,
        remote_devices_manager,
        local_folder_fs,
        local_storage,
        event_bus,
        block_size=DEFAULT_BLOCK_SIZE,
    ):
        self._lock = trio.Lock()
        self.device = device
        self.local_folder_fs = local_folder_fs
        self.local_storage = local_storage
        self.backend_cmds = backend_cmds
        self.remote_devices_manager = remote_devices_manager
        self.event_bus = event_bus
        self.block_size = block_size

    def _get_group_check_local_entries(self):
        entries = []

        def _recursive_get_local_entries_ids(access):
            try:

                manifest = self.local_folder_fs.get_manifest(access)
            except FSManifestLocalMiss:
                # TODO: make the assert true...
                # Root should always be loaded
                assert access is not self.device.user_manifest_access
                return

            if is_folderish_manifest(manifest):
                for child_access in manifest.children.values():
                    _recursive_get_local_entries_ids(child_access)

            entries.append({"id": access.id, "version": manifest.base_version})

        _recursive_get_local_entries_ids(self.device.user_manifest_access)
        return entries

    async def full_sync(self) -> None:
        local_entries = self._get_group_check_local_entries()

        if not local_entries:
            # Nothing in local, so everything is synced ! ;-)
            self.event_bus.send("fs.entry.synced", path="/", id=self.device.user_manifest_access.id)
            return

        need_sync_entries = await self._backend_vlob_group_check(local_entries)
        for need_sync_entry_id in need_sync_entries:
            await self.sync_by_id(need_sync_entry_id)

    async def sync_by_id(self, entry_id: UUID) -> None:
        # TODO: we won't stricly sync this id, but the corresponding path
        # (which may end up being a different id in case of concurrent change)
        # may we should remove the function and only use `sync(path)` ?
        async with self._lock:
            try:
                path, _, _ = self.local_folder_fs.get_entry_path(entry_id)
            except FSEntryNotFound:
                # Entry not locally present, nothing to do
                return
            # TODO: Instead of going recursive here, we should have do a minimal
            # children sync (i.e. sync empty file and folder with the backend)
            # to save time.
            await self._sync_nolock(path, recursive=True)

    async def sync(self, path: FsPath, recursive: bool = True) -> None:
        # Only allow a single synchronizing operation at a time to simplify
        # concurrency. Beside concurrent syncs would make each sync operation
        # slower which would make them less reliable with poor backend connection.
        async with self._lock:
            await self._sync_nolock(path, recursive)

    async def _sync_nolock(self, path: FsPath, recursive: bool) -> None:
        # First retrieve a snapshot of the manifest to sync
        try:
            access, manifest = self.local_folder_fs.get_entry(path)
        except FSManifestLocalMiss:
            # Nothing to do if entry is no present locally
            return

        if not manifest.need_sync and not recursive:
            return

        # In case of placeholder, we must resolve it first (and make sure
        # none of it parents are placeholders themselves)
        if manifest.is_placeholder:
            need_more_sync = await self._resolve_placeholders_in_path(path, access, manifest)
            # If the entry to sync is actually empty the minimal sync was enough
            if not need_more_sync and not recursive:
                return
            # Reload the manifest given the minimal sync has changed it
            try:
                manifest = self.local_folder_fs.get_manifest(access)
            except FSManifestLocalMiss:
                # Nothing to do if entry is no present locally
                return

        # Note from now on it's possible the entry has changed due to
        # concurrent access (or even have been totally removed !).
        # We will deal with this when merged synced data back into the local fs.

        # Now we can do the sync on the entry
        if is_file_manifest(manifest):
            await self._sync_file(path, access, manifest)
        else:
            await self._sync_folder(path, access, manifest, recursive)

    async def _resolve_placeholders_in_path(
        self, path: FsPath, access: Access, manifest: LocalManifest
    ) -> bool:
        """
        Returns: If an additional sync is needed
        """
        # Notes we sync recursively from children to parents, this is more
        # efficient given otherwise we would have to do:
        # 1) sync the parent
        # 2) sync the child
        # 3) re-sync the parent with the child

        is_placeholder = is_placeholder_manifest(manifest)
        if not is_placeholder:
            # Cannot have a non-placeholder with a placeholder parent, hence
            # we don't have to go any further.
            return manifest.need_sync

        else:
            if is_file_manifest(manifest):
                need_more_sync = await self._minimal_sync_file(path, access, manifest)
            else:
                need_more_sync = await self._minimal_sync_folder(path, access, manifest)

            # Once the entry is synced, we must sync it parent as well to have
            # the entry visible for other clients

            if not path.is_root():
                try:
                    parent_access, parent_manifest = self.local_folder_fs.get_entry(path.parent)
                except FSManifestLocalMiss:
                    # Nothing to do if entry is no present locally
                    return False

                if is_placeholder_manifest(parent_manifest):
                    await self._resolve_placeholders_in_path(
                        path.parent, parent_access, parent_manifest
                    )
                else:
                    await self._sync_folder(
                        path.parent, parent_access, parent_manifest, recursive=False
                    )

            if not need_more_sync:
                self.event_bus.send("fs.entry.synced", path=str(path), id=access.id)

            return need_more_sync

    async def _minimal_sync_file(
        self, path: FsPath, access: Access, manifest: LocalFileManifest
    ) -> None:
        raise NotImplementedError()

    async def _minimal_sync_folder(
        self, path: FsPath, access: Access, manifest: LocalFolderManifest
    ) -> None:
        raise NotImplementedError()

    async def _sync_file(self, path: FsPath, access: Access, manifest: LocalFileManifest) -> None:
        raise NotImplementedError()

    async def _sync_folder(
        self, path: FsPath, access: Access, manifest: LocalFolderManifest, recursive: bool
    ) -> None:
        raise NotImplementedError()

    async def _backend_block_create(self, vlob_group, access, blob):
        ciphered = encrypt_raw_with_secret_key(access.key, bytes(blob))
        try:
            await self.backend_cmds.block_create(access.id, vlob_group, ciphered)
        except BackendCmdsBadResponse as exc:
            # If a previous attempt of uploading this block has been processed by
            # the backend but we lost the connection before receiving the response
            # Note we neglect the possibility of another id collision with another
            # unrelated block because we trust probability and uuid4, who doesn't ?
            if exc.args[0]["status"] != "already_exists":
                raise

    async def _backend_block_read(self, access):
        ciphered = await self.backend_cmds.block_read(access.id)
        return decrypt_raw_with_secret_key(access.key, ciphered)

    async def _backend_vlob_group_check(self, to_check):
        changed = await self.backend_cmds.vlob_group_check(to_check)
        return [entry["id"] for entry in changed]

    async def _backend_vlob_read(self, access, version=None):
        args = await self.backend_cmds.vlob_read(access.id, version)
        expected_author_id, expected_timestamp, expected_version, blob = args
        author = await self.remote_devices_manager.get_device(expected_author_id)
        raw = decrypt_and_verify_signed_msg_with_secret_key(
            access.key, blob, expected_author_id, author.verify_key, expected_timestamp
        )
        manifest = remote_manifest_serializer.loads(raw)
        # TODO: better exception !
        assert manifest.version == expected_version
        return manifest

    async def _backend_vlob_create(self, vlob_group, access, manifest):
        assert manifest.version == 1
        now = pendulum.now()
        ciphered = encrypt_signed_msg_with_secret_key(
            self.device.device_id,
            self.device.signing_key,
            access.key,
            remote_manifest_serializer.dumps(manifest),
            now,
        )
        try:
            await self.backend_cmds.vlob_create(vlob_group, access.id, now, ciphered)
        except BackendCmdsBadResponse as exc:
            if exc.status == "already_exists":
                raise SyncConcurrencyError(access)
            raise

    async def _backend_vlob_update(self, access, manifest):
        assert manifest.version > 1
        now = pendulum.now()
        ciphered = encrypt_signed_msg_with_secret_key(
            self.device.device_id,
            self.device.signing_key,
            access.key,
            remote_manifest_serializer.dumps(manifest),
            now,
        )
        try:
            await self.backend_cmds.vlob_update(access.id, manifest.version, now, ciphered)
        except BackendCmdsBadResponse as exc:
            if exc.status == "bad_version":
                raise SyncConcurrencyError(access)
            raise
