# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import time
import inspect
import traceback

import trio
import structlog

logger = structlog.get_logger()


def format_stack(coro):
    if hasattr(coro, "cr_code"):
        module = inspect.getmodule(coro.cr_code)
        if module.__name__.startswith("trio."):
            return
    if hasattr(coro, "cr_frame"):
        yield from traceback.format_stack(coro.cr_frame)
    if hasattr(coro, "cr_await"):
        yield from format_stack(coro.cr_await)


class TaskMonitoringInstrument(trio.abc.Instrument):
    def __init__(self, threshold=0.05):
        self.start_dict = {}
        self.threshold = threshold

    def before_task_step(self, task):
        self.start_dict[task] = time.time()

    def after_task_step(self, task):
        if task not in self.start_dict:
            return
        delta = time.time() - self.start_dict.pop(task)
        if delta <= self.threshold:
            return
        if task.coro.cr_frame is None:
            msg = f"The last step for task `{task.name}` took {delta:.3f} s"
        else:
            msg = f"The previous step for task `{task.name}` took {delta:.3f} s"
            msg += "\nResuming afer:\n"
            msg += "".join(format_stack(task.coro))
        logger.warning(msg)
