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
from tests.common import Backend, CoolorgRpcClients, get_last_realm_certificate_timestamp


async def test_authenticated_realm_rename_rotate_key_ok_subsequent_rename(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    t1 = DateTime.now()
    certif = RealmNameCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=coolorg.wksp1_id,
        key_index=1,
        encrypted_name=b"<encrypted name>",
    )

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_rename(
            realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            initial_name_or_fail=False,
        )
        assert rep == authenticated_cmds.v4.realm_rename.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=t1,
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.alice.device_id.user_id,
                role_removed=False,
            )
        )


async def test_authenticated_realm_rename_rotate_key_ok_initial_rename(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Create a new realm (so it has not been renamed yet)

    t0 = DateTime.now()
    wksp2_id = VlobID.new()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=t0,
        realm_id=wksp2_id,
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

    # New realm must have it initial key rotation to allow for initial rename

    t1 = DateTime.now()
    key = SecretKey.generate()
    key_canary = key.encrypt(b"")
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=wksp2_id,
        key_index=1,
        encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=key_canary,
    )
    realm_key_rotation_certificate = certif.dump_and_sign(coolorg.alice.signing_key)
    outcome = await backend.realm.rotate_key(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        keys_bundle=b"<keys bundle>",
        per_participant_keys_bundle_access={
            coolorg.alice.device_id.user_id: b"<alice keys bundle access>",
        },
        realm_key_rotation_certificate=realm_key_rotation_certificate,
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    # Now do the actual initial rename

    t1 = DateTime.now()
    certif = RealmNameCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=wksp2_id,
        key_index=1,
        encrypted_name=b"<encrypted name>",
    )
    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_rename(
            realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            initial_name_or_fail=True,
        )
        assert rep == authenticated_cmds.v4.realm_rename.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=t1,
                realm_id=wksp2_id,
                user_id=coolorg.alice.device_id.user_id,
                role_removed=False,
            )
        )


async def test_authenticated_realm_rename_initial_name_already_exists(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    t1 = DateTime.now()
    certif = RealmNameCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=coolorg.wksp1_id,
        key_index=1,
        encrypted_name=b"<encrypted name>",
    )

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
    certif = RealmNameCertificate(
        author=author.device_id,
        timestamp=t1,
        realm_id=coolorg.wksp1_id,
        key_index=1,
        encrypted_name=b"<encrypted name>",
    )

    rep = await author.realm_rename(
        realm_name_certificate=certif.dump_and_sign(author.signing_key),
        initial_name_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepAuthorNotAllowed()


async def test_authenticated_realm_rename_realm_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    t1 = DateTime.now()
    certif = RealmNameCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=VlobID.new(),  # Dummy realm ID
        key_index=1,
        encrypted_name=b"<encrypted name>",
    )

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

    t1 = DateTime.now()
    certif = RealmNameCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=wksp_id,
        key_index=bad_key_index,
        encrypted_name=b"<encrypted name>",
    )

    rep = await coolorg.alice.realm_rename(
        realm_name_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        initial_name_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp_last_certificate_timestamp
    )


@pytest.mark.parametrize("kind", ("dummy_data", "bad_author"))
async def test_authenticated_realm_rename_invalid_certificate(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    t1 = DateTime.now()
    match kind:
        case "dummy_data":
            realm_key_rotation_certificate = b"<dummy data>"
        case "bad_author":
            certif = RealmNameCertificate(
                author=coolorg.bob.device_id,
                timestamp=t1,
                realm_id=coolorg.wksp1_id,
                key_index=1,
                encrypted_name=b"<encrypted name>",
            )
            realm_key_rotation_certificate = certif.dump_and_sign(coolorg.bob.signing_key)
        case _:
            assert False

    rep = await coolorg.alice.realm_rename(
        realm_name_certificate=realm_key_rotation_certificate,
        initial_name_or_fail=False,
    )
    assert rep == authenticated_cmds.v4.realm_rename.RepInvalidCertificate()
