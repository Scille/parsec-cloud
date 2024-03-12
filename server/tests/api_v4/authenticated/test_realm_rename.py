# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    SecretKey,
    SecretKeyAlgorithm,
    VlobID,
    authenticated_cmds,
)
from parsec.events import EventRealmCertificate
from tests.common import (
    AuthenticatedRpcClient,
    Backend,
    CoolorgRpcClients,
    get_last_realm_certificate_timestamp,
)


def realm_name_certificate(
    author: AuthenticatedRpcClient,
    timestamp: DateTime = DateTime.now(),
    realm_id: VlobID = VlobID.new(),
    key_index: int = 1,
) -> RealmNameCertificate:
    """Utility function to create a RealmNameCertificate"""
    return RealmNameCertificate(
        author=author.device_id,
        timestamp=timestamp,
        realm_id=realm_id,
        key_index=key_index,
        encrypted_name=b"<encrypted name>",
    )


def realm_role_certificate(
    author: AuthenticatedRpcClient,
    realm_id: VlobID,
    timestamp: DateTime = DateTime.now(),
    role: RealmRole = RealmRole.OWNER,
) -> RealmRoleCertificate:
    """Utility function to create a RealmRoleCertificate"""
    return RealmRoleCertificate(
        author=author.device_id,
        timestamp=timestamp,
        realm_id=realm_id,
        role=role,
        user_id=author.user_id,
    )


def realm_key_rotation_certificate(
    author: AuthenticatedRpcClient,
    realm_id: VlobID,
    key_index: int,
    timestamp: DateTime = DateTime.now(),
):
    """Utility function to perform a RealmKeyRotationCertificate"""
    return RealmKeyRotationCertificate(
        author=author.device_id,
        timestamp=timestamp,
        realm_id=realm_id,
        key_index=key_index,
        encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=SecretKey.generate().encrypt(b""),
    )


async def test_authenticated_realm_rename_rotate_key_ok_subsequent_rename(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    certif = realm_name_certificate(coolorg.alice, realm_id=coolorg.wksp1_id)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_rename(
            realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            initial_name_or_fail=False,
        )
        assert rep == authenticated_cmds.v4.realm_rename.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=certif.timestamp,
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.alice.device_id.user_id,
                role_removed=False,
            )
        )


async def test_authenticated_realm_rename_rotate_key_ok_initial_rename(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # 1) Create a new realm (so it has not been renamed yet)

    new_realm_id = VlobID.new()
    role_certificate = realm_role_certificate(coolorg.alice, new_realm_id)
    outcome = await backend.realm.create(
        now=role_certificate.timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=role_certificate.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmRoleCertificate)

    # 2) New realm must have its initial key rotation to allow for initial rename

    key_rotation_certif = realm_key_rotation_certificate(
        coolorg.alice, realm_id=new_realm_id, key_index=1
    )
    outcome = await backend.realm.rotate_key(
        now=key_rotation_certif.timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        keys_bundle=b"<keys bundle>",
        per_participant_keys_bundle_access={
            coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
        },
        realm_key_rotation_certificate=key_rotation_certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    # 3) Now do the actual (initial) rename

    certif = realm_name_certificate(coolorg.alice, realm_id=new_realm_id)
    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_rename(
            realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            initial_name_or_fail=True,
        )
        assert rep == authenticated_cmds.v4.realm_rename.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=coolorg.alice.device_id.user_id,
                role_removed=False,
            )
        )


