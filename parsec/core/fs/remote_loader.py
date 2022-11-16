# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import math
from contextlib import contextmanager
from typing import (
    TYPE_CHECKING,
    Dict,
    List,
    Iterable,
    Tuple,
    Iterator,
    Callable,
    Awaitable,
)
import trio
from trio import open_memory_channel, MemorySendChannel, MemoryReceiveChannel

from parsec._parsec import (
    DateTime,
    BlockCreateRepOk,
    BlockCreateRepAlreadyExists,
    BlockCreateRepInMaintenance,
    BlockCreateRepNotAllowed,
    BlockCreateRepTimeout,
    BlockReadRepOk,
    BlockReadRepNotFound,
    BlockReadRepInMaintenance,
    BlockReadRepNotAllowed,
    RealmCreateRepOk,
    RealmCreateRepAlreadyExists,
    RealmGetRoleCertificatesRepOk,
    RealmGetRoleCertificatesRepNotAllowed,
    VlobCreateRepOk,
    VlobCreateRepSequesterInconsistency,
    VlobCreateRepAlreadyExists,
    VlobCreateRepBadEncryptionRevision,
    VlobCreateRepInMaintenance,
    VlobCreateRepNotAllowed,
    VlobCreateRepRequireGreaterTimestamp,
    VlobCreateRepRejectedBySequesterService,
    VlobCreateRepTimeout,
    VlobReadRepOk,
    VlobReadRepNotAllowed,
    VlobReadRepBadEncryptionRevision,
    VlobReadRepBadVersion,
    VlobReadRepInMaintenance,
    VlobReadRepNotFound,
    VlobUpdateRepOk,
    VlobUpdateRepBadEncryptionRevision,
    VlobUpdateRepBadVersion,
    VlobUpdateRepInMaintenance,
    VlobUpdateRepNotAllowed,
    VlobUpdateRepNotFound,
    VlobUpdateRepRequireGreaterTimestamp,
    VlobUpdateRepSequesterInconsistency,
    VlobUpdateRepRejectedBySequesterService,
    VlobUpdateRepTimeout,
    VlobListVersionsRepOk,
    VlobListVersionsRepInMaintenance,
    VlobListVersionsRepNotAllowed,
    VlobListVersionsRepNotFound,
)
from parsec.crypto import HashDigest, CryptoError, VerifyKey
from parsec.utils import open_service_nursery
from parsec.api.protocol import UserID, DeviceID, RealmID, RealmRole, VlobID, SequesterServiceID
from parsec.api.data import (
    DataError,
    BlockAccess,
    RealmRoleCertificate,
    AnyRemoteManifest,
    UserCertificate,
    DeviceCertificate,
    RevokedUserCertificate,
    SequesterAuthorityCertificate,
    SequesterServiceCertificate,
)
from parsec.api.data.manifest import manifest_decrypt_verify_and_load
from parsec.core.types import EntryID, ChunkID, LocalDevice, WorkspaceEntry
from parsec.core.backend_connection import (
    BackendConnectionError,
    BackendNotAvailable,
)
from parsec.core.remote_devices_manager import (
    RemoteDevicesManager,
    RemoteDevicesManagerBackendOfflineError,
    RemoteDevicesManagerError,
    RemoteDevicesManagerUserNotFoundError,
    RemoteDevicesManagerDeviceNotFoundError,
    RemoteDevicesManagerInvalidTrustchainError,
)
from parsec.core.fs.exceptions import (
    FSError,
    FSRemoteSyncError,
    FSRemoteOperationError,
    FSRemoteManifestNotFound,
    FSRemoteManifestNotFoundBadVersion,
    FSRemoteBlockNotFound,
    FSBackendOfflineError,
    FSServerUploadTemporarilyUnavailableError,
    FSWorkspaceInMaintenance,
    FSBadEncryptionRevision,
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
    FSUserNotFoundError,
    FSDeviceNotFoundError,
    FSInvalidTrustchainError,
    FSLocalMissError,
    FSSequesterServiceRejectedError,
)
from parsec.core.fs.storage import BaseWorkspaceStorage


if TYPE_CHECKING:
    from parsec.core.backend_connection import BackendAuthenticatedCmds

# This value is used to increment the timestamp provided by the backend
# when a manifest restamping is required. This value should be kept small
# compared to the certificate stamp ahead value, so the certificate updates have
# priority over manifest updates.
MANIFEST_STAMP_AHEAD_US = 100_000  # microseconds, or 0.1 seconds

