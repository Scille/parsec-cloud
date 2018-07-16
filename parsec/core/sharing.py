import trio
import logbook
import traceback
from nacl.public import SealedBox, PublicKey
from nacl.signing import VerifyKey

from parsec.signals import get_signal
from parsec.schema import UnknownCheckedSchema, OneOfSchema, fields
from parsec.core.schemas import LocalVlobAccessSchema
from parsec.core.base import BaseAsyncComponent
from parsec.core.fs import FSInvalidPath
from parsec.core.fs.utils import is_placeholder_access, normalize_path, is_file_manifest
from parsec.core.backend_connection import BackendNotAvailable
from parsec.utils import to_jsonb64


logger = logbook.Logger("parsec.core.sharing")


class SharingError(Exception):
    pass


class SharingBackendMessageError(SharingError):
    pass


class SharingInvalidRecipient(SharingError):
    pass


class SharingUnknownRecipient(SharingError):
    pass


class SharingInvalidMessageError(SharingError):
    pass


class BackendMessageGetRepMessagesSchema(UnknownCheckedSchema):
    count = fields.Int(required=True)
    body = fields.Base64Bytes(required=True)
    sender_id = fields.String(required=True)


class BackendMessageGetRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    messages = fields.List(fields.Nested(BackendMessageGetRepMessagesSchema), required=True)


backend_message_get_rep_schema = BackendMessageGetRepSchema()


class BackendUserGetRepDeviceSchema(UnknownCheckedSchema):
    created_on = fields.DateTime(required=True)
    revocated_on = fields.DateTime(missing=None)
    verify_key = fields.Base64Bytes(required=True)


class BackendUserGetRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    user_id = fields.String(required=True)
    created_on = fields.DateTime(required=True)
    created_by = fields.String(required=True)
    broadcast_key = fields.Base64Bytes(required=True)
    devices = fields.Map(fields.String(), fields.Nested(BackendUserGetRepDeviceSchema), missing={})


backend_user_get_rep_schema = BackendUserGetRepSchema()


class SharingMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("share", required=True)
    author = fields.String(required=True)
    access = fields.Nested(LocalVlobAccessSchema, required=True)
    name = fields.String(required=True)


sharing_message_content_schema = SharingMessageContentSchema()


class PingMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("ping", required=True)
    ping = fields.String(required=True)


class GenericMessageContentSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {"share": SharingMessageContentSchema, "ping": PingMessageContentSchema}

    def get_obj_type(self, obj):
        return obj["type"]


generic_message_content_schema = GenericMessageContentSchema()


