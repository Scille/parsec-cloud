# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pendulum import Pendulum, now as pendulum_now
from typing import Dict, Optional, List, Tuple

from parsec.utils import timestamps_in_the_ballpark
from parsec.crypto import HashDigest, CryptoError
from parsec.api.protocol import UserID, DeviceID, RealmRole
from parsec.api.data import (
    DataError,
    BlockAccess,
    RealmRoleCertificateContent,
    Manifest as RemoteManifest,
)
from parsec.core.backend_connection import (
    BackendCmdsBadResponse,
    BackendCmdsInMaintenance,
    BackendCmdsAlreadyExists,
    BackendCmdsNotFound,
    BackendCmdsNotAllowed,
    BackendCmdsBadVersion,
    BackendNotAvailable,
    BackendConnectionError,
)
from parsec.core.types import EntryID, ChunkID
from parsec.core.fs.exceptions import (
    FSError,
    FSRemoteSyncError,
    FSRemoteManifestNotFound,
    FSRemoteManifestNotFoundBadVersion,
    FSRemoteBlockNotFound,
    FSBackendOfflineError,
    FSWorkspaceInMaintenance,
    FSBadEncryptionRevision,
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
)


class RemoteLoader:
    def __init__(
        self,
        device,
        workspace_id,
        get_workspace_entry,
        backend_cmds,
        remote_device_manager,
        local_storage,
    ):
        self.device = device
        self.workspace_id = workspace_id
        self.get_workspace_entry = get_workspace_entry
        self.backend_cmds = backend_cmds
        self.remote_device_manager = remote_device_manager
        self.local_storage = local_storage

    async def _load_realm_role_certificates(self, realm_id: Optional[EntryID] = None):
        try:
            uv_roles = await self.backend_cmds.realm_get_role_certificates(
                realm_id or self.workspace_id
            )

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsNotAllowed as exc:
            # Seems we lost the access to the realm
            raise FSWorkspaceNoReadAccess("Cannot get workspace roles: no read access") from exc

        except BackendConnectionError as exc:
            raise FSError(f"Cannot retrieve workspace roles: {exc}") from exc

        try:
            # Must read unverified certificates to access metadata
            unsecure_certifs = sorted(
                [
                    (RealmRoleCertificateContent.unsecure_load(uv_role), uv_role)
                    for uv_role in uv_roles
                ],
                key=lambda x: x[0].timestamp,
            )

            current_roles = {}
            owner_only = (RealmRole.OWNER,)
            owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)

            # Now verify each certif
            for unsecure_certif, raw_certif in unsecure_certifs:
                author = await self.remote_device_manager.get_device(unsecure_certif.author)

                RealmRoleCertificateContent.verify_and_load(
                    raw_certif,
                    author_verify_key=author.verify_key,
                    expected_author=author.device_id,
                )

                # Make sure author had the right to do this
                existing_user_role = current_roles.get(unsecure_certif.user_id)
                if not current_roles and unsecure_certif.user_id == author.device_id.user_id:
                    # First user is autosigned
                    needed_roles = (None,)
                elif (
                    existing_user_role in owner_or_manager
                    or unsecure_certif.role in owner_or_manager
                ):
                    needed_roles = owner_only
                else:
                    needed_roles = owner_or_manager
                if current_roles.get(unsecure_certif.author.user_id) not in needed_roles:
                    raise FSError(
                        f"Invalid realm role certificates: "
                        f"{unsecure_certif.author} has not right to give "
                        f"{unsecure_certif.role} role to {unsecure_certif.user_id} "
                        f"on {unsecure_certif.timestamp}"
                    )

                if unsecure_certif.role is None:
                    current_roles.pop(unsecure_certif.user_id, None)
                else:
                    current_roles[unsecure_certif.user_id] = unsecure_certif.role

        # Decryption error
        except DataError as exc:
            raise FSError(f"Invalid realm role certificates: {exc}") from exc

        # Now unsecure_certifs is no longer unsecure we have valided it items
        return [c for c, _ in unsecure_certifs], current_roles

    async def load_realm_role_certificates(
        self, realm_id: Optional[EntryID] = None
    ) -> List[RealmRoleCertificateContent]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
        """
        certificates, _ = await self._load_realm_role_certificates(realm_id)
        return certificates

    async def load_realm_current_roles(
        self, realm_id: Optional[EntryID] = None
    ) -> Dict[UserID, RealmRole]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
        """
        _, current_roles = await self._load_realm_role_certificates(realm_id)
        return current_roles

    async def load_blocks(self, accesses: List[BlockAccess]) -> None:
        """
        Raises:
            FSError
            FSRemoteBlockNotFound
            FSBackendOfflineError
            FSWorkspaceInMaintenance
        """
        for access in accesses:
            await self.load_block(access)

    async def load_block(self, access: BlockAccess) -> None:
        """
        Raises:
            FSError
            FSRemoteBlockNotFound
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSWorkspaceNoAccess
        """
        # Download
        try:
            ciphered_block = await self.backend_cmds.block_read(access.id)

        # Block not found
        except BackendCmdsNotFound as exc:
            raise FSRemoteBlockNotFound(access) from exc

        except BackendCmdsNotAllowed as exc:
            # Seems we lost the access to the realm
            raise FSWorkspaceNoReadAccess("Cannot load block: no read access") from exc

        # Backend not available
        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        # Workspace in maintenance
        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot download block while the workspace in maintenance"
            ) from exc

        # Another backend error
        except BackendConnectionError as exc:
            raise FSError(f"Cannot download block: {exc}") from exc

        # Decryption
        try:
            block = access.key.decrypt(ciphered_block)

        # Decryption error
        except CryptoError as exc:
            raise FSError(f"Cannot decrypt block: {exc}") from exc

        # TODO: let encryption manager do the digest check ?
        assert HashDigest.from_data(block) == access.digest, access
        await self.local_storage.set_clean_block(access.id, block)

    async def upload_block(self, access: BlockAccess, data: bytes):
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSWorkspaceNoAccess
        """
        # Encryption
        try:
            ciphered = access.key.encrypt(data)

        # Encryption error
        except CryptoError as exc:
            raise FSError(f"Cannot encrypt block: {exc}") from exc

        # Upload block
        try:
            await self.backend_cmds.block_create(access.id, self.workspace_id, ciphered)

        # Block already exists
        except BackendCmdsAlreadyExists:
            # Ignore exception if the block has already been uploaded
            # This might happen when a failure occurs before the local storage is updated
            pass

        except BackendCmdsNotAllowed as exc:
            # Seems we lost the access to the realm
            raise FSWorkspaceNoWriteAccess("Cannot upload block: no write access") from exc

        # Backend is not available
        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        # Workspace is in maintenance
        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot upload block while the workspace in maintenance"
            ) from exc

        # Another backend error
        except BackendConnectionError as exc:
            raise FSError(f"Cannot upload block: {exc}") from exc

        # Update local storage
        await self.local_storage.set_clean_block(access.id, data)
        await self.local_storage.clear_chunk(ChunkID(access.id), miss_ok=True)

    async def load_manifest(
        self, entry_id: EntryID, version: int = None, timestamp: Pendulum = None
    ) -> RemoteManifest:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
        """
        # Download the vlob
        workspace_entry = self.get_workspace_entry()
        try:
            args = await self.backend_cmds.vlob_read(
                workspace_entry.encryption_revision, entry_id, version=version, timestamp=timestamp
            )

        # Vlob is not found
        except BackendCmdsNotFound as exc:
            raise FSRemoteManifestNotFound(entry_id) from exc

        except BackendCmdsNotAllowed as exc:
            # Seems we lost the access to the realm
            raise FSWorkspaceNoReadAccess("Cannot load manifest: no read access") from exc

        # Backend is not available
        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        # Workspace is in maintenance
        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot download vlob while the workspace is in maintenance"
            ) from exc

        except BackendCmdsBadResponse as exc:
            if exc.status == "bad_version":
                raise FSRemoteManifestNotFoundBadVersion(entry_id)
            if exc.status == "not_found":
                raise FSRemoteManifestNotFound(entry_id)
            elif exc.status == "bad_encryption_revision":
                raise FSBadEncryptionRevision(
                    f"Cannot fetch vlob {entry_id}: Bad encryption revision provided"
                ) from exc
            else:
                raise FSError(f"Cannot fetch vlob {entry_id}: {exc}") from exc

        # Another backend error
        except BackendConnectionError as exc:
            raise FSError(f"Cannot fetch vlob {entry_id}: {exc}") from exc

        expected_author, expected_timestamp, expected_version, encrypted = args
        if version not in (None, expected_version):
            raise FSError(
                f"Backend returned invalid version for vlob {entry_id} (expecting {version}, got {expected_version})"
            )

        author = await self.remote_device_manager.get_device(expected_author)

        try:
            remote_manifest = RemoteManifest.decrypt_verify_and_load(
                encrypted,
                key=workspace_entry.key,
                author_verify_key=author.verify_key,
                expected_author=expected_author,
                expected_timestamp=expected_timestamp,
                expected_version=expected_version,
                expected_id=entry_id
                # TODO: check parent as well ?
            )
        except DataError as exc:
            raise FSError(f"Cannot decrypt vlob: {exc}") from exc

        # TODO: also store access id in remote_manifest and check it here
        return remote_manifest

    async def list_versions(self, entry_id: EntryID) -> Dict[int, Tuple[Pendulum, DeviceID]]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
        """
        try:
            versions_dict = await self.backend_cmds.vlob_list_versions(entry_id)

        # Vlob is not found
        except BackendCmdsNotFound as exc:
            raise FSRemoteManifestNotFound(entry_id) from exc

        # Backend is not available
        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        # Workspace is in maintenance
        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot download vlob while the workspace is in maintenance"
            ) from exc

        except BackendCmdsBadResponse as exc:
            if exc.status == "not_found":
                raise FSRemoteManifestNotFound(entry_id)
            else:
                raise FSError(f"Cannot fetch vlob {entry_id}: {exc}") from exc

        # Another backend error
        except BackendConnectionError as exc:
            raise FSError(f"Cannot fetch vlob {entry_id}: {exc}") from exc

        return versions_dict

    async def create_realm(self, realm_id: EntryID):
        """
        Raises:
            FSError
            FSBackendOfflineError
        """
        certif = RealmRoleCertificateContent.build_realm_root_certif(
            author=self.device.device_id, timestamp=pendulum_now(), realm_id=realm_id
        ).dump_and_sign(self.device.signing_key)

        try:
            await self.backend_cmds.realm_create(certif)

        except BackendCmdsBadResponse as exc:
            if exc.status == "already_exists":
                # It's possible a previous attempt to create this realm
                # succeeded but we didn't receive the confirmation, hence
                # we play idempotent here.
                return
            else:
                raise FSError(f"Cannot create realm {realm_id}: {exc}") from exc

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Cannot create realm {realm_id}: {exc}") from exc

    async def upload_manifest(self, entry_id: EntryID, manifest: RemoteManifest):
        """
        Raises:
            FSError
            FSRemoteSyncError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSBadEncryptionRevision
        """
        assert manifest.author == self.device.device_id
        assert timestamps_in_the_ballpark(manifest.timestamp, pendulum_now())

        workspace_entry = self.get_workspace_entry()

        try:
            ciphered = manifest.dump_sign_and_encrypt(
                key=workspace_entry.key, author_signkey=self.device.signing_key
            )
        except DataError as exc:
            raise FSError(f"Cannot encrypt vlob: {exc}") from exc

        # Upload the vlob
        if manifest.version == 1:
            await self._vlob_create(
                workspace_entry.encryption_revision, entry_id, ciphered, manifest.timestamp
            )
        else:
            await self._vlob_update(
                workspace_entry.encryption_revision,
                entry_id,
                ciphered,
                manifest.timestamp,
                manifest.version,
            )

    async def _vlob_create(
        self, encryption_revision: int, entry_id: EntryID, ciphered: bytes, now: Pendulum
    ):
        """
        Raises:
            FSError
            FSRemoteSyncError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
        """

        # Vlob updload
        try:
            await self.backend_cmds.vlob_create(
                self.workspace_id, encryption_revision, entry_id, now, ciphered
            )

        # Vlob alread exists
        except BackendCmdsAlreadyExists as exc:
            raise FSRemoteSyncError(entry_id) from exc

        except BackendCmdsNotAllowed as exc:
            # Seems we lost the access to the realm
            raise FSWorkspaceNoWriteAccess("Cannot upload manifest: no write access") from exc

        # Backend not available
        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        # Workspace in maintenance
        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot create vlob while the workspace is in maintenance"
            ) from exc

        except BackendCmdsBadResponse as exc:
            if exc.status == "already_exists":
                raise FSRemoteSyncError(entry_id)
            elif exc.status == "bad_encryption_revision":
                raise FSBadEncryptionRevision(
                    f"Cannot create vlob {entry_id}: Bad encryption revision provided"
                ) from exc
            else:
                raise FSError(f"Cannot create vlob {entry_id}: {exc}") from exc

        # Another backend error
        except BackendConnectionError as exc:
            raise FSError(f"Cannot update vlob: {exc}") from exc

    async def _vlob_update(
        self,
        encryption_revision: int,
        entry_id: EntryID,
        ciphered: bytes,
        now: Pendulum,
        version: int,
    ):
        """
        Raises:
            FSError
            FSRemoteSyncError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
        """
        # Vlob upload
        try:
            await self.backend_cmds.vlob_update(
                encryption_revision, entry_id, version, now, ciphered
            )

        # Vlob not found
        except BackendCmdsNotFound as exc:
            raise FSRemoteSyncError(entry_id) from exc

        except BackendCmdsNotAllowed as exc:
            # Seems we lost the access to the realm
            raise FSWorkspaceNoWriteAccess("Cannot upload manifest: no write access") from exc

        # Workspace in maintenance
        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot create vlob while the workspace is in maintenance"
            ) from exc

        # Versions do not match
        except BackendCmdsBadVersion as exc:
            raise FSRemoteSyncError(entry_id) from exc

        except BackendCmdsBadResponse as exc:
            if exc.status == "bad_version":
                raise FSRemoteSyncError(entry_id)
            elif exc.status == "bad_encryption_revision":
                raise FSBadEncryptionRevision(
                    f"Cannot update vlob {entry_id}: Bad encryption revision provided"
                ) from exc
            else:
                raise FSError(f"Cannot update vlob {entry_id}: {exc}") from exc

        # Backend not available
        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        # Another backend error
        except BackendConnectionError as exc:
            raise FSError(f"Cannot update vlob: {exc}") from exc

    def to_timestamped(self, timestamp):
        return RemoteLoaderTimestamped(self, timestamp)


class RemoteLoaderTimestamped(RemoteLoader):
    def __init__(self, remote_loader: RemoteLoader, timestamp: Pendulum):
        self.device = remote_loader.device
        self.workspace_id = remote_loader.workspace_id
        self.get_workspace_entry = remote_loader.get_workspace_entry
        self.backend_cmds = remote_loader.backend_cmds
        self.remote_device_manager = remote_loader.remote_device_manager
        self.local_storage = remote_loader.local_storage.to_timestamped(timestamp)
        self.timestamp = timestamp

    async def upload_block(self, *e, **ke):
        raise FSError(f"Cannot upload block through a timestamped remote loader")

    async def load_manifest(
        self, entry_id: EntryID, version: int = None, timestamp: Pendulum = None
    ) -> RemoteManifest:
        return await super().load_manifest(
            entry_id,
            version=version,
            timestamp=None
            if version is not None
            else self.timestamp
            if timestamp is None
            else timestamp,
        )

    async def upload_manifest(self, *e, **ke):
        raise FSError(f"Cannot upload manifest through a timestamped remote loader")

    async def _vlob_create(self, *e, **ke):
        raise FSError(f"Cannot create vlob through a timestamped remote loader")

    async def _vlob_update(self, *e, **ke):
        raise FSError(f"Cannot update vlob through a timestamped remote loader")
