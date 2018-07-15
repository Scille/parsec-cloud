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
