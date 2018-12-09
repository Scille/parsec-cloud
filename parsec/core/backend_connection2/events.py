import trio


# TODO
async def backend_listen_events(device, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    task_status.started()
    await trio.sleep_forever()
