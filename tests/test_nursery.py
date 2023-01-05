# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio
from exceptiongroup import BaseExceptionGroup

import parsec

# Importing parsec is enough to add open_service_nursery to trio
parsec


@pytest.mark.trio
async def test_open_service_nursery_exists():
    async with trio.open_service_nursery():
        pass


@pytest.mark.trio
async def test_open_service_nursery_exception_group_collapse(caplog):
    async def _raise(exc):
        raise exc

    with pytest.raises(ZeroDivisionError) as ctx:
        async with trio.open_service_nursery():
            await _raise(ZeroDivisionError(1, 2, 3))

    exception = ctx.value
    assert isinstance(exception, ZeroDivisionError)
    assert exception.args == (1, 2, 3)

    assert not isinstance(exception, BaseExceptionGroup)

    with pytest.raises(ZeroDivisionError) as ctx:
        async with trio.open_service_nursery() as nursery:
            nursery.start_soon(_raise, RuntimeError())
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


@pytest.mark.trio
async def test_open_service_nursery_exception_group_with_cancelled():
    async def _raise(exc):
        raise exc

    with trio.CancelScope() as cancel_scope:
        async with trio.open_service_nursery() as nursery:
            nursery.start_soon(_raise, ZeroDivisionError(1, 2, 3))
            cancel_scope.cancel()
            await trio.sleep(1)

    with trio.CancelScope() as cancel_scope:
        async with trio.open_service_nursery() as nursery:
            nursery.start_soon(_raise, RuntimeError())
            cancel_scope.cancel()
            raise ZeroDivisionError(1, 2, 3)
