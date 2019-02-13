# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio


class NotInitializedError(Exception):
    pass


class AlreadyInitializedError(Exception):
    pass


# TODO: Currently monitors (e.e. `SyncMonitor`) inherit from `BaseAsyncComponent`
# however the init/teardown functions is maybe misleading in this case
# given it use only use to start/stop the monitoring coroutine. Typically during
# unit tests we may want to use the monitor without calling those methods prior.


class BaseAsyncComponent:
    def __init__(self):
        self._lock = trio.Lock()
        self.is_initialized = False

    async def init(self, nursery):
        async with self._lock:
            if self.is_initialized:
                raise AlreadyInitializedError()

            await self._init(nursery)
            self.is_initialized = True

    async def teardown(self):
        async with self._lock:
            if not self.is_initialized:
                raise NotInitializedError()

            await self._teardown()

    async def _init(self, nursery):
        raise NotImplementedError()

    async def _teardown(self):
        raise NotImplementedError()


# TODO: replace by `parsec.utils.call_with_control`
def taskify(func, *args, **kwargs):
    async def _task(*, task_status=trio.TASK_STATUS_IGNORED):
        task = trio.hazmat.current_task()
        task.name = f"{func.__module__}.{func.__qualname__}"

        stopped = trio.Event()
        try:
            with trio.CancelScope() as cancel_scope:

                async def stop():
                    cancel_scope.cancel()
                    await stopped.wait()

                task_status.started(stop)

                await func(*args, **kwargs)

        finally:
            stopped.set()

    return _task
