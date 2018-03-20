import trio
import logbook
import pendulum

from parsec.core.fs.folder import BaseFolderEntry
from parsec.core.backend_connection import BackendNotAvailable


logger = logbook.Logger("parsec.core.synchronizer")


class Synchronizer:
    async def init(self, nursery, fs):
        self.fs = fs
        self.nursery = nursery
        self._synchronizer_task_cancel_scope = await nursery.start(self._synchronizer_task)

    async def teardown(self):
        self._synchronizer_task_cancel_scope.cancel()
        if self._sock:
            await self._sock.aclose()

    async def _synchronizer_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        task_status.started()
        while True:
            await trio.sleep(1)
            trigger_time = pendulum.now() + pendulum.interval(seconds=1)
            try:
                await self._scan_and_sync_fs(self.fs.root, trigger_time)
            except BackendNotAvailable:
                pass

    async def _scan_and_sync_fs(self, entry, trigger_time):
        if entry.need_sync and entry.updated < trigger_time:
            logger.debug('sync {}', entry.path)
            await entry.sync()
        elif isinstance(entry, BaseFolderEntry):
            # TODO: not really elegant to access _children like this.
            # However we don't want to skip the not loadded entries...
            for children_entry in entry._children.values():
                self._scan_and_sync_fs(children_entry)
