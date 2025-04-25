# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import asyncio
from unittest.mock import ANY, Mock

import pytest

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    RealmRole,
    RealmRoleCertificate,
    SecretKeyAlgorithm,
    VlobID,
    authenticated_cmds,
    testbed,
)
from parsec.components.sequester import (
    RejectedBySequesterService,
    SequesterServiceType,
    SequesterServiceUnavailable,
)
from parsec.events import EVENT_VLOB_MAX_BLOB_SIZE, EventVlob
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    SequesteredOrgRpcClients,
    get_last_realm_certificate_timestamp,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)


@pytest.mark.parametrize(
    "kind",
    (
        "key_index_0",
        "as_owner",
        "as_manager",
        "as_contributor",
    ),
)
async def test_authenticated_vlob_update_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    realm_id = coolorg.wksp1_id
    key_index = 1
    last_realm_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=realm_id,
    )

    match kind:
        case "key_index_0":
            assert isinstance(coolorg.alice.event, testbed.TestbedEventBootstrapOrganization)
            realm_id = coolorg.alice.event.first_user_user_realm_id
            key_index = 0
            last_realm_certificate_timestamp = get_last_realm_certificate_timestamp(
                testbed_template=coolorg.testbed_template,
                realm_id=realm_id,
            )
            author = coolorg.alice

        case "as_owner":
            author = coolorg.alice

        case "as_manager":
            last_realm_certificate_timestamp = DateTime.now()
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.MANAGER,
                now=last_realm_certificate_timestamp,
            )
            author = coolorg.bob

        case "as_contributor":
            last_realm_certificate_timestamp = DateTime.now()
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.CONTRIBUTOR,
                now=last_realm_certificate_timestamp,
            )
            author = coolorg.bob

        case unknown:
            assert False, unknown

    vlob_id = VlobID.new()
    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    v1_blob = b"<block content 1>"
    v1_timestamp = DateTime.now()
    outcome = await backend.vlob.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=realm_id,
        vlob_id=vlob_id,
        key_index=key_index,
        blob=v1_blob,
        timestamp=v1_timestamp,
    )
    assert outcome is None, outcome

    with backend.event_bus.spy() as spy:
        v2_blob = b"<block content 2>"
        v2_timestamp = DateTime.now()
        rep = await author.vlob_update(
            realm_id=realm_id,
            vlob_id=vlob_id,
            key_index=key_index,
            version=2,
            blob=v2_blob,
            timestamp=v2_timestamp,
        )
        assert rep == authenticated_cmds.latest.vlob_update.RepOk()

        await spy.wait_event_occurred(
            EventVlob(
                organization_id=coolorg.organization_id,
                author=author.device_id,
                realm_id=realm_id,
                timestamp=v2_timestamp,
                vlob_id=vlob_id,
                version=2,
                blob=v2_blob,
                last_common_certificate_timestamp=DateTime(2000, 1, 6),
                last_realm_certificate_timestamp=last_realm_certificate_timestamp,
            )
        )

    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == {
        **initial_dump,
        realm_id: {
            **initial_dump[realm_id],
            vlob_id: [
                (coolorg.alice.device_id, ANY, v1_blob),
                (author.device_id, ANY, v2_blob),
            ],
        },
    }


