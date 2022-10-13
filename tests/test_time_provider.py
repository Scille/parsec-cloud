# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import trio

from parsec._parsec import TimeProvider, DateTime


async def wait_for_sleeping_stat(tp, expected):
    # `TimeProvider.sleeping_stats` return the number of *tokio* tasks currently
    # awaiting on the `TimeProvider.sleep`.
    # The trick is tokio tasks work in total isolation (and concurrency !) with our
    # Python world. So we must rely on a busy loop to poll for the change we expect.
    while tp.sleeping_stats() != expected:
        await trio.sleep(0)


@pytest.mark.trio
async def test_frozen_mock():
    root_tp = TimeProvider()
    child1_tp = root_tp.new_child()
    child2_tp = root_tp.new_child()
    grandchild1_from_child1_tp = child1_tp.new_child()

    real_now = root_tp.now()
    t1 = DateTime(2001, 1, 1, 0, 0, 0)
    t2 = DateTime(2002, 1, 1, 0, 0, 0)

    # Mock a time provider doesn't impact it coursins or separated time providers
    child1_tp.mock_time(freeze=t1)
    assert TimeProvider().now() >= real_now
    assert child2_tp.now() >= real_now

    # Mock in parent is propagated to children and grandchildren
    child1_tp.mock_time(realtime=True)
    root_tp.mock_time(freeze=t1)
    assert root_tp.now() == t1
    assert child1_tp.now() == t1
    assert grandchild1_from_child1_tp.now() == t1

    # Mock in child overwrite parent mock...
    child1_tp.mock_time(freeze=t2)
    assert child1_tp.now() == t2
    # ...and is also used by it own children
    assert grandchild1_from_child1_tp.now() == t2

    # Unmock time for parent doesn't unmock for child
    root_tp.mock_time(realtime=True)
    assert root_tp.now() >= real_now
    assert child1_tp.now() == t2
    assert grandchild1_from_child1_tp.now() == t2

    # Mock grandchild and unmock child should work as expected
    grandchild1_from_child1_tp.mock_time(t1)
    child1_tp.mock_time(realtime=True)
    assert child1_tp.now() >= real_now
    assert grandchild1_from_child1_tp.now() == t1


@pytest.mark.trio
async def test_shift_mock():
    root_tp = TimeProvider()
    child1_tp = root_tp.new_child()
    grandchild1_from_child1_tp = child1_tp.new_child()

    real_now = root_tp.now()

    child1_tp.mock_time(shift=1)
    grandchild1_from_child1_tp.mock_time(shift=1)

    root_tp_now = root_tp.now()
    child1_tp_now = child1_tp.now()
    grandchild1_from_child1_tp_now = grandchild1_from_child1_tp.now()
    assert root_tp_now >= real_now  # Sanity check
    assert child1_tp_now - root_tp_now >= 1
    assert grandchild1_from_child1_tp_now - root_tp_now >= 2

    # Also test negative shift
    present = child1_tp.now()
    child1_tp.mock_time(shift=-10)
    past = child1_tp.now()
    assert present - past >= 10

    # Combine with freeze

    t1 = DateTime(2001, 1, 1, 0, 0, 0)

    child1_tp.mock_time(freeze=t1)
    assert root_tp.now() >= root_tp_now
    assert child1_tp.now() == t1
    assert grandchild1_from_child1_tp.now() == t1.add(seconds=1)


@pytest.mark.flaky(reruns=3)
@pytest.mark.trio
async def test_sleep_with_mock():
    tp = TimeProvider()
    assert tp.sleeping_stats() == 0  # Sanity check

    t1 = DateTime(2001, 1, 1, 0, 0, 0)
    t2 = DateTime(2002, 1, 2, 0, 0, 0)

    async def _async_mock_time(time_provider, freeze, shift):
        # Make sure we are not changing the mock before time provider sleeps
        await trio.sleep(0.01)
        time_provider.mock_time(freeze=freeze, shift=shift)

    async with trio.open_nursery() as nursery:

        # Test shift mock
        with trio.fail_after(1):
            nursery.start_soon(_async_mock_time, tp, None, 11)
            await tp.sleep(10)
            await wait_for_sleeping_stat(tp, 0)

        # Test freeze mock
        with trio.fail_after(1):
            tp.mock_time(t1)
            nursery.start_soon(_async_mock_time, tp, t2, None)
            await tp.sleep(2**24)  # ~6month wait, better have the mock working !
            await wait_for_sleeping_stat(tp, 0)


@pytest.fixture(params=("raw", "wrapped"))
def maybe_wrap_tp_sleep(request):
    # In raw mode we pass to trio a coroutine that directly yield `TokioTaskAborterFromTrio`
    # (i.e. the low-level control object undestood by trio).
    # In wrapped mode, we pass to trio a coroutine that itself await on the raw coroutine.
    if request.param == "raw":

        def _raw_tp_sleep(tp, time):
            return tp.sleep(time)

        return _raw_tp_sleep

    else:
        # Believe it or not, this is pretty different than directly calling `tp.sleep` !
        async def _wrapped_tp_sleep(tp, time):
            await tp.sleep(time)

        return _wrapped_tp_sleep


