# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import trio
from pendulum import Pendulum

from parsec.monitoring import TaskMonitoringInstrument
from parsec.service_nursery import open_service_nursery

__all__ = ["timestamps_in_the_ballpark", "start_task", "trio_run", "open_service_nursery"]

TIMESTAMP_MAX_DT = 30 * 60


def timestamps_in_the_ballpark(ts1: Pendulum, ts2: Pendulum, max_dt=TIMESTAMP_MAX_DT) -> bool:
    """
    Useful to compare signed message timestamp with the one stored by the
    backend.
    """
    return abs((ts1 - ts2).total_seconds()) < max_dt


# Task status


@attr.s
class TaskStatus:

    # Internal state

    _cancel_scope = attr.ib(default=None)
    _started_value = attr.ib(default=None)
    _finished_event = attr.ib(factory=trio.Event)

    def _set_cancel_scope(self, scope):
        self._cancel_scope = scope

    def _set_started_value(self, value):
        self._started_value = value

    def _set_finished(self):
        self._finished_event.set()

    # Properties

    @property
    def cancel_called(self):
        return self._cancel_scope.cancel_called

    @property
    def finished(self):
        return self._finished_event.is_set()

    @property
    def value(self):
        return self._started_value

    # Methods

    def cancel(self):
        self._cancel_scope.cancel()

    async def join(self):
        await self._finished_event.wait()
        await trio.sleep(0)  # Checkpoint

    async def cancel_and_join(self):
        self.cancel()
        await self.join()

    # Class methods

    @classmethod
    async def wrap_task(cls, corofn, *args, task_status=trio.TASK_STATUS_IGNORED):
        status = cls()
        try:
            async with trio.open_service_nursery() as nursery:
                status._set_cancel_scope(nursery.cancel_scope)
                value = await nursery.start(corofn, *args)
                status._set_started_value(value)
                task_status.started(status)
        finally:
            status._set_finished()


async def start_task(nursery, corofn, *args, name=None):
    """Equivalent to nursery.start but return a TaskStatus object.

    This object can be used to cancel and/or join the task.
    It also contains the started value set by `task_status.started()`.
    """
    return await nursery.start(TaskStatus.wrap_task, corofn, *args, name=name)


def trio_run(async_fn, *args, use_asyncio=False):
    if use_asyncio:
        # trio_asyncio is an optional dependency
        import trio_asyncio

        return trio_asyncio.run(async_fn, *args)
    instruments = (TaskMonitoringInstrument(),)
    return trio.run(async_fn, *args, instruments=instruments)
