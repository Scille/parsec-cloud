# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from functools import wraps
from typing import TypeVar, Callable, Awaitable
from parsec.core.backend_connection.exceptions import BackendNotAvailable

# Once PEP 612 is available, we'll be able to type `expose_cmds` as:
#
#     P = ParamSpec("P")
#     R = TypeVar("R")
#
#     def with_request(f: Callable[Concatenate[Transport, P], R]) -> Callable[P, R]:
#         @wraps(cmd)
#         async def wrapper(self, *args: P.args, **kwargs: P.kwargs) -> R:
#             [...]
#
# This will allow us to properly check the types of the arguments of all the `cmds` commands

R = TypeVar("R")


def expose_cmds(cmd: Callable[..., Awaitable[R]]) -> Callable[..., Awaitable[R]]:
    @wraps(cmd)
    async def wrapper(self, *args: object, **kwargs: object) -> R:
        async with self.acquire_transport() as transport:
            return await cmd(transport, *args, **kwargs)

    return wrapper


def expose_cmds_with_retrier(cmd: Callable[..., Awaitable[R]]) -> Callable[..., Awaitable[R]]:
    @wraps(cmd)
    async def wrapper(self, *args: object, **kwargs: object) -> R:
        # Reusing the transports expose us to `BackendNotAvaiable` exceptions
        # due to inactivity timeout while the transport was in the pool.
        try:
            async with self.acquire_transport(allow_not_available=True) as transport:
                return await cmd(transport, *args, **kwargs)

        except BackendNotAvailable:
            async with self.acquire_transport(force_fresh=True) as transport:
                return await cmd(transport, *args, **kwargs)

    return wrapper
