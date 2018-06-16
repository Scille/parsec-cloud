import trio
import logbook
import traceback
import pendulum

from parsec.core.base import BaseAsyncComponent
from parsec.core.backend_connection import BackendNotAvailable, BackendError
from parsec.core.fs.utils import FSInvalidPath


logger = logbook.Logger("parsec.core.synchronizer")
BACKEND_OFFLINE_WAIT = 5
# Don't sync an entry if it has been modified recently
MIN_WAIT = pendulum.interval(seconds=1)
# If an entry is constantly modified, this threshold force the sync anyway
MAX_WAIT = pendulum.interval(seconds=60)


class Synchronizer(BaseAsyncComponent):
    def __init__(self, auto_sync, fs, signal_ns):
        super().__init__()
        self.auto_sync = auto_sync
        self.fs = fs
        self.signal_ns = signal_ns
        self._synchronizer_task_info = None
        self._sync_candidates_journal = []

    async def _init(self, nursery):
        if self.auto_sync:
            self._synchronizer_task_info = await nursery.start(self._synchronizer_task)

    async def _teardown(self):
        if self._synchronizer_task_info:
            cancel_scope, closed_event = self._synchronizer_task_info
            cancel_scope.cancel()
            await closed_event.wait()

    async def _synchronizer_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        candidates_updated = trio.Event()

        def _on_entry_updated(sender, access, path):
            if not path.endswith("/"):
                path += "/"
            self._sync_candidates_journal.append((pendulum.now(), path))
            candidates_updated.set()

        self.signal_ns.signal("fs_entry_updated").connect(_on_entry_updated, weak=True)

        with trio.open_cancel_scope() as cancel_scope:
            task_status.started(cancel_scope)
            while True:
                await candidates_updated.wait()

                assert len(self._sync_candidates_journal)
                now = pendulum.now()

                # Quick exit if nothing is old enough to be synchronized
                first_ts, first_path = self._sync_candidates_journal[0]
                if now - first_ts < MIN_WAIT:
                    await trio.sleep(now - first_ts)
                    continue

                # Give priority to old modification
                if now - first_ts > MAX_WAIT:
                    to_sync_path = first_path
                else:
                    for ts, path in reversed(self._sync_candidates_journal):
                        if now - ts > MIN_WAIT:
                            to_sync_path = path
                            break

                # Now sync the data
                try:
                    await self.fs.sync(path, recursive=True)
                except BackendNotAvailable:
                    # Time to wait a bit...
                    await trio.sleep(BACKEND_OFFLINE_WAIT)
                    continue
                except FSInvalidPath:
                    # Path destroyed in the meantime...
                    pass
                except BackendError:
                    logger.warning("Error with backend: %s" % traceback.format_exc())

                # Finally update the journal, taking into account the sync was recursive
                new_journal = []
                for ts, path in self._sync_candidates_journal:
                    if path.startswith(to_sync_path) and ts < now:
                        continue
                    new_journal.append((ts, path))

                self._sync_candidates_journal = new_journal
                if not new_journal:
                    candidates_updated.clear()
