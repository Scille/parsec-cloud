# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import invited_cmds
from tests.common import Backend, CoolorgRpcClients


@pytest.mark.parametrize("first_to_run", ("greeter_first", "claimer_first"))
async def test_ok(first_to_run: str, coolorg: CoolorgRpcClients, backend: Backend) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    async def claimer_step() -> invited_cmds.v4.invite_exchange.Rep:
        return await coolorg.invited_alice_dev3.invite_exchange(
            step=0,
            claimer_payload=b"claimer_payload 0",
            reset_reason=invited_cmds.v4.invite_exchange.InviteExchangeResetReason.NORMAL,
        )

    async def greeter_step():
        await backend.invite.conduit_greeter_exchange(
            organization_id=coolorg.organization_id,
            token=invitation_token,
            step=0,
            greeter_payload=b"greeter_payload 0",
        )

    match first_to_run:
        case "greeter_first":
            await greeter_step()
            rep = await claimer_step()
            assert rep == invited_cmds.v4.invite_exchange.RepOk(
                greeter_payload=b"greeter_payload 0",
                last=False,
            )

        case "claimer_first":
            rep = await claimer_step()
            assert rep == invited_cmds.v4.invite_exchange.RepRetryNeeded()

        case unknown:
            assert False, unknown
