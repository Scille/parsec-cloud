# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from pendulum import Pendulum, now as pendulum_now
from typing import List, Tuple, Optional
from uuid import UUID
from structlog import get_logger

from parsec.types import UserID, DeviceID
from parsec.event_bus import EventBus
from parsec.crypto import (
    encrypt_signed_msg_for,
    decrypt_and_verify_signed_msg_for,
    encrypt_signed_msg_with_secret_key,
    decrypt_and_verify_signed_msg_with_secret_key,
    CryptoError,
)
from parsec.serde import SerdeError
from parsec.core.types import (
    LocalDevice,
    LocalWorkspaceManifest,
    WorkspaceManifest,
    ManifestAccess,
    WorkspaceEntry,
    WorkspaceRole,
    LocalUserManifest,
    remote_manifest_serializer,
)
from parsec.core.local_storage import LocalStorage, LocalStorageMissingEntry
from parsec.core.backend_connection import (
    BackendCmdsPool,
    BackendNotAvailable,
    BackendCmdsNotAllowed,
    BackendCmdsAlreadyExists,
    BackendCmdsBadVersion,
    BackendConnectionError,
)
from parsec.core.remote_devices_manager import (
    RemoteDevicesManager,
    RemoteDevicesManagerError,
    RemoteDevicesManagerBackendOfflineError,
)
from parsec.core.fs.utils import is_workspace_manifest
from parsec.core.fs.local_folder_fs import LocalFolderFS
from parsec.core.fs.syncer import Syncer
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.fs.workspacefs import WorkspaceFS
from parsec.core.fs.userfs.merging import merge_local_user_manifests
from parsec.core.fs.userfs.message import message_content_serializer
from parsec.core.fs.exceptions import (
    FSError,
    FSWorkspaceNotFoundError,
    FSBackendOfflineError,
    FSSharingNotAllowedError,
)


logger = get_logger()


