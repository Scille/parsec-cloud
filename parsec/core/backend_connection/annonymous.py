# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from async_generator import asynccontextmanager
from typing import Optional

from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection import cmds
from parsec.core.backend_connection.transport import connect


class BackendAnonymousCmds:
    def __init__(self, addr, acquire_transport):
        self.addr = addr
        self.acquire_transport = acquire_transport

    def _expose_cmds(name):
        cmd = getattr(cmds, name)

        async def wrapper(self, *args, **kwargs):
            async with self.acquire_transport() as transport:
                return await cmd(transport, *args, **kwargs)

        wrapper.__name__ = name

        return wrapper

    ping = _expose_cmds("ping")

    organization_bootstrap = _expose_cmds("organization_bootstrap")

    user_get_invitation_creator = _expose_cmds("user_get_invitation_creator")
    user_claim = _expose_cmds("user_claim")

    device_get_invitation_creator = _expose_cmds("device_get_invitation_creator")
    device_claim = _expose_cmds("device_claim")


@asynccontextmanager
async def backend_anonymous_cmds_factory(
    addr: BackendOrganizationAddr, keepalive: Optional[int] = None
) -> BackendAnonymousCmds:
    """
    Raises:
        BackendConnectionError
    """

    transport = await connect(addr, keepalive=keepalive)
    transport.logger = transport.logger.bind(auth="<anonymous>")
    transport_lock = trio.Lock()

    @asynccontextmanager
    async def _acquire_transport(**kwargs):
        async with transport_lock:
            yield transport

    try:
        yield BackendAnonymousCmds(addr, _acquire_transport)
    finally:
        await transport.aclose()
