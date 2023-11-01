# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import anyio
import pytest

from tests.common import LogCaptureFixture


async def test_create_task_group_exists() -> None:
    async with anyio.create_task_group():
        pass


@pytest.mark.xfail(reason="Should let @vxgmichel investigate if the behavior is ok")
async def test_create_task_group_exception_group_collapse(caplog: LogCaptureFixture) -> None:
    async def _raise(exc: Exception) -> None:
        raise exc

    with pytest.raises(ZeroDivisionError) as ctx:
        async with anyio.create_task_group():
            await _raise(ZeroDivisionError(1, 2, 3))

    exception = ctx.value
    assert isinstance(exception, ZeroDivisionError)
    assert exception.args == (1, 2, 3)

    assert not isinstance(exception, BaseExceptionGroup)

    with pytest.raises(ZeroDivisionError) as ctx:
        async with anyio.create_task_group() as tg:
            tg.start_soon(_raise, RuntimeError())
            await _raise(ZeroDivisionError(1, 2, 3))

    caplog.assert_occurred_once("[warning  ] A BaseExceptionGroup has been detected [parsec.utils]")

    exception = ctx.value
    assert isinstance(exception, ZeroDivisionError)
    assert exception.args == (1, 2, 3)

    assert isinstance(exception, BaseExceptionGroup)
    assert len(exception.exceptions) == 2

    a, b = exception.exceptions
    assert isinstance(a, ZeroDivisionError)
    assert not isinstance(a, BaseExceptionGroup)
    assert isinstance(b, RuntimeError)
    assert not isinstance(b, BaseExceptionGroup)


@pytest.mark.xfail(reason="Should let @vxgmichel investigate if the behavior is ok")
async def test_create_task_group_exception_group_with_cancelled() -> None:
    async def _raise(exc: Exception) -> None:
        raise exc

    with anyio.CancelScope() as cancel_scope:
        async with anyio.create_task_group() as tg:
            tg.start_soon(_raise, ZeroDivisionError(1, 2, 3))
            cancel_scope.cancel()
            await anyio.sleep(1)

    with anyio.CancelScope() as cancel_scope:
        async with anyio.create_task_group() as tg:
            tg.start_soon(_raise, RuntimeError())
            cancel_scope.cancel()
            raise ZeroDivisionError(1, 2, 3)
