# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import anyio
import pytest

from parsec._parsec import PrivateKey, invited_cmds
from parsec.components.invite import ConduitState
from parsec.events import EventEnrollmentConduit
from tests.common import Backend, CoolorgRpcClients
from tests.common.client import RpcTransportError
from tests.common.invite import patch_backend_for_timeout_during_conduit_listen


@pytest.mark.parametrize("first_to_run", ("greeter_first", "claimer_first"))
async def test_ok(first_to_run: str, coolorg: CoolorgRpcClients, backend: Backend) -> None:
    greeter_private_key = PrivateKey.generate()
    claimer_private_key = PrivateKey.generate()
    invitation_token = coolorg.invited_alice_dev3.token
    rep: invited_cmds.v4.invite_1_claimer_wait_peer.Rep | None = None

    async def claimer_step_1():
        nonlocal rep
        rep = await coolorg.invited_alice_dev3.invite_1_claimer_wait_peer(
            claimer_public_key=claimer_private_key.public_key,
        )

    async def greeter_step_1():
        await backend.invite.conduit_exchange(
            organization_id=coolorg.organization_id,
            greeter=coolorg.alice.device_id.user_id,
            token=invitation_token,
            state=ConduitState.STATE_1_WAIT_PEERS,
            payload=greeter_private_key.public_key.encode(),
        )

    match first_to_run:
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
            tg.start_soon(first)
            await spy.wait(EventEnrollmentConduit)

            await second()

    assert rep == invited_cmds.v4.invite_1_claimer_wait_peer.RepOk(
        greeter_public_key=greeter_private_key.public_key
    )


async def test_gateway_timeout_recover_before_peer_arrives(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    greeter_private_key = PrivateKey.generate()
    claimer_private_key = PrivateKey.generate()
    invitation_token = coolorg.invited_alice_dev3.token
    rep: invited_cmds.v4.invite_1_claimer_wait_peer.Rep | None = None

    # 1) Claimer arrives first, waits too long and gets a gateway timeout !

    patch_backend_for_timeout_during_conduit_listen(
        backend, state=ConduitState.STATE_1_WAIT_PEERS, is_greeter=False
    )

    with pytest.raises(RpcTransportError) as exc:
        await coolorg.invited_alice_dev3.invite_1_claimer_wait_peer(
            claimer_public_key=claimer_private_key.public_key,
        )
    assert exc.value.rep.status_code == 504

    # 2) Claimer reconnects before greeter arrives and hence can pretend its timeout never occured

    rep = None

    async def greeter_step_1() -> None:
        nonlocal rep
        rep = await coolorg.invited_alice_dev3.invite_1_claimer_wait_peer(
            claimer_public_key=claimer_private_key.public_key,
        )

    with backend.event_bus.spy() as spy:
        async with anyio.create_task_group() as tg:
            tg.start_soon(greeter_step_1)
            await spy.wait(EventEnrollmentConduit)

            await backend.invite.conduit_exchange(
                organization_id=coolorg.organization_id,
                greeter=coolorg.alice.device_id.user_id,
                token=invitation_token,
                state=ConduitState.STATE_1_WAIT_PEERS,
                payload=greeter_private_key.public_key.encode(),
            )

    assert rep == invited_cmds.v4.invite_1_claimer_wait_peer.RepOk(
        greeter_public_key=greeter_private_key.public_key
    )


async def test_gateway_timeout_recover_after_peer_arrives(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    greeter_private_key = PrivateKey.generate()
    claimer_private_key = PrivateKey.generate()
    invitation_token = coolorg.invited_alice_dev3.token
    rep: invited_cmds.v4.invite_1_claimer_wait_peer.Rep | None = None

    # 1) Claimer arrives first, waits too long and gets a gateway timeout !

    patch_backend_for_timeout_during_conduit_listen(
        backend, state=ConduitState.STATE_1_WAIT_PEERS, is_greeter=False
    )

    with pytest.raises(RpcTransportError) as exc:
        await coolorg.invited_alice_dev3.invite_1_claimer_wait_peer(
            claimer_public_key=claimer_private_key.public_key,
        )
    assert exc.value.rep.status_code == 504

    # 2) Then greeter arrives, talks, but have to wait for the claimer to come back

    with backend.event_bus.spy() as spy:
        async with anyio.create_task_group() as tg:

            async def greeter_step_1() -> None:
                await backend.invite.conduit_exchange(
                    organization_id=coolorg.organization_id,
                    greeter=coolorg.alice.device_id.user_id,
                    token=invitation_token,
                    state=ConduitState.STATE_1_WAIT_PEERS,
                    payload=greeter_private_key.public_key.encode(),
                )

            tg.start_soon(greeter_step_1)
            await spy.wait(EventEnrollmentConduit)

            # 3) Claimer reconnects and pretends its timeout never occured

            rep = await coolorg.invited_alice_dev3.invite_1_claimer_wait_peer(
                claimer_public_key=claimer_private_key.public_key,
            )
            assert rep == invited_cmds.v4.invite_1_claimer_wait_peer.RepOk(
                greeter_public_key=greeter_private_key.public_key
            )
