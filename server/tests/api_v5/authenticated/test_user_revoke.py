# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import Awaitable, Callable

import pytest

from parsec._parsec import (
    DateTime,
    DeviceID,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    VlobID,
    authenticated_cmds,
)
from parsec.events import EventUserRevokedOrFrozen
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    RpcTransportError,
    bob_becomes_admin_and_changes_alice,
    generate_realm_role_certificate,
)
from tests.common.data import alice_gives_profile, wksp1_alice_gives_role


@pytest.mark.parametrize(
    "kind",
    (
        "revoked_is_not_part_of_any_realm",
        "revoked_is_part_of_one_realm_with_vlobs",
        "revoked_is_part_of_one_realm_without_vlobs",
        "revoked_is_part_of_multiple_realms_with_vlobs",
        "revoked_is_part_of_multiple_realms_without_vlobs",
        "revoked_used_to_be_part_of_realm_with_clashing_topic",
        "revoked_used_to_be_part_of_realm_with_clashing_vlob",
    ),
)
async def test_authenticated_user_revoke_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    revoked_timestamp = None
    to_revoke = coolorg.mallory

    match kind:
        case "revoked_is_not_part_of_any_realm":
            pass

        case "revoked_is_part_of_one_realm_with_vlobs":
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            realm_id = VlobID.new()
            certif = generate_realm_role_certificate(
                coolorg,
                author=coolorg.mallory.device_id,
                user_id=coolorg.mallory.user_id,
                role=RealmRole.OWNER,
                realm_id=realm_id,
            )
            outcome = await backend.realm.create(
                now=DateTime.now(),
                organization_id=coolorg.organization_id,
                author=coolorg.mallory.device_id,
                author_verify_key=coolorg.mallory.signing_key.verify_key,
                realm_role_certificate=certif.dump_and_sign(coolorg.mallory.signing_key),
            )
            assert isinstance(outcome, RealmRoleCertificate)

            now = DateTime.now()
            outcome = await backend.vlob.create(
                now=now,
                organization_id=coolorg.organization_id,
                author=coolorg.mallory.device_id,
                realm_id=realm_id,
                vlob_id=VlobID.new(),
                key_index=0,
                timestamp=now,
                blob=b"<dummy>",
            )
            assert outcome is None

        case "revoked_is_part_of_one_realm_without_vlobs":
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            realm_id = VlobID.new()
            certif = generate_realm_role_certificate(
                coolorg,
                author=coolorg.mallory.device_id,
                user_id=coolorg.mallory.user_id,
                role=RealmRole.OWNER,
                realm_id=realm_id,
            )
            outcome = await backend.realm.create(
                now=DateTime.now(),
                organization_id=coolorg.organization_id,
                author=coolorg.mallory.device_id,
                author_verify_key=coolorg.mallory.signing_key.verify_key,
                realm_role_certificate=certif.dump_and_sign(coolorg.mallory.signing_key),
            )
            assert isinstance(outcome, RealmRoleCertificate)

        case "revoked_is_part_of_multiple_realms_with_vlobs":
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            for _ in range(3):
                realm_id = VlobID.new()
                certif = generate_realm_role_certificate(
                    coolorg,
                    author=coolorg.mallory.device_id,
                    user_id=coolorg.mallory.user_id,
                    role=RealmRole.OWNER,
                    realm_id=realm_id,
                )
                outcome = await backend.realm.create(
                    now=DateTime.now(),
                    organization_id=coolorg.organization_id,
                    author=coolorg.mallory.device_id,
                    author_verify_key=coolorg.mallory.signing_key.verify_key,
                    realm_role_certificate=certif.dump_and_sign(coolorg.mallory.signing_key),
                )
                assert isinstance(outcome, RealmRoleCertificate)

                now = DateTime.now()
                outcome = await backend.vlob.create(
                    now=now,
                    organization_id=coolorg.organization_id,
                    author=coolorg.mallory.device_id,
                    realm_id=realm_id,
                    vlob_id=VlobID.new(),
                    key_index=0,
                    timestamp=now,
                    blob=b"<dummy>",
                )
                assert outcome is None

        case "revoked_is_part_of_multiple_realms_without_vlobs":
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            for _ in range(3):
                realm_id = VlobID.new()
                certif = generate_realm_role_certificate(
                    coolorg,
                    author=coolorg.mallory.device_id,
                    user_id=coolorg.mallory.user_id,
                    role=RealmRole.OWNER,
                    realm_id=realm_id,
                )
                outcome = await backend.realm.create(
                    now=DateTime.now(),
                    organization_id=coolorg.organization_id,
                    author=coolorg.mallory.device_id,
                    author_verify_key=coolorg.mallory.signing_key.verify_key,
                    realm_role_certificate=certif.dump_and_sign(coolorg.mallory.signing_key),
                )
                assert isinstance(outcome, RealmRoleCertificate)

        case "revoked_used_to_be_part_of_realm_with_clashing_topic":
            to_revoke = coolorg.bob
            await wksp1_alice_gives_role(
                coolorg, backend, recipient=coolorg.bob.user_id, new_role=None
            )
            revoked_timestamp = DateTime.now()
            await wksp1_alice_gives_role(
                coolorg, backend, recipient=coolorg.mallory.user_id, new_role=RealmRole.READER
            )

        case "revoked_used_to_be_part_of_realm_with_clashing_vlob":
            to_revoke = coolorg.bob
            await wksp1_alice_gives_role(
                coolorg, backend, recipient=coolorg.bob.user_id, new_role=None
            )
            revoked_timestamp = DateTime.now()
            now = revoked_timestamp.add(seconds=1)
            outcome = await backend.vlob.create(
                now=now,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                realm_id=coolorg.wksp1_id,
                vlob_id=VlobID.new(),
                key_index=1,
                timestamp=now,
                blob=b"<dummy>",
            )
            assert outcome is None

        case unknown:
            assert False, unknown

    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=revoked_timestamp or DateTime.now(),
        user_id=to_revoke.user_id,
    )

    expected_dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    expected_dump[to_revoke.user_id].revoked_on = certif.timestamp
    expected_topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    expected_topics.common = certif.timestamp

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.user_revoke(
            revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.latest.user_revoke.RepOk()

        await spy.wait_event_occurred(
            EventUserRevokedOrFrozen(
                organization_id=coolorg.organization_id,
                user_id=to_revoke.user_id,
            )
        )

    dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    assert dump == expected_dump
    topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    assert topics == expected_topics

    # Now check the revoked user can no longer connect
    with pytest.raises(RpcTransportError) as raised:
        await to_revoke.ping(ping="hello")
    assert raised.value.rep.status_code == 461


async def test_disconnect_sse(
    coolorg: CoolorgRpcClients,
) -> None:
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.user_id,
    )

    async with coolorg.bob.events_listen() as bob_sse:
        # 1) Bob starts listening SSE
        rep = await bob_sse.next_event()  # Server always starts by returning a `ServerConfig` event

        # 2) Then Alice revokes Bob

        rep = await coolorg.alice.user_revoke(
            revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.latest.user_revoke.RepOk()

        # 3) Hence Bob gets disconnected...

        with pytest.raises(StopAsyncIteration):
            # Loop given the server might have send us some events before the freeze
            while True:
                await bob_sse.next_event()

    # 4) ...and cannot reconnect !

    async with coolorg.bob.raw_sse_connection() as rep:
        assert rep.status_code == 461


@pytest.mark.parametrize(
    "kind",
    (
        "as_outsider",
        "as_standard",
        "no_longer_allowed",
    ),
)
async def test_authenticated_user_revoke_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    now = DateTime.now()
    match kind:
        case "as_outsider":
            certif = RevokedUserCertificate(
                author=coolorg.mallory.device_id,
                timestamp=now,
                user_id=coolorg.alice.user_id,
            )
            author = coolorg.mallory

        case "as_standard":
            certif = RevokedUserCertificate(
                author=coolorg.bob.device_id,
                timestamp=now,
                user_id=coolorg.alice.user_id,
            )
            author = coolorg.bob

        case "no_longer_allowed":
            await bob_becomes_admin_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_profile=UserProfile.STANDARD
            )
            certif = RevokedUserCertificate(
                author=coolorg.alice.device_id,
                timestamp=now,
                user_id=coolorg.bob.user_id,
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    rep = await author.user_revoke(
        revoked_user_certificate=certif.dump_and_sign(coolorg.bob.signing_key)
    )
    assert rep == authenticated_cmds.latest.user_revoke.RepAuthorNotAllowed()


async def test_authenticated_user_revoke_user_not_found(coolorg: CoolorgRpcClients) -> None:
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=UserID.new(),
    )
    rep = await coolorg.alice.user_revoke(
        revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.latest.user_revoke.RepUserNotFound()


async def test_authenticated_user_revoke_user_already_revoked(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    t1 = DateTime.now()
    certif1 = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        user_id=coolorg.bob.user_id,
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
        user_id=coolorg.bob.user_id,
    )

    rep = await coolorg.alice.user_revoke(
        revoked_user_certificate=certif2.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.latest.user_revoke.RepUserAlreadyRevoked(
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
                user_id=coolorg.bob.user_id,
            ).dump_and_sign(coolorg.alice.signing_key)
        case "revoked_user_certificate_author_device_mismatch":
            certif = RevokedUserCertificate(
                author=DeviceID.test_from_nickname("alice@dev2"),
                timestamp=now,
                user_id=coolorg.bob.user_id,
            ).dump_and_sign(coolorg.alice.signing_key)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.user_revoke(revoked_user_certificate=certif)
    assert rep == authenticated_cmds.latest.user_revoke.RepInvalidCertificate()


async def test_authenticated_user_revoke_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
) -> None:
    t0 = DateTime.now().subtract(seconds=3600)
    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t0,
        user_id=coolorg.bob.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.user_revoke(revoked_user_certificate=certif)
    assert isinstance(rep, authenticated_cmds.latest.user_revoke.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


@pytest.mark.parametrize("timestamp_kind", ("same_timestamp", "previous_timestamp"))
async def test_authenticated_user_revoke_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_kind: str,
    alice_generated_data: Callable[[DateTime], Awaitable[None]],
) -> None:
    # 0) Bob must become ADMIN to be able to revoke Alice

    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        new_profile=UserProfile.ADMIN,
        user_id=coolorg.bob.user_id,
        timestamp=DateTime.now(),
    )
    await backend.user.update_user(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )

    # 1) Alice creates some data (e.g. certificate, vlob) with a given timestamp

    now = DateTime.now()
    match timestamp_kind:
        case "same_timestamp":
            user_revoke_timestamp = now
        case "previous_timestamp":
            user_revoke_timestamp = now.subtract(seconds=1)
        case unknown:
            assert False, unknown

    await alice_generated_data(now)

    # 2) Bob revokes Alice, but at a timestamp that would make Alice's previous data invalid !

    certif = RevokedUserCertificate(
        author=coolorg.bob.device_id,
        timestamp=user_revoke_timestamp,
        user_id=coolorg.alice.user_id,
    ).dump_and_sign(coolorg.bob.signing_key)

    rep = await coolorg.bob.user_revoke(revoked_user_certificate=certif)
    assert rep == authenticated_cmds.latest.user_revoke.RepRequireGreaterTimestamp(
        strictly_greater_than=now
    )


async def test_authenticated_user_revoke_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        now = DateTime.now()
        certif = RevokedUserCertificate(
            author=coolorg.alice.device_id,
            timestamp=now,
            user_id=coolorg.bob.user_id,
        )
        await coolorg.alice.user_revoke(
            revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )

    await authenticated_http_common_errors_tester(do)
