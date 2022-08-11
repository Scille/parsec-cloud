# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from pendulum import datetime

from parsec.crypto import PrivateKey
from parsec.api.protocol import InvitationDeletedReason, InvitationToken

from tests.common import real_clock_timeout
from tests.backend.common import (
    invite_1_greeter_wait_peer,
    invite_2a_greeter_get_hashed_nonce,
    invite_2b_greeter_send_nonce,
    invite_3a_greeter_wait_peer_trust,
    invite_3b_greeter_signify_trust,
    invite_4_greeter_communicate,
)


@pytest.mark.trio
@pytest.mark.parametrize("reason", ("deleted_invitation", "unknown_token"))
async def test_greeter_exchange_bad_access(alice, backend, alice_ws, reason):
    if reason == "deleted_invitation":
        invitation = await backend.invite.new_for_device(
            organization_id=alice.organization_id, greeter_user_id=alice.user_id
        )
        await backend.invite.delete(
            organization_id=alice.organization_id,
            greeter=alice.user_id,
            token=invitation.token,
            on=datetime(2000, 1, 2),
            reason=InvitationDeletedReason.ROTTEN,
        )
        token = invitation.token
        status = "already_deleted"
    else:
        assert reason == "unknown_token"
        token = InvitationToken.new()
        status = "not_found"

    greeter_privkey = PrivateKey.generate()
    for command, params in [
        (
            invite_1_greeter_wait_peer,
            {"token": token, "greeter_public_key": greeter_privkey.public_key},
        ),
        (invite_2a_greeter_get_hashed_nonce, {"token": token}),
        (invite_2b_greeter_send_nonce, {"token": token, "greeter_nonce": b"<greeter_nonce>"}),
        (invite_3a_greeter_wait_peer_trust, {"token": token}),
        (invite_3b_greeter_signify_trust, {"token": token}),
        (invite_4_greeter_communicate, {"token": token, "payload": b"<payload>"}),
    ]:
        async with real_clock_timeout():
            rep = await command(alice_ws, **params)
        assert rep == {"status": status}