@pytest.mark.parametrize(
    "kind",
    (
        "as_reader",
        "not_member",
        "no_longer_allowed",
        "bad_key_index_and_not_allowed",
        "vlob_version_already_exists_and_not_allowed",
        "require_greater_timestamp_and_not_allowed",
    ),
)
async def test_authenticated_vlob_update_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    t0 = DateTime(2009, 12, 31)
    vlob_id = VlobID.new()
    outcome = await backend.vlob.create(
        now=t0,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        blob=b"<block content 1>",
        timestamp=t0,
    )
    assert outcome is None, outcome

    key_index = 1
    now = DateTime.now()

    match kind:
        case "as_reader":
            author = coolorg.bob

        case "not_member":
            author = coolorg.mallory

        case "no_longer_allowed":
            await wksp1_bob_becomes_owner_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_role=RealmRole.READER
            )
            author = coolorg.alice

        case "bad_key_index_and_not_allowed":
            key_index = 42
            author = coolorg.mallory

        case "vlob_already_exists_and_not_allowed":
            outcome = await backend.vlob.create(
                now=now,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                realm_id=coolorg.wksp1_id,
                vlob_id=vlob_id,
                key_index=1,
                timestamp=now,
                blob=b"<dummy>",
            )
            assert outcome is None
            now = DateTime.now()
            author = coolorg.mallory

        case "require_greater_timestamp_and_not_allowed":
            # Just create any certificate in the realm
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.READER
            )
            # Bob is only `READER` in realm `wksp1`
            author = coolorg.bob

        case "bad_key_index_and_not_allowed":
            key_index = 42
            author = coolorg.mallory

        case "vlob_version_already_exists_and_not_allowed":
            outcome = await backend.vlob.update(
                now=now,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                realm_id=coolorg.wksp1_id,
                vlob_id=vlob_id,
                key_index=1,
                timestamp=now,
                version=2,
                blob=b"<dummy>",
            )
            assert outcome is None
            now = DateTime.now()
            author = coolorg.mallory

        case "require_greater_timestamp_and_not_allowed":
            # Just create any certificate in the realm
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.READER
            )
            # Bob is only `READER` in realm `wksp1`
            author = coolorg.bob

        case unknown:
            assert False, unknown

    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    rep = await author.vlob_update(
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=key_index,
        timestamp=now,
        version=2,
        blob=b"<block content>",
    )
    assert rep == authenticated_cmds.latest.vlob_update.RepAuthorNotAllowed()

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == initial_dump


@pytest.mark.parametrize("kind", ("index_1_too_old", "index_0_too_old", "index_2_unknown"))
async def test_authenticated_vlob_update_bad_key_index(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    t0 = DateTime(2009, 12, 31)
    t1 = DateTime(2010, 1, 1)
    t2 = DateTime(2010, 1, 1)

    vlob_id = VlobID.new()
    outcome = await backend.vlob.create(
        now=t0,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        blob=b"<block content 1>",
        timestamp=t0,
    )
    assert outcome is None, outcome

    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    wksp1_last_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=coolorg.wksp1_id,
    )

    match kind:
        case "index_1_too_old":
            certif = RealmKeyRotationCertificate(
                author=coolorg.alice.device_id,
                timestamp=t1,
                realm_id=coolorg.wksp1_id,
                key_index=2,
                encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
                hash_algorithm=HashAlgorithm.SHA256,
                key_canary=b"<dummy canary>",
            )
            outcome = await backend.realm.rotate_key(
                now=t1,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                realm_key_rotation_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
                keys_bundle=b"<dummy keys bundle>",
                per_participant_keys_bundle_access={
                    coolorg.alice.user_id: b"<dummy alice keys bundle access>",
                    coolorg.bob.user_id: b"<dummy bob keys bundle access>",
                },
            )
            assert isinstance(outcome, RealmKeyRotationCertificate)
            bad_key_index = 1
            wksp1_last_certificate_timestamp = t1

        case "index_0_too_old":
            bad_key_index = 0

        case "index_2_unknown":
            bad_key_index = 2

        case unknown:
            assert False, unknown

    rep = await coolorg.alice.vlob_update(
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=bad_key_index,
        timestamp=t2,
        version=2,
        blob=b"<block content>",
    )
    assert rep == authenticated_cmds.latest.vlob_update.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp1_last_certificate_timestamp
    )

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == initial_dump


