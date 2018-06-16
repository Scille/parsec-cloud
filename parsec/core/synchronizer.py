import trio
import logbook
import traceback

from parsec.core.base import BaseAsyncComponent
from parsec.core.backend_connection import BackendNotAvailable, BackendError


logger = logbook.Logger("parsec.core.synchronizer")


class Synchronizer(BaseAsyncComponent):
    def __init__(self, auto_sync, fs):
        super().__init__()
        self.auto_sync = auto_sync
        self.fs = fs
        self._synchronizer_task_info = None

    async def _init(self, nursery):
        if self.auto_sync:
            self._synchronizer_task_info = await nursery.start(self._synchronizer_task)

    async def _teardown(self):
        if self._synchronizer_task_info:
            cancel_scope, closed_event = self._synchronizer_task_info
            cancel_scope.cancel()
            await closed_event.wait()

    async def _synchronizer_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        try:
            closed_event = trio.Event()
            with trio.open_cancel_scope() as cancel_scope:
                task_status.started(cancel_scope, closed_event)
                while True:
                    await trio.sleep(1)
                    # trigger_time = pendulum.now() + pendulum.interval(seconds=1)
                    try:
                        # TODO: quick'n dirty fix...
                        await self.fs.sync("/")
                    # await self._scan_and_sync_fs(self.fs.root, trigger_time)
                    except BackendNotAvailable:
                        pass
                    except BackendError:
                        logger.warning("Error with backend: %s" % traceback.format_exc())
        finally:
            closed_event.set()

    # async def _scan_and_sync_fs(self, entry, trigger_time):
    #     if entry.need_sync and entry.updated < trigger_time:
    #         logger.debug("sync {}", entry.path)
    #         await entry.sync(recursive=True)
    #     elif isinstance(entry, BaseFolderEntry):
    #         # TODO: not really elegant to access _children like this.
    #         # However we don't want to skip the not loadded entries...
    #         for children_entry in entry._children.values():
    #             await self._scan_and_sync_fs(children_entry, trigger_time)
