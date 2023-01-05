# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import threading
from contextlib import asynccontextmanager
from inspect import iscoroutinefunction, signature
from typing import (
    Any,
    AsyncGenerator,
    Awaitable,
    Callable,
    Generic,
    OrderedDict,
    Tuple,
    TypeVar,
    cast,
)

import trio
import trio_typing
from exceptiongroup import BaseExceptionGroup
from PyQt5.QtCore import QObject, pyqtBoundSignal
from structlog import get_logger
from typing_extensions import ParamSpec

from parsec.core.fs import FSError
from parsec.core.mountpoint import MountpointError
from parsec.utils import open_service_nursery

P = ParamSpec("P")
R = TypeVar("R")

logger = get_logger()


JobResultSignal = Tuple[QObject, str]


class JobResultError(Exception):
    def __init__(self, status: str, origin: BaseException | None = None, **kwargs: Any) -> None:
        self.status = status
        self.origin = origin
        self.params = kwargs
        self.params["origin"] = self.origin
        super().__init__(status, kwargs)


class JobSchedulerNotAvailable(Exception):
    pass


class QtToTrioJob(Generic[R]):
    def __init__(
        self,
        fn: Callable[P, Awaitable[R]] | Callable[P, R],
        on_success: JobResultSignal | None,
        on_error: JobResultSignal | None,
        *args: P.args,
        **kwargs: P.kwargs,
    ) -> None:
        # `pyqtBoundSignal` (i.e. `pyqtSignal` connected to a QObject instance) doesn't
        # hold a strong reference on the QObject. Hence if the latter gets freed, calling
        # the signal will result in a segfault !
        # To avoid this, we should instead pass a tuple (<QObject instance>, <name of signal>).
        assert all(not isinstance(a, pyqtBoundSignal) for a in args)
        assert all(not isinstance(v, pyqtBoundSignal) for v in kwargs.values())
        assert not isinstance(on_success, pyqtBoundSignal)
        assert not isinstance(on_error, pyqtBoundSignal)

        self._on_success = on_success
        self._on_error = on_error
        self._fn = fn
        self._args = args
        self._kwargs = kwargs
        self.cancel_scope: None | trio.CancelScope = None
        self._started = trio.Event()
        self._done = threading.Event()
        self.status: None | str = None
        self.ret: None | R = None
        self.exc: None | trio.Cancelled | Exception | BaseExceptionGroup[
            trio.Cancelled
        ] | JobSchedulerNotAvailable = None

    def __str__(self) -> str:
        return f"{self._fn.__name__}"

    @property
    def arguments(self) -> OrderedDict[str, object]:
        bound_arguments = signature(self._fn).bind(*self._args, **self._kwargs)
        bound_arguments.apply_defaults()
        return bound_arguments.arguments

    def is_finished(self) -> bool:
        return self._done.is_set()

    def is_cancelled(self) -> bool:
        return self.status == "cancelled"

    def is_ok(self) -> bool:
        return self.status == "ok"

    def is_errored(self) -> bool:
        return self.status == "ko"

    def is_crashed(self) -> bool:
        return self.status == "crashed"

    async def __call__(
        self,
        *,
        task_status: trio_typing.TaskStatus[None] = trio.TASK_STATUS_IGNORED,
    ) -> None:
        assert self.status is None
        with trio.CancelScope() as self.cancel_scope:
            task_status.started()
            self._started.set()

            try:
                if not iscoroutinefunction(self._fn):
                    result = cast(R, self._fn(*self._args, **self._kwargs))
                else:
                    result = await cast(Awaitable[R], self._fn(*self._args, **self._kwargs))
                if isinstance(result, JobResultError):
                    self.set_exception(result)
                else:
                    self.set_result(result)

            except Exception as exc:
                self.set_exception(exc)

            except trio.Cancelled as exc:
                self.set_cancelled(exc)
                raise

            except BaseExceptionGroup as exc:
                cancelled_errors, other_exceptions = cast(
                    tuple[BaseExceptionGroup[trio.Cancelled], BaseExceptionGroup[trio.Cancelled]],
                    exc.split(trio.Cancelled),
                )
                if other_exceptions is not None:
                    self.set_exception(other_exceptions)
                else:
                    assert cancelled_errors is not None
                    self.set_cancelled(cancelled_errors)
                if cancelled_errors:
                    raise cancelled_errors

    def set_result(self, result: R) -> None:
        self.status = "ok"
        self.ret = result
        self._set_done()

    def set_cancelled(
        self, exc: trio.Cancelled | BaseExceptionGroup[trio.Cancelled] | JobSchedulerNotAvailable
    ) -> None:
        self.status = "cancelled"
        self.exc = exc
        self._set_done()

    def set_exception(self, exc: Exception | BaseExceptionGroup[trio.Cancelled]) -> None:
        if isinstance(exc, JobResultError):
            self.status = exc.status
            self.exc = exc
        elif isinstance(exc, (FSError, MountpointError)):
            self.status = "ko"
            self.exc = exc
        else:
            logger.exception("Uncatched error in Qt/trio job", exc_info=exc)
            wrapped = JobResultError("crashed", exc=exc, info=f"Unexpected error: {repr(exc)}")
            wrapped.__traceback__ = exc.__traceback__
            self.status = wrapped.status
            self.exc = wrapped
        self._set_done()

    def _set_done(self) -> None:
        self._done.set()
        signal = self._on_success if self.is_ok() else self._on_error
        if signal is not None:
            obj, name = signal
            getattr(obj, name).emit(self)

    def cancel(self) -> None:
        if self.cancel_scope:
            self.cancel_scope.cancel()