async def test_authenticated_vlob_update_realm_not_found(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.vlob_update(
        realm_id=VlobID.new(),
        vlob_id=coolorg.wksp1_id,
        key_index=1,
        timestamp=DateTime.now(),
        version=2,
        blob=b"<block content>",
    )
    assert rep == authenticated_cmds.latest.vlob_update.RepRealmNotFound()


async def test_authenticated_vlob_update_vlob_not_found(
    backend: Backend, coolorg: CoolorgRpcClients, minimalorg: MinimalorgRpcClients
) -> None:
    # Create another realm to ensure vlob ID cannot be found in the wrong realm
    t0 = DateTime.now()
    other_realm_id = VlobID.new()
    vlob_id_from_other_realm = VlobID.new()
    await backend.realm.create(
        now=t0,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=t0,
            realm_id=other_realm_id,
            user_id=coolorg.alice.user_id,
            role=RealmRole.OWNER,
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    t1 = DateTime.now()
    await backend.vlob.create(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=other_realm_id,
        vlob_id=vlob_id_from_other_realm,
        key_index=0,
        timestamp=t1,
        blob=b"dummy",
    )

    rep = await coolorg.alice.vlob_update(
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id_from_other_realm,
        key_index=1,
        timestamp=DateTime.now(),
        version=2,
        blob=b"<block content>",
    )
    assert rep == authenticated_cmds.latest.vlob_update.RepVlobNotFound()


@pytest.mark.parametrize(
    "kind", ("version_0_never_possible", "version_1_already_exists", "version_3_not_the_next_one")
)
async def test_authenticated_vlob_update_bad_vlob_version(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    t0 = DateTime(2009, 12, 31)
    vlob_id = VlobID.new()
    outcome = await backend.vlob.create(
        now=t0,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        blob=b"<block content 1>",
        timestamp=t0,
    )
    assert outcome is None, outcome

    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    match kind:
        case "version_0_never_possible":
            bad_version = 0
        case "version_1_already_exists":
            bad_version = 1
        case "version_3_not_the_next_one":
            bad_version = 3
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.vlob_update(
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        timestamp=DateTime.now(),
        version=bad_version,
        blob=b"<block content>",
    )
    assert rep == authenticated_cmds.latest.vlob_update.RepBadVlobVersion()

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == initial_dump


async def test_authenticated_vlob_update_rejected_by_sequester_service(
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
        RejectedBySequesterService(service_id=sequestered_org.sequester_service_2_id, reason=None)
    )
    _mocked_sequester_service_on_vlob_create_or_update = Mock(side_effect=[future])
    monkeypatch.setattr(
        "parsec.webhooks.WebhooksComponent.sequester_service_on_vlob_create_or_update",
        _mocked_sequester_service_on_vlob_create_or_update,
    )

    initial_dump = await backend.vlob.test_dump_vlobs(
        organization_id=sequestered_org.organization_id
    )

    rep = await sequestered_org.alice.vlob_update(
        realm_id=sequestered_org.wksp1_id,
        vlob_id=sequestered_org.wksp1_id,
        version=2,
        key_index=3,
        timestamp=DateTime.now(),
        blob=b"<block content>",
    )
    assert rep == authenticated_cmds.latest.vlob_update.RepRejectedBySequesterService(
        service_id=sequestered_org.sequester_service_2_id, reason=None
    )

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=sequestered_org.organization_id)
    assert dump == initial_dump


async def test_authenticated_vlob_update_sequester_service_unavailable(
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
    _mocked_sequester_service_on_vlob_create_or_update = Mock(side_effect=[future])
    monkeypatch.setattr(
        "parsec.webhooks.WebhooksComponent.sequester_service_on_vlob_create_or_update",
        _mocked_sequester_service_on_vlob_create_or_update,
    )

    initial_dump = await backend.vlob.test_dump_vlobs(
        organization_id=sequestered_org.organization_id
    )

    rep = await sequestered_org.alice.vlob_update(
        realm_id=sequestered_org.wksp1_id,
        vlob_id=sequestered_org.wksp1_id,
        version=2,
        key_index=3,
        timestamp=DateTime.now(),
        blob=b"<block content>",
    )
    assert rep == authenticated_cmds.latest.vlob_update.RepSequesterServiceUnavailable(
        service_id=sequestered_org.sequester_service_2_id
    )

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=sequestered_org.organization_id)
    assert dump == initial_dump


async def test_authenticated_vlob_update_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    t0 = DateTime.now().subtract(seconds=3600)
    rep = await coolorg.alice.vlob_update(
        realm_id=coolorg.wksp1_id,
        vlob_id=coolorg.wksp1_id,
        key_index=1,
        version=2,
        timestamp=t0,
        blob=b"<block content>",
    )
    assert isinstance(rep, authenticated_cmds.latest.vlob_update.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == initial_dump


@pytest.mark.parametrize(
    "timestamp_offset",
    (pytest.param(0, id="same_timestamp"), pytest.param(1, id="previous_timestamp")),
)
async def test_authenticated_vlob_update_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_offset: int,
) -> None:
    last_certificate_timestamp = DateTime.now()
    same_or_previous_timestamp = last_certificate_timestamp.subtract(seconds=timestamp_offset)

    # 1) Rotate key to add a new certificate with last_certificate_timestamp

    author = coolorg.alice.device_id
    realm_id = coolorg.wksp1_id
    vlob_id = coolorg.wksp1_id
    organization_id = coolorg.organization_id

    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=organization_id)

    outcome = await backend.realm.rotate_key(
        now=last_certificate_timestamp,
        organization_id=organization_id,
        author=author,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        keys_bundle=b"",
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"",
            coolorg.bob.user_id: b"",
        },
        realm_key_rotation_certificate=RealmKeyRotationCertificate(
            author=author,
            timestamp=last_certificate_timestamp,
            hash_algorithm=HashAlgorithm.SHA256,
            encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
            key_index=2,
            realm_id=realm_id,
            key_canary=b"",
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    # 2) Try to update a vlob with same or previous timestamp
    #    than the certificate created via key rotation above

    rep = await coolorg.alice.vlob_update(
        realm_id=realm_id,
        vlob_id=vlob_id,
        key_index=2,
        timestamp=same_or_previous_timestamp,
        version=2,
        blob=b"<updated block content>",
    )
    assert rep == authenticated_cmds.latest.vlob_update.RepRequireGreaterTimestamp(
        strictly_greater_than=last_certificate_timestamp
    )

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=organization_id)
    assert dump == initial_dump


async def test_authenticated_vlob_update_max_blob_size(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    author = coolorg.alice.device_id
    realm_id = coolorg.wksp1_id
    organization_id = coolorg.organization_id
    vlob_id = VlobID.new()
    v1_blob = b"<initial block content>"
    v1_timestamp = DateTime.now()
    key_index = 1
    outcome = await backend.vlob.create(
        now=DateTime.now(),
        organization_id=organization_id,
        author=author,
        realm_id=realm_id,
        vlob_id=vlob_id,
        key_index=key_index,
        blob=v1_blob,
        timestamp=v1_timestamp,
    )
    assert outcome is None, outcome

    v2_timestamp = DateTime.now()
    with backend.event_bus.spy() as spy:
        v2_blob = bytes(EVENT_VLOB_MAX_BLOB_SIZE)
        rep = await coolorg.alice.vlob_update(
            realm_id=realm_id,
            vlob_id=vlob_id,
            key_index=key_index,
            timestamp=v2_timestamp,
            version=2,
            blob=v2_blob,
        )
        assert rep == authenticated_cmds.latest.vlob_update.RepOk()

        await spy.wait_event_occurred(
            EventVlob(
                organization_id=organization_id,
                author=author,
                realm_id=realm_id,
                timestamp=v2_timestamp,
                vlob_id=vlob_id,
                version=2,
                blob=None,  # Event should be sent without the blob!
                last_common_certificate_timestamp=DateTime(2000, 1, 6),
                last_realm_certificate_timestamp=DateTime(2000, 1, 12),
            )
        )

    dump = await backend.vlob.test_dump_vlobs(organization_id=organization_id)
    assert dump == {
        **initial_dump,
        realm_id: {
            **initial_dump[realm_id],
            vlob_id: [
                (author, ANY, v1_blob),
                (author, ANY, v2_blob),
            ],
        },
    }


async def test_authenticated_vlob_update_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    vlob_id = VlobID.new()
    v1_blob = b"<block content 1>"
    v1_timestamp = DateTime.now()
    outcome = await backend.vlob.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        blob=v1_blob,
        timestamp=v1_timestamp,
    )
    assert outcome is None, outcome

    async def do():
        v2_blob = b"<block content 2>"
        v2_timestamp = DateTime.now()
        await coolorg.alice.vlob_update(
            realm_id=coolorg.wksp1_id,
            vlob_id=vlob_id,
            key_index=1,
            version=2,
            blob=v2_blob,
            timestamp=v2_timestamp,
        )

    await authenticated_http_common_errors_tester(do)
