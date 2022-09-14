# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec.crypto import HashDigest

from parsec._parsec import (
    Invite1ClaimerWaitPeerRepInvalidState,
    Invite1GreeterWaitPeerRepInvalidState,
    Invite2aClaimerSendHashedNonceHashNonceRepInvalidState,
    Invite2aGreeterGetHashedNonceRepInvalidState,
    Invite2bClaimerSendNonceRepInvalidState,
    Invite2bClaimerSendNonceRepOk,
    Invite2bGreeterSendNonceRepInvalidState,
    Invite3aClaimerSignifyTrustRepInvalidState,
    Invite3aClaimerSignifyTrustRepOk,
    Invite3aGreeterWaitPeerTrustRepInvalidState,
    Invite3aGreeterWaitPeerTrustRepOk,
    Invite3bClaimerWaitPeerTrustRepInvalidState,
    Invite3bClaimerWaitPeerTrustRepOk,
    Invite3bGreeterSignifyTrustRepInvalidState,
    Invite3bGreeterSignifyTrustRepOk,
    Invite4ClaimerCommunicateRepInvalidState,
    Invite4GreeterCommunicateRepInvalidState,
)

INVALID_STATES_TYPES = [
    Invite1ClaimerWaitPeerRepInvalidState,
    Invite1GreeterWaitPeerRepInvalidState,
    Invite2aClaimerSendHashedNonceHashNonceRepInvalidState,
    Invite2aGreeterGetHashedNonceRepInvalidState,
    Invite2bClaimerSendNonceRepInvalidState,
    Invite2bGreeterSendNonceRepInvalidState,
    Invite3aClaimerSignifyTrustRepInvalidState,
    Invite3aGreeterWaitPeerTrustRepInvalidState,
    Invite3bClaimerWaitPeerTrustRepInvalidState,
    Invite3bGreeterSignifyTrustRepInvalidState,
    Invite4ClaimerCommunicateRepInvalidState,
    Invite4GreeterCommunicateRepInvalidState,
]


@pytest.mark.trio
@pytest.mark.parametrize("leader", ("claimer", "greeter"))
async def test_conduit_exchange_good(exchange_testbed, leader):
    tb = exchange_testbed

    # Step 1
    if leader == "greeter":
        await tb.send_order("greeter", "1_wait_peer")
        await tb.send_order("claimer", "1_wait_peer")
    else:
        await tb.send_order("claimer", "1_wait_peer")
        await tb.send_order("greeter", "1_wait_peer")
    greeter_rep = await tb.get_result("greeter")
    claimer_rep = await tb.get_result("claimer")
    assert greeter_rep.claimer_public_key == tb.claimer_privkey.public_key
    assert claimer_rep.greeter_public_key == tb.greeter_privkey.public_key

    # Step 2
    if leader == "greeter":
        await tb.send_order("greeter", "2a_get_hashed_nonce")
        await tb.send_order("claimer", "2a_send_hashed_nonce")
    else:
        await tb.send_order("claimer", "2a_send_hashed_nonce")
        await tb.send_order("greeter", "2a_get_hashed_nonce")

    greeter_rep = await tb.get_result("greeter")
    assert greeter_rep.claimer_hashed_nonce == HashDigest.from_data(b"<claimer_nonce>")
    await tb.send_order("greeter", "2b_send_nonce")

    claimer_rep = await tb.get_result("claimer")
    assert claimer_rep.greeter_nonce == b"<greeter_nonce>"
    await tb.send_order("claimer", "2b_send_nonce")

    greeter_rep = await tb.get_result("greeter")
    assert greeter_rep.claimer_nonce == b"<claimer_nonce>"
    claimer_rep = await tb.get_result("claimer")
    assert isinstance(claimer_rep, Invite2bClaimerSendNonceRepOk)

    # Step 3a
    if leader == "greeter":
        await tb.send_order("greeter", "3a_wait_peer_trust")
        await tb.send_order("claimer", "3a_signify_trust")
    else:
        await tb.send_order("claimer", "3a_signify_trust")
        await tb.send_order("greeter", "3a_wait_peer_trust")
    greeter_rep = await tb.get_result("greeter")
    assert isinstance(greeter_rep, Invite3aGreeterWaitPeerTrustRepOk)
    claimer_rep = await tb.get_result("claimer")
    assert isinstance(claimer_rep, Invite3aClaimerSignifyTrustRepOk)

    # Step 3b
    if leader == "greeter":
        await tb.send_order("greeter", "3b_signify_trust")
        await tb.send_order("claimer", "3b_wait_peer_trust")
    else:
        await tb.send_order("claimer", "3b_wait_peer_trust")
        await tb.send_order("greeter", "3b_signify_trust")
    greeter_rep = await tb.get_result("greeter")
    assert isinstance(greeter_rep, Invite3bGreeterSignifyTrustRepOk)
    claimer_rep = await tb.get_result("claimer")
    assert isinstance(claimer_rep, Invite3bClaimerWaitPeerTrustRepOk)

    # Step 4
    if leader == "greeter":
        await tb.send_order("greeter", "4_communicate", b"hello from greeter")
        await tb.send_order("claimer", "4_communicate", b"hello from claimer")
    else:
        await tb.send_order("claimer", "4_communicate", b"hello from claimer")
        await tb.send_order("greeter", "4_communicate", b"hello from greeter")
    greeter_rep = await tb.get_result("greeter")
    assert greeter_rep.payload == b"hello from claimer"
    claimer_rep = await tb.get_result("claimer")
    assert claimer_rep.payload == b"hello from greeter"

    if leader == "greeter":
        await tb.send_order("greeter", "4_communicate", b"")
        await tb.send_order("claimer", "4_communicate", b"")
    else:
        await tb.send_order("claimer", "4_communicate", b"")
        await tb.send_order("greeter", "4_communicate", b"")
    greeter_rep = await tb.get_result("greeter")
    assert greeter_rep.payload == b""
    claimer_rep = await tb.get_result("claimer")
    assert claimer_rep.payload == b""