@pytest.mark.trio
async def test_sleep_cancellation(maybe_wrap_tp_sleep):
    tp = TimeProvider()

    with trio.fail_after(1):
        with trio.CancelScope() as cancel_scope:
            cancel_scope.cancel()
            await maybe_wrap_tp_sleep(tp, 10)
        await wait_for_sleeping_stat(tp, 0)

    with trio.fail_after(1):
        with pytest.raises(trio.TooSlowError):
            with trio.fail_after(0):
                await maybe_wrap_tp_sleep(tp, 10)
        await wait_for_sleeping_stat(tp, 0)


@pytest.mark.flaky(reruns=3)
@pytest.mark.trio
async def test_sleep_in_nursery(maybe_wrap_tp_sleep):
    tp = TimeProvider()

    # Nursery is of no use
    with trio.fail_after(1):
        async with trio.open_nursery() as nursery:
            await maybe_wrap_tp_sleep(tp, 0.001)
            assert not nursery.child_tasks
        await wait_for_sleeping_stat(tp, 0)

    # Wait for our coroutines done in the nursery's  `__aexit__`
    with trio.fail_after(1):
        async with trio.open_nursery() as nursery:
            nursery.start_soon(maybe_wrap_tp_sleep, tp, 0.001)
            nursery.start_soon(maybe_wrap_tp_sleep, tp, 0.001)
        await wait_for_sleeping_stat(tp, 0)

    # Wait for our coroutines done in the nursery body
    with trio.fail_after(1):
        async with trio.open_nursery() as nursery:
            nursery.start_soon(maybe_wrap_tp_sleep, tp, 0.001)
            nursery.start_soon(maybe_wrap_tp_sleep, tp, 0.001)
            # Remember communication between trio and tokio introduces latency, so we
            # can't just do a `trio.sleep` and expects the tokio tasks to have rescheduled
            # their corresponding trio coroutine.
            await wait_for_sleeping_stat(tp, 2)
            # Here the trio coroutines and their corresponding tokio tasks are sleeping
            await wait_for_sleeping_stat(tp, 0)
            # Here the trio coroutines are done
            # `__aexit__` has nothing left to do


@pytest.mark.trio
async def test_concurrent_sleeps(maybe_wrap_tp_sleep):
    t1 = DateTime(2001, 1, 1, 0, 0, 0)
    t2 = t1.add(seconds=11)
    tp1 = TimeProvider()
    tp2 = TimeProvider()

    async with trio.open_nursery() as nursery:
        with trio.fail_after(1):

            tp1.mock_time(freeze=t1)
            tp2.mock_time(freeze=t1)

            assert tp1.sleeping_stats() == 0  # No-one is sleeping on us
            assert tp2.sleeping_stats() == 0  # No-one is sleeping on us

            nursery.start_soon(maybe_wrap_tp_sleep, tp1, 10)
            nursery.start_soon(maybe_wrap_tp_sleep, tp2, 10)

            # Busy loop to wait for the time providers to sleep
            await wait_for_sleeping_stat(tp1, 1)
            await wait_for_sleeping_stat(tp2, 1)

            # Now make a time jump for a single time provider
            tp1.mock_time(freeze=t2)
            await wait_for_sleeping_stat(tp1, 0)

            assert tp1.sleeping_stats() == 0
            assert tp2.sleeping_stats() == 1

            # Finally resolve the second one
            tp2.mock_time(freeze=t2)
            await wait_for_sleeping_stat(tp2, 0)


@pytest.mark.trio
async def test_reproduce_trio_corruption():
    """
    This test used to produce a trio corruption due to an incorrect
    implementation of `FutureToCouroutine` (related to cancellation handling).
    """
    t1 = DateTime(2001, 1, 1, 0, 0, 0)
    t2 = t1.add(seconds=11)
    tp1 = TimeProvider()
    tp1.mock_time(freeze=t1)

    with pytest.raises(ZeroDivisionError):
        with trio.fail_after(1):
            async with trio.open_nursery() as nursery:
                nursery.start_soon(tp1.sleep, 10)
                await wait_for_sleeping_stat(tp1, 1)
                # Mock and raise at the same time
                tp1.mock_time(freeze=t2)
                await trio.sleep(0)
                1 / 0


@pytest.mark.trio
async def test_reproduce_concurrency_issue(maybe_wrap_tp_sleep):
    """
    This test used to reproduce a concurrency issue due to an incorrect
    implementation of `TimeProvider` (due to a race condition while taking
    a time reference in the `sleep` method)
    """
    t1 = DateTime(2001, 1, 1, 0, 0, 0)
    t2 = t1.add(seconds=11)
    tp1 = TimeProvider()

    async with trio.open_nursery() as nursery:
        with trio.fail_after(1):

            tp1.mock_time(freeze=t1)
            assert tp1.sleeping_stats() == 0

            nursery.start_soon(maybe_wrap_tp_sleep, tp1, 10)
            await trio.sleep(0)

            await wait_for_sleeping_stat(tp1, 1)

            tp1.mock_time(freeze=t2)
            await wait_for_sleeping_stat(tp1, 0)

            assert tp1.sleeping_stats() == 0
