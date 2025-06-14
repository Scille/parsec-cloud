# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import asyncio
from unittest.mock import Mock

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
    SequesterServiceID,
    VlobID,
    authenticated_cmds,
)
from parsec.components.realm import KeysBundle
from parsec.components.sequester import (
    RejectedBySequesterService,
    SequesterServiceType,
    SequesterServiceUnavailable,
)
from parsec.events import EventRealmCertificate
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    get_last_realm_certificate_timestamp,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)
from tests.common.client import SequesteredOrgRpcClients


def wksp1_key_rotation_certificate(
    coolorg: CoolorgRpcClients,
    author: DeviceID | None = None,
    timestamp: DateTime | None = None,
    realm_id: VlobID | None = None,
    key_index: int | None = None,
    encryption_algorithm: SecretKeyAlgorithm | None = None,
    hash_algorithm: HashAlgorithm | None = None,
    key_canary: bytes | None = None,
) -> RealmKeyRotationCertificate:
    return RealmKeyRotationCertificate(
        author=author if author is not None else coolorg.alice.device_id,
        timestamp=timestamp if timestamp is not None else DateTime.now(),
        realm_id=realm_id if realm_id is not None else coolorg.wksp1_id,
        key_index=key_index if key_index is not None else 2,
        encryption_algorithm=encryption_algorithm
        if encryption_algorithm is not None
        else SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=hash_algorithm if hash_algorithm is not None else HashAlgorithm.SHA256,
        key_canary=key_canary if key_canary is not None else SecretKey.generate().encrypt(b""),
    )


@pytest.mark.parametrize("initial_key_rotation", (False, True))
async def test_authenticated_realm_rotate_key_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
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

    certif = wksp1_key_rotation_certificate(
        coolorg,
        realm_id=wksp_id,
        key_index=initial_key_index + 1,
    )
    expected_topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    expected_topics.realms[wksp_id] = certif.timestamp

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_rotate_key(
            realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            per_participant_keys_bundle_access={
                user_id: f"<{user_id} keys bundle access>".encode() for user_id in participants
            },
            per_sequester_service_keys_bundle_access=None,
            keys_bundle=b"<keys bundle>",
        )
        assert rep == authenticated_cmds.latest.realm_rotate_key.RepOk()
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

    topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    assert topics == expected_topics


async def test_authenticated_realm_rotate_key_ok_sequestered(
    sequestered_org: SequesteredOrgRpcClients,
    backend: Backend,
) -> None:
    certif = RealmKeyRotationCertificate(
        author=sequestered_org.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=sequestered_org.wksp1_id,
        key_index=4,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=SecretKey.generate().encrypt(b""),
    )

    with backend.event_bus.spy() as spy:
        rep = await sequestered_org.alice.realm_rotate_key(
            realm_key_rotation_certificate=certif.dump_and_sign(sequestered_org.alice.signing_key),
            per_participant_keys_bundle_access={
                user_id: f"<{user_id} keys bundle access>".encode()
                for user_id in [sequestered_org.alice.user_id, sequestered_org.bob.user_id]
            },
            per_sequester_service_keys_bundle_access={
                sequestered_org.sequester_service_2_id: b"<sequester 2 keys bundle access>",
            },
            keys_bundle=b"<keys bundle>",
        )
        assert rep == authenticated_cmds.latest.realm_rotate_key.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=sequestered_org.organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=sequestered_org.alice.user_id,
                role_removed=False,
            )
        )

    # TODO: Check the database correctly contains the sequester service's keys bundle access