# This value is used to increment the timestamp provided by the backend
# when a certificate restamping is required. This value should be kept big
# compared to the manifest stamp ahead value, so the certificate updates have
# priority over manifest updates.
ROLE_CERTIFICATE_STAMP_AHEAD_US = 500_000  # microseconds, or 0.5 seconds


class VlobRequireGreaterTimestampError(Exception):
    @property
    def strictly_greater_than(self) -> DateTime:
        return self.args[0]


class VlobSequesterInconsistencyError(Exception):
    def __init__(
        self,
        sequester_authority_certificate: bytes,
        sequester_services_certificates: Tuple[bytes],
    ):
        self.sequester_authority_certificate = sequester_authority_certificate
        self.sequester_services_certificates = sequester_services_certificates


def _validate_sequester_config(
    root_verify_key: VerifyKey,
    sequester_authority_certificate: bytes | None,
    sequester_services_certificates: Iterable[bytes] | None,
) -> Tuple[SequesterAuthorityCertificate | None, List[SequesterServiceCertificate] | None]:
    if sequester_authority_certificate is None:
        return None, None

    try:
        # In theory `sequester_authority_certificate` and `sequester_services_certificates`
        # should be both None or both not None. However this is a cheap check to
        # cover the case the server made a mistake.
        sequester_services_certificates = sequester_services_certificates or ()

        # 1) Validate authority certificate
        # Sequestery authority is always signed by the root key, hence `expected_author` is always None
        authority = SequesterAuthorityCertificate.verify_and_load(
            sequester_authority_certificate, author_verify_key=root_verify_key, expected_author=None
        )

        # 2) Validate services certificates
        services = []
        for sc in sequester_services_certificates:
            # Cannot use the regular `verify_and_load` here given authority key is
            # not a regular `parsec.crypto.VerifyKey`
            service = SequesterServiceCertificate.load(authority.verify_key_der.verify(sc))
            services.append(service)

    except (CryptoError, DataError) as exc:
        raise FSInvalidTrustchainError(
            f"Invalid sequester configuration returned by server: {exc}"
        ) from exc

    return authority, services


@contextmanager
def translate_remote_devices_manager_errors() -> Iterator[None]:
    try:
        yield
    except RemoteDevicesManagerBackendOfflineError as exc:
        raise FSBackendOfflineError(str(exc)) from exc
    except RemoteDevicesManagerUserNotFoundError as exc:
        raise FSUserNotFoundError(str(exc)) from exc
    except RemoteDevicesManagerDeviceNotFoundError as exc:
        raise FSDeviceNotFoundError(str(exc)) from exc
    except RemoteDevicesManagerInvalidTrustchainError as exc:
        raise FSInvalidTrustchainError(str(exc)) from exc
    except RemoteDevicesManagerError as exc:
        raise FSRemoteOperationError(str(exc)) from exc


@contextmanager
def translate_backend_cmds_errors() -> Iterator[None]:
    try:
        yield
    except BackendNotAvailable as exc:
        raise FSBackendOfflineError(str(exc)) from exc
    except BackendConnectionError as exc:
        raise FSRemoteOperationError(str(exc)) from exc


