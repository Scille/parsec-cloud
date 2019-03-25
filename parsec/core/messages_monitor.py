# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger

from parsec.core.fs.sharing import SharingError, SharingBackendOffline


logger = get_logger()


async def monitor_messages(fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    msg_arrived = trio.Event()
    backend_online_event = trio.Event()
    backend_online_event.set()
    process_message_cancel_scope = None

    def _on_msg_arrived(event, index=None):
        msg_arrived.set()

    def _on_backend_online(event):
        backend_online_event.set()

    def _on_backend_offline(event):
        backend_online_event.clear()
        if process_message_cancel_scope:
            process_message_cancel_scope.cancel()

    with event_bus.connect_in_context(
        ("backend.message.received", _on_msg_arrived),
        ("backend.message.polling_needed", _on_msg_arrived),
        ("backend.online", _on_backend_online),
        ("backend.offline", _on_backend_offline),
    ):

        task_status.started()
        while True:
            try:

                with trio.CancelScope() as process_message_cancel_scope:
                    event_bus.send("message_monitor.reconnection_message_processing.started")
                    try:
                        await fs.process_last_messages()
                    finally:
                        event_bus.send("message_monitor.reconnection_message_processing.done")
                    while True:
                        await msg_arrived.wait()
                        msg_arrived.clear()
                        try:
                            await fs.process_last_messages()
                        except SharingError:
                            logger.exception("Invalid message from backend")

            except SharingBackendOffline:
                backend_online_event.clear()
            process_message_cancel_scope = None
            msg_arrived.clear()
            await backend_online_event.wait()
