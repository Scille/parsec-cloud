# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from hashlib import sha256
import pendulum

from parsec.crypto import (
    encrypt_signed_msg_with_secret_key,
    decrypt_raw_with_secret_key,
    # encrypt_raw_with_secret_key, TODO: uncomment when upload_block is implemented
    decrypt_and_verify_signed_msg_with_secret_key,
)

from parsec.core.backend_connection import (
    BackendCmdsBadResponse,
    BackendCmdsInMaintenance,
    BackendNotAvailable,
)
from parsec.core.types import (
    LocalManifest,
    EntryID,
    BlockAccess,
    remote_manifest_serializer,
    Manifest,
)
from parsec.core.fs.exceptions import (
    FSRemoteSyncError,
    FSRemoteManifestNotFound,
    FSBackendOfflineError,
    FSWorkspaceInMaintenance,
)


class RemoteLoader:
    def __init__(
        self,
        device,
        workspace_id,
        workspace_key,
        backend_cmds,
        remote_device_manager,
        local_storage,
    ):
        self.device = device
        self.workspace_id = workspace_id
        self.workspace_key = workspace_key
        self.backend_cmds = backend_cmds
        self.remote_device_manager = remote_device_manager
        self.local_storage = local_storage

    async def load_block(self, access: BlockAccess) -> bytes:
        """
        Raises:
            BackendConnectionError
            CryptoError
        """
        ciphered_block = await self.backend_cmds.block_read(access.id)
        # TODO: let encryption manager do the digest check ?
        # TODO: is digest even useful ? Given nacl.secret.Box does digest check
        # on the ciphered data they cannot be tempered. And given each block
        # has an unique key, valid blocks cannot be switched together.
        # TODO: better exceptions
        block = decrypt_raw_with_secret_key(access.key, ciphered_block)
        assert sha256(block).hexdigest() == access.digest, access

        self.local_storage.set_clean_block(access.id, block)
        return block

    async def load_remote_manifest(self, entry_id: EntryID) -> Manifest:
        try:
            # TODO: encryption_revision is not yet handled in core
            args = await self.backend_cmds.vlob_read(1, entry_id)

        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot access workspace data while it is in maintenance"
            ) from exc

        except BackendCmdsBadResponse as exc:
            if exc.status == "not_found":
                raise FSRemoteManifestNotFound(entry_id)
            raise

        expected_author_id, expected_timestamp, expected_version, blob = args
        author = await self.remote_device_manager.get_device(expected_author_id)
        raw = decrypt_and_verify_signed_msg_with_secret_key(
            self.workspace_key, blob, expected_author_id, author.verify_key, expected_timestamp
        )
        remote_manifest = remote_manifest_serializer.loads(raw)
        # TODO: better exception !
        assert remote_manifest.version == expected_version
        assert remote_manifest.author == expected_author_id
        # TODO: also store access id in remote_manifest and check it here
        return remote_manifest

    async def load_manifest(self, entry_id: EntryID) -> LocalManifest:
        remote_manifest = await self.load_remote_manifest(entry_id)
        # TODO: This should only be done if the manifest is not in the local storage
        # The relationship between the local storage and the remote loader needs
        # to be settle so we can refactor this kind of dangerous code
        self.local_storage.set_base_manifest(entry_id, remote_manifest)
        return remote_manifest.to_local(self.device.device_id)

    async def upload_manifest(self, entry_id: EntryID, manifest: Manifest):
        if manifest.version == 1:
            await self._vlob_create(entry_id, manifest)
        else:
            await self._vlob_update(entry_id, manifest)

    async def _vlob_create(self, entry_id: EntryID, manifest: Manifest):
        """Raises: FSBackendOfflineError, FSRemoteSyncError"""
        assert manifest.version == 1
        assert manifest.author == self.device.device_id
        now = pendulum.now()
        ciphered = encrypt_signed_msg_with_secret_key(
            self.device.device_id,
            self.device.signing_key,
            self.workspace_key,
            remote_manifest_serializer.dumps(manifest),
            now,
        )
        try:
            # TODO: encryption_revision is not yet handled in core
            await self.backend_cmds.vlob_create(self.workspace_id, 1, entry_id, now, ciphered)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot access workspace data while it is in maintenance"
            ) from exc

        except BackendCmdsBadResponse as exc:
            if exc.status == "already_exists":
                raise FSRemoteSyncError(entry_id)
            # TODO: does that happen?
            raise

    async def _vlob_update(self, entry_id: EntryID, manifest: Manifest):
        """Raises: FSBackendOfflineError, FSRemoteSyncError"""
        assert manifest.version > 1
        assert manifest.author == self.device.device_id
        now = pendulum.now()
        ciphered = encrypt_signed_msg_with_secret_key(
            self.device.device_id,
            self.device.signing_key,
            self.workspace_key,
            remote_manifest_serializer.dumps(manifest),
            now,
        )
        try:
            # TODO: encryption_revision is not yet handled in core
            await self.backend_cmds.vlob_update(1, entry_id, manifest.version, now, ciphered)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot access workspace data while it is in maintenance"
            ) from exc

        except BackendCmdsBadResponse as exc:
            if exc.status == "bad_version":
                raise FSRemoteSyncError(entry_id)
            # TODO: does that happen?
            raise