async def test_authenticated_realm_rename_initial_name_already_exists(
    coolorg: CoolorgRpcClients,
) -> None:
    certif = realm_name_certificate(coolorg.alice, realm_id=coolorg.wksp1_id)
    rep = await coolorg.alice.realm_rename(
        realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        initial_name_or_fail=True,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepInitialNameAlreadyExists(
        last_realm_certificate_timestamp=get_last_realm_certificate_timestamp(
            coolorg.testbed_template, coolorg.wksp1_id
        )
    )


@pytest.mark.parametrize("kind", ("author_not_realm_owner", "author_no_realm_access"))
async def test_authenticated_realm_rename_author_not_allowed(
    coolorg: CoolorgRpcClients, kind: str
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

    certif = realm_name_certificate(author, realm_id=coolorg.wksp1_id)
    rep = await author.realm_rename(
        realm_name_certificate=certif.dump_and_sign(author.signing_key),
        initial_name_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepAuthorNotAllowed()


async def test_authenticated_realm_rename_realm_not_found(coolorg: CoolorgRpcClients) -> None:
    bad_realm_id = VlobID.new()
    certif = realm_name_certificate(coolorg.alice, realm_id=bad_realm_id)
    rep = await coolorg.alice.realm_rename(
        realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        initial_name_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepRealmNotFound()


@pytest.mark.parametrize(
    "kind",
    (
        "key_index_too_old",
        "key_index_too_far_forward",
        "key_index_is_zero",
        "realm_had_no_key_rotation_yet",
    ),
)
async def test_authenticated_realm_rename_bad_key_index(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "key_index_too_old":
            # Do another key rotation so that current key index is 2
            wksp_id = coolorg.wksp1_id
            key_rotation_certif = realm_key_rotation_certificate(
                coolorg.alice, realm_id=wksp_id, key_index=2
            )
            outcome = await backend.realm.rotate_key(
                now=key_rotation_certif.timestamp,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                keys_bundle=b"<keys bundle>",
                per_participant_keys_bundle_access={
                    coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
                    coolorg.bob.device_id.user_id: b"<bob keys bundle access>",
                },
                realm_key_rotation_certificate=key_rotation_certif.dump_and_sign(
                    coolorg.alice.signing_key
                ),
            )
            assert isinstance(outcome, RealmKeyRotationCertificate)
            bad_key_index = 1
            wksp_last_certificate_timestamp = key_rotation_certif.timestamp
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
            # Create a new realm, which does not have a key rotation yet
            new_realm_id = VlobID.new()
            role_certificate = realm_role_certificate(coolorg.alice, new_realm_id)
            outcome = await backend.realm.create(
                now=role_certificate.timestamp,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                realm_role_certificate=role_certificate.dump_and_sign(coolorg.alice.signing_key),
            )
            assert isinstance(outcome, RealmRoleCertificate)
            bad_key_index = 1
            wksp_id = new_realm_id
            wksp_last_certificate_timestamp = role_certificate.timestamp
        case _:
            assert False

    certif = realm_name_certificate(coolorg.alice, realm_id=wksp_id, key_index=bad_key_index)
    rep = await coolorg.alice.realm_rename(
        realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        initial_name_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp_last_certificate_timestamp
    )


@pytest.mark.parametrize(
    "kind",
    ("dummy_data", "bad_author"),
)
async def test_authenticated_realm_rename_invalid_certificate(
    coolorg: CoolorgRpcClients, kind: str
) -> None:
    match kind:
        case "dummy_data":
            certif = b"<dummy data>"
        case "bad_author":
            certif = realm_name_certificate(coolorg.bob, realm_id=coolorg.wksp1_id).dump_and_sign(
                coolorg.bob.signing_key
            )
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.realm_rename(
        realm_name_certificate=certif,
        initial_name_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepInvalidCertificate()


@pytest.mark.xfail(
    reason="TODO: realm_rename never returns RepRequireGreaterTimestamp. Not implemented?"
)
@pytest.mark.parametrize(
    "timestamp_offset",
    (pytest.param(0, id="same_timestamp"), pytest.param(1, id="previous_timestamp")),
)
async def test_authenticated_realm_rename_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
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

    # 2) Try to create a realm with same or previous timestamp

    certif = realm_name_certificate(
        coolorg.alice, timestamp=same_or_previous_timestamp, realm_id=coolorg.wksp1_id, key_index=2
    )
    rep = await coolorg.alice.realm_rename(
        realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        initial_name_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepRequireGreaterTimestamp(
        strictly_greater_than=last_certificate_timestamp
    )


async def test_authenticated_realm_rename_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    certif = realm_name_certificate(
        coolorg.alice, timestamp=timestamp_out_of_ballpark, realm_id=coolorg.wksp1_id
    )
    rep = await coolorg.alice.realm_rename(
        realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        initial_name_or_fail=True,
    )
    assert isinstance(rep, authenticated_cmds.v4.realm_rename.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark
