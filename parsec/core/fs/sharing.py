# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from structlog import get_logger
from itertools import count

from parsec.types import UserID, DeviceID
from parsec.serde import Serializer, SerdeError, UnknownCheckedSchema, OneOfSchema, fields
from parsec.core.types import FsPath, WorkspaceEntry, ManifestAccess
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss
from parsec.core.fs.utils import is_workspace_manifest
from parsec.core.encryption_manager import EncryptionManagerError
from parsec.core.backend_connection import BackendCmdsBadResponse, BackendCmdsNotAllowed


logger = get_logger()


class SharingError(Exception):
    pass


class SharingBackendMessageError(SharingError):
    pass


class SharingRecipientError(SharingError):
    pass


class SharingNotAWorkspace(SharingError):
    pass


class SharingInvalidMessageError(SharingError):
    pass


class SharingNeedAdminRightError(SharingError):
    pass


class SharingMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("share", required=True)
    author = fields.String(required=True)
    name = fields.String(required=True)
    id = fields.UUID(required=True)
    read_right = fields.Boolean(required=True)
    write_right = fields.Boolean(required=True)
    admin_right = fields.Boolean(required=True)
    # Obviouly won't provide this if we send an end of sharing event !
    key = fields.SymetricKey(missing=None)


sharing_message_content_serializer = Serializer(SharingMessageContentSchema)


class PingMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("ping", required=True)
    ping = fields.String(required=True)


class GenericMessageContentSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {"share": SharingMessageContentSchema, "ping": PingMessageContentSchema}

    def get_obj_type(self, obj):
        return obj["type"]


generic_message_content_serializer = Serializer(GenericMessageContentSchema)


