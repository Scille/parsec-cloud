# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
from pathlib import Path
from trio_typing import TaskStatus
from pendulum import DateTime
from typing import (
    Tuple,
    Optional,
    Union,
    Dict,
    Sequence,
    Pattern,
    Type,
    TypeVar,
    AsyncIterator,
    List,
)
from structlog import get_logger

from async_generator import asynccontextmanager

from parsec.utils import open_service_nursery
from parsec.core.core_events import CoreEvent
from parsec.event_bus import EventBus
from parsec.crypto import SecretKey
from parsec.api.data import (
    DataError,
    RealmRoleCertificateContent,
    BaseMessageContent,
    UserCertificateContent,
    SharingGrantedMessageContent,
    SharingReencryptedMessageContent,
    SharingRevokedMessageContent,
    PingMessageContent,
    UserManifest,
)
from parsec.api.protocol import UserID, DeviceID, MaintenanceType, RealmID, VlobID
from parsec.core.types import (
    EntryID,
    EntryName,
    LocalDevice,
    WorkspaceEntry,
    WorkspaceRole,
    LocalUserManifest,
)
from parsec.core.config import DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE

# TODO: handle exceptions status...
from parsec.core.backend_connection import (
    BackendAuthenticatedCmds,
    BackendConnectionError,
    BackendNotAvailable,
)
from parsec.core.remote_devices_manager import RemoteDevicesManager

from parsec.core.fs.workspacefs import WorkspaceFS
from parsec.core.fs.remote_loader import UserRemoteLoader
from parsec.core.fs.remote_loader import ROLE_CERTIFICATE_STAMP_AHEAD_US, MANIFEST_STAMP_AHEAD_US
from parsec.core.fs.storage import (
    UserStorage,
    WorkspaceStorage,
    workspace_storage_non_speculative_init,
)
from parsec.core.fs.userfs.merging import merge_local_user_manifests, merge_workspace_entry
from parsec.core.fs.exceptions import (
    FSError,
    FSWorkspaceNoAccess,
    FSWorkspaceNotFoundError,
    FSBackendOfflineError,
    FSSharingNotAllowedError,
    FSWorkspaceInMaintenance,
    FSWorkspaceNotInMaintenance,
)


logger = get_logger()

AnyEntryName = Union[EntryName, str]