class UserFS:
    def __init__(
        self,
        device: LocalDevice,
        local_storage: LocalStorage,
        backend_cmds: BackendCmdsPool,
        remote_devices_manager: RemoteDevicesManager,
        event_bus: EventBus,
    ):
        self.device = device
        self.local_storage = local_storage
        self.backend_cmds = backend_cmds
        self.remote_devices_manager = remote_devices_manager
        self.event_bus = event_bus
        # Message processing is done in-order, hence it is pointless to do
        # it concurrently
        self._process_messages_lock = trio.Lock()
        self._update_user_manifest_lock = trio.Lock()

        # TODO: move those attributes into WorkspaceFS (no need to share them anymore)
        self._remote_loader = RemoteLoader(backend_cmds, remote_devices_manager, local_storage)
        self._local_folder_fs = LocalFolderFS(device, local_storage, event_bus)
        self._syncer = Syncer(
            device,
            backend_cmds,
            remote_devices_manager,
            self._local_folder_fs,
            local_storage,
            event_bus,
        )

    @property
    def user_manifest_access(self) -> ManifestAccess:
        return self.device.user_manifest_access

    def get_user_manifest(self) -> LocalUserManifest:
        """
        Raises: Nothing !
        """
        try:
            return self.local_storage.get_manifest(self.user_manifest_access)

        except LocalStorageMissingEntry:
            # In the unlikely event the user manifest is not present in
            # local (e.g. device just created or during tests), we fall
            # back on an empty manifest which is a good aproximation of
            # the very first version of the manifest (field `created` is
            # invalid, but it will be corrected by the merge during sync).
            user_manifest = LocalUserManifest(author=self.device.device_id)
            self.local_storage.set_dirty_manifest(self.user_manifest_access, user_manifest)
            return user_manifest

    def get_workspace(self, workspace_id: UUID) -> WorkspaceFS:
        """
        Raises:
            FSWorkspaceNotFoundError
        """
        # Workspace entry can change at any time, so we provide a way for
        # WorskpaeFS to load it each time it is needed
        def _get_workspace_entry():
            user_manifest = self.get_user_manifest()
            workspace_entry = user_manifest.get_workspace_entry(workspace_id)
            if not workspace_entry:
                raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")
            return workspace_entry

        # Sanity check to make sure workspace_id is valid
        _get_workspace_entry()

        return WorkspaceFS(
            workspace_id,
            get_workspace_entry=_get_workspace_entry,
            device=self.device,
            local_storage=self.local_storage,
            backend_cmds=self.backend_cmds,
            event_bus=self.event_bus,
            _local_folder_fs=self._local_folder_fs,
            _remote_loader=self._remote_loader,
            _syncer=self._syncer,
        )

    async def workspace_create(self, name: str) -> UUID:
        """
        Raises: Nothing !
        """
        workspace_entry = WorkspaceEntry(name)
        workspace_manifest = LocalWorkspaceManifest(author=self.device.device_id)
        async with self._update_user_manifest_lock:
            user_manifest = self.get_user_manifest()
            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(workspace_entry)
            self.local_storage.set_dirty_manifest(workspace_entry.access, workspace_manifest)
            self.local_storage.set_dirty_manifest(self.user_manifest_access, user_manifest)
            self.event_bus.send("fs.entry.updated", id=self.user_manifest_access.id)
            self.event_bus.send("fs.workspace.created", new_entry=workspace_entry)

        return workspace_entry.access.id

    async def workspace_rename(self, workspace_id: UUID, new_name: str) -> None:
        """
        Raises:
            FSWorkspaceNotFoundError
        """
        async with self._update_user_manifest_lock:
            user_manifest = self.get_user_manifest()
            workspace_entry = user_manifest.get_workspace_entry(workspace_id)
            if not workspace_entry:
                raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

            updated_workspace_entry = workspace_entry.evolve(name=new_name)
            updated_user_manifest = user_manifest.evolve_workspaces_and_mark_updated(
                updated_workspace_entry
            )
            self.local_storage.set_dirty_manifest(self.user_manifest_access, updated_user_manifest)
            self.event_bus.send("fs.entry.updated", id=self.user_manifest_access.id)

    async def _fetch_remote_user_manifest(self, version=None):
        """
        Raises:
            FSError
            FSBackendOfflineError
        """
        try:
            args = await self.backend_cmds.vlob_read(self.user_manifest_access.id, version)
            expected_author_id, expected_timestamp, expected_version, blob = args

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Cannot fetch user manifest from backend: {exc}") from exc

        if version and version != expected_version:
            raise FSError(
                f"User manifest version {version} was queried but"
                f" backend returned version {expected_version}"
            )

        author = await self.remote_devices_manager.get_device(expected_author_id)
        try:
            raw = decrypt_and_verify_signed_msg_with_secret_key(
                self.user_manifest_access.key,
                blob,
                expected_author_id,
                author.verify_key,
                expected_timestamp,
            )
            manifest = remote_manifest_serializer.loads(raw)

        except (CryptoError, SerdeError) as exc:
            raise FSError(f"Invalid user manifest: {exc}") from exc

        if manifest.version != expected_version:
            raise FSError(
                "Invalid user manifest: version mismatch between backend"
                f" ({expected_version}) and signed data ({manifest.version})"
            )

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
            # Sync placeholders
            for w in user_manifest.workspaces:
                await self._workspace_minimal_sync(w)

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
        target_um = target_um.to_local()

        base_um = None
        while True:
            if diverged_um.base_version != 0:
                # TODO: keep base manifest somewhere to avoid this query
                base_um = await self._fetch_remote_user_manifest(version=diverged_um.base_version)
                base_um = base_um.to_local()

            # Merge and store result
            async with self._update_user_manifest_lock:
                diverged_um = self.get_user_manifest()
                if target_um.base_version <= diverged_um.base_version:
                    # Sync already achieved by a concurrent operation
                    return
                if base_um and diverged_um.base_version != base_um.base_version:
                    continue
                merged = merge_local_user_manifests(base_um, diverged_um, target_um)
                self.local_storage.set_dirty_manifest(self.user_manifest_access, merged)
                # TODO: deprecated event ?
                self.event_bus.send(
                    "fs.entry.remote_changed", path="/", id=self.user_manifest_access.id
                )
                return

    async def _outbound_sync(self) -> None:
        while True:
            try:
                await self._outbound_sync_inner()
                return

            except (BackendCmdsAlreadyExists, BackendCmdsBadVersion):
                # Concurrency error, fetch and merge remote changes before
                # retrying the sync
                await self._inbound_sync()
                continue

    async def _outbound_sync_inner(self):
        base_um = self.get_user_manifest()
        if not base_um.need_sync:
            return

        # Ignore placeholders (they have been created since the start of the sync)
        non_placeholder_entries = []
        for entry in base_um.workspaces:
            try:
                workspace_manifest = self.local_storage.get_manifest(entry.access)
                if workspace_manifest.is_placeholder:
                    continue
            except LocalStorageMissingEntry:
                # Workspace not in local means it is already synchronized
                pass
            non_placeholder_entries.append(entry)

        # Build vlob
        access = self.user_manifest_access
        to_sync_um = base_um.evolve(
            workspaces=tuple(non_placeholder_entries),
            base_version=base_um.base_version + 1,
            author=self.device.device_id,
            need_sync=False,
            is_placeholder=False,
        )
        now = pendulum_now()
        ciphered = encrypt_signed_msg_with_secret_key(
            self.device.device_id,
            self.device.signing_key,
            access.key,
            remote_manifest_serializer.dumps(to_sync_um.to_remote()),
            now,
        )

        # Sync the vlob with backend
        try:
            if to_sync_um.base_version == 1:
                await self.backend_cmds.vlob_create(access.id, access.id, now, ciphered)
            else:
                await self.backend_cmds.vlob_update(
                    access.id, to_sync_um.base_version, now, ciphered
                )

        except (BackendCmdsAlreadyExists, BackendCmdsBadVersion):
            # Concurrency error (handled by the caller)
            raise

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Cannot sync user manifest: {exc}") from exc

        # Merge back the manifest in local
        async with self._update_user_manifest_lock:
            diverged_um = self.get_user_manifest()
            merged_um = merge_local_user_manifests(base_um, diverged_um, to_sync_um)
            self.local_storage.set_dirty_manifest(self.user_manifest_access, merged_um)
            self.event_bus.send("fs.entry.synced", path="/", id=self.user_manifest_access.id)

    async def _workspace_minimal_sync(self, workspace_entry: WorkspaceEntry):
        """
        Raises:
            FSError
            FSBackendOfflineError
        """
        # TODO: do this in workspacefs ?
        access = workspace_entry.access
        try:
            workspace_manifest = self.local_storage.get_manifest(access)
        except LocalStorageMissingEntry:
            # Workspace not in local means it is already synchronized
            return
        assert is_workspace_manifest(workspace_manifest)
        if not workspace_manifest.is_placeholder:
            return

        # We don't care about the content of the workspace, just
        # synchronize an empty initial version
        remote_workspace_manifest = WorkspaceManifest(
            author=self.device.device_id,
            version=1,
            created=workspace_manifest.created,
            updated=workspace_manifest.created,
            children={},
        )
        now = pendulum_now()
        ciphered = encrypt_signed_msg_with_secret_key(
            self.device.device_id,
            self.device.signing_key,
            access.key,
            remote_manifest_serializer.dumps(remote_workspace_manifest),
            now,
        )
        try:
            await self.backend_cmds.vlob_create(access.id, access.id, now, ciphered)

        except BackendCmdsAlreadyExists:
            # Already synchronized, nothing to do then ;-)
            return

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise FSError(
                f"Cannot synchronize workspace `{access.id}` ({workspace_entry.name}): {exc}"
            ) from exc

        # Finally update local storage info
        # TODO: possible concurrency issue with workspacefs
        workspace_manifest = self.local_storage.get_manifest(access)
        if workspace_manifest.is_placeholder:
            self.local_storage.set_dirty_manifest(
                access, workspace_manifest.evolve(is_placeholder=False, base_version=1)
            )
        # TODO: still useful ?
        self.event_bus.send(
            "fs.entry.minimal_synced", path=f"/{workspace_entry.name}", id=access.id
        )

    async def workspace_share(
        self, workspace_id: UUID, recipient: UserID, role: Optional[WorkspaceRole]
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
        try:
            recipient_user = await self.remote_devices_manager.get_user(recipient)

        except RemoteDevicesManagerBackendOfflineError as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except RemoteDevicesManagerError as exc:
            raise FSError(f"Cannot retreive recipient: {exc}") from exc

        # Note we don't bother to check workspace's access roles given they
        # could be outdated (and backend will do the check anyway)

        # Actual sharing is done in two steps:
        # 1) update access roles for the vlob group corresponding to the workspace
        # 2) communicate to the new collaborator through a message the access to
        #    the workspace manifest
        # Those two steps are not atomic, but this is not that much of a trouble
        # given they are idempotent

        # Step 1)
        try:
            await self.backend_cmds.vlob_group_update_roles(
                workspace_entry.access.id, recipient, role=role
            )

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsNotAllowed as exc:
            raise FSSharingNotAllowedError(
                "Must be Owner or Manager on the workspace is mandatory to share it"
            ) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Error while trying to set vlob group roles in backend: {exc}") from exc

        # Step 2)

        # Build the sharing message
        msg = {"id": workspace_entry.access.id, "name": workspace_entry.name}
        if role:
            msg["type"] = "sharing.granted"
            msg["key"] = workspace_entry.access.key
        else:
            msg["type"] = "sharing.revoked"

        # Should never raise error given we control the inputs
        raw_msg = message_content_serializer.dumps(msg)

        now = pendulum_now()

        # Encrypt&sign message
        try:
            ciphered = encrypt_signed_msg_for(
                self.device.device_id,
                self.device.signing_key,
                recipient_user.public_key,
                raw_msg,
                now,
            )

        except CryptoError as exc:
            raise FSError(f"Cannot create sharing message for `{recipient}`: {exc}") from exc

        # And finally send the message
        try:
            await self.backend_cmds.message_send(recipient=recipient, timestamp=now, body=ciphered)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(*exc.arg) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Error while trying to send sharing message to backend: {exc}") from exc

        # # TODO: participant&creator fields are deprecated, this is just a hack
        # async with self._update_user_manifest_lock:
        #     workspace_manifest = self.local_storage.get_manifest(workspace_entry.access)
        #     workspace_manifest = workspace_manifest.evolve_and_mark_updated(
        #         participants=(*workspace_manifest.participants, recipient))
        #     self.local_storage.set_dirty_manifest(workspace_entry.access, workspace_manifest)
        #     self.event_bus.send("fs.entry.updated", id=workspace_entry.access.id)

    async def process_last_messages(self) -> List[Tuple[int, Exception]]:
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
                messages = await self.backend_cmds.message_get(
                    offset=initial_last_processed_message
                )

            except BackendNotAvailable as exc:
                raise FSBackendOfflineError(str(exc)) from exc

            except BackendConnectionError as exc:
                raise FSError(f"Cannot retrieve user messages: {exc}") from exc

            new_last_processed_message = initial_last_processed_message
            for msg_offset, msg_sender, msg_timestamp, msg_body in messages:
                try:
                    await self._process_message(msg_sender, msg_timestamp, msg_body)
                    new_last_processed_message = msg_offset

                except FSBackendOfflineError:
                    raise

                except FSError as exc:
                    logger.warning(
                        "Invalid message", reason=exc, sender=msg_sender, offset=msg_offset
                    )
                    errors.append((msg_offset, exc))

            # Update message offset in user manifest
            async with self._update_user_manifest_lock:
                user_manifest = self.get_user_manifest()
                if user_manifest.last_processed_message < new_last_processed_message:
                    user_manifest = user_manifest.evolve_and_mark_updated(
                        last_processed_message=new_last_processed_message
                    )
                    self.local_storage.set_dirty_manifest(self.user_manifest_access, user_manifest)
                    self.event_bus.send("fs.entry.updated", id=self.user_manifest_access.id)

        return errors

    async def _process_message(self, sender_id: DeviceID, timestamp: Pendulum, ciphered: bytes):
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSSharingNotAllowedError
        """
        # Retrieve the sender
        try:
            sender = await self.remote_devices_manager.get_device(sender_id)

        except RemoteDevicesManagerBackendOfflineError as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except RemoteDevicesManagerError as exc:
            raise FSError(f"Cannot retrieve message sender `{sender_id}`: {exc}") from exc

        # Decrypt&verify message
        try:
            raw = decrypt_and_verify_signed_msg_for(
                self.device.private_key, ciphered, sender_id, sender.verify_key, timestamp
            )

        except CryptoError as exc:
            raise FSError(f"Cannot decrypt&validate message from `{sender_id}`: {exc}") from exc

        msg = message_content_serializer.loads(raw)

        if msg["type"] == "sharing.granted":
            await self._process_message_sharing_granted(sender, msg["id"], msg["key"], msg["name"])

        elif msg["type"] == "sharing.revoked":
            await self._process_message_sharing_revoked(sender, msg["id"])

        else:
            assert msg["type"] == "ping"
            self.event_bus.send("pinged")

    async def _process_message_sharing_granted(
        self, sender, workspace_id, workspace_key, workspace_name
    ):
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
            roles = await self.backend_cmds.vlob_group_get_roles(workspace_id)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsNotAllowed:
            # Seems we lost the access roles anyway, nothing to do then
            return

        except BackendConnectionError as exc:
            raise FSError(f"Cannot retrieve workspace per-user roles: {exc}") from exc

        if roles.get(sender.user_id, None) not in (WorkspaceRole.OWNER, WorkspaceRole.MANAGER):
            raise FSSharingNotAllowedError(
                f"User {sender.user_id} cannot share workspace `{workspace_id}`"
                " with us (requires owner or manager right)"
            )

        # Determine the access roles we have been given to
        self_role = roles.get(self.device.user_id)

        # Finally insert the new workspace entry into our user manifest
        workspace_entry = WorkspaceEntry(
            # Name are not required to be unique across workspaces, so no check to do here
            name=f"{workspace_name} (shared by {sender.user_id})",
            access=ManifestAccess(id=workspace_id, key=workspace_key),
            role=self_role,
        )

        async with self._update_user_manifest_lock:
            user_manifest = self.get_user_manifest()

            # Check if we already know this workspace
            already_existing_entry = user_manifest.get_workspace_entry(workspace_id)
            if already_existing_entry:
                workspace_entry.evolve(name=already_existing_entry.name)
            if already_existing_entry == workspace_entry:
                # Cheap idempotent check
                return

            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(workspace_entry)
            self.local_storage.set_dirty_manifest(self.user_manifest_access, user_manifest)
            self.event_bus.send("userfs.updated")
            if already_existing_entry:
                self.event_bus.send(
                    "sharing.updated",
                    new_entry=workspace_entry,
                    previous_entry=already_existing_entry,
                )
            else:
                # TODO: remove this event ?
                self.event_bus.send(
                    "fs.entry.synced", id=workspace_entry.access.id, path=f"/{workspace_name}"
                )
                self.event_bus.send("sharing.granted", new_entry=workspace_entry)

    async def _process_message_sharing_revoked(self, sender, workspace_id):
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
            await self.backend_cmds.vlob_group_get_roles(workspace_id)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsNotAllowed:
            # Exactly what we expected !
            pass

        except BackendConnectionError as exc:
            raise FSError(f"Cannot retrieve workspace per-user roles: {exc}") from exc

        else:
            # We still have access over the workspace, nothing to do then
            return

        async with self._update_user_manifest_lock:
            user_manifest = self.get_user_manifest()

            # Save the revocation information in the user manifest
            existing_workspace_entry = user_manifest.get_workspace_entry(workspace_id)
            if not existing_workspace_entry:
                # No workspace entry, nothing to update then
                return
            workspace_entry = existing_workspace_entry.evolve(role=None, granted_on=pendulum_now())

            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(workspace_entry)
            self.local_storage.set_dirty_manifest(self.user_manifest_access, user_manifest)
            self.event_bus.send("userfs.updated")
            self.event_bus.send(
                "sharing.revoked",
                new_entry=workspace_entry,
                previous_entry=existing_workspace_entry,
            )