class QtToTrioJobScheduler:
    def __init__(self, nursery: trio.Nursery) -> None:
        self.nursery = nursery
        self._throttling_scheduled_jobs: dict[str, QtToTrioJob[Any]] = {}
        self._throttling_last_executed: dict[str, float] = {}

    def close(self) -> None:
        self.nursery.cancel_scope.cancel()

    def submit_throttled_job(
        self,
        throttling_id: str,
        delay: float,
        on_success: JobResultSignal | None,
        on_error: JobResultSignal | None,
        fn: Callable[P, Awaitable[R]] | Callable[P, R],
        *args: P.args,
        **kwargs: P.kwargs,
    ) -> QtToTrioJob[R]:
        """
        Throttle execution: immediately execute `fn` unless a job with a similar
        `throttling_id` it has been already executed in the last `delay` seconds,
        in which case we schedule the execution to occur when the delay is elapsed.
        Submitting a job with an already scheduled `throttling_id` will lead to
        a single execution of the last provided job parameters at the soonest delay.
        """

        async def _throttled_execute(
            job: QtToTrioJob[R],
            task_status: trio_typing.TaskStatus[None] = trio.TASK_STATUS_IGNORED,
        ) -> None:
            # Only modify `_throttling_scheduled_jobs` from the trio
            # thread to avoid concurrent access with the Qt thread
            # Note we might be overwriting another job here, it is fine given
            # we only want to execute the last scheduled one
            self._throttling_scheduled_jobs[throttling_id] = job
            task_status.started()

            # Sleep the throttle delay, if last execution occurred long enough
            # this will equivalent of doing `trio.sleep(0)`
            last_executed = self._throttling_last_executed.get(throttling_id, 0.0)
            await trio.sleep_until(last_executed + delay)

            # It is possible our job has been executed by another `_throttled_execute`
            # that had a shorter delay. In such case we have nothing more to do.
            last_executed = self._throttling_last_executed.get(throttling_id, 0.0)
            if last_executed + delay > trio.current_time():
                return
            job = self._throttling_scheduled_jobs.pop(throttling_id, None)
            if not job:
                return

            # Actually start the job execution
            self._throttling_last_executed[throttling_id] = trio.current_time()
            try:
                await self.nursery.start(job)
            except trio.BrokenResourceError:
                # Job scheduler has been closed, nothing more can be done
                pass

        # Create the job but don't execute it: we have to handle throttle first !
        job = QtToTrioJob(
            fn,
            on_success,
            on_error,
            *args,
            **kwargs,
        )
        # Mypy: `trio.Nursery` have the private attribute `_closed` that we rely on.
        # `trio.Nursery` currently don't provide any Api to know if a Nursery is closed.
        if self.nursery._closed:  # type: ignore[attr-defined]
            job.set_cancelled(JobSchedulerNotAvailable())
        else:
            self.nursery.start_soon(_throttled_execute, job)
        return job

    def submit_job(
        self,
        on_success: JobResultSignal | None,
        on_error: JobResultSignal | None,
        fn: Callable[P, Awaitable[R]] | Callable[P, R],
        *args: P.args,
        **kwargs: P.kwargs,
    ) -> QtToTrioJob[R]:
        job: QtToTrioJob[R] = QtToTrioJob(
            fn,
            on_success,
            on_error,
            *args,
            **kwargs,
        )
        # Mypy: `trio.Nursery` have the private attribute `_closed` that we rely on.
        # `trio.Nursery` currently don't provide any Api to know if a Nursery is closed.
        if self.nursery._closed:  # type: ignore[attr-defined]
            job.set_cancelled(JobSchedulerNotAvailable())
        else:
            self.nursery.start_soon(job)
        return job


@asynccontextmanager
async def run_trio_job_scheduler() -> AsyncGenerator[QtToTrioJobScheduler, None]:
    async with open_service_nursery() as nursery:
        yield QtToTrioJobScheduler(nursery)
