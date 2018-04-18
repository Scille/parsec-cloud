import trio
import logbook

# import pendulum
import traceback

from parsec.core.fs.folder import BaseFolderEntry
from parsec.core.backend_connection import BackendNotAvailable, BackendError


logger = logbook.Logger("parsec.core.synchronizer")


class Synchronizer:

    def __init__(self):
        self.fs = None
        self.nursery = None
        self._synchronizer_task_cancel_scope = None

    async def init(self, nursery, fs):
        self.fs = fs
        self.nursery = nursery
        self._synchronizer_task_cancel_scope = await nursery.start(self._synchronizer_task)

    async def teardown(self):
        self._synchronizer_task_cancel_scope.cancel()

    async def _synchronizer_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        with trio.open_cancel_scope() as cancel_scope:
            task_status.started(cancel_scope)
            while True:
                await trio.sleep(1)
                # trigger_time = pendulum.now() + pendulum.interval(seconds=1)
                try:
                    # TODO: quick'n dirty fix...
                    await self.fs.root.sync(recursive=True)
                # await self._scan_and_sync_fs(self.fs.root, trigger_time)
                except BackendNotAvailable:
                    pass
                except BackendError:
                    logger.warning("Error with backend: %s" % traceback.format_exc())

    async def _scan_and_sync_fs(self, entry, trigger_time):
        if entry.need_sync and entry.updated < trigger_time:
            logger.debug("sync {}", entry.path)
            await entry.sync(recursive=True)
        elif isinstance(entry, BaseFolderEntry):
            # TODO: not really elegant to access _children like this.
            # However we don't want to skip the not loadded entries...
            for children_entry in entry._children.values():
                await self._scan_and_sync_fs(children_entry, trigger_time)