@pytest.mark.trio
async def test_conduit_exchange_reset(exchange_testbed):
    tb = exchange_testbed

    def possibilities():
        for leader in ("claimer", "greeter"):
            for reset_step in ("2a", "2b_greeter", "2b_claimer", "3a", "3b", "4", "4'"):
                for reset_actor in ("claimer", "greeter"):
                    print(
                        f"=== leader={leader}, reset_step={reset_step}, reset_actor={reset_actor} ==="
                    )
                    yield leader, reset_step, reset_actor

    async def _send_twin_orders(leader, greeter_order, claimer_order):
        # Greeter and claimer each send an order at the same time
        if leader == "greeter":
            await tb.send_order("greeter", greeter_order)
            await tb.send_order("claimer", claimer_order)
        else:
            assert leader == "claimer"
            await tb.send_order("claimer", claimer_order)
            await tb.send_order("greeter", greeter_order)

    async def _reset_during_twin_orders(leader, reset_actor, greeter_order, claimer_order):
        # Greeter and claimer were supposed to each send an order at the
        # same time, but one of them send a reset instead

        if reset_actor == "greeter":
            greeter_order = "1_wait_peer"
        else:
            assert reset_actor == "claimer"
            claimer_order = "1_wait_peer"

        if leader == "greeter":
            await tb.send_order("greeter", greeter_order)
            await tb.send_order("claimer", claimer_order)
        else:
            assert leader == "claimer"
            await tb.send_order("claimer", claimer_order)
            await tb.send_order("greeter", greeter_order)

        if reset_actor == "greeter":
            claimer_rep = await tb.get_result("claimer")
            assert type(claimer_rep) in INVALID_STATES_TYPES
            await tb.send_order("claimer", "1_wait_peer")
        else:
            assert reset_actor == "claimer"
            greeter_rep = await tb.get_result("greeter")
            assert type(greeter_rep) in INVALID_STATES_TYPES
            await tb.send_order("greeter", "1_wait_peer")

        await tb.assert_ok_rep("greeter")
        await tb.assert_ok_rep("claimer")

    async def _reset_during_peer_order_alone(reset_actor):
        await tb.send_order(reset_actor, "1_wait_peer")
        if reset_actor == "greeter":
            claimer_rep = await tb.get_result("claimer")
            assert type(claimer_rep) in INVALID_STATES_TYPES
            await tb.send_order("claimer", "1_wait_peer")
        else:
            assert reset_actor == "claimer"
            greeter_rep = await tb.get_result("greeter")
            assert type(greeter_rep) in INVALID_STATES_TYPES
            await tb.send_order("greeter", "1_wait_peer")

        await tb.assert_ok_rep("greeter")
        await tb.assert_ok_rep("claimer")

    # Step 1
    await tb.send_order("greeter", "1_wait_peer")
    await tb.send_order("claimer", "1_wait_peer")
    await tb.assert_ok_rep("greeter")
    await tb.assert_ok_rep("claimer")

    for leader, reset_step, reset_actor in possibilities():

        # Step 2a
        if reset_step == "2a":
            await _reset_during_twin_orders(
                leader,
                reset_actor,
                greeter_order="2a_get_hashed_nonce",
                claimer_order="2a_send_hashed_nonce",
            )
            continue
        else:
            await _send_twin_orders(
                leader, greeter_order="2a_get_hashed_nonce", claimer_order="2a_send_hashed_nonce"
            )
        await tb.assert_ok_rep("greeter")

        # Step 2b
        if reset_step == "2b_greeter":
            await _reset_during_peer_order_alone("greeter")
            continue
        else:
            await tb.send_order("greeter", "2b_send_nonce")
        await tb.assert_ok_rep("claimer")
        if reset_step == "2b_claimer":
            await _reset_during_peer_order_alone("claimer")
            continue
        else:
            await tb.send_order("claimer", "2b_send_nonce")
        await tb.assert_ok_rep("claimer")
        await tb.assert_ok_rep("greeter")

        # Step 3a
        if reset_step == "3a":
            await _reset_during_twin_orders(
                leader,
                reset_actor,
                greeter_order="3a_wait_peer_trust",
                claimer_order="3a_signify_trust",
            )
            continue
        else:
            await _send_twin_orders(
                leader, greeter_order="3a_wait_peer_trust", claimer_order="3a_signify_trust"
            )
        await tb.assert_ok_rep("claimer")
        await tb.assert_ok_rep("greeter")

        # Step 3b
        if reset_step == "3b":
            await _reset_during_twin_orders(
                leader,
                reset_actor,
                greeter_order="3b_signify_trust",
                claimer_order="3b_wait_peer_trust",
            )
            continue
        else:
            await _send_twin_orders(
                leader, greeter_order="3b_signify_trust", claimer_order="3b_wait_peer_trust"
            )
        await tb.assert_ok_rep("claimer")
        await tb.assert_ok_rep("greeter")

        # Claimer reset while starting step 4
        if reset_step == "4":
            await _reset_during_twin_orders(
                leader, reset_actor, greeter_order="4_communicate", claimer_order="4_communicate"
            )
            continue
        else:
            await _send_twin_orders(
                leader, greeter_order="4_communicate", claimer_order="4_communicate"
            )
        await tb.assert_ok_rep("claimer")
        await tb.assert_ok_rep("greeter")

        # Step 4 can be run an arbitrary number of times
        if reset_step == "4'":
            await _reset_during_twin_orders(
                leader, reset_actor, greeter_order="4_communicate", claimer_order="4_communicate"
            )
            continue
        else:
            # No reset occured at all... `reset_step` must be wrong !
            assert False, reset_step