class ReencryptionJob:
    def __init__(
        self,
        backend_cmds: BackendAuthenticatedCmds,
        new_workspace_entry: WorkspaceEntry,
        old_workspace_entry: WorkspaceEntry,
    ) -> None:
        self.backend_cmds = backend_cmds
        self.new_workspace_entry = new_workspace_entry
        self.old_workspace_entry = old_workspace_entry
        assert new_workspace_entry.id == old_workspace_entry.id

    async def do_one_batch(self, size: int = 1000) -> Tuple[int, int]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSWorkspaceNoAccess
        """
        workspace_id = RealmID(self.new_workspace_entry.id.uuid)
        new_encryption_revision = self.new_workspace_entry.encryption_revision

        # Get the batch
        try:
            rep = await self.backend_cmds.vlob_maintenance_get_reencryption_batch(
                workspace_id, new_encryption_revision, size
            )
            if rep["status"] in ("not_in_maintenance", "bad_encryption_revision"):
                raise FSWorkspaceNotInMaintenance(f"Reencryption job already finished: {rep}")
            elif rep["status"] == "not_allowed":
                raise FSWorkspaceNoAccess(
                    f"Not allowed to do reencryption maintenance on workspace {workspace_id}: {rep}"
                )
            elif rep["status"] != "ok":
                raise FSError(
                    f"Cannot do reencryption maintenance on workspace {workspace_id}: {rep}"
                )

            donebatch = []
            for item in rep["batch"]:
                cleartext = self.old_workspace_entry.key.decrypt(item["blob"])
                newciphered = self.new_workspace_entry.key.encrypt(cleartext)
                donebatch.append((item["vlob_id"], item["version"], newciphered))

            rep = await self.backend_cmds.vlob_maintenance_save_reencryption_batch(
                workspace_id, new_encryption_revision, donebatch
            )
            if rep["status"] in ("not_in_maintenance", "bad_encryption_revision"):
                raise FSWorkspaceNotInMaintenance(f"Reencryption job already finished: {rep}")
            elif rep["status"] == "not_allowed":
                raise FSWorkspaceNoAccess(
                    f"Not allowed to do reencryption maintenance on workspace {workspace_id}: {rep}"
                )
            elif rep["status"] != "ok":
                raise FSError(
                    f"Cannot do reencryption maintenance on workspace {workspace_id}: {rep}"
                )
            total = rep["total"]
            done = rep["done"]

            if total == done:
                # Finish the maintenance
                rep = await self.backend_cmds.realm_finish_reencryption_maintenance(
                    workspace_id, new_encryption_revision
                )
                if rep["status"] in ("not_in_maintenance", "bad_encryption_revision"):
                    raise FSWorkspaceNotInMaintenance(f"Reencryption job already finished: {rep}")
                elif rep["status"] == "not_allowed":
                    raise FSWorkspaceNoAccess(
                        f"Not allowed to do reencryption maintenance on workspace {workspace_id}: {rep}"
                    )
                elif rep["status"] != "ok":
                    raise FSError(
                        f"Cannot do reencryption maintenance on workspace {workspace_id}: {rep}"
                    )

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(
                f"Cannot do reencryption maintenance on workspace {workspace_id}: {exc}"
            ) from exc

        return total, done


UserFSTypeVar = TypeVar("UserFSTypeVar", bound="UserFS")


class UserFS:
    def __init__(
        self,
        data_base_dir: Path,
        device: LocalDevice,
        backend_cmds: BackendAuthenticatedCmds,
        remote_devices_manager: RemoteDevicesManager,
        event_bus: EventBus,
        prevent_sync_pattern: Pattern[str],
        preferred_language: str,
        workspace_storage_cache_size: int,
    ):
        self.data_base_dir = data_base_dir
        self.device = device
        self.backend_cmds = backend_cmds
        self.remote_devices_manager = remote_devices_manager
        self.event_bus = event_bus
        self.prevent_sync_pattern = prevent_sync_pattern
        self.preferred_language = preferred_language
        self.workspace_storage_cache_size = workspace_storage_cache_size

        self.storage: UserStorage  # Setup by UserStorage.run factory

        # Message processing is done in-order, hence it is pointless to do
        # it concurrently
        self._workspace_storage_nursery: trio.Nursery  # Setup by UserStorage.run factory
        self._process_messages_lock = trio.Lock()
        self._update_user_manifest_lock = trio.Lock()
        self._workspaces: Dict[EntryID, WorkspaceFS] = {}

        timestamp = self.device.timestamp()
        wentry = WorkspaceEntry(
            name=EntryName("<user manifest>"),
            id=device.user_manifest_id,
            key=device.user_manifest_key,
            encryption_revision=1,
            encrypted_on=timestamp,
            role_cached_on=timestamp,
            role=WorkspaceRole.OWNER,
        )

        async def _get_previous_entry() -> WorkspaceEntry:
            raise NotImplementedError

        self.remote_loader = UserRemoteLoader(
            self.device,
            self.device.user_manifest_id,
            lambda: wentry,
            _get_previous_entry,
            self.backend_cmds,
            self.remote_devices_manager,
        )

    @classmethod
    @asynccontextmanager
    async def run(
        cls: Type[UserFSTypeVar],
        data_base_dir: Path,
        device: LocalDevice,
        backend_cmds: BackendAuthenticatedCmds,
        remote_devices_manager: RemoteDevicesManager,
        event_bus: EventBus,
        prevent_sync_pattern: Pattern[str],
        preferred_language: str = "en",
        workspace_storage_cache_size: int = DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
    ) -> AsyncIterator[UserFSTypeVar]:
        self = cls(
            data_base_dir,
            device,
            backend_cmds,
            remote_devices_manager,
            event_bus,
            prevent_sync_pattern,
            preferred_language,
            workspace_storage_cache_size,
        )

        # Run user storage
        async with UserStorage.run(self.data_base_dir, self.device) as self.storage:

            # Nursery for workspace storages
            async with open_service_nursery() as self._workspace_storage_nursery:

                # Make sure all the workspaces are loaded
                # In particular, we want to make sure that any workspace available through
                # `userfs.get_user_manifest().workspaces` is also available through
                # `userfs.get_workspace(workspace_id)`.
                for workspace_entry in self.get_user_manifest().workspaces:
                    await self._load_workspace(workspace_entry.id)

                yield self

                # Stop the workspace storages
                self._workspace_storage_nursery.cancel_scope.cancel()

    @property
    def user_manifest_id(self) -> EntryID:
        return self.device.user_manifest_id

    def get_user_manifest(self) -> LocalUserManifest:
        """
        Raises: ValueError
        """
        if self.storage is None:
            raise ValueError("Storage not set")
        return self.storage.get_user_manifest()

    async def set_user_manifest(self, manifest: LocalUserManifest) -> None:

        if self.storage is None:
            raise ValueError("Storage not set")

        # Make sure all the workspaces are loaded
        # In particular, we want to make sure that any workspace available through
        # `userfs.get_user_manifest().workspaces` is also available through
        # `userfs.get_workspace(workspace_id)`. Note that the loading operation
        # is idempotent, so workspaces do not get reloaded.
        for workspace_entry in manifest.workspaces:
            await self._load_workspace(workspace_entry.id)

        await self.storage.set_user_manifest(manifest)

    async def _instantiate_workspace(self, workspace_id: EntryID) -> WorkspaceFS:
        # Workspace entry can change at any time, so we provide a way for
        # WorkspaceFS to load it each time it is needed
        def get_workspace_entry() -> WorkspaceEntry:
            """
            Return the current workspace entry.

            Raises:
                FSWorkspaceNotFoundError
            """
            user_manifest = self.get_user_manifest()
            workspace_entry = user_manifest.get_workspace_entry(workspace_id)
            if not workspace_entry:
                raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")
            return workspace_entry

        async def get_previous_workspace_entry() -> Optional[WorkspaceEntry]:
            """
            Return the most recent workspace entry using the previous encryption revision.
            This requires one or several calls to the backend.

            Raises:
                FSError
                FSBackendOfflineError
                FSWorkspaceInMaintenance
            """
            workspace_entry = get_workspace_entry()
            return await self._get_previous_workspace_entry(workspace_entry)

        # Instantiate the local storage

        async def workspace_storage_task(
            task_status: TaskStatus[WorkspaceStorage] = trio.TASK_STATUS_IGNORED
        ) -> None:
            async with WorkspaceStorage.run(
                data_base_dir=self.data_base_dir,
                device=self.device,
                workspace_id=workspace_id,
                cache_size=self.workspace_storage_cache_size,
            ) as workspace_storage:
                task_status.started(workspace_storage)
                await trio.sleep_forever()

        local_storage = await self._workspace_storage_nursery.start(workspace_storage_task)

        # Instantiate the workspace
        workspace = WorkspaceFS(
            workspace_id=workspace_id,
            get_workspace_entry=get_workspace_entry,
            get_previous_workspace_entry=get_previous_workspace_entry,
            device=self.device,
            local_storage=local_storage,
            backend_cmds=self.backend_cmds,
            event_bus=self.event_bus,
            remote_devices_manager=self.remote_devices_manager,
            preferred_language=self.preferred_language,
        )

        # Apply the current "prevent sync" pattern
        await workspace.set_and_apply_prevent_sync_pattern(self.prevent_sync_pattern)

        return workspace

    async def _load_workspace(self, workspace_id: EntryID) -> WorkspaceFS:
        """
        Raises:
            FSWorkspaceNotFoundError
        """
        # The workspace has already been instantiated
        if workspace_id in self._workspaces:
            return self._workspaces[workspace_id]

        # Instantiate the workpace
        workspace = await self._instantiate_workspace(workspace_id)

        # Set and return
        return self._workspaces.setdefault(workspace_id, workspace)

    def get_workspace(self, workspace_id: EntryID) -> WorkspaceFS:
        # UserFS provides the guarantee that any workspace available through
        # `userfs.get_user_manifest().workspaces` is also available in
        # `self._workspaces`.
        try:
            workspace = self._workspaces[workspace_id]
        except KeyError:
            raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

        # Sanity check to make sure workspace_id is valid
        workspace.get_workspace_entry()

        return workspace

    async def workspace_create(self, name: EntryName) -> EntryID:
        assert isinstance(name, EntryName)

        async with self._update_user_manifest_lock:
            timestamp = self.device.timestamp()
            workspace_entry = WorkspaceEntry.new(name, timestamp=timestamp)
            user_manifest = self.get_user_manifest()
            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(
                timestamp, workspace_entry
            )
            # Given *we* are the creator of the workspace, our placeholder is
            # the only non-speculative one.
            #
            # Note the save order is important given there is no atomicity
            # between saving the non-speculative workspace manifest placeholder
            # and the save of the user manifest containing the workspace entry.
            # Indeed, if we would save the user manifest first and a crash
            # occured before saving the placeholder, we would endup in the same
            # situation as if the workspace has been created by someone else
            # (i.e. a workspace entry but no local data about this workspace)
            # so we would fallback to a local speculative workspace manifest.
            # However a speculative manifest means the workspace have been
            # created by somebody else, and hence we shouldn't try to create
            # it corresponding realm in the backend !
            await workspace_storage_non_speculative_init(
                data_base_dir=self.data_base_dir,
                device=self.device,
                workspace_id=workspace_entry.id,
            )
            await self.set_user_manifest(user_manifest)
            self.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=self.user_manifest_id)
            self.event_bus.send(CoreEvent.FS_WORKSPACE_CREATED, new_entry=workspace_entry)

        return workspace_entry.id

    async def workspace_rename(self, workspace_id: EntryID, new_name: EntryName) -> None:
        """
        Raises:
            FSWorkspaceNotFoundError
        """
        assert isinstance(new_name, EntryName)

        async with self._update_user_manifest_lock:
            user_manifest = self.get_user_manifest()
            workspace_entry = user_manifest.get_workspace_entry(workspace_id)
            if not workspace_entry:
                raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

            timestamp = self.device.timestamp()
            updated_workspace_entry = workspace_entry.evolve(name=new_name)
            updated_user_manifest = user_manifest.evolve_workspaces_and_mark_updated(
                timestamp, updated_workspace_entry
            )
            await self.set_user_manifest(updated_user_manifest)
            self.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=self.user_manifest_id)

    async def _fetch_remote_user_manifest(self, version: Optional[int] = None) -> UserManifest:
        """
        Raises:
            FSError
            FSWorkspaceInMaintenance
            FSBackendOfflineError
        """
        try:
            # Note encryption_revision is always 1 given we never reencrypt
            # the user manifest's realm
            rep = await self.backend_cmds.vlob_read(1, VlobID(self.user_manifest_id.uuid), version)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        if rep["status"] == "in_maintenance":
            raise FSWorkspaceInMaintenance(
                "Cannot access workspace data while it is in maintenance"
            )
        elif rep["status"] != "ok":
            raise FSError(f"Cannot fetch user manifest from backend: {rep}")

        expected_author = rep["author"]
        expected_timestamp = rep["timestamp"]
        expected_version = rep["version"]
        blob = rep["blob"]

        author = await self.remote_loader.get_device(expected_author)

        try:
            manifest = UserManifest.decrypt_verify_and_load(
                blob,
                key=self.device.user_manifest_key,
                author_verify_key=author.verify_key,
                expected_id=self.device.user_manifest_id,
                expected_author=expected_author,
                expected_timestamp=expected_timestamp,
                expected_version=version if version is not None else expected_version,
            )

        except DataError as exc:
            raise FSError(f"Invalid user manifest: {exc}") from exc

        return manifest

    async def sync(self) -> None:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNotFoundError
        """
        user_manifest = self.get_user_manifest()
        if user_manifest.need_sync:
            await self._outbound_sync()
        else:
            await self._inbound_sync()

    async def _inbound_sync(self) -> None:
        # Retrieve remote
        target_um = await self._fetch_remote_user_manifest()
        diverged_um = self.get_user_manifest()
        if target_um.version == diverged_um.base_version:
            # Nothing new
            return

        # New things in remote, merge is needed
        async with self._update_user_manifest_lock:
            diverged_um = self.get_user_manifest()
            if target_um.version <= diverged_um.base_version:
                # Sync already achieved by a concurrent operation
                return
            merged_um = merge_local_user_manifests(diverged_um, target_um)
            await self.set_user_manifest(merged_um)
            # In case we weren't online when the sharing message arrived,
            # we will learn about the change in the sharing only now.
            # Hence send the corresponding events !
            self._detect_and_send_shared_events(diverged_um, merged_um)
            # TODO: deprecated event ?
            self.event_bus.send(
                CoreEvent.FS_ENTRY_REMOTE_CHANGED, path="/", id=self.user_manifest_id
            )
            return

    def _detect_and_send_shared_events(
        self, old_um: LocalUserManifest, new_um: LocalUserManifest
    ) -> None:
        entries = {}
        for old_entry in old_um.workspaces:
            entries[old_entry.id] = [old_entry, None]
        for new_entry in new_um.workspaces:
            try:
                entries[new_entry.id][1] = new_entry
            except KeyError:
                entries[new_entry.id] = [None, new_entry]

        for old_entry, new_entry in entries.values():
            if new_entry is None:
                logger.warning(
                    "Workspace entry has diseaper from user manifest", workspace_entry=old_entry
                )
            elif old_entry is None:
                if new_entry.role is not None:
                    # New sharing
                    self.event_bus.send(
                        CoreEvent.SHARING_UPDATED, new_entry=new_entry, previous_entry=None
                    )
            else:
                # Sharing role has changed
                # Note it's possible to have `old_entry.role == new_entry.role`
                # (e.g. if our role went A -> B then B -> A while we were offline)
                # We should notify this anyway given it means some events could not have
                # been delivered to us (typically if we got removed from a workspace for
                # a short period of time while a `realm.vlobs_updated` event occured).
                self.event_bus.send(
                    CoreEvent.SHARING_UPDATED, new_entry=new_entry, previous_entry=old_entry
                )

    async def _outbound_sync(self) -> None:
        while True:
            if await self._outbound_sync_inner():
                return
            else:
                # Concurrency error, fetch and merge remote changes before
                # retrying the sync
                await self._inbound_sync()

    async def _outbound_sync_inner(self, timestamp_greater_than: Optional[DateTime] = None) -> bool:
        base_um = self.get_user_manifest()
        if not base_um.need_sync:
            return True

        # Make sure the corresponding realm has been created in the backend
        if base_um.is_placeholder:
            certif = RealmRoleCertificateContent.build_realm_root_certif(
                author=self.device.device_id,
                timestamp=self.device.timestamp(),
                realm_id=RealmID(self.device.user_manifest_id.uuid),
            ).dump_and_sign(self.device.signing_key)

            try:
                rep = await self.backend_cmds.realm_create(certif)

            except BackendNotAvailable as exc:
                raise FSBackendOfflineError(str(exc)) from exc

            except BackendConnectionError as exc:
                raise FSError(f"Cannot create user manifest's realm in backend: {exc}") from exc

            if rep["status"] == "already_exists":
                # It's possible a previous attempt to create this realm
                # succeeded but we didn't receive the confirmation, hence
                # we play idempotent here.
                pass
            elif rep["status"] != "ok":
                raise FSError(f"Cannot create user manifest's realm in backend: {rep}")

        # Sync placeholders
        for w in base_um.workspaces:
            await self._workspace_minimal_sync(w)

        # Build vlob
        timestamp = self.device.timestamp()
        if timestamp_greater_than is not None:
            timestamp = max(
                timestamp, timestamp_greater_than.add(microseconds=MANIFEST_STAMP_AHEAD_US)
            )
        to_sync_um = base_um.to_remote(author=self.device.device_id, timestamp=timestamp)
        ciphered = to_sync_um.dump_sign_and_encrypt(
            author_signkey=self.device.signing_key, key=self.device.user_manifest_key
        )

        # Sync the vlob with backend
        try:
            # Note encryption_revision is always 1 given we never reencrypt
            # the user manifest's realm
            if to_sync_um.version == 1:
                rep = await self.backend_cmds.vlob_create(
                    RealmID(self.user_manifest_id.uuid),
                    1,
                    VlobID(self.user_manifest_id.uuid),
                    timestamp,
                    ciphered,
                )
            else:
                rep = await self.backend_cmds.vlob_update(
                    1, VlobID(self.user_manifest_id.uuid), to_sync_um.version, timestamp, ciphered
                )

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Cannot sync user manifest: {exc}") from exc

        if rep["status"] in ("already_exists", "bad_version"):
            # Concurrency error (handled by the caller)
            return False
        elif rep["status"] == "in_maintenance":
            raise FSWorkspaceInMaintenance(
                f"Cannot modify workspace data while it is in maintenance: {rep}"
            )
        elif rep["status"] == "require_greater_timestamp":
            return await self._outbound_sync_inner(rep["strictly_greater_than"])
        elif rep["status"] != "ok":
            raise FSError(f"Cannot sync user manifest: {rep}")

        # Merge back the manifest in local
        async with self._update_user_manifest_lock:
            diverged_um = self.get_user_manifest()
            # Final merge could have been achieved by a concurrent operation
            if to_sync_um.version > diverged_um.base_version:
                merged_um = merge_local_user_manifests(diverged_um, to_sync_um)
                await self.set_user_manifest(merged_um)
            self.event_bus.send(CoreEvent.FS_ENTRY_SYNCED, id=self.user_manifest_id)

        return True

    async def _workspace_minimal_sync(self, workspace_entry: WorkspaceEntry) -> None:
        """
        Ensure the wokspace is usable from outside the local device:
        - realm is created on the server
        - initial version of the workspace manifest has been uploaded to the server

        In theroy, only realm creation is stricly needed for minimal_sync of the
        workspace: given each device starts using the workspace by creating a
        speculative placeholder workspace manifest, any of them could sync
        it placeholder which would become the initial workspace manifest.
        However we keep the workspace manifest upload as part of the minimal
        sync for compatibility reason (Parsec <= 2.4.2 download the workspace
        manifest instead of starting with a speculative placeholder).

        Raises:
            FSError
            FSBackendOfflineError
        """
        workspace = self.get_workspace(workspace_entry.id)
        try:
            await workspace.minimal_sync(workspace_entry.id)
        except FSWorkspaceNoAccess:
            # Not having full access on the workspace is a proof is owned
            # by somebody else, hence minimal sync is not needed.
            pass

    async def workspace_share(
        self,
        workspace_id: EntryID,
        recipient: UserID,
        role: Optional[WorkspaceRole],
        timestamp_greater_than: Optional[DateTime] = None,
    ) -> None:
        """
        Raises:
            FSError
            FSWorkspaceNotFoundError
            FSBackendOfflineError
            FSSharingNotAllowedError
        """
        if self.device.user_id == recipient:
            raise FSError("Cannot share to oneself")

        user_manifest = self.get_user_manifest()
        workspace_entry = user_manifest.get_workspace_entry(workspace_id)
        if not workspace_entry:
            raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

        # Make sure the workspace is not a placeholder
        await self._workspace_minimal_sync(workspace_entry)

        # Retrieve the user
        recipient_user, revoked_recipient_user = await self.remote_loader.get_user(recipient)

        if revoked_recipient_user:
            raise FSError(f"User {recipient} revoked")

        # Note we don't bother to check workspace's access roles given they
        # could be outdated (and backend will do the check anyway)

        timestamp = self.device.timestamp()
        if timestamp_greater_than is not None:
            timestamp = max(
                timestamp, timestamp_greater_than.add(microseconds=ROLE_CERTIFICATE_STAMP_AHEAD_US)
            )

        # Build the sharing message
        try:
            if role is not None:
                recipient_message: Union[
                    SharingGrantedMessageContent, SharingRevokedMessageContent
                ] = SharingGrantedMessageContent(
                    author=self.device.device_id,
                    timestamp=timestamp,
                    name=workspace_entry.name,
                    id=workspace_entry.id,
                    encryption_revision=workspace_entry.encryption_revision,
                    encrypted_on=workspace_entry.encrypted_on,
                    key=workspace_entry.key,
                )

            else:
                recipient_message = SharingRevokedMessageContent(
                    author=self.device.device_id, timestamp=timestamp, id=workspace_entry.id
                )

            ciphered_recipient_message = recipient_message.dump_sign_and_encrypt_for(
                author_signkey=self.device.signing_key, recipient_pubkey=recipient_user.public_key
            )

        except DataError as exc:
            raise FSError(f"Cannot create sharing message for `{recipient}`: {exc}") from exc

        # Build role certificate
        role_certificate = RealmRoleCertificateContent(
            author=self.device.device_id,
            timestamp=timestamp,
            realm_id=RealmID(workspace_id.uuid),
            user_id=recipient,
            role=role,
        ).dump_and_sign(self.device.signing_key)

        # Actually send the command to the backend
        try:
            rep = await self.backend_cmds.realm_update_roles(
                role_certificate, ciphered_recipient_message
            )

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Error while trying to set vlob group roles in backend: {exc}") from exc

        if rep["status"] == "not_allowed":
            raise FSSharingNotAllowedError(
                f"Must be Owner or Manager on the workspace is mandatory to share it: {rep}"
            )
        elif rep["status"] == "require_greater_timestamp":
            return await self.workspace_share(
                workspace_id, recipient, role, rep["strictly_greater_than"]
            )
        elif rep["status"] == "in_maintenance":
            raise FSWorkspaceInMaintenance(
                f"Cannot share workspace while it is in maintenance: {rep}"
            )
        elif rep["status"] == "already_granted":
            # Stay idempotent
            return
        elif rep["status"] != "ok":
            raise FSError(f"Error while trying to set vlob group roles in backend: {rep}")

    async def process_last_messages(self) -> Sequence[Tuple[int, Exception]]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSSharingNotAllowedError
        """
        errors = []
        # Concurrent message processing is totally pointless
        async with self._process_messages_lock:
            user_manifest = self.get_user_manifest()
            initial_last_processed_message = user_manifest.last_processed_message
            try:
                rep = await self.backend_cmds.message_get(offset=initial_last_processed_message)

            except BackendNotAvailable as exc:
                raise FSBackendOfflineError(str(exc)) from exc

            except BackendConnectionError as exc:
                raise FSError(f"Cannot retrieve user messages: {exc}") from exc

            if rep["status"] != "ok":
                raise FSError(f"Cannot retrieve user messages: {rep}")

            new_last_processed_message = initial_last_processed_message
            for msg in rep["messages"]:
                try:
                    await self._process_message(msg["sender"], msg["timestamp"], msg["body"])
                    new_last_processed_message = msg["count"]

                except FSBackendOfflineError:
                    raise

                except FSError as exc:
                    logger.warning(
                        "Invalid message", reason=exc, sender=msg["sender"], count=msg["count"]
                    )
                    errors.append((msg["count"], exc))

            # Update message offset in user manifest
            async with self._update_user_manifest_lock:
                user_manifest = self.get_user_manifest()
                if user_manifest.last_processed_message < new_last_processed_message:
                    user_manifest = user_manifest.evolve_and_mark_updated(
                        last_processed_message=new_last_processed_message,
                        timestamp=self.device.timestamp(),
                    )
                    await self.set_user_manifest(user_manifest)
                    self.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=self.user_manifest_id)

        return errors

    async def _process_message(
        self, sender_id: DeviceID, expected_timestamp: DateTime, ciphered: bytes
    ) -> None:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSSharingNotAllowedError
        """
        # Retrieve the sender
        sender = await self.remote_loader.get_device(sender_id)

        # Decrypt&verify message
        try:
            msg = BaseMessageContent.decrypt_verify_and_load_for(
                ciphered,
                recipient_privkey=self.device.private_key,
                author_verify_key=sender.verify_key,
                expected_author=sender_id,
                expected_timestamp=expected_timestamp,
            )

        except DataError as exc:
            raise FSError(f"Cannot decrypt&validate message from `{sender_id}`: {exc}") from exc

        if isinstance(msg, (SharingGrantedMessageContent, SharingReencryptedMessageContent)):
            await self._process_message_sharing_granted(msg)

        elif isinstance(msg, SharingRevokedMessageContent):
            await self._process_message_sharing_revoked(msg)

        elif isinstance(msg, PingMessageContent):
            self.event_bus.send(CoreEvent.MESSAGE_PINGED, ping=msg.ping)

    async def _process_message_sharing_granted(
        self, msg: Union[SharingGrantedMessageContent, SharingReencryptedMessageContent]
    ) -> None:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSSharingNotAllowedError
        """
        # We cannot blindly trust the message sender ! Hence we first
        # interrogate the backend to make sure he is a workspace manager/owner.
        # Note this means we refuse to process messages from a former-manager,
        # even if the message was sent at a time the user was manager (in such
        # case the user can still ask for another manager to re-do the sharing
        # so it's no big deal).
        try:
            roles = await self.remote_loader.load_realm_current_roles(msg.id)

        except FSWorkspaceNoAccess:
            # Seems we lost the access roles anyway, nothing to do then
            return

        if roles.get(msg.author.user_id, None) not in (WorkspaceRole.OWNER, WorkspaceRole.MANAGER):
            raise FSSharingNotAllowedError(
                f"User {msg.author.user_id} cannot share workspace `{msg.id}`"
                " with us (requires owner or manager right)"
            )

        # Determine the access roles we have been given to
        self_role = roles.get(self.device.user_id)

        # Finally insert the new workspace entry into our user manifest
        timestamp = self.device.timestamp()
        workspace_entry = WorkspaceEntry(
            # Name are not required to be unique across workspaces, so no check to do here
            name=msg.name,
            id=msg.id,
            key=msg.key,
            encryption_revision=msg.encryption_revision,
            encrypted_on=msg.encrypted_on,
            role=self_role,
            role_cached_on=timestamp,
        )

        async with self._update_user_manifest_lock:
            user_manifest = self.get_user_manifest()

            # Check if we already know this workspace
            already_existing_entry = user_manifest.get_workspace_entry(msg.id)
            if already_existing_entry:
                # Merge with existing as target to keep possible workpace rename
                workspace_entry = merge_workspace_entry(
                    None, workspace_entry, already_existing_entry
                )

            timestamp = self.device.timestamp()
            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(
                timestamp, workspace_entry
            )
            await self.set_user_manifest(user_manifest)
            self.event_bus.send(CoreEvent.USERFS_UPDATED)

            if not already_existing_entry:
                # TODO: remove this event ?
                self.event_bus.send(CoreEvent.FS_ENTRY_SYNCED, id=workspace_entry.id)

            self.event_bus.send(
                CoreEvent.SHARING_UPDATED,
                new_entry=workspace_entry,
                previous_entry=already_existing_entry,
            )

    async def _process_message_sharing_revoked(self, msg: SharingRevokedMessageContent) -> None:
        """
        Raises:
            FSError
            FSBackendOfflineError
        """
        # Unlike when somebody grant us workspace access, here we should no
        # longer be able to access the workspace.
        # This also include workspace participants, hence we have no way
        # verifying the sender is manager/owner... But this is not really a trouble:
        # if we cannot access the workspace info, we have been revoked anyway !
        try:
            await self.remote_loader.load_realm_current_roles(msg.id)

        except FSWorkspaceNoAccess:
            # Exactly what we expected !
            pass

        else:
            # We still have access over the workspace, nothing to do then
            return

        async with self._update_user_manifest_lock:
            user_manifest = self.get_user_manifest()

            # Save the revocation information in the user manifest
            existing_workspace_entry = user_manifest.get_workspace_entry(msg.id)
            if not existing_workspace_entry:
                # No workspace entry, nothing to update then
                return

            timestamp = self.device.timestamp()
            workspace_entry = merge_workspace_entry(
                None,
                existing_workspace_entry,
                existing_workspace_entry.evolve(role=None, role_cached_on=timestamp),
            )
            if existing_workspace_entry == workspace_entry:
                # Cheap idempotent check
                return

            timestamp = self.device.timestamp()
            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(
                timestamp, workspace_entry
            )
            await self.set_user_manifest(user_manifest)
            self.event_bus.send(CoreEvent.USERFS_UPDATED)
            self.event_bus.send(
                CoreEvent.SHARING_UPDATED,
                new_entry=workspace_entry,
                previous_entry=existing_workspace_entry,
            )

    async def _retrieve_participants(self, workspace_id: EntryID) -> List[UserCertificateContent]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
        """
        # First retrieve workspace participants list
        roles = await self.remote_loader.load_realm_current_roles(workspace_id)

        # Then retrieve each participant user data
        users = []
        for user_id in roles.keys():
            user, revoked_user = await self.remote_loader.get_user(user_id)
            if not revoked_user:
                users.append(user)

        return users

    def _generate_reencryption_messages(
        self,
        new_workspace_entry: WorkspaceEntry,
        users: List[UserCertificateContent],
        timestamp: DateTime,
    ) -> Dict[UserID, bytes]:
        """
        Raises:
            FSError
        """
        msg = SharingReencryptedMessageContent(
            author=self.device.device_id,
            timestamp=timestamp,
            name=new_workspace_entry.name,
            id=new_workspace_entry.id,
            encryption_revision=new_workspace_entry.encryption_revision,
            encrypted_on=new_workspace_entry.encrypted_on,
            key=new_workspace_entry.key,
        )

        per_user_ciphered_msgs = {}
        for user in users:
            try:
                ciphered = msg.dump_sign_and_encrypt_for(
                    author_signkey=self.device.signing_key, recipient_pubkey=user.public_key
                )
                per_user_ciphered_msgs[user.user_id] = ciphered
            except DataError as exc:
                raise FSError(
                    f"Cannot create reencryption message for `{user.user_id}`: {exc}"
                ) from exc

        return per_user_ciphered_msgs

    async def _send_start_reencryption_cmd(
        self,
        workspace_id: EntryID,
        encryption_revision: int,
        timestamp: DateTime,
        per_user_ciphered_msgs: Dict[UserID, bytes],
    ) -> bool:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
            BackendCmdsParticipantsMismatchError
        """
        # Finally send command to the backend
        try:
            rep = await self.backend_cmds.realm_start_reencryption_maintenance(
                RealmID(workspace_id.uuid), encryption_revision, timestamp, per_user_ciphered_msgs
            )

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Cannot start maintenance on workspace {workspace_id}: {exc}") from exc

        if rep["status"] == "participants_mismatch":
            # Catched by caller
            return False
        elif rep["status"] == "in_maintenance":
            raise FSWorkspaceInMaintenance(
                f"Workspace {workspace_id} already in maintenance: {rep}"
            )
        elif rep["status"] == "not_allowed":
            raise FSWorkspaceNoAccess(
                f"Not allowed to start maintenance on workspace {workspace_id}: {rep}"
            )
        elif rep["status"] != "ok":
            raise FSError(f"Cannot start maintenance on workspace {workspace_id}: {rep}")
        return True

    async def workspace_start_reencryption(self, workspace_id: EntryID) -> ReencryptionJob:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
            FSWorkspaceNotFoundError
        """
        user_manifest = self.get_user_manifest()
        workspace_entry = user_manifest.get_workspace_entry(workspace_id)
        if not workspace_entry:
            raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

        timestamp = self.device.timestamp()
        new_workspace_entry = workspace_entry.evolve(
            encryption_revision=workspace_entry.encryption_revision + 1,
            encrypted_on=timestamp,
            key=SecretKey.generate(),
        )

        while True:
            # In order to provide the new key to each participant, we must
            # encrypt a message for each of them
            participants = await self._retrieve_participants(workspace_entry.id)
            reencryption_msgs = self._generate_reencryption_messages(
                new_workspace_entry, participants, timestamp
            )

            # Actually ask the backend to start the reencryption
            ok = await self._send_start_reencryption_cmd(
                workspace_entry.id,
                new_workspace_entry.encryption_revision,
                timestamp,
                reencryption_msgs,
            )
            if not ok:
                # Participant list has changed concurrently, retry
                continue

            else:
                break

        # Note we don't update the user manifest here, this will be done when
        # processing the `realm.updated` message from the backend

        return ReencryptionJob(self.backend_cmds, new_workspace_entry, workspace_entry)

    async def workspace_continue_reencryption(self, workspace_id: EntryID) -> ReencryptionJob:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
            FSWorkspaceNotFoundError
        """
        user_manifest = self.get_user_manifest()
        workspace_entry = user_manifest.get_workspace_entry(workspace_id)
        if not workspace_entry:
            raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

        # First make sure the workspace is under maintenance
        try:
            rep = await self.backend_cmds.realm_status(RealmID(workspace_entry.id.uuid))

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(
                f"Cannot continue maintenance on workspace {workspace_id}: {exc}"
            ) from exc

        if rep["status"] == "not_allowed":
            raise FSWorkspaceNoAccess(f"Not allowed to access workspace {workspace_id}: {rep}")
        elif rep["status"] != "ok":
            raise FSError(f"Error while getting status for workspace {workspace_id}: {rep}")

        if not rep["in_maintenance"] or rep["maintenance_type"] != MaintenanceType.REENCRYPTION:
            raise FSWorkspaceNotInMaintenance("Not in reencryption maintenance")
        if rep["encryption_revision"] != workspace_entry.encryption_revision:
            raise FSError("Bad encryption revision")

        previous_workspace_entry = await self._get_previous_workspace_entry(workspace_entry)
        if not previous_workspace_entry:
            raise FSError(
                f"Never had access to encryption revision {workspace_entry.encryption_revision - 1}"
            )
        return ReencryptionJob(self.backend_cmds, workspace_entry, previous_workspace_entry)

    async def _get_previous_workspace_entry(
        self, workspace_entry: WorkspaceEntry
    ) -> Optional[WorkspaceEntry]:
        """
        Return the most recent workspace entry using the previous encryption revision.

        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
        """
        # Must retrieve the previous encryption revision's key
        version_to_fetch = None
        current_encryption_revision = workspace_entry.encryption_revision
        while True:
            previous_user_manifest = await self._fetch_remote_user_manifest(
                version=version_to_fetch
            )
            previous_workspace_entry = previous_user_manifest.get_workspace_entry(
                workspace_entry.id
            )
            if not previous_workspace_entry:
                return None

            if previous_workspace_entry.encryption_revision == current_encryption_revision - 1:
                return previous_workspace_entry
            else:
                version_to_fetch = previous_user_manifest.version - 1
