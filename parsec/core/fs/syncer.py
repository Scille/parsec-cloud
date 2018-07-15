import trio

from parsec.core.fs.data import is_folder_manifest
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss
from parsec.core.fs.sync_base import SyncConcurrencyError
from parsec.core.fs.folder_syncer import FolderSyncerMixin
from parsec.core.fs.file_syncer import FileSyncerMixin
from parsec.core.encryption_manager import decrypt_with_symkey, encrypt_with_symkey
from parsec.core.local_db import LocalDBMissingEntry
from parsec.core.schemas import dumps_manifest, loads_manifest
from parsec.utils import to_jsonb64, from_jsonb64


DEFAULT_BLOCK_SIZE = 2 ** 16  # 65Kio


class Syncer(FolderSyncerMixin, FileSyncerMixin):
    def __init__(
        self,
        device,
        backend_cmds_sender,
        encryption_manager,
        local_folder_fs,
        local_file_fs,
        signal_ns,
        block_size=DEFAULT_BLOCK_SIZE,
    ):
        self._lock = trio.Lock()
        self.device = device
        self.local_folder_fs = local_folder_fs
        self.local_file_fs = local_file_fs
        self.backend_cmds_sender = backend_cmds_sender
        self.encryption_manager = encryption_manager
        self.signal_ns = signal_ns
        self.block_size = block_size

    def _get_group_check_local_entries(self):
        entries = []

        def _recursive_get_local_entries_ids(access):
            try:

                manifest = self.local_folder_fs.get_manifest(access)
            except FSManifestLocalMiss:
                # TODO: make the assert true...
                # # Root should always be loaded
                # assert access is not self.device.user_manifest_access
                return

            if is_folder_manifest(manifest):
                for child_access in manifest["children"].values():
                    _recursive_get_local_entries_ids(child_access)
            print(access)

            entries.append(
                {"id": access["id"], "rts": access["rts"], "version": manifest["base_version"]}
            )

        _recursive_get_local_entries_ids(self.device.user_manifest_access)
        return entries

    async def full_sync(self):
        local_entries = self._get_group_check_local_entries()

        if not local_entries:
            # Nothing in local, so everything is synced ! ;-)
            self.signal_ns.signal("fs.entry.synced").send(
                None, path="/", id=self.device.user_manifest_access["id"]
            )
            return

        need_sync_entries = await self._backend_vlob_group_check(local_entries)
        for changed_item in need_sync_entries["changed"]:
            await self.sync_by_id(changed_item["id"])

    async def sync_by_id(self, entry_id):
        async with self._lock:
            try:
                path, access, _ = self.local_folder_fs.get_entry_path(entry_id)
            except FSManifestLocalMiss:
                # Entry not locally present, nothing to do
                return
            notify_beacons = self.local_folder_fs.get_beacons(path)
            await self._sync_nolock(path, access, recursive=False, notify_beacons=notify_beacons)

    async def sync(self, path, recursive=True):
        # Only allow a single synchronizing operation at a time to simplify
        # concurrency. Beside concurrent syncs would make each sync operation
        # slower which would make them less reliable with poor backend connection.
        async with self._lock:
            try:
                sync_path, sync_recursive = self.local_folder_fs.get_sync_strategy(path, recursive)
                sync_access = self.local_folder_fs.get_access(sync_path)
            except LocalDBMissingEntry:
                # Nothing to do if entry is no present locally
                return
            notify_beacons = self.local_folder_fs.get_beacons(sync_path)
            await self._sync_nolock(sync_path, sync_access, sync_recursive, notify_beacons)

    async def _sync_nolock(self, path, access, recursive, notify_beacons):
        try:
            manifest = self.local_folder_fs.get_manifest(access)
        except LocalDBMissingEntry:
            # Nothing to do if entry is no present locally
            return
        if is_folder_manifest(manifest):
            await self._sync_folder_nolock(path, access, manifest, recursive, notify_beacons)
        else:
            await self._sync_file_nolock(path, access, manifest, notify_beacons)

    async def _backend_block_post(self, access, blob):
        ciphered = encrypt_with_symkey(access["key"], bytes(blob))
        payload = {"cmd": "blockstore_post", "id": access["id"], "block": to_jsonb64(ciphered)}
        ret = await self.backend_cmds_sender.send(payload)
        assert ret["status"] == "ok"

    async def _backend_block_get(self, access):
        payload = {"cmd": "blockstore_get", "id": access["id"]}
        ret = await self.backend_cmds_sender.send(payload)
        assert ret["status"] == "ok"
        ciphered = from_jsonb64(ret["block"])
        blob = decrypt_with_symkey(access["key"], ciphered)
        return blob

    async def _backend_vlob_group_check(self, to_check):
        payload = {"cmd": "vlob_group_check", "to_check": to_check}
        ret = await self.backend_cmds_sender.send(payload)
        assert ret["status"] == "ok"
        return ret

    async def _backend_vlob_read(self, access, version=None):
        payload = {"cmd": "vlob_read", "id": access["id"], "rts": access["rts"], "version": version}
        ret = await self.backend_cmds_sender.send(payload)
        assert ret["status"] == "ok"
        ciphered = from_jsonb64(ret["blob"])
        raw = await self.encryption_manager.decrypt_with_secret_key(access["key"], ciphered)
        return loads_manifest(raw)

    async def _backend_vlob_create(self, access, manifest, notify_beacons):
        ciphered = self.encryption_manager.encrypt_with_secret_key(
            access["key"], dumps_manifest(manifest)
        )
        payload = {
            "cmd": "vlob_create",
            "id": access["id"],
            "wts": access["wts"],
            "rts": access["rts"],
            "blob": to_jsonb64(ciphered),
            "notify_beacons": notify_beacons,
        }
        ret = await self.backend_cmds_sender.send(payload)
        assert ret["status"] == "ok"

    async def _backend_vlob_update(self, access, manifest, notify_beacons):
        ciphered = self.encryption_manager.encrypt_with_secret_key(
            access["key"], dumps_manifest(manifest)
        )
        payload = {
            "cmd": "vlob_update",
            "id": access["id"],
            "wts": access["wts"],
            "version": manifest["version"],
            "blob": to_jsonb64(ciphered),
            "notify_beacons": notify_beacons,
        }
        ret = await self.backend_cmds_sender.send(payload)
        if ret["status"] == "version_error":
            raise SyncConcurrencyError(access)
        assert ret["status"] == "ok"
