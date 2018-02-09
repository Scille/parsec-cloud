import pytest
import trio
import queue
from contextlib import contextmanager
from hypothesis.stateful import RuleBasedStateMachine, run_state_machine_as_test


class ThreadToTrioCommunicator:
    def __init__(self, portal):
        self.portal = portal
        self.queue = queue.Queue()
        self.trio_queue = trio.Queue(1)

    def send(self, msg):
        self.portal.run(self.trio_queue.put, msg)
        return self.queue.get()

    async def trio_recv(self):
        return await self.trio_queue.get()

    async def trio_respond(self, msg):
        self.queue.put(msg)

    def close(self):
        self.queue.put(None)


@pytest.fixture
async def portal():
    return trio.BlockingTrioPortal()


@pytest.fixture
def monitor():
    from trio.monitor import Monitor
    return Monitor()


@pytest.fixture
async def TrioDriverRuleBasedStateMachine(nursery, portal):

    class TrioDriverRuleBasedStateMachine(RuleBasedStateMachine):
        _portal = portal
        _nursery = nursery

        @classmethod
        async def run_test(cls):
            await trio.run_sync_in_worker_thread(run_state_machine_as_test, cls)

        async def trio_runner(self, task_status):
            raise NotImplementedError()

        async def _trio_runner(self, *, task_status=trio.TASK_STATUS_IGNORED):
            with trio.open_cancel_scope() as self._trio_runner_cancel_scope:
                await self.trio_runner(task_status)

        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self._portal.run(self._nursery.start, self._trio_runner)

        def teardown(self):
            self._trio_runner_cancel_scope.cancel()

        @contextmanager
        def open_communicator(self):
            communicator = ThreadToTrioCommunicator(portal)
            try:
                yield communicator
            finally:
                communicator.close()

    return TrioDriverRuleBasedStateMachine
