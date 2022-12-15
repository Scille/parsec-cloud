# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import contextmanager
from typing import Iterator, Optional

import trio

from parsec._parsec import DateTime, mock_time
from parsec.api.protocol import DeviceID
from parsec.core.types import LocalDevice

FreezeContext = dict[DeviceID, tuple[Optional[trio.lowlevel.Task], Optional[DateTime]]]
__freeze_time_dict: FreezeContext = {}


def _freeze_devices(
    devices: list[LocalDevice], current_task: trio.lowlevel.Task | None, time: DateTime
) -> FreezeContext:
    restore_context: FreezeContext = dict()

    for device in devices:
        # Get device id
        dev_id = device.device_id

        # Freeze time with mockup (idempotent) & timeprovider
        print(f"[_freeze_devices] device_id={dev_id}, update time provider config")
        # Mypy doesn't like that we assign a function to a method type
        device.time_provider.mock_time(freeze=time)

        # Save previous context
        previous_task, previous_time = __freeze_time_dict.get(dev_id, (None, None))

        # Ensure time has not been frozen from another coroutine
        assert previous_task in (None, current_task)

        # Set new context
        __freeze_time_dict[dev_id] = (current_task, time)

        restore_context[dev_id] = (previous_task, previous_time)

    return restore_context


def _unfreeze_devices(devices: list[LocalDevice], previous_context: FreezeContext) -> None:
    for device in devices:
        dev_id = device.device_id
        previous_task, previous_time = previous_context[dev_id]
        __freeze_time_dict[dev_id] = (previous_task, previous_time)

        if previous_time is not None:
            device.time_provider.mock_time(freeze=previous_time)
        else:
            device.time_provider.mock_time(realtime=True)


# Global ID that is used to save previous task & time when freezing datetime.
__global_freeze_time_id = DeviceID.new()


@contextmanager
def freeze_time(
    time: DateTime | str | None = None,
    devices: list[LocalDevice] | None = None,
    freeze_datetime: bool = False,
) -> Iterator[DateTime]:
    devices = devices or list()
    # Will freeze `Datetime` if the caller said so or if no devices where provided
    freeze_datetime = freeze_datetime or not devices

    # Get current time if not provided
    if time is None:
        time = DateTime.now()
    elif isinstance(time, str):
        y, m, d = map(int, time.split("-"))
        time = DateTime(y, m, d)

    # Get current trio task
    try:
        current_task = trio.lowlevel.current_task()
    except RuntimeError:
        current_task = None

    # Freeze a multiple devices
    restore_context = _freeze_devices(devices, current_task, time)

    # Save previous context
    global __global_freeze_time_id
    global __freeze_time_dict
    previous_task, previous_time = __freeze_time_dict.get(__global_freeze_time_id, (None, None))

    # Ensure time has not been frozen from another coroutine
    assert previous_task in (None, current_task)

    try:
        # Set freeze datetime
        if freeze_datetime:
            __freeze_time_dict[__global_freeze_time_id] = (current_task, time)
            mock_time(time)

        yield time
    finally:
        # Restore previous context
        if freeze_datetime:
            __freeze_time_dict[__global_freeze_time_id] = (previous_task, previous_time)
            mock_time(previous_time)

        _unfreeze_devices(devices, restore_context)
