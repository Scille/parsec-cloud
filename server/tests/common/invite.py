# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import anyio

from parsec._parsec import HashDigest, PrivateKey, authenticated_cmds, invited_cmds
from parsec.backend import Backend
from parsec.components.invite import ConduitState, InviteConduitExchangeBadOutcome

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


async def pass_state_2a_claimer_send_hashed_nonce(
    claimer: InvitedRpcClient, greeter: AuthenticatedRpcClient, backend: Backend
) -> None:
    """Advance the invitation process past the state 2a (greeter get hashed nonce)"""
    invitation_token = claimer.token
    claimer_hash_nonce = HashDigest.from_data(b"pass_state_2a-claimer-hello-world")
    await pass_state_1_wait_peer(claimer, greeter)

    async def claimer_step_2a():
        await backend.invite.conduit_exchange(
            organization_id=claimer.organization_id,
            greeter=None,
            token=invitation_token,
            state=ConduitState.STATE_2_1_CLAIMER_HASHED_NONCE,
            payload=claimer_hash_nonce.digest,
        )

    async def greeter_step_2a():
        return await greeter.invite_2a_greeter_get_hashed_nonce(
            token=invitation_token,
        )

    async with anyio.create_task_group() as tg:
        tg.start_soon(claimer_step_2a)
        greeter_rep = await greeter_step_2a()

    assert greeter_rep == authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.RepOk(
        claimer_hashed_nonce=claimer_hash_nonce
    )


async def pass_state_2_2_greeter_nonce(
    claimer: InvitedRpcClient, greeter: AuthenticatedRpcClient, backend: Backend
) -> None:
    """Advance the invitation process past the state 2-2 greeter send nonce"""
    invitation_token = claimer.token
    claimer_hashed_nonce = HashDigest.from_data(b"pass_state_2_2-claimer-hello-world")
    greeter_nonce = b"pass_state_2_2-greeter-hello-world"

    await pass_state_1_wait_peer(claimer, greeter)

    async def claimer_step_2_2():
        return await claimer.invite_2a_claimer_send_hashed_nonce(
            claimer_hashed_nonce,
        )

    async def greeter_step_2_2():
        rep = await greeter.invite_2a_greeter_get_hashed_nonce(invitation_token)
        assert rep == authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.RepOk(
            claimer_hashed_nonce
        )
        rep = await backend.invite.conduit_exchange(
            organization_id=claimer.organization_id,
            greeter=greeter.user_id,
            token=invitation_token,
            state=ConduitState.STATE_2_2_GREETER_NONCE,
            payload=greeter_nonce,
        )
        assert not isinstance(rep, InviteConduitExchangeBadOutcome)

    async with anyio.create_task_group() as tg:
        tg.start_soon(greeter_step_2_2)
        rep = await claimer_step_2_2()

    assert rep == invited_cmds.v4.invite_2a_claimer_send_hashed_nonce.RepOk(greeter_nonce)


async def pass_state_2_exchange_nonce(
    claimer: InvitedRpcClient, greeter: AuthenticatedRpcClient
) -> None:
    invitation_token = claimer.token
    claimer_nonce = b"pass_state_2-claimer-hello-world"
    claimer_hashed_nonce = HashDigest.from_data(claimer_nonce)
    greeter_nonce = b"pass_state_2-greeter-hello-world"

    greeter_rep_send, greeter_rep_recv = anyio.create_memory_object_stream()
    claimer_rep_send, claimer_rep_recv = anyio.create_memory_object_stream()

    await pass_state_1_wait_peer(claimer, greeter)

    async def claimer_step_2_nonce():
        rep = await claimer.invite_2a_claimer_send_hashed_nonce(claimer_hashed_nonce)
        assert rep == invited_cmds.v4.invite_2a_claimer_send_hashed_nonce.RepOk(greeter_nonce)
        rep = await claimer.invite_2b_claimer_send_nonce(claimer_nonce)
        await claimer_rep_send.send(rep)

    async def greeter_step_2_nonce():
        rep = await greeter.invite_2a_greeter_get_hashed_nonce(invitation_token)
        assert rep == authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.RepOk(
            claimer_hashed_nonce
        )
        rep = await greeter.invite_2b_greeter_send_nonce(invitation_token, greeter_nonce)
        await greeter_rep_send.send(rep)

    async def wait_responses() -> (
        tuple[
            invited_cmds.v4.invite_2b_claimer_send_nonce.Rep,
            authenticated_cmds.v4.invite_2b_greeter_send_nonce.Rep,
        ]
    ):
        claimer_rep = await claimer_rep_recv.receive()
        greeter_rep = await greeter_rep_recv.receive()
        return (claimer_rep, greeter_rep)

    async with anyio.create_task_group() as tg:
        tg.start_soon(claimer_step_2_nonce)
        tg.start_soon(greeter_step_2_nonce)
        claimer_rep, greeter_rep = await wait_responses()

    assert claimer_rep == invited_cmds.v4.invite_2b_claimer_send_nonce.RepOk()
    assert greeter_rep == authenticated_cmds.v4.invite_2b_greeter_send_nonce.RepOk(claimer_nonce)


async def pass_state_3a_claimer_signify_trust(
    claimer: InvitedRpcClient, greeter: AuthenticatedRpcClient
) -> None:
    invitation_token = claimer.token
    claimer_response = None
    await pass_state_2_exchange_nonce(claimer, greeter)

    async def claimer_step_3a():
        nonlocal claimer_response
        claimer_response = await claimer.invite_3a_claimer_signify_trust()

    async def greeter_step_3a():
        return await greeter.invite_3a_greeter_wait_peer_trust(invitation_token)

    async with anyio.create_task_group() as tg:
        tg.start_soon(claimer_step_3a)
        greeter_response = await greeter_step_3a()

    assert claimer_response == invited_cmds.v4.invite_3a_claimer_signify_trust.RepOk()
    assert greeter_response == authenticated_cmds.v4.invite_3a_greeter_wait_peer_trust.RepOk()
