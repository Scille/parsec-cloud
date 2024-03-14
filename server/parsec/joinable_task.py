# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
from typing import Any, Awaitable, Callable, Generic, TypeVar

import anyio
from anyio.abc import TaskGroup, TaskStatus

__all__ = ("JoinableTaskStatus", "start_joinable_task")


T = TypeVar("T")


class JoinableTaskStatus(Generic[T]):
    def __init__(self, task_status: TaskStatus[JoinableTaskStatus[T]]):
        # Internal state
        self._task_status = task_status
        self._cancel_scope: anyio.CancelScope | None = None
        self._started_value: T | None = None
        self._finished_event = anyio.Event()

    def _set_cancel_scope(self, scope: anyio.CancelScope) -> None:
        self._cancel_scope = scope

    def _set_finished(self) -> None:
        self._finished_event.set()

    # Trio-like methods

    def started(self, value: T | None = None) -> None:
        self._started_value = value
        self._task_status.started(self)

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
        await asyncio.sleep(0)  # Checkpoint

    async def cancel_and_join(self) -> None:
        self.cancel()
        await self.join()

    @classmethod
    async def wrap_task(
        cls,
        corofn: Callable[..., Awaitable[T]],
        *args: Any,
        task_status: TaskStatus[JoinableTaskStatus[T]] = anyio.TASK_STATUS_IGNORED,
    ) -> None:
        status = cls(task_status)
        try:
            with anyio.CancelScope() as cancel_scope:
                status._set_cancel_scope(cancel_scope)
                await corofn(*args, task_status=status)
        finally:
            status._set_finished()


async def start_joinable_task(
    task_group: TaskGroup,
    corofn: Callable[..., Awaitable[T]],
    *args: Any,
    name: str | None = None,
) -> JoinableTaskStatus[T]:
    """Equivalent to `TaskGroup.start` but return a `JoinableTaskStatus` object.

    This object can be used to cancel and/or join the task.
    It also contains the started value set by `task_status.started()`.
    """
    return await task_group.start(JoinableTaskStatus.wrap_task, corofn, *args, name=name)