@pytest.mark.trio
async def test_change_connection_during_exchange(
    backend_asgi_app, backend_invited_ws_factory, backend_authenticated_ws_factory, exchange_testbed
):
    tb = exchange_testbed

    # Step 1
    await tb.send_order("greeter", "1_wait_peer")
    await tb.send_order("claimer", "1_wait_peer")
    await tb.assert_ok_rep("greeter")
    await tb.assert_ok_rep("claimer")

    # Step 2
    await tb.send_order("greeter", "2a_get_hashed_nonce")

    # Change claimer sock, don't close previous sock
    async with backend_invited_ws_factory(
        backend_asgi_app,
        organization_id=tb.organization_id,
        invitation_type=tb.invitation.TYPE,
        token=tb.invitation.token,
    ) as claimer_ws:

        tb.claimer_ws = claimer_ws
        await tb.send_order("claimer", "2a_send_hashed_nonce")

        await tb.assert_ok_rep("greeter")

        # Change greeter sock, don't close previous sock
        async with backend_authenticated_ws_factory(backend_asgi_app, tb.greeter) as greeter_ws:
            tb.greeter_ws = greeter_ws
            await tb.send_order("greeter", "2b_send_nonce")

            await tb.assert_ok_rep("claimer")
            await tb.send_order("claimer", "2b_send_nonce")

            await tb.assert_ok_rep("greeter")
            await tb.assert_ok_rep("claimer")

    # Close previously used claimer&greeter socks and continue with new ones
    # Step 3a
    async with backend_invited_ws_factory(
        backend_asgi_app,
        organization_id=tb.organization_id,
        invitation_type=tb.invitation.TYPE,
        token=tb.invitation.token,
    ) as claimer_ws:
        tb.claimer_ws = claimer_ws
        async with backend_authenticated_ws_factory(backend_asgi_app, tb.greeter) as greeter_ws:
            tb.greeter_ws = greeter_ws

            await tb.send_order("greeter", "3a_wait_peer_trust")
            await tb.send_order("claimer", "3a_signify_trust")
            await tb.assert_ok_rep("claimer")
            await tb.assert_ok_rep("greeter")

    # Don't need to finish the exchange
