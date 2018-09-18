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


def taskify(func, *args, **kwargs):
    async def _task(*, task_status=trio.TASK_STATUS_IGNORED):
        stopped = trio.Event()
        try:
            with trio.open_cancel_scope() as cancel_scope:

                async def stop():
                    cancel_scope.cancel()
                    await stopped.wait()

                task_status.started(stop)

                await func(*args, **kwargs)

        finally:
            stopped.set()

    return _task
