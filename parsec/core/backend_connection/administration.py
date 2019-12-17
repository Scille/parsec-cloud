# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from async_generator import asynccontextmanager
from typing import Optional

from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection import cmds
from parsec.core.backend_connection.transport import connect


class BackendAdministrationCmds:
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

    organization_create = _expose_cmds("organization_create")
    organization_status = _expose_cmds("organization_status")
    organization_stats = _expose_cmds("organization_stats")


@asynccontextmanager
async def backend_administration_cmds_factory(
    addr: BackendOrganizationAddr, token: str, keepalive: Optional[int] = None
) -> BackendAdministrationCmds:
    """
    Raises:
        BackendConnectionError
    """
    transport = await connect(addr, administration_token=token, keepalive=keepalive)
    transport.logger = transport.logger.bind(auth="<administration>")
    transport_lock = trio.Lock()

    @asynccontextmanager
    async def _acquire_transport(**kwargs):
        async with transport_lock:
            yield transport

    try:
        yield BackendAdministrationCmds(addr, _acquire_transport)
    finally:
        await transport.aclose()
