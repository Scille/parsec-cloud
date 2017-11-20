import attr
from nacl.public import Box
from nacl.secret import SecretBox
from nacl.signing import SigningKey
from nacl.exceptions import BadSignatureError, CryptoError

from parsec.core.manifest import (
    PlaceHolderEntry, LocalFileManifest, LocalFolderManifest, LocalUserManifest,
    load_manifest, dump_manifest, SyncedEntry, FolderManifest, FileManifest,
    merge_folder_manifest2, NewlySyncedEntry
)
from parsec.utils import ParsecError


class ManifestDecryptionError(ParsecError):
    status = 'invalid_signature'


class ManifestsManager:
    def __init__(self, user, local_storage, backend_storage):
        self.user = user
        self._local_storage = local_storage
        self._backend_storage = backend_storage

    def _encrypt_manifest(self, entry, manifest):
        raw = dump_manifest(manifest)
        box = SecretBox(entry.key)
        # signed = self.user.signkey.sign(raw)
        # return box.encrypt(signed)
        return box.encrypt(raw)

    def _decrypt_manifest(self, entry, blob):
        box = SecretBox(entry.key)
        try:
            raw = box.decrypt(blob)
            # signed = box.decrypt(blob)
            # raw = self.user.verifykey.verify(signed)
        except (BadSignatureError, CryptoError, ValueError):
            raise ManifestDecryptionError()
        return load_manifest(raw)

    def _encrypt_user_manifest(self, manifest):
        raw = dump_manifest(manifest)
        box = Box(self.user.privkey, self.user.pubkey)
        return box.encrypt(raw)

    def _decrypt_user_manifest(self, blob):
        box = Box(self.user.privkey, self.user.pubkey)
        try:
            raw = box.decrypt(blob)
        except (BadSignatureError, CryptoError, ValueError):
            raise ManifestDecryptionError()
        return load_manifest(raw)

    def fetch_user_manifest(self):
        blob = self._local_storage.fetch_user_manifest()
        if not blob:
            # If the user manifest is not available locally, we don't
            # want to wait for the backend to respond. So we provide
            # version 0 of the manifest (i.e. a fresh new instance of it)
            # that will be merged later with the real one by the synchronizer.
            return LocalUserManifest()
        else:
            return self._decrypt_user_manifest(blob)

    def flush_user_manifest(self, manifest):
        blob = self._encrypt_user_manifest(manifest)
        self._local_storage.flush_user_manifest(blob)

    async def fetch_manifest(self, entry):
        blob = self._local_storage.fetch_manifest(entry.id)
        if not blob and not isinstance(entry, PlaceHolderEntry):
            blob = await self._backend_storage.fetch_manifest(entry.id, entry.rts)
            if blob:
                self._local_storage.flush_manifest(entry.id, blob)
        return self._decrypt_manifest(entry, blob)

    async def fetch_manifest_force_version(self, entry, version=None):
        assert not isinstance(entry, PlaceHolderEntry)
        blob = await self._backend_storage.fetch_manifest(
            entry.id, entry.rts, version=version)
        return self._decrypt_manifest(entry, blob)

    async def fetch_user_manifest_force_version(self, version=None):
        blob = await self._backend_storage.fetch_user_manifest(version=version)
        return self._decrypt_user_manifest(blob)

    def flush_manifest(self, entry, manifest):
        blob = self._encrypt_manifest(entry, manifest)
        self._local_storage.flush_manifest(entry.id, blob)

    def create_placeholder_file(self):
        manifest = LocalFileManifest(need_sync=True)
        entry = PlaceHolderEntry()
        return entry, manifest

    def create_placeholder_folder(self):
        manifest = LocalFolderManifest(need_sync=True)
        entry = PlaceHolderEntry()
        return entry, manifest

    async def sync_manifest(self, entry, manifest):
        if isinstance(entry, PlaceHolderEntry):
            raise RuntimeError('Cannot synchronize placeholder')
        # Turn the local manifest into a regular synced manifest
        sync_manifest = manifest.to_sync_manifest()
        if hasattr(sync_manifest, 'children'):
            for childname, childentry in sync_manifest.children.items():
                if isinstance(childentry, PlaceHolderEntry):
                    synced_entry = await self._sync_placeholder(childentry)
                    sync_manifest.children[childname] = synced_entry
                    # TODO: flush manifest each time a child is synchronized
                    # to avoid losing this info in case of crash or backend
                    # disconnection ?
        blob = self._encrypt_manifest(entry, sync_manifest)
        await self._backend_storage.sync_manifest(
            entry.id, entry.wts, sync_manifest.version, blob)
        return sync_manifest

    async def _sync_placeholder(self, entry):
        assert isinstance(entry, PlaceHolderEntry)
        # The trick here is to act like the placeholder didn't contain anything
        # this way the syncronization is fast (no need to sync the blocks or
        # to go recursive on a folder).
        manifest = await self.fetch_manifest(entry)
        if isinstance(manifest, LocalFileManifest):
            sync_manifest = FileManifest(
                version=1,
                updated=manifest.created,
                created=manifest.created,
            )
        else:
            sync_manifest = FolderManifest(
                version=1,
                updated=manifest.created,
                created=manifest.created,
            )
        blob = self._encrypt_manifest(entry, sync_manifest)
        syncid, rts, wts = await self._backend_storage.sync_new_manifest(blob)
        new_entry = NewlySyncedEntry(syncid, entry.key, rts, wts, placeholder_id=entry.id)
        self._local_storage.move_manifest(entry.id, new_entry.id)
        return new_entry

    async def sync_user_manifest(self, manifest):
        # Turn the local manifest into a regular synced manifest
        sync_manifest = manifest.to_sync_manifest()
        if hasattr(sync_manifest, 'children'):
            for childname, childentry in sync_manifest.children.items():
                if isinstance(childentry, PlaceHolderEntry):
                    synced_entry = await self._sync_placeholder(childentry)
                    sync_manifest.children[childname] = synced_entry
                    # TODO: flush manifest each time a child is synchronized
                    # to avoid losing this info in case of crash or backend
                    # disconnection ?
        blob = self._encrypt_user_manifest(sync_manifest)
        await self._backend_storage.sync_user_manifest(sync_manifest.version, blob)
        return sync_manifest
