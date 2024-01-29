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
from parsec.events import EventRealmCertificate
from tests.common import Backend, CoolorgRpcClients, get_last_realm_certificate_timestamp


async def test_authenticated_realm_share_ok_new_sharing(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    mallory_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.mallory.device_id.user_id
    )
    assert isinstance(mallory_realms, dict)
    assert coolorg.wksp1_id not in mallory_realms

    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.READER,
        user_id=coolorg.mallory.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_share(
            key_index=1,
            realm_role_certificate=certif,
            # Keys bundle access is not redable by the server, so we can put anything here
            recipient_keys_bundle_access=b"<mallory keys bundle access>",
        )
        assert rep == authenticated_cmds.v4.realm_share.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=timestamp,
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.mallory.device_id.user_id,
                role_removed=False,
            )
        )

    mallory_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.mallory.device_id.user_id
    )
    assert isinstance(mallory_realms, dict)
    assert mallory_realms[coolorg.wksp1_id] == RealmRole.READER


async def test_authenticated_realm_share_ok_update_sharing_role(
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
        role=RealmRole.MANAGER,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_share(
            key_index=1,
            realm_role_certificate=certif,
            # Keys bundle access is not redable by the server, so we can put anything here
            recipient_keys_bundle_access=b"<bob keys bundle access>",
        )
        assert rep == authenticated_cmds.v4.realm_share.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=timestamp,
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.device_id.user_id,
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
        # Keys bundle access is not redable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepRoleAlreadyGranted(
        last_realm_certificate_timestamp=get_last_realm_certificate_timestamp(
            coolorg.testbed_template, coolorg.wksp1_id
        )
    )


async def test_authenticated_realm_share_invalid_certificate_role_none(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=None,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif,
        # Keys bundle access is not redable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepInvalidCertificate()


async def test_authenticated_realm_share_invalid_certificate_dummy_data(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=b"<dummy>",
        # Keys bundle access is not redable by the server, so we can put anything here
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
                realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            )
            assert isinstance(outcome, RealmRoleCertificate)
            bad_key_index = 1
            wksp_last_certificate_timestamp = t0
        case _:
            assert False

    t1 = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=wksp_id,
        role=RealmRole.READER,
        user_id=coolorg.mallory.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.realm_share(
        key_index=bad_key_index,
        realm_role_certificate=certif,
        # Keys bundle access is not redable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<mallory keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp_last_certificate_timestamp
    )


# TODO: test share on revoked user
