# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import asyncio
import secrets

import anyio
import pytest

from parsec._parsec import HashDigest, PrivateKey, authenticated_cmds, invited_cmds
from parsec.components.memory.invite import MemoryInviteComponent
from parsec.components.postgresql.invite import ConduitState, PGInviteComponent
from tests.common import CoolorgRpcClients, RpcTransportError


# TODO: Should also add Shamir once implemented !
@pytest.mark.parametrize(
    "invitation",
    (
        "device",
        "user",
    ),
)
@pytest.mark.parametrize(
    "delay_state_4_claimer",  # see issue https://github.com/Scille/parsec-cloud/issues/7212
    (
        pytest.param(False, id="normal"),
        pytest.param(True, id="delay_state_4_claimer"),
    ),
)
async def test_device_enrollment(
    invitation: str,
    coolorg: CoolorgRpcClients,
    delay_state_4_claimer: bool,
    monkeypatch: pytest.MonkeyPatch,
):
    if delay_state_4_claimer:
        # The point here is to reproduce a rare race condition reported in issue #7212 that happens about ~1% of the time on the CI.
        # The following patch causes it to happen 100% of the time instead.
        # More precisely, it makes sure the operations happen in the following order:
        #
        # 1. State 4A greeter talk, registering `<from greeter 1>` payload
        # 2. State 4A claimer talk, registering `<from claimer 1>` payload
        # 3. State 4A greeter listen, updating state to 4B, resetting the payloads and returning `<from claimer 1>`
        # ** Here 4A claimer listen is delayed for some reason **
        # 4. State 4B greeter talk, registering `<from greeter 2>` and setting the `last_exchange` flag to `True`
        # 5. State 4A claimer listen, returning `<from greeter 1>` and `last_exchange` flag to `False`
        #
        # Due to a bug, step 5 used to return the `last_exchange` flag to `True` instead of `False`.

        def patch_conduit_listen(cls):
            greeter_last_listen = asyncio.Event()
            orignal_method = cls._conduit_listen

            async def patched_conduit_listen(self, now, ctx, *args, **kwargs):
                if (
                    not ctx.is_greeter
                    and ctx.state == ConduitState.STATE_4_COMMUNICATE
                    and ctx.payload == b"<from claimer 1>"
                ):
                    await greeter_last_listen.wait()
                if (
                    ctx.is_greeter
                    and ctx.state == ConduitState.STATE_4_COMMUNICATE
                    and ctx.payload == b"<from greeter 2>"
                ):
                    greeter_last_listen.set()
                return await orignal_method(self, now, ctx, *args, **kwargs)

            monkeypatch.setattr(cls, "_conduit_listen", patched_conduit_listen)

        patch_conduit_listen(PGInviteComponent)
        patch_conduit_listen(MemoryInviteComponent)

    match invitation:
        case "device":
            invitation_token = coolorg.invited_alice_dev3.token
            invited_rpc = coolorg.invited_alice_dev3
        case "user":
            invitation_token = coolorg.invited_zack.token
            invited_rpc = coolorg.invited_zack
        case unknown:
            assert False, unknown
    greeter_private_key = PrivateKey.generate()
    claimer_private_key = PrivateKey.generate()

    # Step 1
    print("Step 1")
    async with anyio.create_task_group() as tg:

        async def greeter_step_1() -> None:
            rep = await coolorg.alice.invite_1_greeter_wait_peer(
                token=invitation_token,
                greeter_public_key=greeter_private_key.public_key,
            )
            assert rep == authenticated_cmds.v4.invite_1_greeter_wait_peer.RepOk(
                claimer_public_key=claimer_private_key.public_key
            )

        async def claimer_step_1() -> None:
            rep = await invited_rpc.invite_1_claimer_wait_peer(
                claimer_public_key=claimer_private_key.public_key,
            )
            assert rep == invited_cmds.v4.invite_1_claimer_wait_peer.RepOk(
                greeter_public_key=greeter_private_key.public_key
            )

        tg.start_soon(greeter_step_1)
        tg.start_soon(claimer_step_1)

    # Step 2
    print("Step 2")
    claimer_nonce = secrets.token_bytes(64)
    claimer_hashed_nonce = HashDigest.from_data(claimer_nonce)
    greeter_nonce = secrets.token_bytes(64)
    async with anyio.create_task_group() as tg:

        async def greeter_step_2() -> None:
            rep = await coolorg.alice.invite_2a_greeter_get_hashed_nonce(
                token=invitation_token,
            )
            assert rep == authenticated_cmds.v4.invite_2a_greeter_get_hashed_nonce.RepOk(
                claimer_hashed_nonce=claimer_hashed_nonce
            )

            print("Step greeter 2b")
            rep = await coolorg.alice.invite_2b_greeter_send_nonce(
                token=invitation_token,
                greeter_nonce=greeter_nonce,
            )
            assert rep == authenticated_cmds.v4.invite_2b_greeter_send_nonce.RepOk(
                claimer_nonce=claimer_nonce
            )

        async def claimer_step_2() -> None:
            rep = await invited_rpc.invite_2a_claimer_send_hashed_nonce(
                claimer_hashed_nonce=claimer_hashed_nonce
            )
            assert rep == invited_cmds.v4.invite_2a_claimer_send_hashed_nonce.RepOk(
                greeter_nonce=greeter_nonce
            )

            print("Step claimer 2b")
            rep = await invited_rpc.invite_2b_claimer_send_nonce(
                claimer_nonce=claimer_nonce,
            )
            assert rep == invited_cmds.v4.invite_2b_claimer_send_nonce.RepOk()

        tg.start_soon(greeter_step_2)
        tg.start_soon(claimer_step_2)

    # Step 3
    print("Step 3")
    async with anyio.create_task_group() as tg:

        async def greeter_step_3() -> None:
            rep = await coolorg.alice.invite_3a_greeter_wait_peer_trust(
                token=invitation_token,
            )
            assert rep == authenticated_cmds.v4.invite_3a_greeter_wait_peer_trust.RepOk()

            print("Step greeter 3b")
            rep = await coolorg.alice.invite_3b_greeter_signify_trust(
                token=invitation_token,
            )
            assert rep == authenticated_cmds.v4.invite_3b_greeter_signify_trust.RepOk()

        async def claimer_step_3() -> None:
            rep = await invited_rpc.invite_3a_claimer_signify_trust()
            assert rep == invited_cmds.v4.invite_3a_claimer_signify_trust.RepOk()

            print("Step claimer 3b")
            rep = await invited_rpc.invite_3b_claimer_wait_peer_trust()
            assert rep == invited_cmds.v4.invite_3b_claimer_wait_peer_trust.RepOk()

        tg.start_soon(greeter_step_3)
        tg.start_soon(claimer_step_3)

    # Step 4
    print("Step 4")
    async with anyio.create_task_group() as tg:

        async def greeter_step_4() -> None:
            rep = await coolorg.alice.invite_4_greeter_communicate(
                token=invitation_token,
                payload=b"<from greeter 1>",
                last=False,
            )
            assert rep == authenticated_cmds.v4.invite_4_greeter_communicate.RepOk(
                payload=b"<from claimer 1>",
            )

            print("Step greeter 4 (last communicate)")
            rep = await coolorg.alice.invite_4_greeter_communicate(
                token=invitation_token,
                payload=b"<from greeter 2>",
                last=True,
            )
            assert rep == authenticated_cmds.v4.invite_4_greeter_communicate.RepOk(
                payload=b"<from claimer 2>",
            )

        async def claimer_step_4() -> None:
            rep = await invited_rpc.invite_4_claimer_communicate(
                payload=b"<from claimer 1>",
            )
            assert rep == invited_cmds.v4.invite_4_claimer_communicate.RepOk(
                payload=b"<from greeter 1>",
                last=False,
            )

            print("Step claimer 4 (last communicate)")
            rep = await invited_rpc.invite_4_claimer_communicate(
                payload=b"<from claimer 2>",
            )
            assert rep == invited_cmds.v4.invite_4_claimer_communicate.RepOk(
                payload=b"<from greeter 2>",
                last=True,
            )

        tg.start_soon(greeter_step_4)
        tg.start_soon(claimer_step_4)

    print("Enrollment done")

    # Bonus: make sure the conduit cannot be used

    rep = await coolorg.alice.invite_4_greeter_communicate(
        token=invitation_token,
        payload=b"<from greeter 2>",
        last=True,
    )
    assert rep == authenticated_cmds.v4.invite_4_greeter_communicate.RepInvitationDeleted()

    with pytest.raises(RpcTransportError) as exc:
        await invited_rpc.invite_4_claimer_communicate(
            payload=b"<from claimer 3>",
        )
    assert exc.value.rep.status_code == 410
