# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio


async def freeze_messages_monitor_mockpoint():
    """
    Noop function that could be mocked during tests to be able to freeze the
    monitor coroutine running in background
    """
    pass


async def monitor_messages(user_fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    with event_bus.waiter_on("backend.message.received") as on_message_received:
        await user_fs.process_last_messages()
        task_status.started()
        while True:
            await on_message_received.wait()
            on_message_received.clear()
            await freeze_messages_monitor_mockpoint()
            await user_fs.process_last_messages()
