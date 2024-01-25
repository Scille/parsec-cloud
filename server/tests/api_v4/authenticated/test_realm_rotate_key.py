# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    RealmRole,
    RealmRoleCertificate,
    SecretKey,
    SecretKeyAlgorithm,
    VlobID,
    authenticated_cmds,
)
from parsec.components.realm import KeysBundle
from parsec.events import EventRealmCertificate
from tests.common import Backend, CoolorgRpcClients


@pytest.mark.parametrize("initial_key_rotation", (False, True))
async def test_authenticated_realm_rotate_key_ok(
    coolorg: CoolorgRpcClients, backend: Backend, initial_key_rotation: bool
) -> None:
    if initial_key_rotation:
        t0 = DateTime.now()
        wksp_id = VlobID.new()
        certif = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=t0,
            realm_id=wksp_id,
            role=RealmRole.OWNER,
            user_id=coolorg.alice.device_id.user_id,
        )
        await backend.realm.create(
            now=t0,
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        )
        initial_key_index = 0
        participants = {coolorg.alice.device_id.user_id}
    else:
        # Coolorg's wksp1 is bootstrapped, hence it has already its initial key rotation
        wksp_id = coolorg.wksp1_id
        initial_key_index = 1
        participants = {coolorg.alice.device_id.user_id, coolorg.bob.device_id.user_id}

    t1 = DateTime.now()
    key = SecretKey.generate()
    key_canary = key.encrypt(b"")
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=wksp_id,
        key_index=initial_key_index + 1,
        encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=key_canary,
    )

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_rotate_key(
            realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            per_participant_keys_bundle_access={
                user_id: f"<{user_id} keys bundle access>".encode() for user_id in participants
            },
            keys_bundle=b"<keys bundle>",
            never_legacy_reencrypted_or_fail=False,
        )
        assert rep == authenticated_cmds.v4.realm_rotate_key.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=t1,
                realm_id=wksp_id,
                user_id=coolorg.alice.device_id.user_id,
                role_removed=False,
            )
        )

    keys_bundle = await backend.realm.get_keys_bundle_as_user(
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=wksp_id,
        key_index=initial_key_index + 1,
    )
    assert isinstance(keys_bundle, KeysBundle)
    assert keys_bundle.key_index == initial_key_index + 1
    assert keys_bundle.keys_bundle_access == b"<alice keys bundle access>"
    assert keys_bundle.keys_bundle == b"<keys bundle>"


@pytest.mark.parametrize("kind", ("author_not_realm_owner", "author_no_realm_access"))
async def test_authenticated_realm_rotate_key_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "author_not_realm_owner":
            # Bob has access to the realm, but he is not OWNER
            author = coolorg.bob
        case "author_no_realm_access":
            # Mallory has no access to the realm !
            author = coolorg.mallory
        case _:
            assert False

    t1 = DateTime.now()
    key = SecretKey.generate()
    key_canary = key.encrypt(b"")
    certif = RealmKeyRotationCertificate(
        author=author.device_id,
        timestamp=t1,
        realm_id=coolorg.wksp1_id,
        key_index=2,
        encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=key_canary,
    )

    rep = await author.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(author.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
            coolorg.bob.device_id.user_id: b"<bob keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
        never_legacy_reencrypted_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepAuthorNotAllowed()


async def test_authenticated_realm_rotate_key_realm_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    t1 = DateTime.now()
    key = SecretKey.generate()
    key_canary = key.encrypt(b"")
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=VlobID.new(),  # Dummy realm ID
        key_index=1,
        encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=key_canary,
    )

    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
            coolorg.bob.device_id.user_id: b"<bob keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
        never_legacy_reencrypted_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepRealmNotFound()


@pytest.mark.parametrize(
    "kind", ("key_index_already_exists", "key_index_too_far_forward", "key_index_is_zero")
)
async def test_authenticated_realm_rotate_key_bad_key_index(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    t1 = DateTime.now()
    key = SecretKey.generate()
    key_canary = key.encrypt(b"")
    match kind:
        case "key_index_already_exists":
            bad_key_index = 1
        case "key_index_too_far_forward":
            bad_key_index = 3
        case "key_index_is_zero":
            bad_key_index = 0
        case _:
            assert False
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=coolorg.wksp1_id,
        key_index=bad_key_index,
        encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=key_canary,
    )

    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
            coolorg.bob.device_id.user_id: b"<bob keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
        never_legacy_reencrypted_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepBadKeyIndex()


@pytest.mark.parametrize("kind", ("additional_participant", "missing_participant"))
async def test_authenticated_realm_rotate_key_participant_mismatch(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    t1 = DateTime.now()
    key = SecretKey.generate()
    key_canary = key.encrypt(b"")
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=coolorg.wksp1_id,
        key_index=2,
        encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=key_canary,
    )

    per_participant_keys_bundle_access = {
        coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
        coolorg.bob.device_id.user_id: b"<bob keys bundle access>",
    }
    match kind:
        case "additional_participant":
            per_participant_keys_bundle_access[
                coolorg.mallory.device_id.user_id
            ] = b"<unexpected keys bundle access>"
        case "missing_participant":
            del per_participant_keys_bundle_access[coolorg.bob.device_id.user_id]
        case _:
            assert False
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access=per_participant_keys_bundle_access,
        keys_bundle=b"<keys bundle>",
        never_legacy_reencrypted_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepParticipantMismatch()


@pytest.mark.parametrize("kind", ("dummy_data", "bad_author"))
async def test_authenticated_realm_rotate_key_invalid_certificate(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    t1 = DateTime.now()
    key = SecretKey.generate()
    key_canary = key.encrypt(b"")

    match kind:
        case "dummy_data":
            realm_key_rotation_certificate = b"<dummy data>"
        case "bad_author":
            certif = RealmKeyRotationCertificate(
                author=coolorg.bob.device_id,
                timestamp=t1,
                realm_id=coolorg.wksp1_id,
                key_index=2,
                encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
                hash_algorithm=HashAlgorithm.SHA256,
                key_canary=key_canary,
            )
            realm_key_rotation_certificate = certif.dump_and_sign(coolorg.alice.signing_key)
        case _:
            assert False

    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=realm_key_rotation_certificate,
        per_participant_keys_bundle_access={
            coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
            coolorg.bob.device_id.user_id: b"<bob keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
        never_legacy_reencrypted_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepInvalidCertificate()
