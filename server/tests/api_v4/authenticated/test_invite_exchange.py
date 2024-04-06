# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import authenticated_cmds
from parsec.components.invite import (
    InviteConduitClaimerExchangeBadOutcome,
    InviteConduitExchangeResetReason,
)
from tests.common import Backend, CoolorgRpcClients


@pytest.mark.parametrize("first_to_run", ("greeter_first", "claimer_first"))
async def test_ok(first_to_run: str, coolorg: CoolorgRpcClients, backend: Backend) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    async def greeter_step(step) -> authenticated_cmds.v4.invite_exchange.Rep:
        return await coolorg.alice.invite_exchange(
            step=step,
            token=invitation_token,
            greeter_payload=f"greeter_payload {step}".encode(),
            reset_reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL,
            last=False,
        )

    async def claimer_step(step: int):
        await backend.invite.conduit_claimer_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=step,
            claimer_payload=f"claimer_payload {step}".encode(),
        )

    match first_to_run:
        case "greeter_first":
            rep = await greeter_step(step=0)
            assert rep == authenticated_cmds.v4.invite_exchange.RepRetryNeeded()

        case "claimer_first":
            await claimer_step(step=0)
            rep = await greeter_step(step=0)
            assert rep == authenticated_cmds.v4.invite_exchange.RepOk(
                claimer_payload=b"claimer_payload 0",
            )
            # Now we can go to the next step
            rep = await greeter_step(step=1)
            assert rep == authenticated_cmds.v4.invite_exchange.RepRetryNeeded()

        case unknown:
            assert False, unknown


@pytest.mark.parametrize("initial_step", (0, 1))
async def test_next_step_without_waiting_for_peer(
    initial_step: int, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    if initial_step == 1:
        # Do step 0
        await backend.invite.conduit_claimer_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=0,
            claimer_payload=b"claimer_payload 0",
        )
        await backend.invite.conduit_greeter_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=0,
            greeter_payload=b"greeter_payload 0",
        )
        next_step = 1
    else:
        next_step = 0

    rep = await coolorg.alice.invite_exchange(
        step=next_step,
        token=invitation_token,
        greeter_payload=b"greeter_payload next step",
        reset_reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL,
        last=False,
    )
    assert rep == authenticated_cmds.v4.invite_exchange.RepRetryNeeded()

    # Going to the next step is only allowed once we have received the peer's payload

    next_step += 1

    rep = await coolorg.alice.invite_exchange(
        step=next_step,
        token=invitation_token,
        greeter_payload=b"greeter_payload next step",
        reset_reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL,
        last=False,
    )
    assert rep == authenticated_cmds.v4.invite_exchange.RepEnrollmentWrongStep(
        reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL
    )


@pytest.mark.parametrize("current_step", (0, 1))
async def test_wrong_next_step(
    current_step: int, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    if current_step == 1:
        # Do step 0
        await backend.invite.conduit_claimer_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=0,
            claimer_payload=b"claimer_payload 0",
        )
        await backend.invite.conduit_greeter_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=0,
            greeter_payload=b"greeter_payload 0",
        )
        next_step = 1
    else:
        next_step = 0

    wrong_step = next_step + 1
    rep = await coolorg.alice.invite_exchange(
        step=wrong_step,
        token=invitation_token,
        greeter_payload=b"greeter_payload wrong step",
        reset_reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL,
        last=False,
    )
    assert rep == authenticated_cmds.v4.invite_exchange.RepEnrollmentWrongStep(
        reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL
    )


async def test_replay_previous_steps(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    # Do step 0, 1 and 2
    for step in range(3):
        rep = await backend.invite.conduit_claimer_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=step,
            claimer_payload=f"claimer_payload {step}".encode(),
        )
        # Claimer talks first at each step, hence it always gets a need retry
        assert rep == InviteConduitClaimerExchangeBadOutcome.RETRY_NEEDED

        rep = await backend.invite.conduit_greeter_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=step,
            greeter_payload=f"greeter_payload {step}".encode(),
        )
        assert rep == f"claimer_payload {step}".encode()

    # Replay steps with the same payload
    # Note we ignore step 0 given replaying it would cause a reset
    for step in range(1, 3):
        rep = await coolorg.alice.invite_exchange(
            step=step,
            token=invitation_token,
            greeter_payload=f"greeter_payload {step}".encode(),
            reset_reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL,
            last=False,
        )
        assert rep == authenticated_cmds.v4.invite_exchange.RepOk(
            claimer_payload=f"claimer_payload {step}".encode()
        )


