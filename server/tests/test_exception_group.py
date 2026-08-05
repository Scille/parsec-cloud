# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import anyio
import pytest


# Simple task function to raise the exception received
async def _raise(exc: Exception) -> None:
    raise exc


async def test_create_task_group_exists() -> None:
    async with anyio.create_task_group():
        pass


async def test_create_task_group_exception_group() -> None:
    # A single exception should not "collapse", it should be wrapped in an ExceptionGroup (since anyio>=4)
    # See: https://anyio.readthedocs.io/en/stable/migration.html#task-groups-now-wrap-single-exceptions-in-groups
    with pytest.raises(ExceptionGroup) as exc:
        async with anyio.create_task_group():
            await _raise(ZeroDivisionError(1, 2, 3))

    assert not isinstance(exc, BaseExceptionGroup)
    assert len(exc.value.exceptions) == 1
    assert isinstance(exc.value.exceptions[0], ZeroDivisionError)
    assert exc.value.exceptions[0].args, (1, 2, 3)

    # Multiple exceptions should be wrapped in an ExceptionGroup
    with pytest.raises(ExceptionGroup) as exc:
        async with anyio.create_task_group() as tg:
            tg.start_soon(_raise, RuntimeError())  # will raise RuntimeError
            await _raise(ZeroDivisionError(1, 2, 3))  # will raise ZeroDivisionError

    assert not isinstance(exc, BaseExceptionGroup)
    assert len(exc.value.exceptions) == 2
    zero_division_error, runtime_error = exc.value.exceptions
    assert isinstance(runtime_error, RuntimeError)
    assert isinstance(zero_division_error, ZeroDivisionError)
    assert zero_division_error.args, (1, 2, 3)


async def test_create_task_group_exception_group_with_cancelled() -> None:
    # Cancel scope after raise in a task group should result in an ExceptionGroup
    with pytest.raises(ExceptionGroup) as exc:
        async with anyio.create_task_group() as tg:
            tg.start_soon(_raise, ZeroDivisionError(1, 2, 3))
            tg.cancel_scope.cancel()
            await anyio.sleep(1)

    assert not isinstance(exc, BaseExceptionGroup)
    assert len(exc.value.exceptions) == 1
    assert isinstance(exc.value.exceptions[0], ZeroDivisionError)
    assert exc.value.exceptions[0].args, (1, 2, 3)

    # Cancel scope after raise in a task group should also result in an ExceptionGroup
    with pytest.raises(ExceptionGroup) as exc:
        async with anyio.create_task_group() as tg:
            tg.start_soon(_raise, RuntimeError())
            tg.cancel_scope.cancel()
            raise ZeroDivisionError(1, 2, 3)

    assert not isinstance(exc, BaseExceptionGroup)
    assert len(exc.value.exceptions) == 2
    zero_division_error, runtime_error = exc.value.exceptions
    assert isinstance(runtime_error, RuntimeError)
    assert isinstance(zero_division_error, ZeroDivisionError)
    assert zero_division_error.args, (1, 2, 3)
