# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import gc
import time
import inspect
import traceback
from typing import Any, Iterable, Tuple

import trio
import structlog
from trio.lowlevel import Task

logger = structlog.get_logger()


def format_stack(coroutine: Any) -> Iterable[str]:
    # Stop when reaching trio modules
    if hasattr(coroutine, "cr_code"):
        module = inspect.getmodule(coroutine.cr_code)
        if module and module.__name__.startswith("trio."):
            return

    # Work around https://bugs.python.org/issue32810
    if hasattr(coroutine, "__class__") and coroutine.__class__.__name__ in (
        "async_generator_asend",
        "async_generator_athrow",
    ):
        coroutine, *_ = gc.get_referents(coroutine)
        if not inspect.isasyncgen(coroutine):
            return

    # Follow a generator
    if getattr(coroutine, "ag_frame", None):
        yield from traceback.format_stack(coroutine.ag_frame)
        yield from format_stack(coroutine.ag_await)
        return

    # Follow a coroutine
    if getattr(coroutine, "cr_frame", None):
        yield from traceback.format_stack(coroutine.cr_frame)
        yield from format_stack(coroutine.cr_await)
        return

    # Follow a decorated coroutine
    if getattr(coroutine, "gi_frame", None):
        yield from traceback.format_stack(coroutine.gi_frame)
        yield from format_stack(coroutine.gi_yieldfrom)
        return


class TaskMonitoringInstrument(trio.abc.Instrument):
    def __init__(
        self, cpu_threshold: float = 0.2, io_threshold: float = 0.1, trace_all: bool = False
    ) -> None:
        self.start_dict: dict[Task, Tuple[float, float, str]] = {}
        self.cpu_threshold = cpu_threshold
        self.io_threshold = io_threshold
        self.trace_all = trace_all

    def get_stacktrace(self, task: Task) -> str:
        return "".join(format_stack(task.coro))

    def before_task_step(self, task: Task) -> None:
        stack = self.get_stacktrace(task) if self.trace_all else ""
        self.start_dict[task] = (time.monotonic(), time.process_time(), stack)

    def after_task_step(self, task: Task) -> None:
        if task not in self.start_dict:
            return

        # Pop the information and compute the deltas
        monotonic_start, process_start, stack_before = self.start_dict.pop(task)
        monotonic_delta = round(time.monotonic() - monotonic_start, 3)
        cpu_delta = round(time.process_time() - process_start, 3)
        io_delta = round(monotonic_delta - cpu_delta, 3)

        # Nothing to report
        if io_delta <= self.io_threshold and cpu_delta <= self.cpu_threshold:
            return

        # Report either a blocking IO or a heavy CPU usage
        status = "Blocking IO" if io_delta > self.io_threshold else "Heavy CPU usage"
        stack_after = self.get_stacktrace(task)
        logger.warning(
            f"{status} detected",
            task_name=task.name,
            total_time=monotonic_delta,
            cpu_time=cpu_delta,
            io_time=io_delta,
            stack_before=stack_before,
            stack_after=stack_after,
        )
