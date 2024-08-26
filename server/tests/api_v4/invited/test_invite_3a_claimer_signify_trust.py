# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
import anyio
import pytest

from parsec._parsec import authenticated_cmds, invited_cmds
from tests.common import CoolorgRpcClients, HttpCommonErrorsTester, pass_state_2_exchange_nonce

Response = invited_cmds.v4.invite_3a_claimer_signify_trust.Rep | None


@pytest.mark.parametrize("run_order", ("greeter_first", "claimer_first"))
async def test_invited_invite_3a_claimer_signify_trust_ok(
    run_order: str, coolorg: CoolorgRpcClients
) -> None:
    rep: Response = None
    await pass_state_2_exchange_nonce(coolorg.invited_alice_dev3, coolorg.alice)

    async def claimer_step_3a(cancel_scope: anyio.CancelScope):
        nonlocal rep
        rep = await coolorg.invited_alice_dev3.invite_3a_claimer_signify_trust()
        cancel_scope.cancel()

    async def greeter_step_3a(cancel_scope: anyio.CancelScope):
        rep = await coolorg.alice.invite_3a_greeter_wait_peer_trust(
            coolorg.invited_alice_dev3.token
        )
        assert rep == authenticated_cmds.v4.invite_3a_greeter_wait_peer_trust.RepOk()

    match run_order:
        case "greeter_first":
            first = greeter_step_3a
            second = claimer_step_3a
        case "claimer_first":
            first = claimer_step_3a
            second = greeter_step_3a
        case unknown:
            assert False, unknown

    async with anyio.create_task_group() as tg:
        tg.start_soon(first, tg.cancel_scope)

        await second(tg.cancel_scope)

    assert rep == invited_cmds.v4.invite_3a_claimer_signify_trust.RepOk()


async def test_invited_invite_3a_claimer_signify_trust_enrollment_wrong_state(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_3a_claimer_signify_trust()
    assert rep == invited_cmds.v4.invite_3a_claimer_signify_trust.RepEnrollmentWrongState()


@pytest.mark.skip(
    reason="TODO: test complex to implemented and this API is soon-to-be-removed anyway..."
)
async def test_invited_invite_3a_claimer_signify_trust_http_common_errors(
    coolorg: CoolorgRpcClients, invited_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    pass
