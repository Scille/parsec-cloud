# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import anyio
import pytest

from parsec._parsec import HashDigest, InvitationToken, authenticated_cmds
from parsec.components.invite import ConduitState
from tests.common import Backend, CoolorgRpcClients
from tests.common.invite import pass_state_1_wait_peer

Response = authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.Rep | None


@pytest.mark.parametrize("run_order", ("greeter_first", "claimer_first"))
async def test_ok(run_order: str, coolorg: CoolorgRpcClients, backend: Backend) -> None:
    rep: Response = None
    invitation_token = coolorg.invited_alice_dev3.token
    await pass_state_1_wait_peer(coolorg.invited_alice_dev3, coolorg.alice)

    async def claimer_step_2a(cancel_scope: anyio.CancelScope):
        await backend.invite.conduit_exchange(
            organization_id=coolorg.organization_id,
            greeter=None,
            token=invitation_token,
            state=ConduitState.STATE_2_1_CLAIMER_HASHED_NONCE,
            payload=HashDigest.from_data(b"hello-world").digest,
        )

    async def greeter_step_2a(cancel_scope: anyio.CancelScope):
        nonlocal rep
        rep = await coolorg.alice.invite_2a_greeter_get_hashed_nonce(
            token=invitation_token,
        )
        cancel_scope.cancel()

    match run_order:
        case "greeter_first":
            first = greeter_step_2a
            second = claimer_step_2a
        case "claimer_first":
            first = claimer_step_2a
            second = greeter_step_2a
        case unknown:
            assert False, unknown

    async with anyio.create_task_group() as tg:
        tg.start_soon(first, tg.cancel_scope)

        await second(tg.cancel_scope)

    assert rep == authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.RepOk(
        claimer_hashed_nonce=HashDigest.from_data(b"hello-world")
    )


async def test_invitation_not_found(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_2a_greeter_get_hashed_nonce(
        token=InvitationToken.new(),
    )
    assert rep == authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.RepInvitationNotFound()


async def test_invitation_deleted(coolorg: CoolorgRpcClients) -> None:
    await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_2a_greeter_get_hashed_nonce(
        token=coolorg.invited_alice_dev3.token,
    )
    assert rep == authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.RepInvitationDeleted()


async def test_enrollment_wrong_state(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_2a_greeter_get_hashed_nonce(
        token=coolorg.invited_alice_dev3.token,
    )

    assert rep == authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.RepEnrollmentWrongState()
