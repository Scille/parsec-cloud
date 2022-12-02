# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import math
import time
from contextlib import asynccontextmanager

import pytest
import trio
from trio.testing import MockClock


# In the test we often want to wait on something that in theory would be
# intaneously available but in fact depends on side effects.
# Exemple of side effects:
# - How fast & currently loaded is the CPU (especially when running on the CI)
# - Network socket
# - PostgreSQL database
# We have to decide how long we want to wait on those things, considering:
# - the shorter we wait, the more convenient it is for when developping
# - the longer we wait, the more we avoid false positive when side effets are unexpectly long
# - if we wait forever we get rid of the false positive but we also hang forever
#   in case a mistake in the code lead to a deadlock :(
# So the solution is to make this configurable: a good middleground by default
# and a long long time on the CI.
_SIDE_EFFECTS_TIMEOUT = 3


def get_side_effects_timeout() -> float:
    return _SIDE_EFFECTS_TIMEOUT


def _set_side_effects_timeout(timeout: float) -> None:
    global _SIDE_EFFECTS_TIMEOUT
    _SIDE_EFFECTS_TIMEOUT = timeout


@asynccontextmanager
async def real_clock_timeout():
    # In tests we use a mock clock to make parsec code faster by not staying idle,
    # however we might also want ensure some test code doesn't take too long.
    # Hence using `trio.fail_after` in test code doesn't play nice with mock clock
    # (especially given the CI can be unpredictably slow)...
    # The solution is to have our own fail_after that use the real monotonic clock.

    # Timeout is not configurable by design to avoid letting the user think
    # this parameter can be used to make the mocked trio clock advance
    timeout = get_side_effects_timeout()

    # Starting a thread can be very slow (looking at you, Windows) so better
    # take the starting time here
    start = time.monotonic()
    event_occured = False
    async with trio.open_nursery() as nursery:

        def _run_until_timeout_or_event_occured():
            while not event_occured and time.monotonic() - start < timeout:
                # cancelling `_watchdog` coroutine doesn't stop the thread,
                # so we sleep only by a short amount of time in order to
                # detect early enough that we are no longer needed
                time.sleep(0.01)

        async def _watchdog():
            await trio.to_thread.run_sync(_run_until_timeout_or_event_occured)
            if not event_occured:
                raise trio.TooSlowError()

        # Note: We could have started the thread directly instead of using
        # trio's thread support.
        # This would allow us to use a non-async contextmanager to better mimic
        # `trio.fail_after`, however this would prevent us from using trio's
        # threadpool system which is good given it allows us to reuse the thread
        # and hence avoid most of it cost
        nursery.start_soon(_watchdog)
        try:
            yield
        finally:
            event_occured = True
        nursery.cancel_scope.cancel()


@pytest.fixture
def mock_clock():
    # Prevent from using pytest_trio's `mock_clock` fixture.
    raise RuntimeError("Use `frozen_clock` fixture instead !!!")


@pytest.fixture
def autojump_clock():
    # Prevent from using pytest_trio's `autojump` fixture.
    raise RuntimeError("Use `frozen_clock` fixture instead !!!")


@pytest.fixture
def frozen_clock():
    # Mocked clock is a slippy slope: we want time to go faster (or even to
    # jump to arbitrary point in time !) on some part of our application while
    # some other parts should keep using the real time.
    # For instance we want to make sure some part of a test doesn't take more than
    # x seconds in real life (typically to detect deadlock), but this test might
    # be about a ping occurring every 30s so we want to simulate this wait.
    #
    # The simple solution is to use `MockClock.rate` to make time go faster,
    # but it's bad idea given we end up with two antagonistic goals:
    # - rate should be as high as possible so that ping wait goes as fast as possible
    # - the highest rate is, the smallest real time window we have when checking for
    #   deadlock, this is especially an issue given developer machine is a behemoth
    #   while CI run on potatoes (especially on MacOS) shared with other builds...
    #
    # So the solution we choose here is to separate the two times:
    # - Parsec codebase uses the trio clock and `trio.fail_after/move_on_after`
    # - Test codebase can use `trio.fail_after/move_on_after` as long as the test
    #   doesn't use a mock clock
    # - In case of mock clock, test codebase must use `real_clock_timeout` that
    #   relies on monotonic clock and hence is totally isolated from trio's clock.
    #
    # On top of that we must be careful about the configuration of the mock clock !
    # As we said the Parsec codebase (i.e. not the tests) uses the trio clock for
    # timeout handling & sleep (e.g. in the managers), hence:
    # - Using `MockClock.rate` with a high value still lead to the issue dicussed above.
    # - `trio.to_thread.run_sync` doesn't play nice with `MockClock.autojump_threshold = 0`
    #   given trio considers the coroutine waiting for the thread is idle and hence
    #   trigger the clock jump. So a perfectly fine async code may break tests in
    #   an unexpected way if it starts using `trio.to_thread.run_sync`...
    #
    # So the idea of the `frozen_clock` is to only advance when expecially
    # specified in the test (i.e. rate 0 and no autojump_threshold).
    # This way only the test code has control over the application timeout
    # handling, and we have a clean separation with the test timeout (i.e. using
    # `real_clock_timeout` to detect the test endup in a deadlock)
    #
    # The drawback of this approach is manually handling time jump can be cumbersome.
    # For instance the backend connection retry logic:
    # - sleeps for some time
    # - connects to the backend
    # - starts sync&message monitors
    # - message monitor may trigger modifications in the sync monitor
    # - in case of modification, sync monitor is going to sleep for a short time
    #   before doing the sync of the modification
    #
    # So to avoid having to mix `MockClock.jump` and `trio.testing.wait_all_tasks_blocked`
    # in a very complex and fragile way, we introduce the `sleep_with_autojump()`
    # method that is the only place where clock is going to move behind our back, but
    # for only the amount of time we choose, and only in a very explicit manner.
    #
    # Finally, an additional bonus to this approach is we can use breakpoint in the
    # code without worrying about triggering a timeout ;-)

    clock = MockClock(rate=0, autojump_threshold=math.inf)

    clock.real_clock_timeout = real_clock_timeout  # Quick access helper

    async def _sleep_with_autojump(seconds):
        old_rate = clock.rate
        old_autojump_threshold = clock.autojump_threshold
        clock.rate = 0
        clock.autojump_threshold = 0.01
        try:
            await trio.sleep(seconds)
        finally:
            clock.rate = old_rate
            clock.autojump_threshold = old_autojump_threshold

    clock.sleep_with_autojump = _sleep_with_autojump
    yield clock
