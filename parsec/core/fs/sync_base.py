import trio
from uuid import UUID

from parsec.crypto import decrypt_raw_with_secret_key, encrypt_raw_with_secret_key
from parsec.core.backend_connection import BackendCmdsBadResponse
from parsec.core.schemas import dumps_manifest, loads_manifest
from parsec.core.fs.utils import is_file_manifest, is_folder_manifest, is_placeholder_manifest
from parsec.core.fs.types import Path, Access, LocalFolderManifest, LocalFileManifest, LocalManifest
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss, FSEntryNotFound


class SyncConcurrencyError(Exception):
    pass


DEFAULT_BLOCK_SIZE = 2 ** 16  # 64Kio


class BaseSyncer:
    def __init__(
        self,
        device,
        backend_cmds,
        encryption_manager,
        local_folder_fs,
        local_file_fs,
        event_bus,
        block_size=DEFAULT_BLOCK_SIZE,
    ):
        self._lock = trio.Lock()
        self.device = device
        self.local_folder_fs = local_folder_fs
        self.local_file_fs = local_file_fs
        self.backend_cmds = backend_cmds
        self.encryption_manager = encryption_manager
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

            if is_folder_manifest(manifest):
                for child_access in manifest["children"].values():
                    _recursive_get_local_entries_ids(child_access)

            entries.append(
                {"id": access["id"], "rts": access["rts"], "version": manifest["base_version"]}
            )

        _recursive_get_local_entries_ids(self.device.user_manifest_access)
        return entries

    async def full_sync(self) -> None:
        local_entries = self._get_group_check_local_entries()

        if not local_entries:
            # Nothing in local, so everything is synced ! ;-)
            self.event_bus.send(
                "fs.entry.synced", path="/", id=self.device.user_manifest_access["id"]
            )
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

    async def sync(self, path: Path, recursive: bool = True) -> None:
        # Only allow a single synchronizing operation at a time to simplify
        # concurrency. Beside concurrent syncs would make each sync operation
        # slower which would make them less reliable with poor backend connection.
        async with self._lock:
            await self._sync_nolock(path, recursive)

    async def _sync_nolock(self, path: Path, recursive: bool) -> None:
        # First retrieve a snapshot of the manifest to sync
        try:
            access, manifest = self.local_folder_fs.get_entry(path)
        except FSManifestLocalMiss:
            # Nothing to do if entry is no present locally
            return

        if not manifest["need_sync"] and not recursive:
            return

        # In case of placeholder, we must resolve it first (and make sure
        # none of it parents are placeholders themselves)
        if manifest["is_placeholder"]:
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
        self, path: Path, access: Access, manifest: LocalManifest
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
            return manifest["need_sync"]

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
                self.event_bus.send("fs.entry.synced", path=str(path), id=access["id"])

            return need_more_sync

    async def _minimal_sync_file(
        self, path: Path, access: Access, manifest: LocalFileManifest
    ) -> None:
        raise NotImplementedError()

    async def _minimal_sync_folder(
        self, path: Path, access: Access, manifest: LocalFolderManifest
    ) -> None:
        raise NotImplementedError()

    async def _sync_file(self, path: Path, access: Access, manifest: LocalFileManifest) -> None:
        raise NotImplementedError()

    async def _sync_folder(
        self, path: Path, access: Access, manifest: LocalFolderManifest, recursive: bool
    ) -> None:
        raise NotImplementedError()

    async def _backend_block_create(self, access, blob):
        ciphered = encrypt_raw_with_secret_key(access["key"], bytes(blob))
        try:
            await self.backend_cmds.blockstore_create(access["id"], ciphered)
        except BackendCmdsBadResponse as exc:
            # If a previous attempt of uploading this block has been processed by
            # the backend but we lost the connection before receiving the response
            # Note we neglect the possibility of another id collision with another
            # unrelated block because we trust probability and uuid4, who doesn't ?
            if exc.args[0]["status"] != "already_exists":
                raise

    async def _backend_block_read(self, access):
        ciphered = await self.backend_cmds.blockstore_read(access["id"])
        return decrypt_raw_with_secret_key(access["key"], ciphered)

    async def _backend_vlob_group_check(self, to_check):
        ret = await self.backend_cmds.vlob_group_check(to_check)
        return [entry["id"] for entry in ret["changed"]]

    async def _backend_vlob_read(self, access, version=None):
        _, blob = await self.backend_cmds.vlob_read(access["id"], access["rts"], version)
        raw = await self.encryption_manager.decrypt_with_secret_key(access["key"], blob)
        return loads_manifest(raw)

    async def _backend_vlob_create(self, access, manifest, notify_beacon):
        assert manifest["version"] == 1
        ciphered = self.encryption_manager.encrypt_with_secret_key(
            access["key"], dumps_manifest(manifest)
        )
        try:
            await self.backend_cmds.vlob_create(
                access["id"], access["rts"], access["wts"], ciphered, notify_beacon
            )
        except BackendCmdsBadResponse as exc:
            if exc.status == "already_exists":
                raise SyncConcurrencyError(access)
            raise

    async def _backend_vlob_update(self, access, manifest, notify_beacon):
        assert manifest["version"] > 1
        ciphered = self.encryption_manager.encrypt_with_secret_key(
            access["key"], dumps_manifest(manifest)
        )
        try:
            await self.backend_cmds.vlob_update(
                access["id"], access["wts"], manifest["version"], ciphered, notify_beacon
            )
        except BackendCmdsBadResponse as exc:
            if exc.status == "bad_version":
                raise SyncConcurrencyError(access)
            raise
