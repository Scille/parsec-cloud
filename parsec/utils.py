# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from typing import Any, AsyncIterator, Awaitable, Callable, Generic, TypeVar, cast

import attr
import trio
import trio_typing
from exceptiongroup import BaseExceptionGroup
from structlog import get_logger

from parsec import service_nursery
from parsec._parsec import DateTime
from parsec.monitoring import TaskMonitoringInstrument

__all__ = [
    "timestamps_in_the_ballpark",
    "start_task",
    "trio_run",
    "open_service_nursery",
]

logger = get_logger()

T = TypeVar("T")

# Those values are used by the backend to determine whether the client
# operates in an acceptable time window. A client early offset of
# 300 seconds means that a timestamp provided by the client cannot be
# higher than 300 seconds after the backend current time. A client late
# offset of 320 seconds means that a timestamp provided by the client
# cannot be lower than 320 seconds before the backend current time.
#
# Those values used to be higher (30 minutes), but an argument for
# decreasing this value is that client clocks are usually either
# fully synchronized (with NTP) or fully desynchronized (with an
# incorrect configuration). It's possible for a client clock to
# slowly drift over time if it lost access to an NTP but drifting
# an entire minute would take weeks or months.
#
# However, we observed that such a case could occur, where a client's
# clock was 72 seconds late while the offset was 50/70 seconds. The
# cause being setting the clock manually, which can easily cause drift
# of 1 or 2 minutes. Since it's impossible to predict such behavior,
# the offset has been changed from 50/70 to 300/320 seconds, which
# seems good enough to prevent such cases and is windows' standard
# tolerance for clock drifting.
#
# Note that those values also have to take into account the fact the
# that the timestamps being compared are not produced at the same
# moment. Typically:
# - The client generates a timestamp
# - The client generates a request including this timestamp
# - The client sends this request to the backend over the network
# - The backend receives and processes the request
# - The backend compares the timestamp to its current time
# The worse case scenario would be a slow client machine, a large
# request, a slow network connection and a busy server. Even
# in this scenario, a 10 seconds time difference is hardly
# imaginable on a properly functioning system.
#
# This is an argument for making this comparison asymmetrical: with no
# clock drift between client and server, communication latency makes data
# arriving to the backend always in the past. Hence we should be more
# forgiving of data in the past than in the future !
#
# A more radical check would be to not accept more that 10 seconds
# delay and 10 seconds shifting, yielding a 10/20 seconds window
# (10 seconds in advance or 20 seconds late). This would effectively
# reduce the previous 50/70 seconds time window by a factor of 4.
# This however seems unrealistic as the 50/70 window turned out
# too narrow.
#
# The ballpark client tolerance is the ratio applied to the offsets
# while performing the ballpark checks. We use an arbitrary value of
# 80% in order to make sure that a clock shift is caught during the
# handshake instead of being caught by another API call later on.

BALLPARK_CLIENT_EARLY_OFFSET = 300  # seconds
BALLPARK_CLIENT_LATE_OFFSET = 320  # seconds
BALLPARK_CLIENT_TOLERANCE = 0.8  # 80%
BALLPARK_ALWAYS_OK = False  # Useful for disabling ballpark checks in the tests


def timestamps_in_the_ballpark(
    client: DateTime,
    backend: DateTime,
    ballpark_client_early_offset: float = BALLPARK_CLIENT_EARLY_OFFSET,
    ballpark_client_late_offset: float = BALLPARK_CLIENT_LATE_OFFSET,
) -> bool:
    """
    Useful to compare signed message timestamp with the one stored by the
    backend.
    """
    if BALLPARK_ALWAYS_OK:
        return True
    seconds = backend - client
    return -ballpark_client_early_offset < seconds < ballpark_client_late_offset


# Task status


@attr.s
class TaskStatus(Generic[T]):
    # Internal state
    _trio_task_status: trio_typing.TaskStatus[TaskStatus[T]] = attr.ib()
    _cancel_scope: trio.CancelScope | None = attr.ib(default=None)
    _started_value: T | None = attr.ib(default=None)
    _finished_event: trio.Event = attr.ib(factory=trio.Event)

    def _set_cancel_scope(self, scope: trio.CancelScope) -> None:
        self._cancel_scope = scope

    def _set_finished(self) -> None:
        self._finished_event.set()

    # Trio-like methods

    def started(self, value: T | None = None) -> None:
        self._started_value = value
        self._trio_task_status.started(self)

    # Properties

    @property
    def cancel_called(self) -> bool:
        assert self._cancel_scope is not None
        return self._cancel_scope.cancel_called

    @property
    def finished(self) -> bool:
        return self._finished_event.is_set()

    @property
    def value(self) -> T | None:
        return self._started_value

    # Methods

    def cancel(self) -> None:
        assert self._cancel_scope is not None
        self._cancel_scope.cancel()

    async def join(self) -> None:
        await self._finished_event.wait()
        await trio.sleep(0)  # Checkpoint

    async def cancel_and_join(self) -> None:
        self.cancel()
        await self.join()

    @classmethod
    async def wrap_task(  # type: ignore[misc]
        cls,
        corofn: Callable[..., Awaitable[T]],
        *args: Any,
        task_status: trio_typing.TaskStatus[TaskStatus[T]] = trio.TASK_STATUS_IGNORED,
    ) -> None:
        status = cls(task_status)
        try:
            with trio.CancelScope() as cancel_scope:
                status._set_cancel_scope(cancel_scope)
                await corofn(*args, task_status=status)
        finally:
            status._set_finished()


