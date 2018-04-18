import trio
import logbook
import traceback
from nacl.public import SealedBox
from nacl.signing import VerifyKey

from parsec.utils import from_jsonb64, ejson_loads
from parsec.core.fs import FSInvalidPath
from parsec.core.backend_connection import BackendNotAvailable, BackendError


logger = logbook.Logger("parsec.core.sharing")


class Sharing:

    def __init__(self, device, signal_ns, fs, backend_connection):
        self.signal_ns = signal_ns
        self.fs = fs
        self.backend_connection = backend_connection
        self.device = device
        self.msg_arrived = trio.Event()

    async def _message_listener_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        with trio.open_cancel_scope() as cancel_scope:
            task_status.started(cancel_scope)
            while True:
                try:
                    await self.msg_arrived.wait()
                    self.msg_arrived.clear()
                    # TODO: should keep a message counter in the user manifest
                    # too know which message should be processed here
                    rep = await self.backend_connection.send({"cmd": "message_get"})
                    assert rep["status"] == "ok"
                    msg = rep["messages"][-1]

                    sender_user_id, sender_device_name = msg["sender_id"].split("@")
                    rep = await self.backend_connection.send(
                        {"cmd": "user_get", "user_id": sender_user_id}
                    )
                    assert rep["status"] == "ok"
                    # TODO: handle crash, handle key validity expiration

                    sender_verifykey = VerifyKey(
                        from_jsonb64(rep["devices"][sender_device_name]["verify_key"])
                    )
                    box = SealedBox(self.device.user_privkey)

                    # TODO: handle bad signature, bad encryption, bad json, bad payload...
                    sharing_msg_encrypted = from_jsonb64(msg["body"])
                    sharing_msg_signed = box.decrypt(sharing_msg_encrypted)
                    sharing_msg_clear = sender_verifykey.verify(sharing_msg_signed)
                    sharing_msg = ejson_loads(sharing_msg_clear.decode("utf8"))

                    # TODO: handle other type of message
                    assert sharing_msg["type"] == "share"
                    sharing_access = self.fs._vlob_access_cls(
                        sharing_msg["content"]["id"],
                        sharing_msg["content"]["rts"],
                        sharing_msg["content"]["wts"],
                        from_jsonb64(sharing_msg["content"]["key"]),
                    )

                    shared_with_folder_name = "shared-with-%s" % sender_user_id
                    if shared_with_folder_name not in self.fs.root:
                        shared_with_folder = await self.fs.root.create_folder(
                            shared_with_folder_name
                        )
                    else:
                        shared_with_folder = await self.fs.root.fetch_child(shared_with_folder_name)
                    sharing_name = sharing_msg["name"]
                    while True:
                        try:
                            child = await shared_with_folder.insert_child_from_access(
                                sharing_name, sharing_access
                            )
                            break

                        except FSInvalidPath:
                            sharing_name += "-dup"

                    self.signal_ns.signal("new_sharing").send(child.path)
                except BackendNotAvailable:
                    pass
                except BackendError:
                    logger.warning("Error with backend: " % traceback.format_exc())

    def _msg_arrived_cb(self, sender):
        self.msg_arrived.set()

    async def init(self, nursery):
        self._message_listener_task_cancel_scope = await nursery.start(self._message_listener_task)
        await self.backend_connection.subscribe_event("message_arrived", self.device.user_id)
        self.signal_ns.signal("message_arrived").connect(self._msg_arrived_cb, weak=True)

    async def teardown(self):
        self._message_listener_task_cancel_scope.cancel()