async def test_replay_step_with_different_payload(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    # Do step 0

    rep = await backend.invite.conduit_claimer_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=0,
        claimer_payload=b"claimer_payload 0",
    )
    assert rep == InviteConduitClaimerExchangeBadOutcome.RETRY_NEEDED
    rep = await backend.invite.conduit_greeter_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=0,
        greeter_payload=b"greeter_payload 0",
    )
    assert rep == b"claimer_payload 0"

    # Greeter do step 1...

    await backend.invite.conduit_greeter_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=1,
        greeter_payload=b"greeter_payload 1",
    )

    # ...then redo it with a different payload !

    rep = await coolorg.alice.invite_exchange(
        step=1,
        token=invitation_token,
        greeter_payload=b"different greeter_payload 1",
        reset_reason=None,
        last=False,
    )
    assert rep == authenticated_cmds.v4.invite_exchange.RepEnrollmentWrongStep(
        reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL
    )


@pytest.mark.parametrize("overwrite", (False, True))
async def test_step_after_last_one(
    overwrite: bool, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    await backend.invite.conduit_greeter_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=0,
        greeter_payload=b"greeter_payload 0",
        last=True,
    )
    await backend.invite.conduit_claimer_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=0,
        claimer_payload=b"claimer_payload 0",
    )

    rep = await coolorg.alice.invite_exchange(
        step=1,
        token=invitation_token,
        greeter_payload=b"greeter_payload 1",
        reset_reason=None,
        # Not allowed to overwrite the last flag once it has been set
        last=overwrite,
    )
    assert rep == authenticated_cmds.v4.invite_exchange.RepEnrollmentWrongStep(
        reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL
    )


@pytest.mark.parametrize("claimer_has_done_step_0", (False, True))
async def test_reset_at_step_0(
    claimer_has_done_step_0: bool, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    # Do step 0

    await backend.invite.conduit_greeter_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=0,
        greeter_payload=b"greeter_payload 0",
        last=True,
    )

    if claimer_has_done_step_0:
        await backend.invite.conduit_claimer_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=0,
            claimer_payload=b"claimer_payload 0",
        )

    # Reset step 0

    rep = await coolorg.alice.invite_exchange(
        step=0,
        token=invitation_token,
        greeter_payload=b"new greeter_payload 0",
        reset_reason=authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL,
        last=False,
    )
    assert rep == authenticated_cmds.v4.invite_exchange.RepRetryNeeded()

    # Claimer can do a new step 0 with an arbitrary payload thanks to the reset
    rep = await backend.invite.conduit_claimer_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=0,
        claimer_payload=b"new claimer_payload 0",
    )
    assert rep == (b"new greeter_payload 0", False)


@pytest.mark.parametrize("reason", ("normal", "bad_sas_code"))
async def test_reset_after_step_0(
    reason: str, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    # Do step 0 & 1

    for step in range(2):
        await backend.invite.conduit_greeter_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=step,
            greeter_payload=f"greeter_payload {step}".encode(),
            last=True,
        )
        await backend.invite.conduit_claimer_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=step,
            claimer_payload=f"claimer_payload {step}".encode(),
        )

    # Reset instead of doing step 2
    match reason:
        case "normal":
            api_reset_reason = (
                authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL
            )
            reset_reason = InviteConduitExchangeResetReason.NORMAL
        case "bad_sas_code":
            api_reset_reason = (
                authenticated_cmds.v4.invite_exchange.InviteExchangeResetReason.BAD_SAS_CODE
            )
            reset_reason = InviteConduitExchangeResetReason.BAD_SAS_CODE
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.invite_exchange(
        step=0,
        token=invitation_token,
        # Note payload has stayed the same that in our previous step 0
        greeter_payload=b"greeter_payload 0",
        reset_reason=api_reset_reason,
        last=False,
    )
    assert rep == authenticated_cmds.v4.invite_exchange.RepRetryNeeded()

    # Now trying to do the step 2 is not possible

    rep = await backend.invite.conduit_claimer_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=2,
        claimer_payload=b"claimer_payload 2",
    )
    assert rep == reset_reason

    rep = await backend.invite.conduit_greeter_exchange(
        organization_id=coolorg.organization_id,
        token=invitation_token,
        step=2,
        greeter_payload=b"greeter_payload 2",
    )
    assert rep == reset_reason
