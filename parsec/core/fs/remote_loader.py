# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from hashlib import sha256

from parsec.crypto import decrypt_raw_with_secret_key, decrypt_and_verify_signed_msg_with_secret_key
from parsec.core.types import BlockAccess, ManifestAccess, remote_manifest_serializer


class RemoteLoader:
    def __init__(self, backend_cmds, remote_devices_manager, local_storage):
        self.backend_cmds = backend_cmds
        self.remote_devices_manager = remote_devices_manager
        self.local_storage = local_storage

    async def load_block(self, access: BlockAccess) -> None:
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

    async def load_manifest(self, access: ManifestAccess) -> None:
        args = await self.backend_cmds.vlob_read(access.id)
        expected_author_id, expected_timestamp, expected_version, blob = args
        author = await self.remote_devices_manager.get_device(expected_author_id)
        raw = decrypt_and_verify_signed_msg_with_secret_key(
            access.key, blob, expected_author_id, author.verify_key, expected_timestamp
        )
        remote_manifest = remote_manifest_serializer.loads(raw)
        # TODO: better exception !
        assert remote_manifest.version == expected_version

        local_manifest = remote_manifest.to_local()
        self.local_storage.set_clean_manifest(access, local_manifest)
