# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger

from parsec.core.fs import FSError, FSBackendOfflineError


logger = get_logger()


async def freeze_messages_monitor_mockpoint():
    """
    Noop function that could be mocked during tests to be able to freeze the
    monitor coroutine running in background
    """
    pass


async def monitor_messages(user_fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    msg_arrived = trio.Event()
    backend_online_event = trio.Event()
    process_message_cancel_scope = None

    def _on_msg_arrived(event, index=None):
        msg_arrived.set()

    def _on_backend_online(event):
        backend_online_event.set()

    def _on_backend_offline(event):
        nonlocal backend_online_event
        backend_online_event = trio.Event()
        if process_message_cancel_scope:
            process_message_cancel_scope.cancel()

    async def _process_last_messages():
        try:
            await user_fs.process_last_messages()

        except FSBackendOfflineError:
            raise

        except FSError:
            logger.exception("Invalid message from backend")

    with event_bus.connect_in_context(
        ("backend.message.received", _on_msg_arrived),
        ("backend.message.polling_needed", _on_msg_arrived),
        ("backend.online", _on_backend_online),
        ("backend.offline", _on_backend_offline),
    ):

        task_status.started()
        while True:
            await backend_online_event.wait()

            try:
                with trio.CancelScope() as process_message_cancel_scope:
                    event_bus.send("message_monitor.reconnection_message_processing.started")
                    await _process_last_messages()
                    event_bus.send("message_monitor.reconnection_message_processing.done")

                    while True:
                        await msg_arrived.wait()
                        msg_arrived = trio.Event()
                        await freeze_messages_monitor_mockpoint()
                        await _process_last_messages()

            except FSBackendOfflineError:
                backend_online_event = trio.Event()
                process_message_cancel_scope = None
                msg_arrived = trio.Event()