@pytest.mark.parametrize(
    "kind",
    (
        "as_reader",
        "as_contributor",
        "as_manager",
        "never_allowed",
        "no_longer_allowed",
        "bad_key_index_and_not_allowed",
        "participant_mismatch_and_not_allowed",
        "require_greater_timestamp_and_not_allowed",
    ),
)
async def test_authenticated_realm_rotate_key_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    per_participant_keys_bundle_access = {
        coolorg.alice.user_id: b"<alice keys bundle access>",
        coolorg.bob.user_id: b"<bob keys bundle access>",
    }
    certif_kwargs = {}

    match kind:
        case "as_reader":
            # Bob is only `READER` in realm `wksp1`
            author = coolorg.bob

        case "as_contributor":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR
            )
            author = coolorg.bob

        case "as_manager":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            author = coolorg.bob

        case "never_allowed":
            # Mallory has no access to the realm !
            author = coolorg.mallory

        case "no_longer_allowed":
            await wksp1_bob_becomes_owner_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_role=RealmRole.MANAGER
            )
            author = coolorg.alice

        case "bad_key_index_and_not_allowed":
            certif_kwargs["key_index"] = 42
            # Bob is only `READER` in realm `wksp1`
            author = coolorg.bob

        case "participant_mismatch_and_not_allowed":
            per_participant_keys_bundle_access.pop(coolorg.alice.user_id)
            # Bob is only `READER` in realm `wksp1`
            author = coolorg.bob

        case "require_greater_timestamp_and_not_allowed":
            certif_kwargs["timestamp"] = DateTime.now()
            # Just create any certificate in the realm
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.READER
            )
            # Bob is only `READER` in realm `wksp1`
            author = coolorg.bob

        case _:
            assert False

    certif = wksp1_key_rotation_certificate(coolorg, author=author.device_id, **certif_kwargs)
    rep = await author.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(author.signing_key),
        per_participant_keys_bundle_access=per_participant_keys_bundle_access,
        per_sequester_service_keys_bundle_access=None,
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepAuthorNotAllowed()


async def test_authenticated_realm_rotate_key_realm_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    bad_realm_id = VlobID.new()
    certif = wksp1_key_rotation_certificate(
        coolorg,
        realm_id=bad_realm_id,
    )
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice keys bundle access>",
            coolorg.bob.user_id: b"<bob keys bundle access>",
        },
        per_sequester_service_keys_bundle_access=None,
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepRealmNotFound()


@pytest.mark.parametrize("initial_key_rotation", (False, True))
@pytest.mark.parametrize(
    "kind", ("key_index_already_exists", "key_index_too_far_forward", "key_index_is_zero")
)
async def test_authenticated_realm_rotate_key_bad_key_index(
    coolorg: CoolorgRpcClients,
    backend: Backend,
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

    certif = wksp1_key_rotation_certificate(coolorg, realm_id=wksp_id, key_index=bad_key_index)
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            user_id: b"<keys bundle access>" for user_id in participants
        },
        per_sequester_service_keys_bundle_access=None,
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp_last_certificate_timestamp
    )


@pytest.mark.parametrize("kind", ("additional_participant", "missing_participant"))
async def test_authenticated_realm_rotate_key_participant_mismatch(
    coolorg: CoolorgRpcClients,
    kind: str,
) -> None:
    certif = wksp1_key_rotation_certificate(coolorg)
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
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access=per_participant_keys_bundle_access,
        per_sequester_service_keys_bundle_access=None,
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepParticipantMismatch(
        last_realm_certificate_timestamp=DateTime(2000, 1, 12)
    )


@pytest.mark.parametrize("kind", ("dummy_data", "bad_author"))
async def test_authenticated_realm_rotate_key_invalid_certificate(
    coolorg: CoolorgRpcClients,
    kind: str,
) -> None:
    match kind:
        case "dummy_data":
            certif = b"<dummy data>"
        case "bad_author":
            certif = wksp1_key_rotation_certificate(
                coolorg, author=coolorg.bob.device_id
            ).dump_and_sign(coolorg.alice.signing_key)
        case _:
            assert False

    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif,
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice keys bundle access>",
            coolorg.bob.user_id: b"<bob keys bundle access>",
        },
        per_sequester_service_keys_bundle_access=None,
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepInvalidCertificate()


async def test_authenticated_realm_rotate_key_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    certif = wksp1_key_rotation_certificate(
        coolorg, timestamp=timestamp_out_of_ballpark, key_index=2
    )
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={coolorg.alice.user_id: b"<alice keys bundle access>"},
        per_sequester_service_keys_bundle_access=None,
        keys_bundle=b"<keys bundle>",
    )
    assert isinstance(rep, authenticated_cmds.latest.realm_rotate_key.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.parametrize(
    "timestamp_offset",
    (pytest.param(0, id="same_timestamp"), pytest.param(1, id="previous_timestamp")),
)
async def test_authenticated_realm_rotate_key_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_offset: int,
) -> None:
    last_certificate_timestamp = DateTime.now()
    same_or_previous_timestamp = last_certificate_timestamp.subtract(seconds=timestamp_offset)

    # 1) Perform a key rotation to add a new certificate at last_certificate_timestamp

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
        per_sequester_service_keys_bundle_access=None,
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

    certif = wksp1_key_rotation_certificate(
        coolorg, timestamp=same_or_previous_timestamp, key_index=3
    )
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice keys bundle access>",
            coolorg.bob.user_id: b"<bob keys bundle access>",
        },
        per_sequester_service_keys_bundle_access=None,
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepRequireGreaterTimestamp(
        strictly_greater_than=last_certificate_timestamp
    )


