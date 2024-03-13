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
from tests.common import (
    Backend,
    CoolorgRpcClients,
    get_last_realm_certificate_timestamp,
    patch_realm_role_certificate,
)


@pytest.fixture
def alice_owner_role_certificate(coolorg: CoolorgRpcClients) -> RealmRoleCertificate:
    return RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=VlobID.new(),
        user_id=coolorg.alice.user_id,
        role=RealmRole.OWNER,
    )


async def test_authenticated_realm_create_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_owner_role_certificate: RealmRoleCertificate,
) -> None:
    certif = patch_realm_role_certificate(alice_owner_role_certificate)
    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_create(
            realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.v4.realm_create.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role_removed=False,
            )
        )


async def test_authenticated_realm_create_already_exists(
    coolorg: CoolorgRpcClients,
    alice_owner_role_certificate: RealmRoleCertificate,
) -> None:
    certif = patch_realm_role_certificate(alice_owner_role_certificate, realm_id=coolorg.wksp1_id)
    rep = await coolorg.alice.realm_create(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.v4.realm_create.RepRealmAlreadyExists(
        last_realm_certificate_timestamp=get_last_realm_certificate_timestamp(
            testbed_template=coolorg.testbed_template,
            realm_id=coolorg.wksp1_id,
        )
    )


@pytest.mark.parametrize(
    "kind",
    (
        "dummy_data",
        "bad_author",
    ),
)
async def test_authenticated_realm_create_invalid_certificate(
    coolorg: CoolorgRpcClients,
    kind: str,
    alice_owner_role_certificate: RealmRoleCertificate,
) -> None:
    match kind:
        case "dummy_data":
            certif = b"<dummy data>"
        case "bad_author":
            certif = patch_realm_role_certificate(
                alice_owner_role_certificate, author=coolorg.bob.device_id
            ).dump_and_sign(coolorg.bob.signing_key)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.realm_create(realm_role_certificate=certif)
    assert rep == authenticated_cmds.v4.realm_create.RepInvalidCertificate()


async def test_authenticated_realm_create_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    timestamp_out_of_ballpark: DateTime,
    alice_owner_role_certificate: RealmRoleCertificate,
) -> None:
    certif = patch_realm_role_certificate(
        alice_owner_role_certificate, timestamp=timestamp_out_of_ballpark
    )
    rep = await coolorg.alice.realm_create(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )
    assert isinstance(rep, authenticated_cmds.v4.realm_create.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.parametrize(
    "timestamp_offset",
    (pytest.param(0, id="same_timestamp"), pytest.param(1, id="previous_timestamp")),
)
async def test_authenticated_realm_create_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_owner_role_certificate: RealmRoleCertificate,
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

    certif = patch_realm_role_certificate(
        alice_owner_role_certificate, timestamp=same_or_previous_timestamp
    )
    rep = await coolorg.alice.realm_create(
        realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.v4.realm_create.RepRequireGreaterTimestamp(
        strictly_greater_than=last_certificate_timestamp
    )
