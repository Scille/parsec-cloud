# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, RevokedUserCertificate, authenticated_cmds
from parsec.events import EventUserRevoked
from tests.common import Backend, CoolorgRpcClients, RpcTransportError


async def test_authenticated_user_revoke_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.device_id.user_id,
    )

    expected_dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    expected_dump[coolorg.bob.device_id.user_id].is_revoked = True

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.user_revoke(
            revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.v4.user_revoke.RepOk()

        await spy.wait_event_occurred(
            EventUserRevoked(
                organization_id=coolorg.organization_id,
                user_id=coolorg.bob.device_id.user_id,
            )
        )

    dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    assert dump == expected_dump

    # Now Bob can no longer connect
    with pytest.raises(RpcTransportError) as raised:
        await coolorg.bob.ping(ping="hello")
    assert raised.value.rep.status_code == 461