class UserRemoteLoader:
    def __init__(
        self,
        device: LocalDevice,
        workspace_id: EntryID,
        get_workspace_entry: Callable[[], WorkspaceEntry],
        get_previous_workspace_entry: Callable[[], Awaitable[WorkspaceEntry | None]],
        backend_cmds: BackendAuthenticatedCmds,
        remote_devices_manager: RemoteDevicesManager,
    ):
        self.device = device
        self.workspace_id = workspace_id
        self.get_workspace_entry = get_workspace_entry
        self.get_previous_workspace_entry = get_previous_workspace_entry
        self.backend_cmds = backend_cmds
        self.remote_devices_manager = remote_devices_manager
        self._realm_role_certificates_cache: List[RealmRoleCertificate] | None = None
        self._sequester_services_cache: List[SequesterServiceCertificate] | None = None

    def clear_realm_role_certificate_cache(self) -> None:
        self._realm_role_certificates_cache = None

    async def _get_user_realm_role_at(
        self, user_id: UserID, timestamp: DateTime, author_last_role_granted_on: DateTime
    ) -> RealmRole | None:

        # Lazily iterate over user certificates from newest to oldest
        def _get_user_certificates_from_cache() -> Iterator[RealmRoleCertificate]:
            if self._realm_role_certificates_cache is None:
                return
            for certif in reversed(self._realm_role_certificates_cache):
                if certif.user_id == user_id:
                    yield certif

        # Reload cache certificates if necessary
        last_certif = next(_get_user_certificates_from_cache(), None)
        if last_certif is None or (
            last_certif.timestamp < timestamp
            and last_certif.timestamp < author_last_role_granted_on
        ):
            self._realm_role_certificates_cache, _ = await self._load_realm_role_certificates()

        # Find the corresponding role
        assert self._realm_role_certificates_cache is not None
        for certif in _get_user_certificates_from_cache():
            if certif.timestamp <= timestamp:
                return certif.role
        else:
            return None

    async def _load_realm_role_certificates(
        self, realm_id: EntryID | None = None
    ) -> Tuple[List[RealmRoleCertificate], Dict[UserID, RealmRole]]:
        with translate_backend_cmds_errors():
            rep = await self.backend_cmds.realm_get_role_certificates(
                RealmID(realm_id or self.workspace_id)
            )
        if isinstance(rep, RealmGetRoleCertificatesRepNotAllowed):
            # Seems we lost the access to the realm
            raise FSWorkspaceNoReadAccess("Cannot get workspace roles: no read access")
        elif not isinstance(rep, RealmGetRoleCertificatesRepOk):
            raise FSError(f"Cannot retrieve workspace roles: {rep}")

        try:
            # Must read unverified certificates to access metadata
            unsecure_certifs = sorted(
                [
                    (RealmRoleCertificate.unsecure_load(uv_role), uv_role)
                    for uv_role in rep.certificates
                ],
                key=lambda x: x[0].timestamp,
            )

            current_roles: Dict[UserID, RealmRole] = {}
            owner_only = (RealmRole.OWNER,)
            owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)

            # Now verify each certif
            for unsecure_certif, raw_certif in unsecure_certifs:
                certif_author = unsecure_certif.author
                if certif_author is None:
                    raise FSError("Expected a certificate signed by a user")

                with translate_remote_devices_manager_errors():
                    author = await self.remote_devices_manager.get_device(certif_author)

                RealmRoleCertificate.verify_and_load(
                    raw_certif,
                    author_verify_key=author.verify_key,
                    expected_author=author.device_id,
                )

                # Make sure author had the right to do this
                existing_user_role = current_roles.get(unsecure_certif.user_id)
                if not current_roles and unsecure_certif.user_id == author.device_id.user_id:
                    # First user is auto-signed
                    needed_roles: Tuple[RealmRole | None, ...] = (None,)
                elif (
                    existing_user_role in owner_or_manager
                    or unsecure_certif.role in owner_or_manager
                ):
                    needed_roles = owner_only
                else:
                    needed_roles = owner_or_manager

                if current_roles.get(certif_author.user_id) not in needed_roles:
                    raise FSError(
                        f"Invalid realm role certificates: "
                        f"{unsecure_certif.author} has not right to give "
                        f"{unsecure_certif.role} role to {unsecure_certif.user_id.str} "
                        f"on {unsecure_certif.timestamp}"
                    )

                if unsecure_certif.role is None:
                    current_roles.pop(unsecure_certif.user_id, None)
                else:
                    current_roles[unsecure_certif.user_id] = unsecure_certif.role

        # Decryption error
        except DataError as exc:
            raise FSError(f"Invalid realm role certificates: {exc}") from exc

        # Now unsecure_certifs is no longer unsecure given we have validated its items
        return [c for c, _ in unsecure_certifs], current_roles

    async def load_realm_role_certificates(
        self, realm_id: EntryID | None = None
    ) -> List[RealmRoleCertificate]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSRemoteOperationError
            FSWorkspaceNoAccess
            FSUserNotFoundError
            FSDeviceNotFoundError
            FSInvalidTrustchainError
        """
        certificates, _ = await self._load_realm_role_certificates(realm_id)
        return certificates

    async def load_realm_current_roles(
        self, realm_id: EntryID | None = None
    ) -> Dict[UserID, RealmRole]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSRemoteOperationError
            FSWorkspaceNoAccess
            FSUserNotFoundError
            FSDeviceNotFoundError
            FSInvalidTrustchainError
        """
        _, current_roles = await self._load_realm_role_certificates(realm_id)
        return current_roles

    async def get_user(
        self, user_id: UserID, no_cache: bool = False
    ) -> Tuple[UserCertificate, RevokedUserCertificate | None]:
        """
        Raises:
            FSRemoteOperationError
            FSBackendOfflineError
            FSUserNotFoundError
            FSInvalidTrustchainError
        """
        with translate_remote_devices_manager_errors():
            return await self.remote_devices_manager.get_user(user_id, no_cache=no_cache)

    async def get_device(self, device_id: DeviceID, no_cache: bool = False) -> DeviceCertificate:
        """
        Raises:
            FSRemoteOperationError
            FSBackendOfflineError
            FSUserNotFoundError
            FSDeviceNotFoundError
            FSInvalidTrustchainError
        """
        with translate_remote_devices_manager_errors():
            return await self.remote_devices_manager.get_device(device_id, no_cache=no_cache)

    async def list_versions(self, entry_id: EntryID) -> Dict[int, Tuple[DateTime, DeviceID]]:
        """
        Raises:
            FSError
            FSRemoteOperationError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
        """
        with translate_backend_cmds_errors():
            rep = await self.backend_cmds.vlob_list_versions(VlobID(entry_id))
        if isinstance(rep, VlobListVersionsRepNotAllowed):
            # Seems we lost the access to the realm
            raise FSWorkspaceNoReadAccess("Cannot load manifest: no read access")
        elif isinstance(rep, VlobListVersionsRepNotFound):
            raise FSRemoteManifestNotFound(entry_id)
        elif isinstance(rep, VlobListVersionsRepInMaintenance):
            raise FSWorkspaceInMaintenance(
                "Cannot download vlob while the workspace is in maintenance"
            )
        elif not isinstance(rep, VlobListVersionsRepOk):
            raise FSError(f"Cannot fetch vlob {entry_id.hex}: {rep}")

        return rep.versions

    async def create_realm(self, realm_id: EntryID) -> None:
        """
        Raises:
            FSError
            FSRemoteOperationError
            FSBackendOfflineError
        """
        timestamp = self.device.timestamp()
        certif = RealmRoleCertificate.build_realm_root_certif(
            author=self.device.device_id, timestamp=timestamp, realm_id=RealmID(realm_id)
        ).dump_and_sign(self.device.signing_key)

        with translate_backend_cmds_errors():
            rep = await self.backend_cmds.realm_create(certif)

        if isinstance(rep, RealmCreateRepAlreadyExists):
            # It's possible a previous attempt to create this realm
            # succeeded but we didn't receive the confirmation, hence
            # we play idempotent here.
            return
        elif not isinstance(rep, RealmCreateRepOk):
            raise FSError(f"Cannot create realm {realm_id.hex}: {rep}")


