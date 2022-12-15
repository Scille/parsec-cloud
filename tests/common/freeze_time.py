# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import contextmanager

import trio

from parsec._parsec import DateTime, mock_time
from parsec.api.protocol import DeviceID
from parsec.core.types import LocalDevice

__freeze_time_dict: dict[DeviceID, DateTime] = {}


def _timestamp_mockup(device: LocalDevice) -> DateTime:
    _, time = __freeze_time_dict.get(device.device_id, (None, None))
    return time or device.time_provider.now()


@contextmanager
def freeze_device_time(device: LocalDevice | DeviceID, current_time: DateTime | str):
    # Parse time
    if isinstance(current_time, str):
        [y, m, d] = current_time.split("-")
        current_time = DateTime(int(y), int(m), int(d))

    # Get device id
    if isinstance(device, LocalDevice):
        device_id = device.device_id
    elif isinstance(device, DeviceID):
        device_id = device
    else:
        assert False, device

    device.time_provider.mock_time(freeze=current_time)

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
def freeze_time(time: DateTime | str = None, device: LocalDevice | DeviceID | None = None):
    mocks_stack = []

    # Get current time if not provided
    if time is None:
        time = DateTime.now()

    # Freeze a single device
    if device is not None:
        with freeze_device_time(device, time) as time:
            yield time
        return

    # Parse time
    global __freeze_time_task
    if isinstance(time, str):
        [y, m, d] = time.split("-")
        time = DateTime(int(y), int(m), int(d))

    # Save previous context
    previous_task = __freeze_time_task
    previous_time = mocks_stack.pop() if mocks_stack else None

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
        mocks_stack.append(time)
        mock_time(time)

        yield time
    finally:
        # Restore previous context
        __freeze_time_task = previous_task
        mocks_stack.append(previous_time)
        mock_time(previous_time)
