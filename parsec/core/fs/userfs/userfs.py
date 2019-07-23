# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from pendulum import Pendulum, now as pendulum_now
from typing import List, Tuple, Optional
from structlog import get_logger

from parsec.types import UserID, DeviceID
from parsec.event_bus import EventBus
from parsec.crypto import (
    build_realm_self_role_certificate,
    build_realm_role_certificate,
    encrypt_signed_msg_for,
    decrypt_and_verify_signed_msg_for,
    encrypt_signed_msg_with_secret_key,
    decrypt_and_verify_signed_msg_with_secret_key,
    encrypt_raw_with_secret_key,
    decrypt_raw_with_secret_key,
    CryptoError,
    SecretKey,
)
from parsec.serde import SerdeError
from parsec.api.protocol import MaintenanceType
from parsec.core.types import (
    EntryID,
    LocalDevice,
    LocalWorkspaceManifest,
    WorkspaceEntry,
    WorkspaceRole,
    UserManifest,
    LocalUserManifest,
    remote_manifest_serializer,
)
from parsec.core.local_storage import LocalStorage, LocalStorageMissingError
from parsec.core.backend_connection import (
    BackendCmdsPool,
    BackendNotAvailable,
    BackendCmdsBadResponse,
    BackendCmdsNotAllowed,
    BackendCmdsAlreadyExists,
    BackendCmdsRoleAlreadyGranted,
    BackendCmdsBadVersion,
    BackendCmdsInMaintenance,
    BackendCmdsParticipantsMismatchError,
    BackendConnectionError,
)
from parsec.core.remote_devices_manager import (
    RemoteDevicesManager,
    RemoteDevicesManagerError,
    RemoteDevicesManagerBackendOfflineError,
)

from parsec.core.fs.workspacefs import WorkspaceFS
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.fs.userfs.merging import merge_local_user_manifests, merge_workspace_entry
from parsec.core.fs.userfs.message import message_content_serializer
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


