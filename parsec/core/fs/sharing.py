from structlog import get_logger
from itertools import count

from parsec.schema import UnknownCheckedSchema, OneOfSchema, fields
from parsec.core.schemas import ManifestAccessSchema
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss
from parsec.core.fs.utils import is_placeholder_manifest, is_workspace_manifest
from parsec.core.fs.types import Path
from parsec.core.encryption_manager import EncryptionManagerError
from parsec.utils import to_jsonb64


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


class _BackendMessageGetRepMessagesSchema(UnknownCheckedSchema):
    count = fields.Int(required=True)
    body = fields.Base64Bytes(required=True)
    sender_id = fields.String(required=True)


BackendMessageGetRepMessagesSchema = _BackendMessageGetRepMessagesSchema()


class _BackendMessageGetRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    messages = fields.List(fields.Nested(BackendMessageGetRepMessagesSchema), required=True)


backend_message_get_rep_schema = _BackendMessageGetRepSchema()


class _BackendUserGetRepDeviceSchema(UnknownCheckedSchema):
    created_on = fields.DateTime(required=True)
    revocated_on = fields.DateTime(missing=None)
    verify_key = fields.Base64Bytes(required=True)


BackendUserGetRepDeviceSchema = _BackendUserGetRepDeviceSchema()


class _BackendUserGetRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    user_id = fields.String(required=True)
    created_on = fields.DateTime(required=True)
    broadcast_key = fields.Base64Bytes(required=True)
    devices = fields.Map(fields.String(), fields.Nested(BackendUserGetRepDeviceSchema), missing={})


backend_user_get_rep_schema = _BackendUserGetRepSchema()


class _SharingMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("share", required=True)
    author = fields.String(required=True)
    access = fields.Nested(ManifestAccessSchema, required=True)
    name = fields.String(required=True)


sharing_message_content_schema = _SharingMessageContentSchema()


class _PingMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("ping", required=True)
    ping = fields.String(required=True)


PingMessageContentSchema = _PingMessageContentSchema()


class _GenericMessageContentSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {"share": sharing_message_content_schema, "ping": PingMessageContentSchema}

    def get_obj_type(self, obj):
        return obj["type"]


generic_message_content_schema = _GenericMessageContentSchema()


class Sharing:
    def __init__(
        self,
        device,
        backend_cmds_sender,
        encryption_manager,
        local_folder_fs,
        syncer,
        remote_loader,
        event_bus,
    ):
        self.device = device
        self.backend_cmds_sender = backend_cmds_sender
        self.encryption_manager = encryption_manager
        self.local_folder_fs = local_folder_fs
        self.syncer = syncer
        self.remote_loader = remote_loader
        self.event_bus = event_bus

    async def share(self, path: Path, recipient: str):
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

        # First make sure there is no placeholder in the path and the entry
        # is up to date
        await self.syncer.sync(path)

        # Now retrieve the access/manifest to share. Note a concurrency
        # modification on local fs can make this call raise a FileNotFoundError
        access, manifest = self.local_folder_fs.get_entry(path)

        # Concurrency could also have changed what's pointed by path (e.g. the
        # entry we just synced has been removed and replaced by a placeholder)
        if is_placeholder_manifest(manifest):
            # Given the timing should be pretty close, we can just pretend
            # we arrived between the deletion and the placeholder creation
            raise FileNotFoundError(2, "No such file or directory", path)

        if not is_workspace_manifest(manifest):
            raise SharingNotAWorkspace(f"`{path}` is not a workspace, hence cannot be shared")

        # Build sharing message
        msg = {"type": "share", "author": self.device.id, "access": access, "name": path.name}
        raw, errors = sharing_message_content_schema.dumps(msg)
        if errors:
            # TODO: Do we really want to log the message content ? Wouldn't
            # it be better just to raise a RuntimeError given we should never
            # be in this case ?
            logger.error("Cannot dump sharing message", msg=msg, errors=errors)
            raise SharingError("Internal error")
        try:
            ciphered = await self.encryption_manager.encrypt_for(recipient, raw.encode("utf-8"))
        except EncryptionManagerError as exc:
            raise SharingRecipientError(f"Cannot create message for `{recipient}`") from exc

        # Send sharing message
        rep = await self.backend_cmds_sender.send(
            {"cmd": "message_new", "recipient": recipient, "body": to_jsonb64(ciphered)}
        )
        if rep.get("status") != "ok":
            raise SharingBackendMessageError(
                "Error while trying to send sharing message to backend: %r" % rep
            )

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
        initial_last_processed_message = user_manifest["last_processed_message"]
        rep = await self.backend_cmds_sender.send(
            {"cmd": "message_get", "offset": initial_last_processed_message}
        )
        rep, errors = backend_message_get_rep_schema.load(rep)
        if errors:
            raise SharingBackendMessageError(
                "Cannot retreive user messages: %r (errors: %r)" % (rep, errors)
            )

        new_last_processed_message = initial_last_processed_message
        for msg in rep["messages"]:
            try:
                await self._process_message(msg["sender_id"], msg["body"])
                new_last_processed_message = msg["count"]
            except SharingError as exc:
                logger.warning("Invalid sharing message", reason=exc)

        user_manifest_access, user_manifest = await self._get_user_manifest()
        if user_manifest["last_processed_message"] < new_last_processed_message:
            user_manifest["last_processed_message"] = new_last_processed_message
            self.local_folder_fs.update_manifest(user_manifest_access, user_manifest)

    async def _process_message(self, sender_id, ciphered):
        """
        Raises:
            SharingRecipientError
            SharingInvalidMessageError
        """
        expected_user_id, expected_device_name = sender_id.split("@")
        try:
            sender_user_id, sender_device_name, raw = await self.encryption_manager.decrypt_for_self(
                ciphered
            )
        except EncryptionManagerError as exc:
            raise SharingRecipientError(f"Cannot decrypt message from `{sender_id}`") from exc
        if sender_user_id != expected_user_id or sender_device_name != expected_device_name:
            raise SharingRecipientError(
                f"Message was said to be send by `{sender_id}`, "
                f" but is signed by {sender_user_id@sender_device_name}"
            )

        msg, errors = generic_message_content_schema.loads(raw.decode("utf-8"))
        if errors:
            raise SharingInvalidMessageError("Not a valid message: %r" % errors)

        if msg["type"] == "share":
            user_manifest_access, user_manifest = await self._get_user_manifest()

            for child_access in user_manifest["children"].values():
                if child_access == msg["access"]:
                    # Shared entry already present, nothing to do then
                    return

            for i in count(1):
                sharing_name = f"{msg['name']} (shared by {sender_user_id})"
                if i > 1:
                    sharing_name += f" {i}"
                if sharing_name not in user_manifest["children"]:
                    break
            user_manifest["children"][sharing_name] = msg["access"]
            self.local_folder_fs.update_manifest(user_manifest_access, user_manifest)

            path = f"/{sharing_name}"
            self.event_bus.send("sharing.new", path=path, access=msg["access"])
            self.event_bus.send("fs.entry.updated", id=user_manifest_access["id"])
            self.event_bus.send("fs.entry.synced", id=msg["access"]["id"], path=path)

        elif msg["type"] == "ping":
            self.event_bus.send("pinged")
