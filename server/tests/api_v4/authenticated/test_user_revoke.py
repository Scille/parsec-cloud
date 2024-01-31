# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx
import pytest

from parsec._parsec import DateTime, RevokedUserCertificate, authenticated_cmds
from parsec.events import EventUserRevokedOrFrozen
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
            EventUserRevokedOrFrozen(
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


async def test_disconnect_sse(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.device_id.user_id,
    )

    async with coolorg.bob.events_listen() as bob_sse:
        # 1) Bob starts listening SSE
        rep = await bob_sse.next_event()  # Server always starts by returning a `ServerConfig` event

        # 2) Then Alice revokes Bob

        rep = await coolorg.alice.user_revoke(
            revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.v4.user_revoke.RepOk()

        # 3) Hence Bob gets disconnected...

        with pytest.raises(StopAsyncIteration):
            # Loop given the server might have send us some events before the freeze
            while True:
                await bob_sse.next_event()

    # 4) ...and cannot reconnect !

    rep = await coolorg.bob.raw_sse_connection()
    assert rep.status_code == 461
