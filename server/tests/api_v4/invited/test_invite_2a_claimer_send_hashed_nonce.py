# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import anyio
import pytest

from parsec._parsec import HashDigest, invited_cmds
from parsec.components.invite import ConduitState
from tests.common import Backend, CoolorgRpcClients
from tests.common.invite import pass_state_1_wait_peer

Response = invited_cmds.v4.invite_2a_claimer_send_hashed_nonce.Rep | None


@pytest.mark.parametrize("run_order", ("greeter_first", "claimer_first"))
async def test_ok(run_order: str, coolorg: CoolorgRpcClients, backend: Backend) -> None:
    rep: Response = None
    invitation_token = coolorg.invited_alice_dev3.token
    await pass_state_1_wait_peer(coolorg.invited_alice_dev3, coolorg.alice)

    async def claimer_step_2(cancel_scope: anyio.CancelScope) -> None:
        nonlocal rep
        rep = await coolorg.invited_alice_dev3.invite_2a_claimer_send_hashed_nonce(
            claimer_hashed_nonce=HashDigest.from_data(b"hello-world")
        )
        cancel_scope.cancel()

    async def greeter_step_2(cancel_scope: anyio.CancelScope) -> None:
        await backend.invite.conduit_exchange(
            organization_id=coolorg.organization_id,
            greeter=coolorg.alice.device_id.user_id,
            token=invitation_token,
            state=ConduitState.STATE_2_1_CLAIMER_HASHED_NONCE,
            payload=b"",
        )
        await backend.invite.conduit_exchange(
            organization_id=coolorg.organization_id,
            greeter=coolorg.alice.device_id.user_id,
            token=invitation_token,
            state=ConduitState.STATE_2_2_GREETER_NONCE,
            payload=b"hello-world",
        )

    match run_order:
        case "greeter_first":
            first = greeter_step_2
            second = claimer_step_2
        case "claimer_first":
            first = claimer_step_2
            second = greeter_step_2
        case unknown:
            assert False, unknown

    async with anyio.create_task_group() as tg:
        tg.start_soon(first, tg.cancel_scope)

        await second(tg.cancel_scope)

    assert rep == invited_cmds.v4.invite_2a_claimer_send_hashed_nonce.RepOk(
        greeter_nonce=b"hello-world"
    )


async def test_enrollment_wrong_state(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.invited_alice_dev3.invite_2a_claimer_send_hashed_nonce(
        claimer_hashed_nonce=HashDigest.from_data(b"hello-world")
    )

    assert rep == invited_cmds.v4.invite_2a_claimer_send_hashed_nonce.RepEnrollmentWrongState()
