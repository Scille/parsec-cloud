# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
from async_generator import asynccontextmanager
from typing import Optional, AsyncGenerator

from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection import cmds
from parsec.core.backend_connection.transport import apiv1_connect
from parsec.core.backend_connection.exceptions import BackendNotAvailable
from parsec.core.backend_connection.expose_cmds import expose_cmds
from parsec.api.protocol import APIV1_ANONYMOUS_CMDS


class APIV1_BackendAnonymousCmds:
    def __init__(self, addr, acquire_transport):
        self.addr = addr
        self.acquire_transport = acquire_transport

    user_claim = expose_cmds(cmds.user_claim)
    user_get_invitation_creator = expose_cmds(cmds.user_get_invitation_creator)
    device_claim = expose_cmds(cmds.device_claim)
    device_get_invitation_creator = expose_cmds(cmds.device_get_invitation_creator)
    organization_bootstrap = expose_cmds(cmds.organization_bootstrap)
    ping = expose_cmds(cmds.ping)


for cmd in APIV1_ANONYMOUS_CMDS:
    assert hasattr(APIV1_BackendAnonymousCmds, cmd)


@asynccontextmanager
async def apiv1_backend_anonymous_cmds_factory(
    addr: BackendOrganizationAddr, keepalive: Optional[int] = None
) -> AsyncGenerator[APIV1_BackendAnonymousCmds, None]:
    """
    Raises:
        BackendConnectionError
    """
    transport_lock = trio.Lock()
    transport = None
    closed = False

    async def _init_transport():
        nonlocal transport
        if not transport:
            if closed:
                raise trio.ClosedResourceError
            transport = await apiv1_connect(addr, keepalive=keepalive)
            transport.logger = transport.logger.bind(auth="<anonymous>")

    async def _destroy_transport():
        nonlocal transport
        if transport:
            await transport.aclose()
            transport = None

    @asynccontextmanager
    async def _acquire_transport(**kwargs):
        nonlocal transport

        async with transport_lock:
            await _init_transport()
            try:
                yield transport
            except BackendNotAvailable:
                await _destroy_transport()
                raise

    try:
        yield APIV1_BackendAnonymousCmds(addr, _acquire_transport)

    finally:
        async with transport_lock:
            closed = True
            await _destroy_transport()
