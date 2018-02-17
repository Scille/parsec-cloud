import pytest
import trio
import queue
from functools import wraps
from contextlib import contextmanager
from hypothesis.stateful import RuleBasedStateMachine, run_state_machine_as_test, invariant

from tests.common import QuitTestDueToBrokenStream


def skip_on_broken_stream(fn):

    @wraps(fn)
    def wrapper(*args, **kwargs):
        try:
            return fn(*args, **kwargs)
        except QuitTestDueToBrokenStream:
            pass

    return wrapper


class ThreadToTrioCommunicator:
    def __init__(self, portal):
        self.portal = portal
        self.queue = queue.Queue()
        self.trio_queue = trio.Queue(1)

    def send(self, msg):
        self.portal.run(self.trio_queue.put, msg)
        ret = self.queue.get()
        if isinstance(ret, Exception):
            raise ret
        return ret

    async def trio_recv(self):
        ret = await self.trio_queue.get()
        return ret

    async def trio_respond(self, msg):
        self.queue.put(msg)

    def close(self):
        self.queue.put(QuitTestDueToBrokenStream())


@contextmanager
def open_communicator(portal):
    communicator = ThreadToTrioCommunicator(portal)
    try:
        yield communicator
    finally:
        communicator.close()


@pytest.fixture
async def portal():
    return trio.BlockingTrioPortal()


@pytest.fixture
def monitor():
    from trio.monitor import Monitor
    return Monitor()


@pytest.fixture
async def TrioDriverRuleBasedStateMachine(nursery, portal, loghandler):

    class TrioDriverRuleBasedStateMachine(RuleBasedStateMachine):
        _portal = portal
        _nursery = nursery
        _running = trio.Lock()

        @classmethod
        async def run_test(cls):
            await trio.run_sync_in_worker_thread(run_state_machine_as_test, cls)

        async def trio_runner(self, task_status):
            raise NotImplementedError()

        @property
        def communicator(self):
            assert self._communicator
            return self._communicator

        async def _trio_runner(self, *, task_status=trio.TASK_STATUS_IGNORED):
            # We need to hijack `task_status.started` callback because error
            # handling of trio_runner coroutine depends of it (see below).
            task_started = False
            vanilla_task_status_started = task_status.started

            def task_status_started_hook(ret=None):
                nonlocal task_started
                task_started = True
                vanilla_task_status_started(ret)

            task_status.started = task_status_started_hook

            # Drop previous run logs, preventing flooding stdout
            loghandler.records.clear()
            try:
                # This lock is to make sure the hypothesis thread doesn't start
                # another `_trio_runner` coroutine while this one hasn't done
                # it teardown yet.
                async with self._running:
                    with trio.open_cancel_scope() as self._trio_runner_cancel_scope:
                        with open_communicator(self._portal) as self._communicator:
                            await self.trio_runner(task_status)
            except Exception as exc:
                if not task_started:
                    # If the crash occurs during the init phase, hypothesis
                    # thread is synchrone with this coroutine so raising the
                    # exception here will have the expected effect.
                    raise
                # The trick is to avoid raising the exception here given
                # otherwise hypothesis will consider the crash comes from the
                # previously executed rule.
                self._trio_runner_crash = exc

        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self._trio_runner_crash = None
            self._portal.run(self._nursery.start, self._trio_runner)

        def teardown(self):
            self._trio_runner_cancel_scope.cancel()

        @invariant()
        def check_trio_runner_crash(self):
            # After each rule we make sure the trio runner hasn't crashed,
            # so we can be sure which rule caused it.
            if self._trio_runner_crash:
                raise self._trio_runner_crash

    return TrioDriverRuleBasedStateMachine
