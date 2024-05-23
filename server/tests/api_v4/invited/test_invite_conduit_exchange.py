# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import InvitationStatus, invited_cmds
from tests.common import Backend, CoolorgRpcClients


@pytest.mark.parametrize("kind", ("step_0", "step_1", "step_1_and_last"))
async def test_invited_invite_conduit_exchange_ok(
    kind: str, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    match kind:
        case "step_0":
            # Only greeter does step 0

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 0>",
                last=False,
            )
            assert outcome is None

            step = 0
            expected_greeter_payload = b"<greeter payload step 0>"
            expected_last = False

        case "step_1":
            # Do step 0

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                claimer_payload=b"<claimer payload step 0>",
            )
            assert outcome is None  # Greeter hasn't answered yet

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 0>",
                last=False,
            )
            assert outcome == b"<claimer payload step 0>"

            # Only greeter does step 1

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=1,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 1>",
                last=False,
            )
            assert outcome is None

            step = 1
            expected_greeter_payload = b"<greeter payload step 1>"
            expected_last = False

        case "step_1_and_last":
            # Do step 0

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                claimer_payload=b"<claimer payload step 0>",
            )
            assert outcome is None  # Greeter hasn't answered yet

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 0>",
                last=False,
            )
            assert outcome == b"<claimer payload step 0>"

            # Only claimer does step 1

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=1,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 1>",
                last=True,
            )
            assert outcome is None

            step = 1
            expected_greeter_payload = b"<greeter payload step 1>"
            expected_last = True

        case unknown:
            assert False, unknown

    outcome = await coolorg.invited_alice_dev3.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        claimer_payload=b"<claimer payload>",
        step=step,
    )
    assert outcome == invited_cmds.v4.invite_conduit_exchange.RepOk(
        greeter_payload=expected_greeter_payload,
        last=expected_last,
    )

    invitations = await backend.invite.test_dump_all_invitations(
        organization_id=coolorg.organization_id
    )
    invitation = next(
        (
            x
            for x in invitations[coolorg.alice.user_id]
            if x.token == coolorg.invited_alice_dev3.token
        )
    )
    if expected_last:
        assert invitation.status == InvitationStatus.FINISHED
    else:
        assert invitation.status == InvitationStatus.IDLE


@pytest.mark.parametrize(
    "kind", ("step_0", "step_1", "reset", "reset_claimer_has_talked", "reset_greeter_has_talked")
)
async def test_invited_invite_conduit_exchange_greeter_payload_not_ready(
    kind: str, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    match kind:
        case "step_0":
            step = 0

        case "step_1":
            # Do step 0

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                claimer_payload=b"<claimer payload step 0>",
                step=0,
                token=coolorg.invited_alice_dev3.token,
            )
            assert outcome is None  # Greeter hasn't answered yet

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 0>",
                last=False,
            )
            assert outcome == b"<claimer payload step 0>"

            step = 1

        case "reset":
            # Do step 0 & 1
            for step in (0, 1):
                outcome = await backend.invite.claimer_conduit_exchange(
                    organization_id=coolorg.organization_id,
                    token=coolorg.invited_alice_dev3.token,
                    step=step,
                    claimer_payload=b"<claimer payload>",
                )
                assert outcome is None  # Greeter hasn't answered yet

                outcome = await backend.invite.greeter_conduit_exchange(
                    organization_id=coolorg.organization_id,
                    token=coolorg.invited_alice_dev3.token,
                    step=step,
                    greeter=coolorg.alice.device_id.user_id,
                    greeter_payload=b"<greeter payload>",
                    last=False,
                )
                assert outcome == b"<claimer payload>"

            step = 0

        case "reset_greeter_has_talked":
            # Do step 0

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                claimer_payload=b"<claimer payload step 0>",
            )
            assert outcome is None  # Greeter hasn't answered yet

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 0>",
                last=False,
            )
            assert outcome == b"<claimer payload step 0>"

            # Only greeter has done step 1

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=1,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 1>",
                last=False,
            )
            assert outcome is None

            step = 0

        case "reset_claimer_has_talked":
            # Do step 0

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                claimer_payload=b"<claimer payload step 0>",
            )
            assert outcome is None  # Greeter hasn't answered yet

            outcome = await backend.invite.greeter_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                greeter=coolorg.alice.device_id.user_id,
                greeter_payload=b"<greeter payload step 0>",
                last=False,
            )
            assert outcome == b"<claimer payload step 0>"

            # Only claimer has done step 1

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=1,
                claimer_payload=b"<claimer payload step 1>",
            )
            assert outcome is None

            step = 0

        case unknown:
            assert False, unknown

    outcome = await coolorg.invited_alice_dev3.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        step=step,
        claimer_payload=b"<claimer payload>",
    )
    assert outcome == invited_cmds.v4.invite_conduit_exchange.RepGreeterPayloadNotReady()


@pytest.mark.parametrize("kind", ("current_step", "old_step"))
async def test_invited_invite_conduit_exchange_claimer_payload_mismatch(
    kind: str, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Do step 0

    outcome = await backend.invite.claimer_conduit_exchange(
        organization_id=coolorg.organization_id,
        token=coolorg.invited_alice_dev3.token,
        step=0,
        claimer_payload=b"<claimer payload step 0>",
    )
    assert outcome is None  # Greeter hasn't answered yet

    outcome = await backend.invite.greeter_conduit_exchange(
        organization_id=coolorg.organization_id,
        token=coolorg.invited_alice_dev3.token,
        step=0,
        greeter=coolorg.alice.device_id.user_id,
        greeter_payload=b"<greeter payload step 0>",
        last=False,
    )
    assert outcome == b"<claimer payload step 0>"

    # Do step 1

    outcome = await backend.invite.claimer_conduit_exchange(
        organization_id=coolorg.organization_id,
        token=coolorg.invited_alice_dev3.token,
        step=1,
        claimer_payload=b"<claimer payload step 1>",
    )
    assert outcome is None  # Greeter hasn't answered yet

    outcome = await backend.invite.greeter_conduit_exchange(
        organization_id=coolorg.organization_id,
        token=coolorg.invited_alice_dev3.token,
        step=1,
        greeter=coolorg.alice.device_id.user_id,
        greeter_payload=b"<greeter payload step 1>",
        last=False,
    )
    assert outcome == b"<claimer payload step 1>"

    # Claimer do step 2...

    outcome = await backend.invite.claimer_conduit_exchange(
        organization_id=coolorg.organization_id,
        token=coolorg.invited_alice_dev3.token,
        step=2,
        claimer_payload=b"<claimer payload step 2>",
    )
    assert outcome is None  # Greeter hasn't answered yet

    # Now we retry a step with a mismatched payload

    match kind:
        case "current_step":
            step = 2
        case "old_step":
            step = 1
        case unknown:
            assert False, unknown

    outcome = await coolorg.invited_alice_dev3.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        step=step,
        claimer_payload=b"<other payload>",
    )
    assert outcome == invited_cmds.v4.invite_conduit_exchange.RepClaimerPayloadMismatch()


async def test_invited_invite_conduit_exchange_bad_step(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    outcome = await coolorg.invited_alice_dev3.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        step=1,
        claimer_payload=b"<claimer payload step 1>",
    )

    assert outcome == invited_cmds.v4.invite_conduit_exchange.RepBadStep()
