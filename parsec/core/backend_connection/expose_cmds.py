# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from functools import wraps
from typing import TYPE_CHECKING, Awaitable, Callable, TypeVar, Union

from typing_extensions import Concatenate, ParamSpec

from parsec.api.transport import Transport
from parsec.core.backend_connection import BackendNotAvailable

if TYPE_CHECKING:
    from parsec.core.backend_connection.authenticated import BackendAuthenticatedCmds
    from parsec.core.backend_connection.invited import BackendInvitedCmds

    T_BACKEND = Union[BackendInvitedCmds, BackendAuthenticatedCmds]

P = ParamSpec("P")
R = TypeVar("R")


def expose_cmds(
    cmd: Callable[Concatenate[Transport, P], Awaitable[R]]
) -> Callable[Concatenate[T_BACKEND, P], Awaitable[R]]:
    @wraps(cmd)
    async def wrapper(self: T_BACKEND, *args: P.args, **kwargs: P.kwargs) -> R:
        async with self.acquire_transport() as transport:
            return await cmd(transport, *args, **kwargs)

    return wrapper


def expose_cmds_with_retrier(
    cmd: Callable[Concatenate[Transport, P], Awaitable[R]]
) -> Callable[Concatenate[T_BACKEND, P], Awaitable[R]]:
    @wraps(cmd)
    async def wrapper(self: T_BACKEND, *args: P.args, **kwargs: P.kwargs) -> R:
        # Reusing the transports expose us to `BackendNotAvailable` exceptions
        # due to inactivity timeout while the transport was in the pool.
        try:
            async with self.acquire_transport(allow_not_available=True) as transport:
                return await cmd(transport, *args, **kwargs)

        except BackendNotAvailable:
            raise

    return wrapper
