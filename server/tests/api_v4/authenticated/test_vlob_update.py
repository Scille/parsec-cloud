# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    SecretKeyAlgorithm,
    VlobID,
    authenticated_cmds,
)
from parsec.events import EventVlob
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


# TODO: check that blob bigger than EVENT_VLOB_MAX_BLOB_SIZE doesn't get in the event
