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
from tests.common import (
    Backend,
    CoolorgRpcClients,
    get_last_realm_certificate_timestamp,
    patch_realm_role_certificate,
)


@pytest.fixture
def alice_share_bob_certificate(coolorg: CoolorgRpcClients) -> RealmRoleCertificate:
    return RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.bob.user_id,
        role=RealmRole.MANAGER,
    )


@pytest.fixture
def alice_share_mallory_certificate(coolorg: CoolorgRpcClients) -> RealmRoleCertificate:
    return RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.mallory.user_id,
        role=RealmRole.READER,
    )


async def test_authenticated_realm_share_ok_new_sharing(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_share_mallory_certificate: RealmRoleCertificate,
) -> None:
    mallory_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.mallory.device_id.user_id
    )
    assert isinstance(mallory_realms, dict)
    assert coolorg.wksp1_id not in mallory_realms

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_share(
            key_index=1,
            realm_role_certificate=alice_share_mallory_certificate.dump_and_sign(
                coolorg.alice.signing_key
            ),
            # Keys bundle access is not readable by the server, so we can put anything here
            recipient_keys_bundle_access=b"<mallory keys bundle access>",
        )
        assert rep == authenticated_cmds.v4.realm_share.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=alice_share_mallory_certificate.timestamp,
                realm_id=alice_share_mallory_certificate.realm_id,
                user_id=alice_share_mallory_certificate.user_id,
                role_removed=False,
            )
        )

    mallory_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.mallory.device_id.user_id
    )
    assert isinstance(mallory_realms, dict)
    assert mallory_realms[coolorg.wksp1_id] == RealmRole.READER


async def test_authenticated_realm_share_ok_update_sharing_role(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_share_bob_certificate: RealmRoleCertificate,
) -> None:
    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.device_id.user_id
    )
    assert isinstance(bob_realms, dict)
    assert bob_realms[coolorg.wksp1_id] == RealmRole.READER

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_share(
            key_index=1,
            realm_role_certificate=alice_share_bob_certificate.dump_and_sign(
                coolorg.alice.signing_key
            ),
            # Keys bundle access is not readable by the server, so we can put anything here
            recipient_keys_bundle_access=b"<bob keys bundle access>",
        )
        assert rep == authenticated_cmds.v4.realm_share.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=alice_share_bob_certificate.timestamp,
                realm_id=alice_share_bob_certificate.realm_id,
                user_id=alice_share_bob_certificate.user_id,
                role_removed=False,
            )
        )

    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.device_id.user_id
    )
    assert isinstance(bob_realms, dict)
    assert bob_realms[coolorg.wksp1_id] == RealmRole.MANAGER


async def test_authenticated_realm_share_role_already_granted(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.device_id.user_id
    )
    assert isinstance(bob_realms, dict)
    assert bob_realms[coolorg.wksp1_id] == RealmRole.READER

    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.READER,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif,
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepRoleAlreadyGranted(
        last_realm_certificate_timestamp=get_last_realm_certificate_timestamp(
            coolorg.testbed_template, coolorg.wksp1_id
        )
    )


@pytest.mark.parametrize("kind", ("dummy_certif", "role_none", "self_share"))
async def test_authenticated_realm_share_invalid_certificate(
    coolorg: CoolorgRpcClients,
    kind: str,
    alice_share_bob_certificate: RealmRoleCertificate,
) -> None:
    match kind:
        case "dummy_certif":
            certif = b"<dummy>"
        case "role_none":
            certif = patch_realm_role_certificate(
                alice_share_bob_certificate, force_no_role=True
            ).dump_and_sign(coolorg.alice.signing_key)
        case "self_share":
            certif = patch_realm_role_certificate(
                alice_share_bob_certificate, user_id=coolorg.alice.user_id
            ).dump_and_sign(coolorg.alice.signing_key)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif,
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepInvalidCertificate()


@pytest.mark.parametrize(
    "kind",
    (
        "key_index_too_old",
        "key_index_too_far_forward",
        "key_index_is_zero",
        "realm_had_no_key_rotation_yet",
    ),
)
async def test_authenticated_realm_share_key_bad_key_index(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
    alice_share_mallory_certificate: RealmRoleCertificate,
) -> None:
    match kind:
        case "key_index_too_old":
            # Do another key rotation so that current key index is 2
            t0 = DateTime.now()
            key = SecretKey.generate()
            key_canary = key.encrypt(b"")
            certif = RealmKeyRotationCertificate(
                author=coolorg.alice.device_id,
                timestamp=t0,
                realm_id=coolorg.wksp1_id,
                key_index=2,
                encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
                hash_algorithm=HashAlgorithm.SHA256,
                key_canary=key_canary,
            )
            realm_key_rotation_certificate = certif.dump_and_sign(coolorg.alice.signing_key)
            outcome = await backend.realm.rotate_key(
                now=t0,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                keys_bundle=b"<keys bundle>",
                per_participant_keys_bundle_access={
                    coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
                    coolorg.bob.device_id.user_id: b"<bob keys bundle access>",
                },
                realm_key_rotation_certificate=realm_key_rotation_certificate,
            )
            assert isinstance(outcome, RealmKeyRotationCertificate)
            bad_key_index = 1
            wksp_id = coolorg.wksp1_id
            wksp_last_certificate_timestamp = t0
        case "key_index_too_far_forward":
            bad_key_index = 2
            wksp_id = coolorg.wksp1_id
            wksp_last_certificate_timestamp = get_last_realm_certificate_timestamp(
                coolorg.testbed_template, wksp_id
            )
        case "key_index_is_zero":
            bad_key_index = 0
            wksp_id = coolorg.wksp1_id
            wksp_last_certificate_timestamp = get_last_realm_certificate_timestamp(
                coolorg.testbed_template, wksp_id
            )
        case "realm_had_no_key_rotation_yet":
            t0 = DateTime.now()
            wksp_id = VlobID.new()
            certif = RealmRoleCertificate(
                author=coolorg.alice.device_id,
                timestamp=t0,
                realm_id=wksp_id,
                role=RealmRole.OWNER,
                user_id=coolorg.alice.device_id.user_id,
            )
            outcome = await backend.realm.create(
                now=t0,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            )
            assert isinstance(outcome, RealmRoleCertificate)
            bad_key_index = 1
            wksp_last_certificate_timestamp = t0
        case _:
            assert False

    certif = patch_realm_role_certificate(
        alice_share_mallory_certificate, timestamp=DateTime.now(), realm_id=wksp_id
    )
    rep = await coolorg.alice.realm_share(
        key_index=bad_key_index,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp_last_certificate_timestamp
    )


