# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

import pytest

from parsec._parsec import (
    DateTime,
    DeviceID,
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
from tests.common.client import get_last_realm_certificate_timestamp


@pytest.fixture
def wksp1_key_rotation_certificate(coolorg: CoolorgRpcClients) -> RealmKeyRotationCertificate:
    return RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        key_index=2,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=SecretKey.generate().encrypt(b""),
    )


def patch_realm_key_rotation_certificate(
    certif: RealmKeyRotationCertificate,
    author: Optional[DeviceID] = None,
    timestamp: Optional[DateTime] = None,
    realm_id: Optional[VlobID] = None,
    key_index: Optional[int] = None,
) -> RealmKeyRotationCertificate:
    """Utility function to patch one or more RealmKeyRotationCertificate fields"""
    return RealmKeyRotationCertificate(
        author=author or certif.author,
        timestamp=timestamp or certif.timestamp,
        realm_id=realm_id or certif.realm_id,
        key_index=key_index if key_index is not None else certif.key_index,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=SecretKey.generate().encrypt(b""),
    )


@pytest.mark.parametrize("initial_key_rotation", (False, True))
async def test_authenticated_realm_rotate_key_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    wksp1_key_rotation_certificate: RealmKeyRotationCertificate,
    initial_key_rotation: bool,
) -> None:
    if initial_key_rotation:
        t0 = DateTime.now()
        wksp_id = VlobID.new()
        certif = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=t0,
            realm_id=wksp_id,
            role=RealmRole.OWNER,
            user_id=coolorg.alice.user_id,
        )
        await backend.realm.create(
            now=t0,
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            author_verify_key=coolorg.alice.signing_key.verify_key,
            realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        )
        initial_key_index = 0
        participants = {coolorg.alice.user_id}
    else:
        # Coolorg's wksp1 is bootstrapped, hence it has already its initial key rotation
        wksp_id = coolorg.wksp1_id
        initial_key_index = 1
        participants = {coolorg.alice.user_id, coolorg.bob.user_id}

    certif = patch_realm_key_rotation_certificate(
        wksp1_key_rotation_certificate,
        realm_id=wksp_id,
        key_index=initial_key_index + 1,
    )
    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_rotate_key(
            realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            per_participant_keys_bundle_access={
                user_id: f"<{user_id} keys bundle access>".encode() for user_id in participants
            },
            keys_bundle=b"<keys bundle>",
        )
        assert rep == authenticated_cmds.v4.realm_rotate_key.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=coolorg.alice.user_id,
                role_removed=False,
            )
        )

    keys_bundle = await backend.realm.get_keys_bundle(
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=wksp_id,
        key_index=initial_key_index + 1,
    )
    assert isinstance(keys_bundle, KeysBundle)
    assert keys_bundle.key_index == initial_key_index + 1
    assert (
        keys_bundle.keys_bundle_access
        == b"<a11cec00-1000-0000-0000-000000000000 keys bundle access>"
    )
    assert keys_bundle.keys_bundle == b"<keys bundle>"


@pytest.mark.parametrize("kind", ("author_not_realm_owner", "author_no_realm_access"))
async def test_authenticated_realm_rotate_key_author_not_allowed(
    coolorg: CoolorgRpcClients,
    wksp1_key_rotation_certificate: RealmKeyRotationCertificate,
    kind: str,
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

    certif = patch_realm_key_rotation_certificate(
        wksp1_key_rotation_certificate,
        author=author.device_id,
    )
    rep = await author.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(author.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice keys bundle access>",
            coolorg.bob.user_id: b"<bob keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepAuthorNotAllowed()


async def test_authenticated_realm_rotate_key_realm_not_found(
    coolorg: CoolorgRpcClients,
    wksp1_key_rotation_certificate: RealmKeyRotationCertificate,
) -> None:
    bad_realm_id = VlobID.new()
    certif = patch_realm_key_rotation_certificate(
        wksp1_key_rotation_certificate,
        realm_id=bad_realm_id,
        #        key_index=1,
    )
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice keys bundle access>",
            coolorg.bob.user_id: b"<bob keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepRealmNotFound()


@pytest.mark.parametrize("initial_key_rotation", (False, True))
@pytest.mark.parametrize(
    "kind", ("key_index_already_exists", "key_index_too_far_forward", "key_index_is_zero")
)
async def test_authenticated_realm_rotate_key_bad_key_index(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    wksp1_key_rotation_certificate: RealmKeyRotationCertificate,
    initial_key_rotation: bool,
    kind: str,
) -> None:
    if initial_key_rotation:
        t0 = DateTime.now()
        wksp_id = VlobID.new()
        certif = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=t0,
            realm_id=wksp_id,
            role=RealmRole.OWNER,
            user_id=coolorg.alice.user_id,
        )
        await backend.realm.create(
            now=t0,
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            author_verify_key=coolorg.alice.signing_key.verify_key,
            realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        )
        initial_key_index = 0
        participants = {coolorg.alice.user_id}
        wksp_last_certificate_timestamp = t0

    else:
        # Coolorg's wksp1 is bootstrapped, hence it has already its initial key rotation
        wksp_id = coolorg.wksp1_id
        initial_key_index = 1
        participants = {coolorg.alice.user_id, coolorg.bob.user_id}
        wksp_last_certificate_timestamp = get_last_realm_certificate_timestamp(
            coolorg.testbed_template, wksp_id
        )

    match kind:
        case "key_index_already_exists":
            bad_key_index = initial_key_index
        case "key_index_too_far_forward":
            bad_key_index = initial_key_index + 2
        case "key_index_is_zero":
            bad_key_index = 0
        case _:
            assert False

    certif = patch_realm_key_rotation_certificate(
        wksp1_key_rotation_certificate, realm_id=wksp_id, key_index=bad_key_index
    )
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            user_id: b"<keys bundle access>" for user_id in participants
        },
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp_last_certificate_timestamp
    )


