# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import Coroutine
from dataclasses import dataclass
from typing import Any, Protocol

import anyio
import pytest

from parsec._parsec import DateTime, InvitationToken, PrivateKey, authenticated_cmds
from parsec.components.invite import ConduitState
from parsec.events import EventEnrollmentConduit, EventInvitation
from tests.common import Backend, CoolorgRpcClients

Response = authenticated_cmds.v4.invite_1_greeter_wait_peer.Rep | None


class InviteStep(Protocol):
    def __call__(self, cancel_scope: anyio.CancelScope) -> Coroutine[Any, Any, None]:
        ...


class Config:
    private_key: PrivateKey
    invitation_token: InvitationToken

    def __init__(
        self, invitation_token: InvitationToken, private_key: PrivateKey | None = None
    ) -> None:
        self.private_key = private_key or PrivateKey.generate()
        self.invitation_token = invitation_token


@dataclass
class RunOrderConfig:
    first: InviteStep
    second: InviteStep
    rep: Response


class RunOrder(Protocol):
    def __call__(self, greeter_config: Config, claimer_config: Config) -> RunOrderConfig:
        ...


@pytest.fixture(params=["greeter_first", "claimer_first"])
def run_order(
    backend: Backend, coolorg: CoolorgRpcClients, request: pytest.FixtureRequest
) -> RunOrder:
    def _run_order(greeter_config: Config, claimer_config: Config) -> RunOrderConfig:
        config = RunOrderConfig(
            None,  # pyright: ignore[reportArgumentType]
            None,  # pyright: ignore[reportArgumentType]
            None,
        )

        async def claimer_step_1(cancel_scope: anyio.CancelScope) -> None:
            await backend.invite.conduit_exchange(
                organization_id=coolorg.organization_id,
                greeter=None,
                token=claimer_config.invitation_token,
                state=ConduitState.STATE_1_WAIT_PEERS,
                payload=claimer_config.private_key.public_key.encode(),
            )

        async def greeter_step_1(cancel_scope: anyio.CancelScope) -> None:
            nonlocal config
            config.rep = await coolorg.alice.invite_1_greeter_wait_peer(
                token=greeter_config.invitation_token,
                greeter_public_key=greeter_config.private_key.public_key,
            )
            cancel_scope.cancel()

        match request.param:
            case "greeter_first":
                config.first = greeter_step_1
                config.second = claimer_step_1
            case "claimer_first":
                config.first = claimer_step_1
                config.second = greeter_step_1
            case unknown:
                assert False, unknown

        return config

    return _run_order


async def test_invite_1_greeter_wait_peer_ok(
    run_order: RunOrder, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    claimer_private_key = PrivateKey.generate()
    invitation_token = coolorg.invited_alice_dev3.token

    config = run_order(
        greeter_config=Config(invitation_token=invitation_token),
        claimer_config=Config(invitation_token=invitation_token, private_key=claimer_private_key),
    )

    with backend.event_bus.spy() as spy:
        async with anyio.create_task_group() as tg:
            tg.start_soon(config.first, tg.cancel_scope)
            await spy.wait(EventEnrollmentConduit)

            await config.second(tg.cancel_scope)

    assert config.rep == authenticated_cmds.v4.invite_1_greeter_wait_peer.RepOk(
        claimer_public_key=claimer_private_key.public_key
    )


async def test_invitation_not_found(
    run_order: RunOrder, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    config = run_order(
        greeter_config=Config(invitation_token=InvitationToken.new()),
        claimer_config=Config(invitation_token=invitation_token),
    )

    with backend.event_bus.spy() as spy:
        async with anyio.create_task_group() as tg:
            tg.start_soon(config.first, tg.cancel_scope)
            await spy.wait(EventEnrollmentConduit)

            await config.second(tg.cancel_scope)

    assert config.rep == authenticated_cmds.v4.invite_1_greeter_wait_peer.RepInvitationNotFound()


async def test_invitation_deleted(
    run_order: RunOrder, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    config = run_order(
        greeter_config=Config(invitation_token=invitation_token),
        claimer_config=Config(invitation_token=invitation_token),
    )

    with backend.event_bus.spy() as spy:
        async with anyio.create_task_group() as tg:
            await backend.invite.cancel(
                now=DateTime.now(),
                organization_id=coolorg.organization_id,
                author=coolorg.alice.user_id,
                token=invitation_token,
            )

            tg.start_soon(config.first, tg.cancel_scope)
            await spy.wait(EventInvitation)

            await config.second(tg.cancel_scope)

    assert config.rep == authenticated_cmds.v4.invite_1_greeter_wait_peer.RepInvitationDeleted()
