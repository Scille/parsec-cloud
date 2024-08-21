# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Awaitable, Callable

import pytest

from parsec._parsec import (
    DateTime,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    UserID,
    VlobID,
    authenticated_cmds,
)
from parsec.events import EventRealmCertificate
from tests.common import Backend, CoolorgRpcClients, patch_realm_role_certificate


@pytest.fixture
def alice_unshare_bob_certificate(coolorg: CoolorgRpcClients) -> RealmRoleCertificate:
    return RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.bob.user_id,
        role=None,
    )


@pytest.mark.parametrize("revoked_user", (False, True))
async def test_authenticated_realm_unshare_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    revoked_user: bool,
) -> None:
    if revoked_user:
        await backend.user.revoke_user(
            now=DateTime.now(),
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            author_verify_key=coolorg.alice.signing_key.verify_key,
            revoked_user_certificate=RevokedUserCertificate(
                author=coolorg.alice.device_id,
                timestamp=DateTime.now(),
                user_id=coolorg.bob.user_id,
            ).dump_and_sign(coolorg.alice.signing_key),
        )

    alice_unshare_bob_certificate = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.bob.user_id,
        role=None,
    )

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_unshare(
            realm_role_certificate=alice_unshare_bob_certificate.dump_and_sign(
                coolorg.alice.signing_key
            ),
        )
        assert rep == authenticated_cmds.v4.realm_unshare.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=alice_unshare_bob_certificate.timestamp,
                realm_id=alice_unshare_bob_certificate.realm_id,
                user_id=alice_unshare_bob_certificate.user_id,
                role_removed=True,
            )
        )

    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.user_id
    )
    assert isinstance(bob_realms, dict)
    assert coolorg.wksp1_id not in bob_realms


@pytest.mark.parametrize("kind", ("author_is_reader", "author_is_not_shared"))
async def test_authenticated_realm_unshare_author_not_allowed(
    coolorg: CoolorgRpcClients,
    alice_unshare_bob_certificate: RealmRoleCertificate,
    kind: str,
) -> None:
    match kind:
        case "author_is_reader":
            author = coolorg.bob
        case "author_is_not_shared":
            author = coolorg.mallory
        case unknown:
            assert False, unknown

    certif = patch_realm_role_certificate(
        alice_unshare_bob_certificate, author=author.device_id, user_id=coolorg.alice.user_id
    )
    rep = await author.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(author.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepAuthorNotAllowed()


async def test_authenticated_realm_unshare_realm_not_found(
    coolorg: CoolorgRpcClients,
    alice_unshare_bob_certificate: RealmRoleCertificate,
) -> None:
    bad_realm_id = VlobID.new()
    certif = patch_realm_role_certificate(alice_unshare_bob_certificate, realm_id=bad_realm_id)
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRealmNotFound()


async def test_authenticated_realm_unshare_recipient_not_found(
    coolorg: CoolorgRpcClients,
    alice_unshare_bob_certificate: RealmRoleCertificate,
) -> None:
    bad_recipient = UserID.new()
    certif = patch_realm_role_certificate(
        alice_unshare_bob_certificate, user_id=bad_recipient, timestamp=DateTime.now()
    )
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRecipientNotFound()


async def test_authenticated_realm_unshare_recipient_already_unshared(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_unshare_bob_certificate: RealmRoleCertificate,
) -> None:
    # 1) Use certificate to unshare Bob via the backend
    outcome = await backend.realm.unshare(
        alice_unshare_bob_certificate.timestamp,
        coolorg.organization_id,
        coolorg.alice.device_id,
        coolorg.alice.signing_key.verify_key,
        alice_unshare_bob_certificate.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmRoleCertificate)

    # 2) Try to unshare Bob again, now via the API
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=alice_unshare_bob_certificate.dump_and_sign(
            coolorg.alice.signing_key
        ),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRecipientAlreadyUnshared(
        last_realm_certificate_timestamp=alice_unshare_bob_certificate.timestamp
    )


@pytest.mark.parametrize("kind", ("dummy_certif", "invalid_role", "self_unshare"))
async def test_authenticated_realm_unshare_invalid_certificate(
    coolorg: CoolorgRpcClients, alice_unshare_bob_certificate: RealmRoleCertificate, kind: str
) -> None:
    match kind:
        case "dummy_certif":
            certif = b"<dummy>"
        case "invalid_role":
            certif = patch_realm_role_certificate(
                alice_unshare_bob_certificate, role=RealmRole.READER
            ).dump_and_sign(coolorg.alice.signing_key)
        case "self_unshare":
            certif = patch_realm_role_certificate(
                alice_unshare_bob_certificate, user_id=coolorg.alice.user_id
            ).dump_and_sign(coolorg.alice.signing_key)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif,
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepInvalidCertificate()


async def test_authenticated_realm_unshare_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    alice_unshare_bob_certificate: RealmRoleCertificate,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    certif = patch_realm_role_certificate(
        alice_unshare_bob_certificate, timestamp=timestamp_out_of_ballpark
    )
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(rep, authenticated_cmds.v4.realm_unshare.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.parametrize("timestamp_kind", ("same_timestamp", "previous_timestamp"))
async def test_authenticated_realm_unshare_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_kind: str,
    alice_generated_realm_wksp1_data: Callable[[DateTime], Awaitable[None]],
) -> None:
    # 0) Bob must become OWNER to be able to unshare with Alice

    t0 = DateTime.now().subtract(seconds=100)
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.bob.user_id,
        timestamp=t0,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.OWNER,
    )
    await backend.realm.share(
        now=t0,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )

    # 1) Create some data (e.g. certificate, vlob) with a given timestamp

    now = DateTime.now()
    match timestamp_kind:
        case "same_timestamp":
            realm_unshare_timestamp = now
        case "previous_timestamp":
            realm_unshare_timestamp = now.subtract(seconds=1)
        case unknown:
            assert False, unknown

    await alice_generated_realm_wksp1_data(now)

    # 2) Create realm unshare certificate where timestamp is clashing with the previous data

    certif = RealmRoleCertificate(
        author=coolorg.bob.device_id,
        timestamp=realm_unshare_timestamp,
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.alice.user_id,
        role=None,
    )
    rep = await coolorg.bob.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRequireGreaterTimestamp(
        strictly_greater_than=now
    )