class Sharing(BaseAsyncComponent):
    def __init__(self, device, fs, backend_connection, backend_event_manager):
        super().__init__()
        self.fs = fs
        self._backend_connection = backend_connection
        self._backend_event_manager = backend_event_manager
        self.device = device
        self.msg_arrived = trio.Event()
        self._message_listener_task_info = None

    async def _init(self, nursery):
        self._message_listener_task_info = await nursery.start(self._message_listener_task)
        await self._backend_event_manager.subscribe_backend_event(
            "message_arrived", self.device.user_id
        )
        get_signal("message_arrived").connect(self._msg_arrived_cb, weak=True)

    async def _teardown(self):
        cancel_scope, closed_event = self._message_listener_task_info
        cancel_scope.cancel()
        await closed_event.wait()

    async def _retrieve_device(self, user_id):
        rep = await self._backend_connection.send({"cmd": "user_get", "user_id": user_id})
        loaded_rep, errors = backend_user_get_rep_schema.load(rep)
        if errors:
            if rep.get("status") == "not_found":
                raise SharingUnknownRecipient(rep.get("reason", "Unknown user %r" % user_id))
            else:
                raise SharingBackendMessageError(
                    "Error while trying to retrieve user %r from the backend: %r" % (user_id, rep)
                )
        return loaded_rep

    async def _process_message(self, sender_id, body):
        sender_user_id, sender_device_name = sender_id.split("@")
        rep = await self._retrieve_device(sender_user_id)

        try:
            device = rep["devices"][sender_device_name]
        except KeyError:
            raise SharingBackendMessageError("Message sender %r device doesn't exists" % sender_id)

        # TODO: handle key validity expiration
        sender_verifykey = VerifyKey(device["verify_key"])
        box = SealedBox(self.device.user_privkey)

        # TODO: handle bad signature, bad encryption, bad json, bad payload...
        msg_encrypted = body
        msg_signed = box.decrypt(msg_encrypted)
        msg_clear = sender_verifykey.verify(msg_signed)

        msg, errors = generic_message_content_schema.loads(msg_clear.decode("utf-8"))
        if errors:
            raise SharingInvalidMessageError("Not a valid message: %r" % errors)

        if msg["type"] == "share":
            shared_with_folder_name = "shared-with-%s" % sender_user_id
            # TODO: leaky abstraction...
            parent_manifest = None
            parent_path = "/%s" % shared_with_folder_name
            while not parent_manifest:
                try:
                    parent_access, parent_manifest = await self.fs._local_tree.retrieve_entry(
                        parent_path
                    )
                    # If we are really unlucky, parent_path exists but is a file.
                    # In such a case we must create a new folder elsewhere.
                    if is_file_manifest(parent_manifest):
                        parent_manifest = None
                        parent_path += "-dup"
                        continue
                except FSInvalidPath:
                    await self.fs.folder_create(parent_path)

            sharing_name = msg["name"]
            while sharing_name in parent_manifest["children"]:
                sharing_name += "-dup"
            parent_manifest["children"][sharing_name] = msg["access"]
            self.fs._local_tree.update_entry(parent_access, parent_manifest)
            get_signal("new_sharing").send("%s/%s" % (parent_path, sharing_name))

        elif msg["type"] == "ping":
            get_signal("ping").send(msg["ping"])

    async def _process_all_last_messages(self):
        rep = await self._backend_connection.send(
            {"cmd": "message_get", "offset": self.fs.get_last_processed_message()}
        )

        loaded_rep, errors = backend_message_get_rep_schema.load(rep)
        if errors:
            raise SharingBackendMessageError(
                "Cannot retrieve user messages: %r (errors: %r)" % (rep, errors)
            )

        for msg in loaded_rep["messages"]:
            try:
                await self._process_message(msg["sender_id"], msg["body"])
                self.fs.update_last_processed_message(msg["count"])
            except SharingError as exc:
                logger.warning(exc.args[0])

    async def _message_listener_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        try:
            closed_event = trio.Event()
            with trio.open_cancel_scope() as cancel_scope:
                task_status.started((cancel_scope, closed_event))
                while True:
                    try:
                        await self.msg_arrived.wait()
                        self.msg_arrived.clear()
                        await self._process_all_last_messages()
                    except BackendNotAvailable:
                        pass
                    except SharingError:
                        logger.exception("Error with backend: " % traceback.format_exc())
        finally:
            closed_event.set()

    def _msg_arrived_cb(self, sender):
        self.msg_arrived.set()

    async def share(self, path, recipient):
        if self.device.user_id == recipient:
            raise SharingInvalidRecipient("Cannot share to oneself.")
        # TODO: leaky abstraction...
        # Retreive the entry and make sure it is not a placeholder
        _, entry_name = normalize_path(path)
        while True:
            access, _ = await self.fs._local_tree.retrieve_entry(path)
            # Cannot share a placeholder !
            if is_placeholder_access(access):
                await self.fs.sync(path)
            else:
                break

        sharing_msg = {
            "type": "share",
            "author": self.device.id,
            "access": access,
            "name": entry_name,
        }

        rep = await self._retrieve_device(recipient)

        # TODO Build the broadcast_key with the encryption manager
        broadcast_key = PublicKey(rep["broadcast_key"])
        box = SealedBox(broadcast_key)
        sharing_msg_clear, errors = sharing_message_content_schema.dumps(sharing_msg)
        if errors:
            raise RuntimeError("Cannot dump sharing message %r: %r" % (sharing_msg, errors))
        sharing_msg_signed = self.device.device_signkey.sign(sharing_msg_clear.encode("utf-8"))
        sharing_msg_encrypted = box.encrypt(sharing_msg_signed)

        rep = await self._backend_connection.send(
            {
                "cmd": "message_new",
                "recipient": recipient,
                "body": to_jsonb64(sharing_msg_encrypted),
            }
        )
        if rep["status"] != "ok":
            raise SharingError("Error while trying to send sharing message to backend: %r" % rep)
