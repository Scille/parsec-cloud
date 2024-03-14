# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import anyio
import pytest

from parsec._parsec import InvitationToken, PrivateKey, authenticated_cmds
from parsec.components.invite import ConduitState
from parsec.events import EventEnrollmentConduit
from tests.common import Backend, CoolorgRpcClients

Response = authenticated_cmds.v4.invite_1_greeter_wait_peer.Rep | None


@pytest.mark.parametrize("run_order", ("greeter_first", "claimer_first"))
async def test_ok(run_order: str, coolorg: CoolorgRpcClients, backend: Backend) -> None:
    claimer_private_key = PrivateKey.generate()
    invitation_token = coolorg.invited_alice_dev3.token
    rep: Response = None

    async def claimer_step_1(cancel_scope: anyio.CancelScope) -> None:
        await backend.invite.conduit_exchange(
            organization_id=coolorg.organization_id,
            greeter=None,
            token=invitation_token,
            state=ConduitState.STATE_1_WAIT_PEERS,
            payload=claimer_private_key.public_key.encode(),
        )

    async def greeter_step_1(cancel_scope: anyio.CancelScope) -> None:
        nonlocal rep
        rep = await coolorg.alice.invite_1_greeter_wait_peer(
            token=invitation_token,
            greeter_public_key=PrivateKey.generate().public_key,
        )
        cancel_scope.cancel()

    match run_order:
        case "greeter_first":
            first = greeter_step_1
            second = claimer_step_1
        case "claimer_first":
            first = claimer_step_1
            second = greeter_step_1
        case unknown:
            assert False, unknown

    with backend.event_bus.spy() as spy:
        async with anyio.create_task_group() as tg:
            tg.start_soon(first, tg.cancel_scope)
            await spy.wait(EventEnrollmentConduit)

            await second(tg.cancel_scope)

    assert rep == authenticated_cmds.v4.invite_1_greeter_wait_peer.RepOk(
        claimer_public_key=claimer_private_key.public_key
    )


async def test_invitation_not_found(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_1_greeter_wait_peer(
        token=InvitationToken.new(),
        greeter_public_key=PrivateKey.generate().public_key,
    )

    assert rep == authenticated_cmds.v4.invite_1_greeter_wait_peer.RepInvitationNotFound()


async def test_invitation_deleted(coolorg: CoolorgRpcClients) -> None:
    await coolorg.alice.invite_cancel(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_1_greeter_wait_peer(
        token=coolorg.invited_alice_dev3.token,
        greeter_public_key=PrivateKey.generate().public_key,
    )

    assert rep == authenticated_cmds.v4.invite_1_greeter_wait_peer.RepInvitationDeleted()
