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
        if module and module.__name__.startswith("trio."):
            return
    if hasattr(coro, "cr_frame"):
        yield from traceback.format_stack(coro.cr_frame)
        yield from format_stack(coro.cr_await)
        return
    if hasattr(coro, "gi_frame"):
        yield from traceback.format_stack(coro.gi_frame)
        yield from format_stack(coro.gi_yieldfrom)
        return


class TaskMonitoringInstrument(trio.abc.Instrument):
    def __init__(self, threshold=0.2):
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
        coro = task.coro
        stack = None if coro.cr_frame is None else "".join(format_stack(coro))
        msg = "Task step took too long"
        logger.warning(msg, task_name=task.name, duration=delta, stack=stack)
