# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from contextlib import contextmanager
from typing import Iterator, Optional

import trio

from parsec._parsec import DateTime, mock_time
from parsec.api.protocol import DeviceID

FreezeContext = dict[DeviceID, tuple[Optional[trio.lowlevel.Task], Optional[DateTime]]]
__freeze_time_dict: FreezeContext = {}


# Global ID that is used to save previous task & time when freezing datetime.
__global_freeze_time_id = DeviceID.new()


@contextmanager
def freeze_time(
    time: DateTime | str | None = None,
) -> Iterator[DateTime]:
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

    # Save previous context
    global __global_freeze_time_id
    global __freeze_time_dict
    previous_task, previous_time = __freeze_time_dict.get(__global_freeze_time_id, (None, None))

    # Ensure time has not been frozen from another coroutine
    assert previous_task in (None, current_task)

    try:
        # Set freeze datetime
        __freeze_time_dict[__global_freeze_time_id] = (current_task, time)
        mock_time(time)

        yield time
    finally:
        # Restore previous context
        __freeze_time_dict[__global_freeze_time_id] = (previous_task, previous_time)
        mock_time(previous_time)
