import trio


async def monitor_messages(fs, event_bus):
    msg_arrived = trio.Event()
    backend_online_event = trio.Event()
    process_message_cancel_scope = None

    def _on_msg_arrived(event, index):
        msg_arrived.set()

    event_bus.connect("backend.message.received", _on_msg_arrived, weak=True)
    event_bus.connect("backend.message.polling_needed", _on_msg_arrived, weak=True)

    def _on_backend_online(self, event):
        backend_online_event.set()

    def _on_backend_offline(self, event):
        backend_online_event.clear()
        if process_message_cancel_scope:
            process_message_cancel_scope.cancel()

    event_bus.connect("backend.online", _on_backend_online, weak=True)
    event_bus.connect("backend.offline", _on_backend_offline, weak=True)

    while True:
        await backend_online_event.wait()
        try:

            with trio.open_cancel_scope() as process_message_cancel_scope:
                while True:
                    try:
                        await fs.process_messages()
                    except SharingError:
                        logger.exception("Invalid message from backend")
                    await msg_arrived.wait()
                    msg_arrived.clear()

        except BackendNotAvailable:
            pass
        process_message_cancel_scope = None


class MessageMonitor(BaseAsyncComponent):
    def __init__(
        self,
        device,
        backend_cmds_sender,
        encryption_manager,
        remote_loader,
        local_folder_fs,
        signal_ns,
    ):
        super().__init__()
        self.device = device
        self.backend_cmds_sender = backend_cmds_sender
        self.encryption_manager = encryption_manager
        self.remote_loader = remote_loader
        self.local_folder_fs = local_folder_fs
        self.signal_ns = signal_ns
        self._msg_arrived = trio.Event()
        self._task_info = None

        self.signal_ns.signal("backend.message.received").connect(self._on_msg_arrived, weak=True)
        self.signal_ns.signal("backend.message.polling_needed").connect(
            self._on_msg_arrived, weak=True
        )

    def _on_msg_arrived(self, sender, index=None):
        self._msg_arrived.set()

    async def _init(self, nursery):
        for beacon_id in self.local_folder_fs.get_local_beacons():
            self.signal_ns.signal("backend.beacon.listen").send(None, beacon_id=beacon_id)

        self._task_info = await nursery.start(self._task)

    async def _teardown(self):
        cancel_scope, closed_event = self._task_info
        cancel_scope.cancel()
        await closed_event.wait()
        self._task_info = None

    async def get_user_manifest(self):
        user_manifest_access = self.device.user_manifest_access
        while True:
            try:
                user_manifest = self.local_folder_fs.get_manifest(user_manifest_access)
                return user_manifest_access, user_manifest
            except FSManifestLocalMiss:
                await self.remote_loader.load_manifest(user_manifest_access)

    async def _task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        closed_event = trio.Event()
        try:
            with trio.open_cancel_scope() as cancel_scope:
                task_status.started((cancel_scope, closed_event))
                while True:
                    try:
                        await self._msg_arrived.wait()
                        self._msg_arrived.clear()
                        # TODO: what to do if we get a FSManifestLocalMiss raised ?
                        await self.process_last_messages()
                    except BackendNotAvailable:
                        pass
                    except SharingError:
                        # TODO: Does `logger.exception` already do `traceback.format_exc` for us ?
                        logger.exception("Error with backend: " % traceback.format_exc())
        finally:
            closed_event.set()

    async def process_last_messages(self):
        """
        Raises:
            SharingError
            BackendNotAvailable
            SharingBackendMessageError
            FSManifestLocalMiss: If user manifest is available in local
        """

        _, user_manifest = await self.get_user_manifest()
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
                logger.warning(exc.args[0])

        user_manifest_access, user_manifest = await self.get_user_manifest()
        if user_manifest["last_processed_message"] < new_last_processed_message:
            user_manifest["last_processed_message"] = new_last_processed_message
            self.local_folder_fs.update_manifest(user_manifest_access, user_manifest)

    async def _process_message(self, sender_id, ciphered):
        """
        Raises:
            SharingRecipientError
            SharingInvalidMessageError
            FSManifestLocalMiss: If user manifest is available in local
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
            user_manifest_access, user_manifest = await self.get_user_manifest()

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
            self.signal_ns.signal("sharing.new").send(
                None, path=f"/{sharing_name}", access=msg["access"]
            )

        elif msg["type"] == "ping":
            self.signal_ns.signal("pinged").send(None)
