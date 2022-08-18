# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import trio
from contextlib import asynccontextmanager
from typing import Optional, AsyncGenerator

from parsec.api.protocol import INVITED_CMDS
from parsec.core.types import BackendInvitationAddr
from parsec.core.backend_connection import cmds
from parsec.core.backend_connection.transport import connect_as_invited
from parsec.core.backend_connection.exceptions import BackendNotAvailable
from parsec.core.backend_connection.expose_cmds import expose_cmds_with_retrier


class BackendInvitedCmds:
    def __init__(self, addr, acquire_transport):
        self.addr = addr
        self.acquire_transport = acquire_transport

    ping = expose_cmds_with_retrier(cmds.ping)
    invite_info = expose_cmds_with_retrier(cmds.invite_info)
    invite_1_claimer_wait_peer = expose_cmds_with_retrier(cmds.invite_1_claimer_wait_peer)
    invite_2a_claimer_send_hashed_nonce = expose_cmds_with_retrier(
        cmds.invite_2a_claimer_send_hashed_nonce
    )
    invite_2b_claimer_send_nonce = expose_cmds_with_retrier(cmds.invite_2b_claimer_send_nonce)
    invite_3a_claimer_signify_trust = expose_cmds_with_retrier(cmds.invite_3a_claimer_signify_trust)
    invite_3b_claimer_wait_peer_trust = expose_cmds_with_retrier(
        cmds.invite_3b_claimer_wait_peer_trust
    )
    invite_4_claimer_communicate = expose_cmds_with_retrier(cmds.invite_4_claimer_communicate)


for cmd in INVITED_CMDS:
    assert hasattr(BackendInvitedCmds, cmd)


@asynccontextmanager
async def backend_invited_cmds_factory(
    addr: BackendInvitationAddr, keepalive: Optional[int] = None
) -> AsyncGenerator[BackendInvitedCmds, None]:
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
            transport = await connect_as_invited(addr, keepalive=keepalive)
            transport.logger = transport.logger.bind(auth="<invited>")

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
        yield BackendInvitedCmds(addr, _acquire_transport)

    finally:
        async with transport_lock:
            closed = True
            await _destroy_transport()
