# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx
import pytest

from parsec._parsec import (
    DateTime,
    DeviceID,
    RevokedUserCertificate,
    authenticated_cmds,
)
from parsec.events import EventUserRevokedOrFrozen
from tests.api_v4.authenticated.test_user_create import (
    generate_new_mike_device_certificates,
    generate_new_mike_user_certificates,
)
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


async def test_authenticated_user_revoke_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.bob.device_id,
        timestamp=now,
        user_id=coolorg.alice.device_id.user_id,
    )

    expected_dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    expected_dump[coolorg.alice.device_id.user_id].is_revoked = False

    rep = await coolorg.bob.user_revoke(
        revoked_user_certificate=certif.dump_and_sign(coolorg.bob.signing_key)
    )
    assert rep == authenticated_cmds.v4.user_revoke.RepAuthorNotAllowed()

    dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    assert dump == expected_dump


async def test_authenticated_user_revoke_user_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=DeviceID("mike@new_dev").user_id,
    )
    rep = await coolorg.alice.user_revoke(
        revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.v4.user_revoke.RepUserNotFound()


async def test_authenticated_user_revoke_user_already_revoked(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    t1 = DateTime.now()
    certif1 = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        user_id=coolorg.bob.device_id.user_id,
    )

    outcome = await backend.user.revoke_user(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif1.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RevokedUserCertificate)

    t2 = DateTime.now()
    certif2 = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t2,
        user_id=coolorg.bob.device_id.user_id,
    )

    rep = await coolorg.alice.user_revoke(
        revoked_user_certificate=certif2.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.v4.user_revoke.RepUserAlreadyRevoked(
        last_common_certificate_timestamp=t1
    )


@pytest.mark.parametrize(
    "kind",
    (
        "revoked_user_certificate",
        "revoked_user_certificate_not_author_user_id",
        "revoked_user_certificate_author_device_mismatch",
    ),
)
async def test_authenticated_user_revoke_invalid_certificate(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    now = DateTime.now()

    match kind:
        case "revoked_user_certificate":
            certif = b"<dummy>"
        case "revoked_user_certificate_not_author_user_id":
            certif = RevokedUserCertificate(
                author=coolorg.bob.device_id,
                timestamp=now,
                user_id=coolorg.bob.device_id.user_id,
            ).dump_and_sign(coolorg.alice.signing_key)
        case "revoked_user_certificate_author_device_mismatch":
            certif = RevokedUserCertificate(
                author=DeviceID("alice@dev2"),
                timestamp=now,
                user_id=coolorg.bob.device_id.user_id,
            ).dump_and_sign(coolorg.alice.signing_key)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.user_revoke(revoked_user_certificate=certif)
    assert rep == authenticated_cmds.v4.user_revoke.RepInvalidCertificate()


async def test_authenticated_user_revoke_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    t0 = DateTime.now().subtract(seconds=3600)
    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t0,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.user_revoke(revoked_user_certificate=certif)
    assert isinstance(rep, authenticated_cmds.v4.user_revoke.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


@pytest.mark.parametrize("kind", ("same_timestamp", "previous_timestamp"))
async def test_authenticated_user_revoke_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    now = DateTime.now()
    match kind:
        case "same_timestamp":
            revoked_user_timestamp = now
        case "previous_timestamp":
            revoked_user_timestamp = now.subtract(seconds=1)
        case unknown:
            assert False, unknown

    # 1) Create a certificate in the organization

    user_certif, redacted_user_certif = generate_new_mike_user_certificates(coolorg.alice, now)
    device_certif, redacted_device_certif = generate_new_mike_device_certificates(
        coolorg.alice, now
    )

    await backend.user.create_user(
        organization_id=coolorg.organization_id,
        now=now,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_certificate=user_certif,
        device_certificate=device_certif,
        redacted_user_certificate=redacted_user_certif,
        redacted_device_certificate=redacted_device_certif,
    )

    # 2) Create revoke user certificate where timestamp is clashing
    #    with the previous certificate

    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=revoked_user_timestamp,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.user_revoke(revoked_user_certificate=certif)
    assert rep == authenticated_cmds.v4.user_revoke.RepRequireGreaterTimestamp(
        strictly_greater_than=now
    )
