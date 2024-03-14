# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import anyio

from parsec._parsec import PrivateKey, authenticated_cmds, invited_cmds

from .client import AuthenticatedRpcClient, InvitedRpcClient


async def pass_state_1_wait_peer(
    claimer: InvitedRpcClient, greeter: AuthenticatedRpcClient
) -> None:
    """Advance the invitation process past the state 1 (wait peer)"""
    greeter_private_key = PrivateKey.generate()
    claimer_private_key = PrivateKey.generate()
    greeter_rep_send, greeter_rep_recv = anyio.create_memory_object_stream()
    claimer_rep_send, claimer_rep_recv = anyio.create_memory_object_stream()
    claimer_rep: invited_cmds.v4.invite_1_claimer_wait_peer.Rep | None = None
    greeter_rep: authenticated_cmds.v4.invite_1_greeter_wait_peer.Rep | None = None

    async def claimer_step_1():
        claimer_rep = await claimer.invite_1_claimer_wait_peer(
            claimer_public_key=claimer_private_key.public_key,
        )
        await claimer_rep_send.send(claimer_rep)

    async def greeter_step_1():
        greeter_rep = await greeter.invite_1_greeter_wait_peer(
            greeter_public_key=greeter_private_key.public_key, token=claimer.token
        )
        await greeter_rep_send.send(greeter_rep)

    async def wait_responses() -> (
        tuple[
            invited_cmds.v4.invite_1_claimer_wait_peer.Rep,
            authenticated_cmds.v4.invite_1_greeter_wait_peer.Rep,
        ]
    ):
        claimer_rep = await claimer_rep_recv.receive()
        greeter_rep = await greeter_rep_recv.receive()
        return (claimer_rep, greeter_rep)

    async with anyio.create_task_group() as tg:
        tg.start_soon(claimer_step_1)
        tg.start_soon(greeter_step_1)
        claimer_rep, greeter_rep = await wait_responses()

    assert claimer_rep == invited_cmds.v4.invite_1_claimer_wait_peer.RepOk(
        greeter_public_key=greeter_private_key.public_key
    )
    assert greeter_rep == authenticated_cmds.v4.invite_1_greeter_wait_peer.RepOk(
        claimer_public_key=claimer_private_key.public_key
    )
