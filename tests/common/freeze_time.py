# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import trio
import pendulum
from contextlib import contextmanager

from parsec.api.protocol import DeviceID
from parsec.core.types import LocalDevice


__freeze_time_dict = {}


def _timestamp_mockup(device):
    _, time = __freeze_time_dict.get(device.device_id, (None, None))
    return time if time is not None else pendulum.now()


@contextmanager
def freeze_device_time(device, current_time):
    # Parse time
    if isinstance(current_time, str):
        current_time = pendulum.parse(current_time)

    # Get device id
    if isinstance(device, LocalDevice):
        device_id = device.device_id
    elif isinstance(device, DeviceID):
        device_id = device
    else:
        assert False, device

    # Apply mockup (idempotent)
    type(device).timestamp = _timestamp_mockup

    # Save previous context
    previous_task, previous_time = __freeze_time_dict.get(device_id, (None, None))

    # Get current trio task
    try:
        current_task = trio.lowlevel.current_task()
    except RuntimeError:
        current_task = None

    # Ensure time has not been frozen from another coroutine
    assert previous_task in (None, current_task)

    try:
        # Set new context
        __freeze_time_dict[device_id] = (current_task, current_time)
        yield current_time
    finally:
        # Restore previous context
        __freeze_time_dict[device_id] = (previous_task, previous_time)


__freeze_time_task = None


@contextmanager
def freeze_time(time=None, device=None):
    # Get current time if not provided
    if time is None:
        time = pendulum.now()

    # Freeze a single device
    if device is not None:
        with freeze_device_time(device, time) as time:
            yield time
        return

    # Parse time
    global __freeze_time_task
    if isinstance(time, str):
        time = pendulum.parse(time)

    # Save previous context
    previous_task = __freeze_time_task
    previous_time = pendulum.get_test_now()

    # Get current trio task
    try:
        current_task = trio.lowlevel.current_task()
    except RuntimeError:
        current_task = None

    # Ensure time has not been frozen from another coroutine
    assert previous_task in (None, current_task)

    try:
        # Set new context
        __freeze_time_task = current_task
        pendulum.set_test_now(time)
        try:
            from libparsec.types import freeze_time as _Rs_freeze_time
        except ImportError:
            pass
        else:
            _Rs_freeze_time(time)

        yield time
    finally:
        # Restore previous context
        __freeze_time_task = previous_task
        pendulum.set_test_now(previous_time)
        try:
            from libparsec.types import freeze_time as _Rs_freeze_time
        except ImportError:
            pass
        else:
            _Rs_freeze_time(previous_time)
