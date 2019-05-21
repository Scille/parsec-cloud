# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from hashlib import sha256
import pendulum

from parsec.crypto import (
    encrypt_signed_msg_with_secret_key,
    decrypt_raw_with_secret_key,
    # encrypt_raw_with_secret_key,
    decrypt_and_verify_signed_msg_with_secret_key,
)

from parsec.core.backend_connection import BackendCmdsBadResponse
from parsec.core.types import LocalManifest, BlockAccess
from parsec.core.types import ManifestAccess, remote_manifest_serializer, Manifest


class RemoteSyncError(Exception):
    pass


class RemoteManifestNotFound(Exception):
    pass


class RemoteLoader:
    def __init__(self, device, workspace_id, backend_cmds, remote_device_manager, local_storage):
        self.device = device
        self.workspace_id = workspace_id
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

        self.local_storage.set_clean_block(access, block)
        return block

    async def load_remote_manifest(self, access: ManifestAccess) -> Manifest:
        try:
            args = await self.backend_cmds.vlob_read(access.id)
        except BackendCmdsBadResponse as exc:
            if exc.status == "not_found":
                raise RemoteManifestNotFound(access)
            raise
        expected_author_id, expected_timestamp, expected_version, blob = args
        author = await self.remote_device_manager.get_device(expected_author_id)
        raw = decrypt_and_verify_signed_msg_with_secret_key(
            access.key, blob, expected_author_id, author.verify_key, expected_timestamp
        )
        remote_manifest = remote_manifest_serializer.loads(raw)
        # TODO: better exception !
        assert remote_manifest.version == expected_version
        assert remote_manifest.author == expected_author_id
        # TODO: also store access id in remote_manifest and check it here
        return remote_manifest

    async def load_manifest(self, access: ManifestAccess) -> LocalManifest:
        remote_manifest = await self.load_remote_manifest(access)
        # TODO: This should only be done if the manifest is not in the local storage
        # The relationship between the local storage and the remote loader needs
        # to be settle so we can refactor this kind of dangerous code
        self.local_storage.set_base_manifest(access, remote_manifest)
        return remote_manifest.to_local(self.device.device_id)

    async def upload_manifest(self, access: ManifestAccess, manifest: Manifest):
        if manifest.version == 1:
            await self._vlob_create(access, manifest)
        else:
            await self._vlob_update(access, manifest)

    async def _vlob_create(self, access: ManifestAccess, manifest: Manifest):
        assert manifest.version == 1
        assert manifest.author == self.device.device_id
        now = pendulum.now()
        ciphered = encrypt_signed_msg_with_secret_key(
            self.device.device_id,
            self.device.signing_key,
            access.key,
            remote_manifest_serializer.dumps(manifest),
            now,
        )
        try:
            await self.backend_cmds.vlob_create(self.workspace_id, access.id, now, ciphered)
        except BackendCmdsBadResponse as exc:
            if exc.status == "already_exists":
                raise RemoteSyncError(access)
            raise

    async def _vlob_update(self, access: ManifestAccess, manifest: Manifest):
        assert manifest.version > 1
        assert manifest.author == self.device.device_id
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
                raise RemoteSyncError(access)
            raise
