# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import InvitationStatus, InvitationToken, authenticated_cmds
from tests.common import Backend, CoolorgRpcClients


@pytest.mark.parametrize("kind", ("step_0", "step_1", "step_1_and_last"))
async def test_authenticated_invite_conduit_exchange_ok(
    kind: str, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    match kind:
        case "step_0":
            # Only claimer does step 0

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=0,
                claimer_payload=b"<claimer payload step 0>",
            )
            assert outcome is None

            step = 0
            last = False
            expected_claimer_payload = b"<claimer payload step 0>"

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

            # Only claimer does step 1

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=1,
                claimer_payload=b"<claimer payload step 1>",
            )
            assert outcome is None

            step = 1
            last = False
            expected_claimer_payload = b"<claimer payload step 1>"

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

            outcome = await backend.invite.claimer_conduit_exchange(
                organization_id=coolorg.organization_id,
                token=coolorg.invited_alice_dev3.token,
                step=1,
                claimer_payload=b"<claimer payload step 1>",
            )
            assert outcome is None

            step = 1
            last = True
            expected_claimer_payload = b"<claimer payload step 1>"

        case unknown:
            assert False, unknown

    outcome = await coolorg.alice.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        greeter_payload=b"<greeter payload>",
        step=step,
        last=last,
    )
    assert outcome == authenticated_cmds.v4.invite_conduit_exchange.RepOk(
        claimer_payload=expected_claimer_payload,
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
    if last:
        assert invitation.status == InvitationStatus.FINISHED
    else:
        assert invitation.status == InvitationStatus.IDLE


@pytest.mark.parametrize(
    "kind", ("step_0", "step_1", "reset", "reset_claimer_has_talked", "reset_greeter_has_talked")
)
async def test_authenticated_invite_conduit_exchange_claimer_payload_not_ready(
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

    outcome = await coolorg.alice.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        step=step,
        greeter_payload=b"<greeter payload>",
        last=False,
    )
    assert outcome == authenticated_cmds.v4.invite_conduit_exchange.RepClaimerPayloadNotReady()


@pytest.mark.parametrize("kind", ("current_step", "old_step", "param_last"))
async def test_authenticated_invite_conduit_exchange_greeter_payload_mismatch(
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

    # Greeter do step 2...

    outcome = await backend.invite.greeter_conduit_exchange(
        organization_id=coolorg.organization_id,
        token=coolorg.invited_alice_dev3.token,
        step=2,
        greeter=coolorg.alice.device_id.user_id,
        greeter_payload=b"<greeter payload step 2>",
        last=False,
    )
    assert outcome is None  # Claimer hasn't answered yet

    # Now we retry a step with a mismatched payload
    greeter_payload = b"<other payload>"
    last = False
    match kind:
        case "current_step":
            step = 2
        case "old_step":
            step = 1
        case "param_last":
            step = 1
            greeter_payload = b"<greeter payload step 1>"
            last = True
        case unknown:
            assert False, unknown

    outcome = await coolorg.alice.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        step=step,
        greeter_payload=greeter_payload,
        last=last,
    )
    assert outcome == authenticated_cmds.v4.invite_conduit_exchange.RepGreeterPayloadMismatch()


async def test_authenticated_invite_conduit_exchange_bad_step(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    outcome = await coolorg.alice.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        step=1,
        greeter_payload=b"<greeter payload step 1>",
        last=False,
    )

    assert outcome == authenticated_cmds.v4.invite_conduit_exchange.RepBadStep()


async def test_authenticated_invite_conduit_exchange_invitation_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    dummy_token = InvitationToken.new()

    outcome = await coolorg.alice.invite_conduit_exchange(
        token=dummy_token,
        step=0,
        greeter_payload=b"<greeter payload step 0>",
        last=False,
    )

    assert outcome == authenticated_cmds.v4.invite_conduit_exchange.RepInvitationNotFound()


async def test_authenticated_invite_conduit_exchange_invitation_deleted(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    outcome = await backend.invite.cancel(
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        token=coolorg.invited_alice_dev3.token,
        now=coolorg.alice.now_factory(),
    )
    assert outcome is None

    outcome = await coolorg.alice.invite_conduit_exchange(
        token=coolorg.invited_alice_dev3.token,
        step=0,
        greeter_payload=b"<greeter payload step 0>",
        last=False,
    )

    assert outcome == authenticated_cmds.v4.invite_conduit_exchange.RepInvitationDeleted()
