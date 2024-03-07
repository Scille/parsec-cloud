# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    SecretKeyAlgorithm,
    SequesterServiceID,
    VlobID,
    authenticated_cmds,
)
from parsec.events import EVENT_VLOB_MAX_BLOB_SIZE, EventVlob
from tests.common import Backend, CoolorgRpcClients, get_last_realm_certificate_timestamp


async def test_authenticated_vlob_update_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    vlob_id = VlobID.new()
    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

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
        sequester_blob=None,
    )
    assert outcome is None, outcome

    with backend.event_bus.spy() as spy:
        v2_blob = b"<block content 2>"
        v2_timestamp = DateTime.now()
        rep = await coolorg.alice.vlob_update(
            vlob_id=vlob_id,
            key_index=1,
            version=2,
            blob=v2_blob,
            timestamp=v2_timestamp,
            sequester_blob=None,
        )
        assert rep == authenticated_cmds.v4.vlob_update.RepOk()

        await spy.wait_event_occurred(
            EventVlob(
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                realm_id=coolorg.wksp1_id,
                timestamp=v2_timestamp,
                vlob_id=vlob_id,
                version=2,
                blob=v2_blob,
                last_common_certificate_timestamp=DateTime(2000, 1, 6),
                last_realm_certificate_timestamp=DateTime(2000, 1, 12),
            )
        )

    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == {
        **initial_dump,
        vlob_id: [
            (coolorg.alice.device_id, ANY, coolorg.wksp1_id, v1_blob),
            (coolorg.alice.device_id, ANY, coolorg.wksp1_id, v2_blob),
        ],
    }


async def test_authenticated_vlob_update_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend
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
        sequester_blob=None,
    )
    assert outcome is None, outcome

    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    rep = await coolorg.mallory.vlob_update(
        vlob_id=vlob_id,
        key_index=1,
        timestamp=DateTime.now(),
        version=2,
        blob=b"<block content>",
        sequester_blob=None,
    )
    assert rep == authenticated_cmds.v4.vlob_update.RepAuthorNotAllowed()

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
        sequester_blob=None,
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
                encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
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
        vlob_id=vlob_id,
        key_index=bad_key_index,
        timestamp=t2,
        version=2,
        blob=b"<block content>",
        sequester_blob=None,
    )
    assert rep == authenticated_cmds.v4.vlob_update.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp1_last_certificate_timestamp
    )

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == initial_dump


async def test_authenticated_vlob_update_vlob_not_found(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.vlob_update(
        vlob_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        version=2,
        blob=b"<block content>",
        sequester_blob=None,
    )
    assert rep == authenticated_cmds.v4.vlob_update.RepVlobNotFound()


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
        sequester_blob=None,
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
        vlob_id=vlob_id,
        key_index=1,
        timestamp=DateTime.now(),
        version=bad_version,
        blob=b"<block content>",
        sequester_blob=None,
    )
    assert rep == authenticated_cmds.v4.vlob_update.RepBadVlobVersion()

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == initial_dump


async def test_authenticated_vlob_update_organization_not_sequestered(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    vlob_id = VlobID.new()
    v1_timestamp = DateTime.now()
    key_index = 1
    outcome = await backend.vlob.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=key_index,
        blob=b"<initial block content>",
        timestamp=v1_timestamp,
        sequester_blob=None,
    )
    assert outcome is None, outcome
    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    v2_timestamp = DateTime.now()
    rep = await coolorg.alice.vlob_update(
        vlob_id=vlob_id,
        key_index=key_index,
        timestamp=v2_timestamp,
        version=2,
        blob=b"<updated block content>",
        sequester_blob={SequesterServiceID.new(): b"<dummy>"},
    )
    assert rep == authenticated_cmds.v4.vlob_update.RepOrganizationNotSequestered()

    # Ensure no changes were made
    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == initial_dump


@pytest.mark.skip(reason="TODO: missing test sequester")
async def test_authenticated_vlob_update_sequester_inconsistency(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    raise Exception("Not implemented")


@pytest.mark.skip(reason="TODO: missing test sequester")
async def test_authenticated_vlob_update_rejected_by_sequester_service(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    raise Exception("Not implemented")


@pytest.mark.skip(reason="TODO: missing test sequester")
async def test_authenticated_vlob_update_sequester_service_unavailable(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    raise Exception("Not implemented")


async def test_authenticated_vlob_update_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    t0 = DateTime.now().subtract(seconds=3600)
    rep = await coolorg.alice.vlob_create(
        realm_id=coolorg.wksp1_id,
        vlob_id=VlobID.new(),
        key_index=1,
        timestamp=t0,
        blob=b"<block content>",
        sequester_blob=None,
    )
    assert isinstance(rep, authenticated_cmds.v4.vlob_create.RepTimestampOutOfBallpark)
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
async def test_authenticated_vlob_create_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_offset: int,
) -> None:
    dt1 = DateTime.now()
    dt2 = dt1.add(seconds=10)
    bad_timestamp = dt2.subtract(seconds=timestamp_offset)

    # 1) Create a first vlob

    author = coolorg.alice.device_id
    realm_id = coolorg.wksp1_id
    organization_id = coolorg.organization_id
    vlob_id = VlobID.new()
    outcome = await backend.vlob.create(
        now=dt1,
        organization_id=organization_id,
        author=author,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        blob=b"<initial block content>",
        timestamp=dt1,
        sequester_blob=None,
    )
    assert outcome is None, outcome

    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=organization_id)

    outcome = await backend.realm.rotate_key(
        now=dt2,
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
            timestamp=dt2,
            hash_algorithm=HashAlgorithm.SHA256,
            encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
            key_index=2,
            realm_id=realm_id,
            key_canary=b"",
        ).dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    # 2) Create second vlob with same or previous timestamp

    rep = await coolorg.alice.vlob_update(
        vlob_id=vlob_id,
        key_index=2,
        timestamp=bad_timestamp,
        version=2,
        blob=b"<updated block content>",
        sequester_blob=None,
    )
    assert rep == authenticated_cmds.v4.vlob_update.RepRequireGreaterTimestamp(
        strictly_greater_than=dt2
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
        sequester_blob=None,
    )
    assert outcome is None, outcome

    v2_timestamp = DateTime.now()
    with backend.event_bus.spy() as spy:
        v2_blob = bytes(EVENT_VLOB_MAX_BLOB_SIZE)
        rep = await coolorg.alice.vlob_update(
            vlob_id=vlob_id,
            key_index=key_index,
            timestamp=v2_timestamp,
            version=2,
            blob=v2_blob,
            sequester_blob=None,
        )
        assert rep == authenticated_cmds.v4.vlob_update.RepOk()

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
        vlob_id: [
            (author, ANY, realm_id, v1_blob),
            (author, ANY, realm_id, v2_blob),
        ],
    }
