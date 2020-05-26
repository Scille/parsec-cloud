# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.backend_connection import cmds
from parsec.core.backend_connection.exceptions import BackendNotAvailable


def expose_cmds(name: str, apiv1: bool = False):
    if apiv1:
        cmd = getattr(cmds, f"apiv1_{name}", None) or getattr(cmds, name)
    else:
        cmd = getattr(cmds, name)

    async def wrapper(self, *args, **kwargs):
        async with self.acquire_transport() as transport:
            return await cmd(transport, *args, **kwargs)

    wrapper.__name__ = name

    return wrapper


def expose_cmds_with_retrier(name: str, apiv1: bool = False):
    if apiv1:
        cmd = getattr(cmds, f"apiv1_{name}", None) or getattr(cmds, name)
    else:
        cmd = getattr(cmds, name)

    async def wrapper(self, *args, **kwargs):
        # Reusing the transports expose us to `BackendNotAvaiable` exceptions
        # due to inactivity timeout while the transport was in the pool.
        try:
            async with self.acquire_transport(allow_not_available=True) as transport:
                return await cmd(transport, *args, **kwargs)

        except BackendNotAvailable:
            async with self.acquire_transport(force_fresh=True) as transport:
                return await cmd(transport, *args, **kwargs)

    wrapper.__name__ = name

    return wrapper