async def test_authenticated_realm_share_recipient_revoked(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_share_mallory_certificate: RealmRoleCertificate,
) -> None:
    # 1) Revoke user Mallory

    revoked_timestamp = DateTime.now()
    outcome = await backend.user.revoke_user(
        now=revoked_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=RevokedUserCertificate(
            author=coolorg.alice.device_id,
            timestamp=revoked_timestamp,
            user_id=coolorg.mallory.device_id.user_id,
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RevokedUserCertificate)

    # 2) Try to share with mallory (which is now revoked)

    certif = patch_realm_role_certificate(alice_share_mallory_certificate, timestamp=DateTime.now())
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepRecipientRevoked()


async def test_authenticated_realm_share_bad_key_index(
    coolorg: CoolorgRpcClients,
    alice_share_mallory_certificate: RealmRoleCertificate,
) -> None:
    rep = await coolorg.alice.realm_share(
        key_index=2,
        realm_role_certificate=alice_share_mallory_certificate.dump_and_sign(
            coolorg.alice.signing_key
        ),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepBadKeyIndex(
        last_realm_certificate_timestamp=get_last_realm_certificate_timestamp(
            coolorg.testbed_template, coolorg.wksp1_id
        )
    )


async def test_authenticated_realm_share_author_not_allowed(
    coolorg: CoolorgRpcClients,
    alice_share_mallory_certificate: RealmRoleCertificate,
) -> None:
    certif = patch_realm_role_certificate(
        alice_share_mallory_certificate, author=coolorg.bob.device_id
    )
    rep = await coolorg.bob.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepAuthorNotAllowed()


async def test_authenticated_realm_share_realm_not_found(
    coolorg: CoolorgRpcClients,
    alice_share_mallory_certificate: RealmRoleCertificate,
) -> None:
    bad_realm_id = VlobID.new()
    certif = patch_realm_role_certificate(alice_share_mallory_certificate, realm_id=bad_realm_id)
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepRealmNotFound()


async def test_authenticated_realm_share_recipient_not_found(
    coolorg: CoolorgRpcClients,
    alice_share_mallory_certificate: RealmRoleCertificate,
) -> None:
    bad_user_id = UserID("dummy")
    certif = patch_realm_role_certificate(alice_share_mallory_certificate, user_id=bad_user_id)
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepRecipientNotFound()


@pytest.mark.parametrize("role", (RealmRole.MANAGER, RealmRole.OWNER))
async def test_authenticated_realm_share_role_incompatible_with_outsider(
    coolorg: CoolorgRpcClients,
    alice_share_mallory_certificate: RealmRoleCertificate,
    role: RealmRole,
) -> None:
    certif = patch_realm_role_certificate(alice_share_mallory_certificate, role=role)
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepRoleIncompatibleWithOutsider()


async def test_authenticated_realm_share_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    alice_share_mallory_certificate: RealmRoleCertificate,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    certif = patch_realm_role_certificate(
        alice_share_mallory_certificate, timestamp=timestamp_out_of_ballpark
    )
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert isinstance(rep, authenticated_cmds.v4.realm_share.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.parametrize(
    "timestamp_offset",
    (pytest.param(0, id="same_timestamp"), pytest.param(1, id="previous_timestamp")),
)
async def test_authenticated_realm_share_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_share_mallory_certificate: RealmRoleCertificate,
    timestamp_offset: int,
) -> None:
    last_certificate_timestamp = DateTime.now()
    same_or_previous_timestamp = last_certificate_timestamp.subtract(seconds=timestamp_offset)

    # 1) Create a new certificate in the realm

    certif = patch_realm_role_certificate(
        alice_share_mallory_certificate, timestamp=last_certificate_timestamp
    )
    outcome = await backend.realm.share(
        now=last_certificate_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    # 2) Try to share the realm with same or previous timestamp

    certif = patch_realm_role_certificate(
        alice_share_mallory_certificate,
        timestamp=same_or_previous_timestamp,
        role=RealmRole.CONTRIBUTOR,
    )
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        # Keys bundle access is not readable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepRequireGreaterTimestamp(
        strictly_greater_than=last_certificate_timestamp
    )
