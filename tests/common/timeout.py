# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from contextlib import asynccontextmanager
import time

import trio


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