async def test_authenticated_realm_rotate_key_http_common_errors(
    coolorg: CoolorgRpcClients,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        certif = wksp1_key_rotation_certificate(coolorg)
        await coolorg.alice.realm_rotate_key(
            realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            per_participant_keys_bundle_access={
                user_id: f"<{user_id} keys bundle access>".encode()
                for user_id in [coolorg.alice.user_id, coolorg.bob.user_id]
            },
            per_sequester_service_keys_bundle_access=None,
            keys_bundle=b"<keys bundle>",
        )

    await authenticated_http_common_errors_tester(do)


async def test_authenticated_realm_rotate_key_organization_not_sequestered(
    coolorg: CoolorgRpcClients,
) -> None:
    certif = wksp1_key_rotation_certificate(coolorg)
    rep = await coolorg.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        per_participant_keys_bundle_access={
            user_id: f"<{user_id} keys bundle access>".encode()
            for user_id in [coolorg.alice.user_id, coolorg.bob.user_id]
        },
        per_sequester_service_keys_bundle_access={
            SequesterServiceID.new(): b"<sequester 1 keys bundle access>"
        },
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepOrganizationNotSequestered()


@pytest.mark.parametrize("kind", ("revoked_service", "unknown_service", "missing_service"))
async def test_authenticated_realm_rotate_key_sequester_service_mismatch(
    sequestered_org: SequesteredOrgRpcClients, kind: str
) -> None:
    match kind:
        case "revoked_service":
            per_sequester_service_keys_bundle_access = {
                sequestered_org.sequester_service_1_id: b"<sequester 1 keys bundle access>",
                sequestered_org.sequester_service_2_id: b"<sequester 2 keys bundle access>",
            }
        case "unknown_service":
            per_sequester_service_keys_bundle_access = {
                sequestered_org.sequester_service_2_id: b"<sequester 2 keys bundle access>",
                SequesterServiceID.new(): b"<dummy sequester keys bundle access>",
            }
        case "missing_service":
            per_sequester_service_keys_bundle_access = {}
        case unknown:
            assert False, unknown

    certif = RealmKeyRotationCertificate(
        author=sequestered_org.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=sequestered_org.wksp1_id,
        key_index=4,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=SecretKey.generate().encrypt(b""),
    )

    rep = await sequestered_org.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(sequestered_org.alice.signing_key),
        per_participant_keys_bundle_access={
            user_id: f"<{user_id} keys bundle access>".encode()
            for user_id in [sequestered_org.alice.user_id, sequestered_org.bob.user_id]
        },
        per_sequester_service_keys_bundle_access=per_sequester_service_keys_bundle_access,
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepSequesterServiceMismatch(
        last_sequester_certificate_timestamp=DateTime(2000, 1, 18)
    )


async def test_authenticated_realm_rotate_key_rejected_by_sequester_service(
    sequestered_org: SequesteredOrgRpcClients,
    backend: Backend,
    monkeypatch: pytest.MonkeyPatch,
    xfail_if_postgresql: None,
) -> None:
    outcome = await backend.sequester.update_config_for_service(
        organization_id=sequestered_org.organization_id,
        service_id=sequestered_org.sequester_service_2_id,
        config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
    )
    assert outcome is None

    future = asyncio.Future()
    future.set_result(
        RejectedBySequesterService(
            service_id=sequestered_org.sequester_service_2_id, reason="Not in the mood"
        )
    )
    _mocked_sequester_service_on_realm_rotate_key = Mock(side_effect=[future])
    monkeypatch.setattr(
        "parsec.webhooks.WebhooksComponent.sequester_service_on_realm_rotate_key",
        _mocked_sequester_service_on_realm_rotate_key,
    )

    initial_dump = await backend.vlob.test_dump_vlobs(
        organization_id=sequestered_org.organization_id
    )

    certif = RealmKeyRotationCertificate(
        author=sequestered_org.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=sequestered_org.wksp1_id,
        key_index=4,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=SecretKey.generate().encrypt(b""),
    )

    rep = await sequestered_org.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(sequestered_org.alice.signing_key),
        per_participant_keys_bundle_access={
            user_id: f"<{user_id} keys bundle access>".encode()
            for user_id in [sequestered_org.alice.user_id, sequestered_org.bob.user_id]
        },
        per_sequester_service_keys_bundle_access={
            sequestered_org.sequester_service_2_id: b"<sequester 2 keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepRejectedBySequesterService(
        service_id=sequestered_org.sequester_service_2_id, reason="Not in the mood"
    )

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=sequestered_org.organization_id)
    assert dump == initial_dump


