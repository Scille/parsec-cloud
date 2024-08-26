# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
import anyio
import pytest

from parsec._parsec import invited_cmds
from parsec.components.invite import (
    ConduitState,
    InviteConduitExchangeBadOutcome,
)
from parsec.events import EventEnrollmentConduit
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    pass_state_2_2_greeter_nonce,
)

Response = invited_cmds.v4.invite_2b_claimer_send_nonce.Rep | None


@pytest.mark.parametrize("run_order", ("greeter_first", "claimer_first"))
async def test_invited_invite_2b_claimer_send_nonce_ok(
    run_order: str, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep: Response = None

    await pass_state_2_2_greeter_nonce(coolorg.invited_alice_dev3, coolorg.alice, backend)

    async def claimer_step_2b(cancel_scope: anyio.CancelScope):
        nonlocal rep
        rep = await coolorg.invited_alice_dev3.invite_2b_claimer_send_nonce(
            claimer_nonce=b"claimer-hello-world",
        )
        cancel_scope.cancel()

    async def greeter_step_2b(cancel_scope: anyio.CancelScope):
        rep = await backend.invite.conduit_exchange(
            organization_id=coolorg.organization_id,
            greeter=coolorg.alice.user_id,
            token=coolorg.invited_alice_dev3.token,
            state=ConduitState.STATE_2_3_CLAIMER_NONCE,
            payload=b"",
        )
        assert not isinstance(rep, InviteConduitExchangeBadOutcome)

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

    assert rep == invited_cmds.v4.invite_2b_claimer_send_nonce.RepOk()


async def test_invited_invite_2b_claimer_send_nonce_enrollment_wrong_state(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_2b_claimer_send_nonce(
        claimer_nonce=b"claimer-hello-world",
    )
    assert rep == invited_cmds.v4.invite_2b_claimer_send_nonce.RepEnrollmentWrongState()


@pytest.mark.skip(
    reason="TODO: test complex to implemented and this API is soon-to-be-removed anyway..."
)
async def test_invited_invite_2b_claimer_send_nonce_http_common_errors(
    coolorg: CoolorgRpcClients, invited_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    pass