async def start_task(
    nursery: trio.Nursery,
    corofn: Callable[..., Awaitable[T]],
    *args: Any,
    name: str | None = None,
) -> TaskStatus[T]:
    """Equivalent to nursery.start but return a TaskStatus object.

    This object can be used to cancel and/or join the task.
    It also contains the started value set by `task_status.started()`.
    """
    return await nursery.start(TaskStatus.wrap_task, corofn, *args, name=name)


def trio_run(
    async_fn: Callable[..., Awaitable[T]],
    *args: Any,
    use_asyncio: bool = False,
    monitor_tasks: bool = True,
) -> T:
    if use_asyncio:
        # trio_asyncio is an optional dependency
        import trio_asyncio

        return trio_asyncio.run(async_fn, *args)
    instruments = (TaskMonitoringInstrument(),) if monitor_tasks else ()
    return trio.run(async_fn, *args, instruments=instruments)


# BaseExceptionGroup handling


async def check_cancellation(exc: BaseExceptionGroup[Any]) -> None:
    # If the exception group contains a Cancelled(), that means a cancellation
    # has leaked outside of the nursery. This indicates the current scope
    # has likely been cancelled. In this case, there is no point in handling
    # both the cancelled and regular exceptions: simply check for cancellation
    # and let a new Cancelled() raise if necessary, discarding the original
    # exception.
    cancelled_errors, _ = exc.split(trio.Cancelled)
    if cancelled_errors is not None:
        await trio.lowlevel.checkpoint_if_cancelled()


E = TypeVar("E", bound=BaseException)


def collapse_exception_group(exception_group: BaseExceptionGroup[E]) -> Exception:
    # Pick the first exception as the reference exception
    pick = cast(Exception, exception_group.exceptions[0])
    try:
        # Craft a new a name to indicate the exception is collapsed
        name = f"{type(pick).__name__}AndFriends"
        # Init method should be ignored as it might get called by `BaseExceptionGroup([result])`
        attrs = {"__init__": lambda *args, **kwargs: None}
        # Craft the specific class, inheriting from BaseExceptionGroup
        cls: Any = type(name, (type(pick), BaseExceptionGroup), attrs)
        # Instantiate the instance
        result = cls()
        # Replicate the picked exception inner state
        result.__dict__.update(pick.__dict__)
        result.args = pick.args
        # Replicate the exception group inner state
        result._message = exception_group.message
        result._exceptions = exception_group.exceptions
        result.__cause__ = exception_group.__cause__
        result.__context__ = exception_group.__context__
        result.__traceback__ = exception_group.__traceback__
        # Suppress context, we do not want the collapsing to appear in the stacktrace
        result.__suppress_context__ = True
        return result
    except Exception:
        # Something went wrong while collapsing
        logger.exception("Could not create a collapsed exception")
        return pick


@asynccontextmanager
async def open_service_nursery() -> AsyncIterator[trio.Nursery]:
    """Open a service nursery.

    This nursery does not raise BaseExceptionGroup exceptions.
    Instead, it collapses the BaseExceptionGroup into a single exception.
    More precisely, the first exception of the BaseExceptionGroup is raised,
    patched with extra BaseExceptionGroup capabilities.
    """
    try:
        async with service_nursery.open_service_nursery_with_exception_group() as nursery:
            yield nursery
    except BaseExceptionGroup as exc:
        # Re-raise a Cancelled() if the exception contains a Cancelled()
        await check_cancellation(exc)
        # Collapse the BaseExceptionGroup into a single exception
        logger.warning("A BaseExceptionGroup has been detected")
        raise collapse_exception_group(exc)


async def cancel_and_checkpoint(scope: trio.CancelScope) -> None:
    scope.cancel()
    await trio.lowlevel.checkpoint_if_cancelled()


# Add it to trio
trio.open_service_nursery = open_service_nursery  # type: ignore[attr-defined]
trio.CancelScope.cancel_and_checkpoint = cancel_and_checkpoint  # type: ignore[attr-defined]