async def test_authenticated_realm_rotate_key_sequester_service_unavailable(
    sequestered_org: SequesteredOrgRpcClients,
    backend: Backend,
    monkeypatch: pytest.MonkeyPatch,
    xfail_if_postgresql: None,
) -> None:
    outcome = await backend.sequester.update_config_for_service(
        organization_id=sequestered_org.organization_id,
        service_id=sequestered_org.sequester_service_2_id,
        config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
    )
    assert outcome is None

    future = asyncio.Future()
    future.set_result(
        SequesterServiceUnavailable(service_id=sequestered_org.sequester_service_2_id)
    )
    _mocked_sequester_service_on_realm_rotate_key = Mock(side_effect=[future])
    monkeypatch.setattr(
        "parsec.webhooks.WebhooksComponent.sequester_service_on_realm_rotate_key",
        _mocked_sequester_service_on_realm_rotate_key,
    )

    initial_dump = await backend.vlob.test_dump_vlobs(
        organization_id=sequestered_org.organization_id
    )

    certif = RealmKeyRotationCertificate(
        author=sequestered_org.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=sequestered_org.wksp1_id,
        key_index=4,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=SecretKey.generate().encrypt(b""),
    )

    rep = await sequestered_org.alice.realm_rotate_key(
        realm_key_rotation_certificate=certif.dump_and_sign(sequestered_org.alice.signing_key),
        per_participant_keys_bundle_access={
            user_id: f"<{user_id} keys bundle access>".encode()
            for user_id in [sequestered_org.alice.user_id, sequestered_org.bob.user_id]
        },
        per_sequester_service_keys_bundle_access={
            sequestered_org.sequester_service_2_id: b"<sequester 2 keys bundle access>",
        },
        keys_bundle=b"<keys bundle>",
    )
    assert rep == authenticated_cmds.latest.realm_rotate_key.RepSequesterServiceUnavailable(
        service_id=sequestered_org.sequester_service_2_id
    )

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=sequestered_org.organization_id)
    assert dump == initial_dump


async def test_authenticated_realm_rotate_key_concurrency(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    t0 = DateTime.now()

    # Create 10 realms
    realm_ids = [VlobID.new() for _ in range(10)]
    for realm_id in realm_ids:
        certif = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=t0,
            realm_id=realm_id,
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

    # Create 10 key rotation certificates
    t1 = DateTime.now()
    t2 = t1.add(seconds=1)
    certifs = [
        wksp1_key_rotation_certificate(
            coolorg,
            realm_id=realm_id,
            key_index=1,
            timestamp=timestamp,
        )
        for realm_id in realm_ids
        for timestamp in (t1, t2)
    ]

    # Create 2 requests for each certificate
    per_participant_keys_bundle_access = {
        coolorg.alice.user_id: f"<{coolorg.alice.user_id} keys bundle access>".encode()
    }
    coroutines = [
        coolorg.alice.realm_rotate_key(
            realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            per_participant_keys_bundle_access=per_participant_keys_bundle_access,
            per_sequester_service_keys_bundle_access=None,
            keys_bundle=b"<keys bundle>",
        )
        for certif in certifs
    ]

    # Run all the coroutines concurrently
    reps = await asyncio.gather(*coroutines)

    # We expect 10 successful responses
    ok_reps = [rep for rep in reps if rep == authenticated_cmds.latest.realm_rotate_key.RepOk()]
    assert len(ok_reps) == 10

    # We expect 10 bad key index or require greater timestamp responses
    non_ok_reps = [
        rep
        for rep in reps
        if isinstance(
            rep,
            authenticated_cmds.latest.realm_rotate_key.RepBadKeyIndex
            | authenticated_cmds.latest.realm_rotate_key.RepRequireGreaterTimestamp,
        )
    ]
    assert len(non_ok_reps) == 10