class ReencryptionJob:
    def __init__(self, backend_cmds, new_workspace_entry, old_workspace_entry):
        self.backend_cmds = backend_cmds
        self.new_workspace_entry = new_workspace_entry
        self.old_workspace_entry = old_workspace_entry
        assert new_workspace_entry.id == old_workspace_entry.id

    async def do_one_batch(self, size=100) -> Tuple[int, int]:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceInMaintenance
            FSWorkspaceNoAccess
        """
        workspace_id = self.new_workspace_entry.id
        new_encryption_revision = self.new_workspace_entry.encryption_revision

        # Get the batch
        try:
            batch = await self.backend_cmds.vlob_maintenance_get_reencryption_batch(
                workspace_id, new_encryption_revision, size
            )

            donebatch = []
            for vlob_id, version, blob in batch:
                cleartext = decrypt_raw_with_secret_key(self.old_workspace_entry.key, blob)
                newciphered = encrypt_raw_with_secret_key(self.new_workspace_entry.key, cleartext)
                donebatch.append((vlob_id, version, newciphered))

            total, done = await self.backend_cmds.vlob_maintenance_save_reencryption_batch(
                workspace_id, new_encryption_revision, donebatch
            )

            if total == done:
                # Finish the maintenance
                await self.backend_cmds.realm_finish_reencryption_maintenance(
                    workspace_id, new_encryption_revision
                )

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            if exc.status in ("not_in_maintenance", "bad_encryption_revision"):
                raise FSWorkspaceNotInMaintenance("Reencryption job already finished") from exc

            if exc.status == "not_allowed":
                raise FSWorkspaceNoAccess(
                    f"Not allowed to do reencryption maintenance on workspace {workspace_id}: {exc}"
                ) from exc

            raise FSError(
                f"Cannot do reencryption maintenance on workspace {workspace_id}: {exc}"
            ) from exc

        return total, done


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

        wentry = WorkspaceEntry(
            name="<user manifest>",
            id=device.user_manifest_id,
            key=device.user_manifest_key,
            encryption_revision=1,
        )
        self.remote_loader = RemoteLoader(
            self.device,
            self.device.user_manifest_id,
            lambda: wentry,
            self.backend_cmds,
            self.remote_devices_manager,
            self.local_storage,
        )

    @property
    def user_manifest_id(self) -> EntryID:
        return self.device.user_manifest_id

    async def get_user_manifest(self) -> LocalUserManifest:
        """
        Raises: Nothing !
        """
        try:
            return self.local_storage.get_manifest(self.user_manifest_id)

        except LocalStorageMissingError:
            # In the unlikely event the user manifest is not present in
            # local (e.g. device just created or during tests), we fall
            # back on an empty manifest which is a good aproximation of
            # the very first version of the manifest (field `created` is
            # invalid, but it will be corrected by the merge during sync).
            user_manifest = LocalUserManifest(author=self.device.device_id)
            async with self.local_storage.lock_entry_id(self.user_manifest_id):
                self.local_storage.set_manifest(self.user_manifest_id, user_manifest)
            return user_manifest

    async def get_workspace(self, workspace_id: EntryID) -> WorkspaceFS:
        """
        Raises:
            FSWorkspaceNotFoundError
        """
        # Workspace entry can change at any time, so we provide a way for
        # WorkpaceFS to load it each time it is needed
        async def _get_workspace_entry():
            user_manifest = await self.get_user_manifest()
            workspace_entry = user_manifest.get_workspace_entry(workspace_id)
            if not workspace_entry:
                raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")
            return workspace_entry

        # Sanity check to make sure workspace_id is valid
        await _get_workspace_entry()

        return WorkspaceFS(
            workspace_id=workspace_id,
            get_workspace_entry=_get_workspace_entry,
            device=self.device,
            local_storage=self.local_storage,
            backend_cmds=self.backend_cmds,
            event_bus=self.event_bus,
            remote_device_manager=self.remote_devices_manager,
        )

    async def workspace_create(self, name: str) -> EntryID:
        """
        Raises: Nothing !
        """
        workspace_entry = WorkspaceEntry(name)
        # TODO: At the moment, a workspace manifest is its own parent
        # Maybe a the data model should be updated to remove the parent_id
        # attribute for the workspace manifest classes
        workspace_manifest = LocalWorkspaceManifest.make_placeholder(
            author=self.device.device_id, parent_id=workspace_entry.id
        )
        async with self._update_user_manifest_lock:
            user_manifest = await self.get_user_manifest()
            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(workspace_entry)
            async with self.local_storage.lock_entry_id(workspace_entry.id):
                self.local_storage.set_manifest(workspace_entry.id, workspace_manifest)
            async with self.local_storage.lock_entry_id(self.user_manifest_id):
                self.local_storage.set_manifest(self.user_manifest_id, user_manifest)
            self.event_bus.send("fs.entry.updated", id=self.user_manifest_id)
            self.event_bus.send("fs.workspace.created", new_entry=workspace_entry)

        return workspace_entry.id

    async def workspace_rename(self, workspace_id: EntryID, new_name: str) -> None:
        """
        Raises:
            FSWorkspaceNotFoundError
        """
        async with self._update_user_manifest_lock:
            user_manifest = await self.get_user_manifest()
            workspace_entry = user_manifest.get_workspace_entry(workspace_id)
            if not workspace_entry:
                raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

            updated_workspace_entry = workspace_entry.evolve(name=new_name)
            updated_user_manifest = user_manifest.evolve_workspaces_and_mark_updated(
                updated_workspace_entry
            )
            async with self.local_storage.lock_entry_id(self.user_manifest_id):
                self.local_storage.set_manifest(self.user_manifest_id, updated_user_manifest)
            self.event_bus.send("fs.entry.updated", id=self.user_manifest_id)

    async def _fetch_remote_user_manifest(self, version: int = None) -> UserManifest:
        """
        Raises:
            FSError
            FSWorkspaceInMaintenance
            FSBackendOfflineError
        """
        try:
            # Note encryption_revision is always 1 given we never reencrypt
            # the user manifest's realm
            args = await self.backend_cmds.vlob_read(1, self.user_manifest_id, version)
            expected_author_id, expected_timestamp, expected_version, blob = args

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot access workspace data while it is in maintenance"
            ) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Cannot fetch user manifest from backend: {exc}") from exc

        if version and version != expected_version:
            raise FSError(
                f"User manifest version {version} was queried but"
                f" backend returned version {expected_version}"
            )

        try:
            author = await self.remote_devices_manager.get_device(expected_author_id)

        except RemoteDevicesManagerBackendOfflineError as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except RemoteDevicesManagerError as exc:
            raise FSError(f"Cannot retrieve author public key: {exc}") from exc

        try:
            raw = decrypt_and_verify_signed_msg_with_secret_key(
                self.device.user_manifest_key,
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
        user_manifest = await self.get_user_manifest()
        if user_manifest.need_sync:
            await self._outbound_sync()
        else:
            await self._inbound_sync()

    async def _inbound_sync(self) -> None:
        # Retrieve remote
        target_um = await self._fetch_remote_user_manifest()
        diverged_um = await self.get_user_manifest()
        if target_um.version == diverged_um.base_version:
            # Nothing new
            return

        # New things in remote, merge is needed
        target_um = target_um.to_local(self.device.device_id)

        base_um = None
        while True:
            if diverged_um.base_version != 0:
                # TODO: keep base manifest somewhere to avoid this query
                base_um = await self._fetch_remote_user_manifest(version=diverged_um.base_version)
                base_um = base_um.to_local(self.device.device_id)

            # Merge and store result
            async with self._update_user_manifest_lock:
                diverged_um = await self.get_user_manifest()
                if target_um.base_version <= diverged_um.base_version:
                    # Sync already achieved by a concurrent operation
                    return
                if base_um and diverged_um.base_version != base_um.base_version:
                    continue
                merged = merge_local_user_manifests(base_um, diverged_um, target_um)
                async with self.local_storage.lock_entry_id(self.user_manifest_id):
                    self.local_storage.set_manifest(self.user_manifest_id, merged)
                # TODO: deprecated event ?
                self.event_bus.send("fs.entry.remote_changed", path="/", id=self.user_manifest_id)
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
        base_um = await self.get_user_manifest()
        if not base_um.need_sync:
            return

        # Make sure the corresponding realm has been created in the backend
        if base_um.is_placeholder:
            certif = build_realm_self_role_certificate(
                self.device.device_id,
                self.device.signing_key,
                self.device.user_manifest_id,
                pendulum_now(),
            )

            try:
                await self.backend_cmds.realm_create(certif)

            except BackendCmdsBadResponse as exc:
                if exc.status == "already_exists":
                    # It's possible a previous attempt to create this realm
                    # succeeded but we didn't receive the confirmation, hence
                    # we play idempotent here.
                    pass
                else:
                    raise FSError(f"Cannot create user manifest's realm in backend: {exc}") from exc

            except BackendNotAvailable as exc:
                raise FSBackendOfflineError(str(exc)) from exc

            except BackendConnectionError as exc:
                raise FSError(f"Cannot create user manifest's realm in backend: {exc}") from exc

        # Sync placeholders
        for w in base_um.workspaces:
            await self._workspace_minimal_sync(w)

        # Build vlob
        to_sync_um = base_um.evolve(
            workspaces=base_um.workspaces,
            base_version=base_um.base_version + 1,
            author=self.device.device_id,
            need_sync=False,
            is_placeholder=False,
        )
        now = pendulum_now()
        ciphered = encrypt_signed_msg_with_secret_key(
            self.device.device_id,
            self.device.signing_key,
            self.device.user_manifest_key,
            remote_manifest_serializer.dumps(to_sync_um.to_remote()),
            now,
        )

        # Sync the vlob with backend
        try:
            # Note encryption_revision is always 1 given we never reencrypt
            # the user manifest's realm
            if to_sync_um.base_version == 1:
                await self.backend_cmds.vlob_create(
                    self.user_manifest_id, 1, self.user_manifest_id, now, ciphered
                )
            else:
                await self.backend_cmds.vlob_update(
                    1, self.user_manifest_id, to_sync_um.base_version, now, ciphered
                )

        except (BackendCmdsAlreadyExists, BackendCmdsBadVersion):
            # Concurrency error (handled by the caller)
            raise

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot modify workspace data while it is in maintenance"
            ) from exc

        except BackendConnectionError as exc:
            raise FSError(f"Cannot sync user manifest: {exc}") from exc

        # Merge back the manifest in local
        async with self._update_user_manifest_lock:
            diverged_um = await self.get_user_manifest()
            # Final merge could have been achieved by a concurrent operation
            if to_sync_um.base_version > diverged_um.base_version:
                merged_um = merge_local_user_manifests(base_um, diverged_um, to_sync_um)
                async with self.local_storage.lock_entry_id(self.user_manifest_id):
                    self.local_storage.set_manifest(self.user_manifest_id, merged_um)
            self.event_bus.send("fs.entry.synced", path="/", id=self.user_manifest_id)

    async def _workspace_minimal_sync(self, workspace_entry: WorkspaceEntry):
        """
        Raises:
            FSError
            FSBackendOfflineError
        """
        workspace = await self.get_workspace(workspace_entry.id)
        await workspace.minimal_sync(workspace_entry.id)

    async def workspace_share(
        self, workspace_id: EntryID, recipient: UserID, role: Optional[WorkspaceRole]
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

        user_manifest = await self.get_user_manifest()
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
        role_certificate = build_realm_role_certificate(
            certifier_id=self.device.device_id,
            certifier_key=self.device.signing_key,
            realm_id=workspace_id,
            user_id=recipient,
            role=role,
            timestamp=pendulum_now(),
        )

        try:
            await self.backend_cmds.realm_update_roles(role_certificate)

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendCmdsNotAllowed as exc:
            raise FSSharingNotAllowedError(
                "Must be Owner or Manager on the workspace is mandatory to share it"
            ) from exc

        except BackendCmdsInMaintenance as exc:
            raise FSWorkspaceInMaintenance(
                f"Cannot share workspace while it is in maintenance"
            ) from exc

        except BackendCmdsRoleAlreadyGranted:
            # Stay idempotent
            return

        except BackendConnectionError as exc:
            raise FSError(f"Error while trying to set vlob group roles in backend: {exc}") from exc

        # Step 2)

        # Build the sharing message
        msg = {"id": workspace_entry.id, "name": workspace_entry.name}
        if role:
            msg["type"] = "sharing.granted"
            msg["key"] = workspace_entry.key
            msg["encryption_revision"] = workspace_entry.encryption_revision
            msg["encrypted_on"] = workspace_entry.encrypted_on
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
            user_manifest = await self.get_user_manifest()
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
                user_manifest = await self.get_user_manifest()
                if user_manifest.last_processed_message < new_last_processed_message:
                    user_manifest = user_manifest.evolve_and_mark_updated(
                        last_processed_message=new_last_processed_message
                    )
                    async with self.local_storage.lock_entry_id(self.user_manifest_id):
                        self.local_storage.set_manifest(self.user_manifest_id, user_manifest)
                    self.event_bus.send("fs.entry.updated", id=self.user_manifest_id)

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

        if msg["type"] in ("sharing.granted", "sharing.reencrypted"):
            await self._process_message_sharing_granted(
                sender,
                msg["id"],
                msg["key"],
                msg["encryption_revision"],
                msg["encrypted_on"],
                msg["name"],
            )

        elif msg["type"] == "sharing.revoked":
            await self._process_message_sharing_revoked(sender, msg["id"])

        else:
            assert msg["type"] == "ping"
            self.event_bus.send("pinged")

    async def _process_message_sharing_granted(
        self,
        sender,
        workspace_id,
        workspace_key,
        workspace_encryption_revision,
        workspace_encrypted_on,
        workspace_name,
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
            roles = await self.remote_loader.load_realm_current_roles(workspace_id)

        except FSWorkspaceNoAccess:
            # Seems we lost the access roles anyway, nothing to do then
            return

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
            id=workspace_id,
            key=workspace_key,
            encryption_revision=workspace_encryption_revision,
            encrypted_on=workspace_encrypted_on,
            role=self_role,
            role_cached_on=pendulum_now(),
        )

        async with self._update_user_manifest_lock:
            user_manifest = await self.get_user_manifest()

            # Check if we already know this workspace
            already_existing_entry = user_manifest.get_workspace_entry(workspace_id)
            if already_existing_entry:
                # Merge with existing as target to keep possible workpace rename
                workspace_entry = merge_workspace_entry(
                    None, workspace_entry, already_existing_entry
                )

            if already_existing_entry == workspace_entry:
                # Cheap idempotent check
                return

            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(workspace_entry)
            async with self.local_storage.lock_entry_id(self.user_manifest_id):
                self.local_storage.set_manifest(self.user_manifest_id, user_manifest)
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
                    "fs.entry.synced", id=workspace_entry.id, path=f"/{workspace_name}"
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
            await self.remote_loader.load_realm_current_roles(workspace_id)

        except FSWorkspaceNoAccess:
            # Exactly what we expected !
            pass

        else:
            # We still have access over the workspace, nothing to do then
            return

        async with self._update_user_manifest_lock:
            user_manifest = await self.get_user_manifest()

            # Save the revocation information in the user manifest
            existing_workspace_entry = user_manifest.get_workspace_entry(workspace_id)
            if not existing_workspace_entry:
                # No workspace entry, nothing to update then
                return

            workspace_entry = merge_workspace_entry(
                None,
                existing_workspace_entry,
                existing_workspace_entry.evolve(role=None, role_cached_on=pendulum_now()),
            )
            if existing_workspace_entry == workspace_entry:
                # Cheap idempotent check
                return

            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(workspace_entry)
            async with self.local_storage.lock_entry_id(self.user_manifest_id):
                self.local_storage.set_manifest(self.user_manifest_id, user_manifest)
            self.event_bus.send("userfs.updated")
            self.event_bus.send(
                "sharing.revoked",
                new_entry=workspace_entry,
                previous_entry=existing_workspace_entry,
            )

    async def _retreive_participants(self, workspace_id):
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
        """
        # First retrieve workspace participants list
        roles = await self.remote_loader.load_realm_current_roles(workspace_id)

        # Then retrieve each participant user data
        try:
            users = []
            for user_id in roles.keys():
                user = await self.remote_devices_manager.get_user(user_id)
                users.append(user)

        except RemoteDevicesManagerBackendOfflineError as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except RemoteDevicesManagerError as exc:
            raise FSError(f"Cannot retrieve workspace {workspace_id} participants: {exc}") from exc

        return users

    def _generate_reencryption_messages(self, new_workspace_entry, users):
        """
        Raises:
            FSError
        """
        # Generate&sign reencryption message
        msg = {
            "type": "sharing.reencrypted",
            "id": new_workspace_entry.id,
            "key": new_workspace_entry.key,
            "encryption_revision": new_workspace_entry.encryption_revision,
            "encrypted_on": new_workspace_entry.encrypted_on,
            "name": new_workspace_entry.name,
        }
        # Should never raise error given we control the inputs
        raw_msg = message_content_serializer.dumps(msg)

        # Encrypt message for each user
        per_user_ciphered_msgs = {}
        for user in users:
            try:
                ciphered = encrypt_signed_msg_for(
                    self.device.device_id,
                    self.device.signing_key,
                    user.public_key,
                    raw_msg,
                    new_workspace_entry.encrypted_on,
                )
                per_user_ciphered_msgs[user.user_id] = ciphered

            except CryptoError as exc:
                raise FSError(
                    f"Cannot create reencryption message for `{user.user_id}`: {exc}"
                ) from exc

        return per_user_ciphered_msgs

    async def _send_start_reencryption_cmd(
        self, workspace_id, encryption_revision, timestamp, per_user_ciphered_msgs
    ):
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
            BackendCmdsParticipantsMismatchError
        """
        # Finally send command to the backend
        try:
            await self.backend_cmds.realm_start_reencryption_maintenance(
                workspace_id, encryption_revision, timestamp, per_user_ciphered_msgs
            )

        except BackendCmdsParticipantsMismatchError:
            # Catched by caller
            raise

        except BackendNotAvailable as exc:
            raise FSBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            if exc.status == "in_maintenance":
                raise FSWorkspaceInMaintenance(f"Workspace {workspace_id} already in maintenance")

            if exc.status == "not_allowed":
                raise FSWorkspaceNoAccess(
                    f"Not allowed to start maintenance on workspace {workspace_id}: {exc}"
                ) from exc

            raise FSError(f"Cannot start maintenance on workspace {workspace_id}: {exc}") from exc

    async def workspace_start_reencryption(self, workspace_id: EntryID) -> ReencryptionJob:
        """
        Raises:
            FSError
            FSBackendOfflineError
            FSWorkspaceNoAccess
            FSWorkspaceNotFoundError
        """
        user_manifest = await self.get_user_manifest()
        workspace_entry = user_manifest.get_workspace_entry(workspace_id)
        if not workspace_entry:
            raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

        now = pendulum_now()
        new_workspace_entry = workspace_entry.evolve(
            encryption_revision=workspace_entry.encryption_revision + 1,
            encrypted_on=now,
            key=SecretKey.generate(),
        )

        while True:
            # In order to provide the new key to each participant, we must
            # encrypt a message for each of them
            participants = await self._retreive_participants(workspace_entry.id)
            reencryption_msgs = self._generate_reencryption_messages(
                new_workspace_entry, participants
            )

            # Actually ask the backend to start the reencryption
            try:
                await self._send_start_reencryption_cmd(
                    workspace_entry.id,
                    new_workspace_entry.encryption_revision,
                    now,
                    reencryption_msgs,
                )

            except BackendCmdsParticipantsMismatchError:
                # Participant list has changed concurrently
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
        user_manifest = await self.get_user_manifest()
        workspace_entry = user_manifest.get_workspace_entry(workspace_id)
        if not workspace_entry:
            raise FSWorkspaceNotFoundError(f"Unknown workspace `{workspace_id}`")

        # First make sure the workspace is under maintenance
        status = await self.backend_cmds.realm_status(workspace_entry.id)
        if (
            not status["in_maintenance"]
            or status["maintenance_type"] != MaintenanceType.REENCRYPTION
        ):
            raise FSWorkspaceNotInMaintenance("Not in reencryption maintenance")
        current_encryption_revision = status["encryption_revision"]
        if status["encryption_revision"] != workspace_entry.encryption_revision:
            raise FSError("Bad encryption revision")

        # Must retreive the previous encryption revision's key
        version_to_fetch = None
        while True:
            previous_user_manifest = await self._fetch_remote_user_manifest(
                version=version_to_fetch
            )
            previous_workspace_entry = previous_user_manifest.get_workspace_entry(
                workspace_entry.id
            )
            if not previous_workspace_entry:
                raise FSError(
                    f"Never had access to encryption revision {current_encryption_revision - 1}"
                )

            if previous_workspace_entry.encryption_revision == current_encryption_revision - 1:
                break
            else:
                version_to_fetch = previous_workspace_entry.version - 1

        return ReencryptionJob(self.backend_cmds, workspace_entry, previous_workspace_entry)
