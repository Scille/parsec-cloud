# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio
from quart.testing.connections import WebsocketDisconnectError

from parsec._parsec import DateTime
from parsec.api.protocol import InvitationDeletedReason
from parsec.backend.events import BackendEvent
from tests.common import real_clock_timeout


@pytest.mark.trio
@pytest.mark.parametrize("during_step", ("not_started", "1", "2a", "2b", "3a", "3b", "4"))
async def test_delete_invitation_while_claimer_connected(exchange_testbed, backend, during_step):
    tb = exchange_testbed

    async def _delete_invitation_and_assert_claimer_left(retrieve_previous_result):
        # Delete the invitation, claimer connection should be closed automatically
        with backend.event_bus.listen() as spy:
            await backend.invite.delete(
                organization_id=tb.organization_id,
                greeter=tb.greeter.user_id,
                token=tb.invitation.token,
                on=DateTime(2000, 1, 2),
                reason=InvitationDeletedReason.ROTTEN,
            )
            await spy.wait_with_timeout(BackendEvent.INVITE_STATUS_CHANGED)

        with pytest.raises(WebsocketDisconnectError):
            async with real_clock_timeout():
                if retrieve_previous_result:
                    await tb.get_result("claimer")
                # Even if we had to retrieve an existing result, it could have
                # been returned by backend before the invitation delete occured,
                # hence we must poll with additional requests not matter what.
                # On top of that claimer connection can take some time to be
                # closed, so we need a polling loop here.
                while True:
                    await tb.send_order("claimer", "invite_info")
                    rep = await tb.get_result("claimer")
                    # Invitation info are cached for the connection at handshake
                    # time, hence the command won't take into account the fact
                    # that the invitation has been deleted
                    assert rep["status"] == "ok"

    if during_step == "not_started":
        await _delete_invitation_and_assert_claimer_left(retrieve_previous_result=False)
        return

    # Step 1
    await tb.send_order("greeter", "1_wait_peer")
    await tb.send_order("claimer", "1_wait_peer")
    if during_step == "1":
        await _delete_invitation_and_assert_claimer_left(retrieve_previous_result=True)
        return
    await tb.assert_ok_rep("greeter")
    await tb.assert_ok_rep("claimer")

    # Step 2
    await tb.send_order("greeter", "2a_get_hashed_nonce")
    await tb.send_order("claimer", "2a_send_hashed_nonce")
    if during_step == "2a":
        await _delete_invitation_and_assert_claimer_left(retrieve_previous_result=True)
        return

    await tb.assert_ok_rep("greeter")
    await tb.send_order("greeter", "2b_send_nonce")

    await tb.assert_ok_rep("claimer")
    await tb.send_order("claimer", "2b_send_nonce")
    if during_step == "2b":
        await _delete_invitation_and_assert_claimer_left(retrieve_previous_result=True)
        return

    await tb.assert_ok_rep("greeter")
    await tb.assert_ok_rep("claimer")

    # Step 3a
    await tb.send_order("greeter", "3a_wait_peer_trust")
    await tb.send_order("claimer", "3a_signify_trust")
    if during_step == "3a":
        await _delete_invitation_and_assert_claimer_left(retrieve_previous_result=True)
        return
    await tb.assert_ok_rep("greeter")
    await tb.assert_ok_rep("claimer")

    # Step 3b
    await tb.send_order("greeter", "3b_signify_trust")
    await tb.send_order("claimer", "3b_wait_peer_trust")
    if during_step == "3b":
        await _delete_invitation_and_assert_claimer_left(retrieve_previous_result=True)
        return
    await tb.assert_ok_rep("greeter")
    await tb.assert_ok_rep("claimer")

    # Step 4
    await tb.send_order("greeter", "4_communicate", b"hello from greeter")
    await tb.send_order("claimer", "4_communicate", b"hello from claimer")
    if during_step == "4":
        await _delete_invitation_and_assert_claimer_left(retrieve_previous_result=True)
        return
    await tb.assert_ok_rep("greeter")
    await tb.assert_ok_rep("claimer")


@pytest.mark.trio
async def test_delete_invitation_then_claimer_action_before_backend_closes_connection(
    exchange_testbed, backend
):
    tb = exchange_testbed

    # Disable the callback responsible for closing the claimer's connection
    # on invitation deletion. This way we can test connection behavior
    # when the automatic closing takes time to be processed.
    backend.event_bus.mute(BackendEvent.INVITE_STATUS_CHANGED)

    await backend.invite.delete(
        organization_id=tb.organization_id,
        greeter=tb.greeter.user_id,
        token=tb.invitation.token,
        on=DateTime(2000, 1, 2),
        reason=InvitationDeletedReason.ROTTEN,
    )

    # No need to be in the correct exchange state here given checking
    # the invitation status should be the very first thing done
    for action in [
        # `invite_info` uses a cache populated during connection handshake
        # so it will fail this test. However this is ok given not touching the
        # db precisely makes it a read-only operation.
        "1_wait_peer",
        "2a_send_hashed_nonce",
        "2b_send_nonce",
        "3a_signify_trust",
        "3b_wait_peer_trust",
        "4_communicate",
    ]:
        # In theory we should also watch for `WebsocketDisconnectError`, but
        # `quart_trio.testing.connection` implementation seems a bit broken...
        with pytest.raises((WebsocketDisconnectError, trio.EndOfChannel)):
            async with real_clock_timeout():
                await tb.send_order("claimer", action)
                await tb.get_result("claimer")
