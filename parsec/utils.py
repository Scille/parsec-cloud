import attr
import trio


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
            async with trio.open_nursery() as nursery:
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