class Sharing:
    def __init__(
        self,
        device,
        backend_cmds,
        encryption_manager,
        local_folder_fs,
        syncer,
        remote_loader,
        event_bus,
    ):
        self.device = device
        self.backend_cmds = backend_cmds
        self.encryption_manager = encryption_manager
        self.local_folder_fs = local_folder_fs
        self.syncer = syncer
        self.remote_loader = remote_loader
        self.event_bus = event_bus

    async def share(
        self,
        path: FsPath,
        recipient: UserID,
        admin_right=False,
        read_right=False,
        write_right=False,
    ):
        """
        Raises:
            SharingError
            SharingBackendMessageError
            SharingRecipientError
            SharingNotAWorkspace
            FileNotFoundError
            FSManifestLocalMiss: If path is not available in local
            ValueError: If path is not a valid absolute path
        """
        if self.device.user_id == recipient:
            raise SharingRecipientError("Cannot share to oneself.")

        # First retreive the manifest and make sure it is a workspace
        access, manifest = self.local_folder_fs.get_entry(path)
        if not is_workspace_manifest(manifest):
            raise SharingNotAWorkspace(f"`{path}` is not a workspace, hence cannot be shared")

        # Note we don't bother to check `access.admin_right` given it could
        # be outdated (and backend will do the check anyway)

        # We should keep up to date the participants list in the manifest.
        # Note this is not done in a strictly atomic way so this information
        # can be erronous (consider it more of a UX helper than something to
        # rely on)
        if recipient not in manifest.participants:
            participants = sorted({*manifest.participants, recipient})
            manifest = manifest.evolve_and_mark_updated(participants=participants)
            self.local_folder_fs.set_dirty_manifest(access, manifest)

        # Make sure there is no placeholder in the path and the entry
        # is up to date
        await self.syncer.sync(path, recursive=False)

        # Actual sharing is done in two steps:
        # 1) update access rights for the vlob group corresponding to the workspace
        # 2) communicate to the new collaborator through a message the access to
        #    the workspace manifest
        # Those two steps are not atomics, but this is not that much a trouble
        # given they are idempotent

        # Step 1)
        try:
            await self.backend_cmds.vlob_group_update_rights(
                access.id,
                recipient,
                admin_right=admin_right,
                read_right=read_right,
                write_right=write_right,
            )

        except BackendCmdsNotAllowed as exc:
            raise SharingNeedAdminRightError(
                "Admin right on the workspace is mandatory to share it"
            ) from exc

        except BackendCmdsBadResponse as exc:
            raise SharingBackendMessageError(
                f"Error while trying to set vlob group rights in backend: {exc}"
            ) from exc

        # Step 2)

        # Build the sharing message...
        msg = {
            "type": "share",
            "author": self.device.device_id,
            "name": path.name,
            "id": access.id,
            "admin_right": admin_right,
            "read_right": read_right,
            "write_right": write_right,
        }
        if admin_right or read_right or write_right:
            msg["key"] = access.key

        try:
            raw = sharing_message_content_serializer.dumps(msg)

        except SerdeError as exc:
            # TODO: Do we really want to log the message content ? Wouldn't
            # it be better just to raise a RuntimeError given we should never
            # be in this case ?
            logger.error("Cannot dump sharing message", msg=msg, errors=exc.errors)
            raise SharingError("Internal error") from exc

        try:
            ciphered = await self.encryption_manager.encrypt_for(recipient, raw)

        except EncryptionManagerError as exc:
            raise SharingRecipientError(f"Cannot create message for `{recipient}`") from exc

        # ...And finally send the message
        try:
            await self.backend_cmds.message_send(recipient=recipient, body=ciphered)

        except BackendCmdsBadResponse as exc:
            raise SharingBackendMessageError(
                f"Error while trying to send sharing message to backend: {exc}"
            ) from exc

    # TODO: message handling should be in it own module, but given it is
    # only used for sharing so far...

    async def _get_user_manifest(self):
        user_manifest_access = self.device.user_manifest_access
        while True:
            try:
                user_manifest = self.local_folder_fs.get_manifest(user_manifest_access)
                return user_manifest_access, user_manifest

            except FSManifestLocalMiss:
                await self.remote_loader.load_manifest(user_manifest_access)

    async def process_last_messages(self):
        """
        Raises:
            SharingError
            BackendNotAvailable
            SharingBackendMessageError
        """

        _, user_manifest = await self._get_user_manifest()
        initial_last_processed_message = user_manifest.last_processed_message
        try:
            messages = await self.backend_cmds.message_get(offset=initial_last_processed_message)

        except BackendCmdsBadResponse as exc:
            raise SharingBackendMessageError(f"Cannot retreive user messages: {exc}") from exc

        new_last_processed_message = initial_last_processed_message
        for msg_count, msg_sender, msg_body in messages:
            try:
                await self._process_message(msg_sender, msg_body)
                new_last_processed_message = msg_count

            except SharingError as exc:
                logger.warning("Invalid sharing message", reason=exc)

        user_manifest_access, user_manifest = await self._get_user_manifest()
        if user_manifest.last_processed_message < new_last_processed_message:
            user_manifest = user_manifest.evolve_and_mark_updated(
                last_processed_message=new_last_processed_message
            )
            self.local_folder_fs.set_dirty_manifest(user_manifest_access, user_manifest)

    async def _process_message(self, sender_id: DeviceID, ciphered: bytes):
        """
        Raises:
            SharingRecipientError
            SharingInvalidMessageError
        """
        expected_user_id, expected_device_name = sender_id.split("@")
        try:
            real_sender_id, raw = await self.encryption_manager.decrypt_for_self(ciphered)

        except EncryptionManagerError as exc:
            raise SharingRecipientError(f"Cannot decrypt message from `{sender_id}`") from exc

        if real_sender_id != sender_id:
            raise SharingRecipientError(
                f"Message was said to be send by `{sender_id}`, "
                f" but is signed by {real_sender_id}"
            )

        try:
            msg = generic_message_content_serializer.loads(raw)

        except SerdeError as exc:
            raise SharingInvalidMessageError(f"Not a valid message: {exc.errors}") from exc

        # TODO: Currently we blindly trust the message sender, this pose multiple issues:
        # - We never check if the message was send by a user with admin right
        #   to this workspace
        # - We never check if the access rights exposed in this message was the right ones
        # - A malicious user can send us a end of sharing message (even if he
        #   doesn't have access to this workspace !)
        if msg["type"] == "share":
            user_manifest_access, user_manifest = await self._get_user_manifest()

            # Do we already know this workspace ?
            existing_workspace_entry = next(
                (we for we in user_manifest.workspaces if we.access.id == msg["id"]), None
            )

            key = msg.get("key")
            if not key:
                if existing_workspace_entry:
                    key = existing_workspace_entry.access.key
                else:
                    # We don't know this workspace, nothing we can do then
                    return

            # Now make sure it name is not clashing
            for i in count(1):
                if i == 1:
                    sharing_name = msg["name"]
                else:
                    sharing_name = f"{msg['name']} {i}"
                if all(
                    we.name != sharing_name or we.access.id == msg["id"]
                    for we in user_manifest.workspaces
                ):
                    break

            # All set ! We can update/insert the workspace entry
            new_workpsace_entry = WorkspaceEntry(
                name=sharing_name,
                access=ManifestAccess(id=msg["id"], key=key),
                admin_right=msg["admin_right"],
                read_right=msg["read_right"],
                write_right=msg["write_right"],
            )
            user_manifest = user_manifest.evolve_workspaces_and_mark_updated(new_workpsace_entry)
            self.local_folder_fs.set_dirty_manifest(user_manifest_access, user_manifest)

            # Finally send events
            path = f"/{sharing_name}"
            if not existing_workspace_entry:
                self.event_bus.send("sharing.new", path=path, access=new_workpsace_entry.access)
                self.event_bus.send("fs.entry.updated", id=user_manifest_access.id)
                self.event_bus.send(
                    "fs.entry.synced", id=new_workpsace_entry.access.id, path=str(path)
                )

            else:
                if (
                    new_workpsace_entry.admin_right
                    or new_workpsace_entry.read_right
                    or new_workpsace_entry.write_right
                ):
                    event_type = "sharing.updated"
                else:
                    event_type = "sharing.lost"
                self.event_bus.send(event_type, path=path, access=new_workpsace_entry.access)
                self.event_bus.send("fs.entry.updated", id=user_manifest_access.id)

        elif msg["type"] == "ping":
            self.event_bus.send("pinged")