class RemoteLoader(UserRemoteLoader):
    def __init__(
        self,
        device: LocalDevice,
        workspace_id: EntryID,
        get_workspace_entry: Callable[[], WorkspaceEntry],
        get_previous_workspace_entry: Callable[[], Awaitable[WorkspaceEntry | None]],
        backend_cmds: BackendAuthenticatedCmds,
        remote_devices_manager: RemoteDevicesManager,
        local_storage: BaseWorkspaceStorage,
    ):
        super().__init__(
            device,
            workspace_id,
            get_workspace_entry,
            get_previous_workspace_entry,
            backend_cmds,
            remote_devices_manager,
        )
        self.local_storage = local_storage

    async def load_blocks(self, accesses: List[BlockAccess]) -> None:
        async with open_service_nursery() as nursery:
            async with await self.receive_load_blocks(accesses, nursery) as receive_channel:
                async for value in receive_channel:
                    pass

    async def receive_load_blocks(
        self, blocks: List[BlockAccess], nursery: trio.Nursery
    ) -> "MemoryReceiveChannel[BlockAccess]":
        """
        Raises:
            FSError
            FSRemoteBlockNotFound
            FSBackendOfflineError
            FSWorkspaceInMaintenance
        """
        blocks_iter = iter(blocks)

        send_channel, receive_channel = open_memory_channel[BlockAccess](math.inf)

        async def _loader(send_channel: "MemorySendChannel[BlockAccess]") -> None:
            async with send_channel:
                while True:
                    access = next(blocks_iter, None)
                    if not access:
                        break
                    await self.load_block(access)
                    await send_channel.send(access)

        async with send_channel:
            for _ in range(4):
                nursery.start_soon(_loader, send_channel.clone())

        return receive_channel

    async def load_block(self, access: BlockAccess) -> None:
        """
        Raises:
            FSError
            FSRemoteBlockNotFound
            FSBackendOfflineError
            FSRemoteOperationError
            FSWorkspaceInMaintenance
            FSWorkspaceNoAccess
        """
        # Download
        with translate_backend_cmds_errors():
            rep = await self.backend_cmds.block_read(access.id)
        if isinstance(rep, BlockReadRepNotFound):
            raise FSRemoteBlockNotFound(access)
        elif isinstance(rep, BlockReadRepNotAllowed):
            # Seems we lost the access to the realm
            raise FSWorkspaceNoReadAccess("Cannot load block: no read access")
        elif isinstance(rep, BlockReadRepInMaintenance):
            raise FSWorkspaceInMaintenance(
                "Cannot download block while the workspace in maintenance"
            )
        elif not isinstance(rep, BlockReadRepOk):
            raise FSError(f"Cannot download block: `{rep}`")

        # Decryption
        try:
            block = access.key.decrypt(rep.block)

        # Decryption error
        except CryptoError as exc:
            raise FSError(f"Cannot decrypt block: {exc}") from exc

        # TODO: let encryption manager do the digest check ?
        assert HashDigest.from_data(block) == access.digest, access
        await self.local_storage.set_clean_block(access.id, block)

    async def upload_blocks(self, blocks: List[BlockAccess]) -> None:
        blocks_iter = iter(blocks)

        async def _uploader() -> None:
            while True:
                access = next(blocks_iter, None)
                if not access:
                    break
                try:
                    data = await self.local_storage.get_dirty_block(access.id)
                except FSLocalMissError:
                    continue
                await self.upload_block(access, data)

        async with open_service_nursery() as nursery:
            for _ in range(4):
                nursery.start_soon(_uploader)

    async def upload_block(self, access: BlockAccess, data: bytes) -> None:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSRemoteOperationError
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
        with translate_backend_cmds_errors():
            rep = await self.backend_cmds.block_create(
                access.id, RealmID(self.workspace_id), ciphered
            )

        if isinstance(rep, BlockCreateRepAlreadyExists):
            # Ignore exception if the block has already been uploaded
            # This might happen when a failure occurs before the local storage is updated
            pass
        elif isinstance(rep, BlockCreateRepNotAllowed):
            # Seems we lost the access to the realm
            raise FSWorkspaceNoWriteAccess("Cannot upload block: no write access")
        elif isinstance(rep, BlockCreateRepInMaintenance):
            raise FSWorkspaceInMaintenance("Cannot upload block while the workspace in maintenance")
        elif isinstance(rep, BlockCreateRepTimeout):
            raise FSServerUploadTemporarilyUnavailableError("Temporary failure during block upload")
        elif not isinstance(rep, BlockCreateRepOk):
            raise FSError(f"Cannot upload block: {rep}")

        # Update local storage
        await self.local_storage.set_clean_block(access.id, data)
        await self.local_storage.clear_chunk(ChunkID(access.id), miss_ok=True)

    async def load_manifest(
        self,
        entry_id: EntryID,
        version: int | None = None,
        timestamp: DateTime | None = None,
        expected_backend_timestamp: DateTime | None = None,
        workspace_entry: WorkspaceEntry | None = None,
    ) -> AnyRemoteManifest:
        """
        Download a manifest.

        Only one from version or timestamp parameters can be specified at the same time.
        expected_backend_timestamp enables to check a timestamp against the one returned by the
        backend.

        Raises:
            FSError
            FSBackendOfflineError
            FSRemoteOperationError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
            FSUserNotFoundError
            FSDeviceNotFoundError
            FSInvalidTrustchainError
        """
        assert (
            timestamp is None or version is None
        ), "Either timestamp or version argument should be provided"
        # Get the current and requested workspace entry
        # They're usually the same, except when loading from a workspace while it's in maintenance
        current_workspace_entry = self.get_workspace_entry()
        workspace_entry = current_workspace_entry if workspace_entry is None else workspace_entry
        # Download the vlob
        with translate_backend_cmds_errors():
            rep = await self.backend_cmds.vlob_read(
                workspace_entry.encryption_revision,
                VlobID(entry_id),
                version=version,
                timestamp=timestamp if version is None else None,
            )
        # Special case for loading manifest while in maintenance.
        # This is done to allow users to fetch data from a workspace while it's being reencrypted.
        # If the workspace is in maintenance for another reason (such as garbage collection),
        # the recursive call to load manifest will simply also fail with an FSWorkspaceInMaintenance.
        if (
            isinstance(rep, VlobReadRepInMaintenance)
            and workspace_entry.encryption_revision == current_workspace_entry.encryption_revision
        ):
            # Getting the last workspace entry with the previous encryption revision
            # requires one or several calls to the backend, meaning the following exceptions might get raised:
            # - FSError
            # - FSBackendOfflineError
            # - FSWorkspaceInMaintenance
            # It is fine to let those exceptions bubble up as there all valid reasons for failing to load a manifest.
            previous_workspace_entry = await self.get_previous_workspace_entry()
            if previous_workspace_entry is not None:
                # Make sure we don't fall into an infinite loop because of some other bug
                assert (
                    previous_workspace_entry.encryption_revision
                    < self.get_workspace_entry().encryption_revision
                )
                # Recursive call to `load_manifest`, requiring an older encryption revision than the current one
                return await self.load_manifest(
                    entry_id,
                    version=version,
                    timestamp=timestamp,
                    expected_backend_timestamp=expected_backend_timestamp,
                    workspace_entry=previous_workspace_entry,
                )

        if isinstance(rep, VlobReadRepNotFound):
            raise FSRemoteManifestNotFound(entry_id)
        elif isinstance(rep, VlobReadRepNotAllowed):
            # Seems we lost the access to the realm
            raise FSWorkspaceNoReadAccess("Cannot load manifest: no read access")
        elif isinstance(rep, VlobReadRepBadVersion):
            raise FSRemoteManifestNotFoundBadVersion(entry_id)
        elif isinstance(rep, VlobReadRepBadEncryptionRevision):
            raise FSBadEncryptionRevision(
                f"Cannot fetch vlob {entry_id.hex}: Bad encryption revision provided"
            )
        elif isinstance(rep, VlobReadRepInMaintenance):
            raise FSWorkspaceInMaintenance(
                "Cannot download vlob while the workspace is in maintenance"
            )
        elif not isinstance(rep, VlobReadRepOk):
            raise FSError(f"Cannot fetch vlob {entry_id.hex}: {rep}")

        expected_version = rep.version
        expected_author = rep.author
        expected_timestamp = rep.timestamp
        if version not in (None, expected_version):
            raise FSError(
                f"Backend returned invalid version for vlob {entry_id.hex} (expecting {version}, "
                f"got {expected_version})"
            )

        if expected_backend_timestamp and expected_backend_timestamp != expected_timestamp:
            raise FSError(
                f"Backend returned invalid expected timestamp for vlob {entry_id.hex} at version "
                f"{version} (expecting {expected_backend_timestamp}, got {expected_timestamp})"
            )

        with translate_remote_devices_manager_errors():
            author = await self.remote_devices_manager.get_device(expected_author)

        try:
            remote_manifest = manifest_decrypt_verify_and_load(
                rep.blob,
                key=workspace_entry.key,
                author_verify_key=author.verify_key,
                expected_author=expected_author,
                expected_timestamp=expected_timestamp,
                expected_version=expected_version,
                expected_id=entry_id,
            )
        except DataError as exc:
            raise FSError(f"Cannot decrypt vlob: {exc}") from exc

        # Get the timestamp of the last role for this particular user
        author_last_role_granted_on = rep.author_last_role_granted_on
        # Compatibility with older backends (best effort strategy)
        if author_last_role_granted_on is None:
            author_last_role_granted_on = self.device.timestamp()

        # Finally make sure author was allowed to create this manifest
        role_at_timestamp = await self._get_user_realm_role_at(
            expected_author.user_id, expected_timestamp, author_last_role_granted_on
        )
        if role_at_timestamp is None:
            raise FSError(
                f"Manifest was created at {expected_timestamp} by `{expected_author.str}` "
                "which had no right to access the workspace at that time"
            )
        elif role_at_timestamp == RealmRole.READER:
            raise FSError(
                f"Manifest was created at {expected_timestamp} by `{expected_author.str}` "
                "which had no right to write on the workspace at that time"
            )

        return remote_manifest

    async def upload_manifest(
        self,
        entry_id: EntryID,
        manifest: AnyRemoteManifest,
        timestamp_greater_than: DateTime | None = None,
    ) -> AnyRemoteManifest:
        """
        Raises:
            FSError
            FSRemoteSyncError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSBadEncryptionRevision
            FSInvalidTrustchainError: if backend send invalid sequester configuration
        """
        assert manifest.author == self.device.device_id

        # Restamp the manifest before uploading
        timestamp = self.device.timestamp()
        if timestamp_greater_than is not None:
            timestamp = max(
                timestamp, timestamp_greater_than.add(microseconds=MANIFEST_STAMP_AHEAD_US)
            )

        manifest = manifest.evolve(timestamp=timestamp)

        workspace_entry = self.get_workspace_entry()

        if self._sequester_services_cache is None:
            # Regular mode: we only encrypt the blob with the workspace symmetric key
            sequester_blob = None
            try:
                ciphered = manifest.dump_sign_and_encrypt(
                    key=workspace_entry.key, author_signkey=self.device.signing_key
                )
            except DataError as exc:
                raise FSError(f"Cannot encrypt vlob: {exc}") from exc

        else:
            # Sequestered organization mode: we also encrypt the blob with each
            # sequester services' asymmetric encryption key
            try:
                signed = manifest.dump_and_sign(author_signkey=self.device.signing_key)
            except DataError as exc:
                raise FSError(f"Cannot encrypt vlob: {exc}") from exc

            ciphered = workspace_entry.key.encrypt(signed)
            sequester_blob = {}
            for service in self._sequester_services_cache:
                sequester_blob[service.service_id] = service.encryption_key_der.encrypt(signed)

        # Upload the vlob
        try:
            if manifest.version == 1:
                await self._vlob_create(
                    workspace_entry.encryption_revision,
                    entry_id,
                    ciphered,
                    manifest.timestamp,
                    sequester_blob,
                )
            else:
                await self._vlob_update(
                    workspace_entry.encryption_revision,
                    entry_id,
                    ciphered,
                    manifest.timestamp,
                    manifest.version,
                    sequester_blob,
                )
        # The backend notified us that some restamping is required
        except VlobRequireGreaterTimestampError as exc:
            return await self.upload_manifest(entry_id, manifest, exc.strictly_greater_than)
        # The backend notified us that we didn't encrypt the blob for the right sequester
        # services. This typically occurs for the first vlob update/create (since we lazily
        # fetch sequester config) or if a sequester service has been created/disabled.
        except VlobSequesterInconsistencyError as exc:
            # Ensure the config send by the backend is valid
            _, sequester_services = _validate_sequester_config(
                root_verify_key=self.device.root_verify_key,
                sequester_authority_certificate=exc.sequester_authority_certificate,
                sequester_services_certificates=exc.sequester_services_certificates,
            )
            # Update our cache and retry the request
            self._sequester_services_cache = sequester_services
            return await self.upload_manifest(entry_id, manifest)
        except FSSequesterServiceRejectedError as exc:
            # Small hack to provide the manifest object that was lacking when the exception was raised
            exc.manifest = manifest
            raise exc
        else:
            return manifest

    async def _vlob_create(
        self,
        encryption_revision: int,
        entry_id: EntryID,
        ciphered: bytes,
        now: DateTime,
        sequester_blob: Dict[SequesterServiceID, bytes] | None,
    ) -> None:
        """
        Raises:
            FSError
            FSRemoteSyncError
            FSBackendOfflineError
            FSRemoteOperationError
            FSWorkspaceInMaintenance
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
        """

        # Vlob upload
        with translate_backend_cmds_errors():
            rep = await self.backend_cmds.vlob_create(
                RealmID(self.workspace_id),
                encryption_revision,
                VlobID(entry_id),
                now,
                ciphered,
                sequester_blob,
            )
        if isinstance(rep, VlobCreateRepAlreadyExists):
            raise FSRemoteSyncError(entry_id)
        elif isinstance(rep, VlobCreateRepNotAllowed):
            # Seems we lost the access to the realm
            raise FSWorkspaceNoWriteAccess("Cannot upload manifest: no write access")
        elif isinstance(rep, VlobCreateRepRequireGreaterTimestamp):
            raise VlobRequireGreaterTimestampError(rep.strictly_greater_than)
        elif isinstance(rep, VlobCreateRepBadEncryptionRevision):
            raise FSBadEncryptionRevision(
                f"Cannot create vlob {entry_id.hex}: Bad encryption revision provided"
            )
        elif isinstance(rep, VlobCreateRepInMaintenance):
            raise FSWorkspaceInMaintenance(
                "Cannot create vlob while the workspace is in maintenance"
            )
        elif isinstance(rep, VlobCreateRepSequesterInconsistency):
            raise VlobSequesterInconsistencyError(
                sequester_authority_certificate=rep.sequester_authority_certificate,
                sequester_services_certificates=rep.sequester_services_certificates,
            )
        elif isinstance(rep, VlobCreateRepRejectedBySequesterService):
            raise FSSequesterServiceRejectedError(
                id=entry_id,
                service_id=rep.service_id,
                service_label=rep.service_label,
                reason=rep.reason,
            )
        elif isinstance(rep, VlobCreateRepTimeout):
            raise FSServerUploadTemporarilyUnavailableError("Temporary failure during vlob upload")
        elif not isinstance(rep, VlobCreateRepOk):
            raise FSError(f"Cannot create vlob {entry_id.hex}: {rep}")

    async def _vlob_update(
        self,
        encryption_revision: int,
        entry_id: EntryID,
        ciphered: bytes,
        now: DateTime,
        version: int,
        sequester_blob: Dict[SequesterServiceID, bytes] | None,
    ) -> None:
        """
        Raises:
            FSError
            FSRemoteSyncError
            FSBackendOfflineError
            FSRemoteOperationError
            FSWorkspaceInMaintenance
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
        """
        # Vlob upload
        with translate_backend_cmds_errors():
            rep = await self.backend_cmds.vlob_update(
                encryption_revision, VlobID(entry_id), version, now, ciphered, sequester_blob
            )

        if isinstance(rep, VlobUpdateRepNotFound):
            raise FSRemoteSyncError(entry_id)
        elif isinstance(rep, VlobUpdateRepNotAllowed):
            # Seems we lost the access to the realm
            raise FSWorkspaceNoWriteAccess("Cannot upload manifest: no write access")
        elif isinstance(rep, VlobUpdateRepRequireGreaterTimestamp):
            raise VlobRequireGreaterTimestampError(rep.strictly_greater_than)
        elif isinstance(rep, VlobUpdateRepBadVersion):
            raise FSRemoteSyncError(entry_id)
        elif isinstance(rep, VlobUpdateRepBadEncryptionRevision):
            raise FSBadEncryptionRevision(
                f"Cannot update vlob {entry_id.hex}: Bad encryption revision provided"
            )
        elif isinstance(rep, VlobUpdateRepInMaintenance):
            raise FSWorkspaceInMaintenance(
                "Cannot create vlob while the workspace is in maintenance"
            )
        elif isinstance(rep, VlobUpdateRepSequesterInconsistency):
            raise VlobSequesterInconsistencyError(
                sequester_authority_certificate=rep.sequester_authority_certificate,
                sequester_services_certificates=rep.sequester_services_certificates,
            )
        elif isinstance(rep, VlobUpdateRepRejectedBySequesterService):
            raise FSSequesterServiceRejectedError(
                id=entry_id,
                service_id=rep.service_id,
                service_label=rep.service_label,
                reason=rep.reason,
            )
        elif isinstance(rep, VlobUpdateRepTimeout):
            raise FSServerUploadTemporarilyUnavailableError("Temporary failure during vlob upload")
        elif not isinstance(rep, VlobUpdateRepOk):
            raise FSError(f"Cannot update vlob {entry_id.hex}: {rep}")

    def to_timestamped(self, timestamp: DateTime) -> "RemoteLoaderTimestamped":
        return RemoteLoaderTimestamped(self, timestamp)


