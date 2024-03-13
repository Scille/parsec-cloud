# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SecretKey,
    SecretKeyAlgorithm,
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


async def test_authenticated_realm_unshare_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_unshare_bob_certificate: RealmRoleCertificate,
) -> None:
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
        coolorg.organization_id, coolorg.bob.device_id.user_id
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


@pytest.mark.parametrize(
    "kind",
    (
        "user_unknown",
        pytest.param(
            "user_revoked",
            marks=pytest.mark.xfail(
                reason="TODO: Should fail if user to be unshared is revoked. Not implemented?"
            ),
        ),
    ),
)
async def test_authenticated_realm_unshare_recipient_not_found(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_unshare_bob_certificate: RealmRoleCertificate,
    kind: str,
) -> None:
    match kind:
        case "user_unknown":
            bad_recipient = UserID("unknown")
        case "user_revoked":
            bad_recipient = coolorg.bob.user_id
            # Revoke user Bob
            revoke_timestamp = DateTime.now()
            outcome = await backend.user.revoke_user(
                now=revoke_timestamp,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                revoked_user_certificate=RevokedUserCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=revoke_timestamp,
                    user_id=coolorg.bob.device_id.user_id,
                ).dump_and_sign(coolorg.alice.signing_key),
            )
            assert isinstance(outcome, RevokedUserCertificate)
        case unknown:
            assert False, unknown
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


@pytest.mark.parametrize(
    "timestamp_offset",
    (pytest.param(0, id="same_timestamp"), pytest.param(1, id="previous_timestamp")),
)
async def test_authenticated_realm_unshare_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_unshare_bob_certificate: RealmRoleCertificate,
    timestamp_offset: int,
) -> None:
    last_certificate_timestamp = DateTime.now()
    same_or_previous_timestamp = last_certificate_timestamp.subtract(seconds=timestamp_offset)

    # 1) Performa a key rotation to add a new certificate at last_certificate_timestamp

    outcome = await backend.realm.rotate_key(
        now=last_certificate_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        keys_bundle=b"",
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice keys bundle access>",
            coolorg.bob.user_id: b"<bob keys bundle access>",
        },
        realm_key_rotation_certificate=RealmKeyRotationCertificate(
            author=coolorg.alice.device_id,
            timestamp=last_certificate_timestamp,
            hash_algorithm=HashAlgorithm.SHA256,
            encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
            key_index=2,
            realm_id=coolorg.wksp1_id,
            key_canary=SecretKey.generate().encrypt(b""),
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    # 2) Try to unshare a realm with same or previous timestamp

    certif = patch_realm_role_certificate(
        alice_unshare_bob_certificate, timestamp=same_or_previous_timestamp
    )
    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepRequireGreaterTimestamp(
        strictly_greater_than=last_certificate_timestamp
    )