@pytest.mark.parametrize("kind", ("additional_participant", "missing_participant"))
async def test_authenticated_realm_rotate_key_participant_mismatch(
    coolorg: CoolorgRpcClients,
    wksp1_key_rotation_certificate: RealmKeyRotationCertificate,
    kind: str,
) -> None:
    per_participant_keys_bundle_access = {
        coolorg.alice.user_id: b"<alice keys bundle access>",
        coolorg.bob.user_id: b"<bob keys bundle access>",
    }
    match kind:
        case "additional_participant":
            per_participant_keys_bundle_access[coolorg.mallory.user_id] = (
                b"<unexpected keys bundle access>"
            )
        case "missing_participant":
            del per_participant_keys_bundle_access[coolorg.bob.user_id]
        case _:
            assert False
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=wksp1_key_rotation_certificate.dump_and_sign(
            coolorg.alice.signing_key
        ),
        per_participant_keys_bundle_access=per_participant_keys_bundle_access,
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepParticipantMismatch()


@pytest.mark.parametrize("kind", ("dummy_data", "bad_author"))
async def test_authenticated_realm_rotate_key_invalid_certificate(
    coolorg: CoolorgRpcClients,
    wksp1_key_rotation_certificate: RealmKeyRotationCertificate,
    kind: str,
) -> None:
    match kind:
        case "dummy_data":
            certif = b"<dummy data>"
        case "bad_author":
            certif = patch_realm_key_rotation_certificate(
                wksp1_key_rotation_certificate, author=coolorg.bob.device_id
            ).dump_and_sign(coolorg.alice.signing_key)
        case _:
            assert False

    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif,
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice keys bundle access>",
            coolorg.bob.user_id: b"<bob keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.v4.realm_rotate_key.RepInvalidCertificate()


async def test_authenticated_realm_rotate_key_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    timestamp_out_of_ballpark: DateTime,
    wksp1_key_rotation_certificate: RealmKeyRotationCertificate,
) -> None:
    certif = patch_realm_key_rotation_certificate(
        wksp1_key_rotation_certificate, timestamp=timestamp_out_of_ballpark, key_index=2
    )
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: "<alice keys bundle access>".encode()
        },
        keys_bundle=b"<keys bundle>",
    )
    assert isinstance(rep, authenticated_cmds.v4.realm_rotate_key.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.xfail(
    reason="TODO: realm_rotate_key never returns RepRequireGreaterTimestamp. Not implemented?"
)
@pytest.mark.parametrize(
    "timestamp_offset",
    (pytest.param(0, id="same_timestamp"), pytest.param(1, id="previous_timestamp")),
)
async def test_authenticated_realm_rotate_key_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    wksp1_key_rotation_certificate: RealmKeyRotationCertificate,
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
            encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
            key_index=2,
            realm_id=coolorg.wksp1_id,
            key_canary=SecretKey.generate().encrypt(b""),
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    # 2) Try to create a realm with same or previous timestamp

    certif = patch_realm_key_rotation_certificate(
        wksp1_key_rotation_certificate, timestamp=same_or_previous_timestamp, key_index=3
    )
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: "<alice keys bundle access>".encode(),
            coolorg.bob.user_id: b"<bob keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.v4.realm_create.RepRequireGreaterTimestamp(
        strictly_greater_than=last_certificate_timestamp
    )