class RemoteLoaderTimestamped(RemoteLoader):
    def __init__(self, remote_loader: RemoteLoader, timestamp: DateTime):
        self.device = remote_loader.device
        self.workspace_id = remote_loader.workspace_id
        self.get_workspace_entry = remote_loader.get_workspace_entry
        self.get_previous_workspace_entry = remote_loader.get_previous_workspace_entry
        self.backend_cmds = remote_loader.backend_cmds
        self.remote_devices_manager = remote_loader.remote_devices_manager
        self.local_storage = remote_loader.local_storage.to_timestamped(timestamp)
        self._realm_role_certificates_cache = None
        self.timestamp = timestamp

    async def upload_block(self, access: BlockAccess, data: bytes) -> None:
        raise FSError("Cannot upload block through a timestamped remote loader")

    async def load_manifest(
        self,
        entry_id: EntryID,
        version: int | None = None,
        timestamp: DateTime | None = None,
        expected_backend_timestamp: DateTime | None = None,
        workspace_entry: WorkspaceEntry | None = None,
    ) -> AnyRemoteManifest:
        """
        Allows to have manifests at all timestamps as it is needed by the versions method of either
        a WorkspaceFS or a WorkspaceFSTimestamped

        Only one from version or timestamp can be specified at the same time.
        expected_backend_timestamp enables to check a timestamp against the one returned by the
        backend.

        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSRemoteManifestNotFound
            FSBadEncryptionRevision
            FSWorkspaceNoAccess
        """
        if timestamp is None and version is None:
            timestamp = self.timestamp
        return await super().load_manifest(
            entry_id,
            version=version,
            timestamp=timestamp,
            expected_backend_timestamp=expected_backend_timestamp,
            workspace_entry=workspace_entry,
        )

    async def upload_manifest(
        self,
        entry_id: EntryID,
        manifest: AnyRemoteManifest,
        timestamp_greater_than: DateTime | None = None,
    ) -> AnyRemoteManifest:
        raise FSError("Cannot upload manifest through a timestamped remote loader")

    async def _vlob_create(
        self,
        encryption_revision: int,
        entry_id: EntryID,
        ciphered: bytes,
        now: DateTime,
        sequester_blob: Dict[SequesterServiceID, bytes] | None,
    ) -> None:
        raise FSError("Cannot create vlob through a timestamped remote loader")

    async def _vlob_update(
        self,
        encryption_revision: int,
        entry_id: EntryID,
        ciphered: bytes,
        now: DateTime,
        version: int,
        sequester_blob: Dict[SequesterServiceID, bytes] | None,
    ) -> None:
        raise FSError("Cannot update vlob through a timestamped remote loader")
