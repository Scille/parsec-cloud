# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import anyio
import pytest

from parsec._parsec import InvitationToken, authenticated_cmds
from parsec.components.invite import ConduitState
from parsec.events import EventEnrollmentConduit
from tests.common import Backend, CoolorgRpcClients
from tests.common.invite import pass_state_2a_claimer_send_hashed_nonce

Response = authenticated_cmds.v4.invite_2b_greeter_send_nonce.Rep | None


@pytest.mark.parametrize("run_order", ("greeter_first", "claimer_first"))
async def test_ok(run_order: str, coolorg: CoolorgRpcClients, backend: Backend) -> None:
    rep: Response = None
    invitation_token = coolorg.invited_alice_dev3.token
    await pass_state_2a_claimer_send_hashed_nonce(
        coolorg.invited_alice_dev3, coolorg.alice, backend
    )

    async def claimer_step_2b(cancel_scope: anyio.CancelScope):
        await backend.invite.conduit_exchange(
            organization_id=coolorg.organization_id,
            greeter=None,
            token=invitation_token,
            state=ConduitState.STATE_2_2_GREETER_NONCE,
            payload=b"",
        )
        await backend.invite.conduit_exchange(
            organization_id=coolorg.organization_id,
            greeter=None,
            token=invitation_token,
            state=ConduitState.STATE_2_3_CLAIMER_NONCE,
            payload=b"claimer-hello-world",
        )

    async def greeter_step_2b(cancel_scope: anyio.CancelScope):
        nonlocal rep
        rep = await coolorg.alice.invite_2b_greeter_send_nonce(
            token=invitation_token,
            greeter_nonce=b"greeter-hello-world",
        )
        cancel_scope.cancel()

    match run_order:
        case "greeter_first":
            first = greeter_step_2b
            second = claimer_step_2b
        case "claimer_first":
            first = claimer_step_2b
            second = greeter_step_2b
        case unknown:
            assert False, unknown

    with backend.event_bus.spy() as spy:
        async with anyio.create_task_group() as tg:
            tg.start_soon(first, tg.cancel_scope)
            await spy.wait(EventEnrollmentConduit)

            await second(tg.cancel_scope)

    assert rep == authenticated_cmds.v4.invite_2b_greeter_send_nonce.RepOk(
        claimer_nonce=b"claimer-hello-world"
    )


async def test_invitation_not_found(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_2b_greeter_send_nonce(
        token=InvitationToken.new(),
        greeter_nonce=b"greeter-hello-world",
    )

    assert rep == authenticated_cmds.v4.invite_2b_greeter_send_nonce.RepInvitationNotFound()


async def test_invitation_deleted(coolorg: CoolorgRpcClients) -> None:
    await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_2b_greeter_send_nonce(
        token=coolorg.invited_alice_dev3.token,
        greeter_nonce=b"greeter-hello-world",
    )

    assert rep == authenticated_cmds.v4.invite_2b_greeter_send_nonce.RepInvitationDeleted()


async def test_enrollment_wrong_state(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_2b_greeter_send_nonce(
        token=coolorg.invited_alice_dev3.token,
        greeter_nonce=b"greeter-hello-world",
    )

    assert rep == authenticated_cmds.v4.invite_2b_greeter_send_nonce.RepEnrollmentWrongState()
