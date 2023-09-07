# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    DateTime,
    LocalDevice,
    RealmArchivingCertificate,
    RealmArchivingConfiguration,
    RealmID,
    RealmUpdateArchivingRepArchivingPeriodTooShort,
    RealmUpdateArchivingRepBadTimestamp,
    RealmUpdateArchivingRepInvalidCertification,
    RealmUpdateArchivingRepNotAllowed,
    RealmUpdateArchivingRepNotFound,
    RealmUpdateArchivingRepOk,
    RealmUpdateArchivingRepRealmDeleted,
    RealmUpdateArchivingRepRequireGreaterTimestamp,
)
from parsec.backend.events import BackendEvent
from tests.backend.common import realm_update_archiving


@pytest.mark.trio
async def test_create_realm_archiving_certificate(
    alice: LocalDevice,
    realm: RealmID,
):
    now = DateTime.now()

    available = RealmArchivingConfiguration.available()
    assert available.str == "AVAILABLE"
    assert available.is_available()

    archived = RealmArchivingConfiguration.archived()
    assert archived.str == "ARCHIVED"
    assert archived.is_archived()

    deletion_planned = RealmArchivingConfiguration.deletion_planned(now.add(days=31))
    assert deletion_planned.str == "DELETION_PLANNED"
    assert deletion_planned.deletion_date == now.add(days=31)
    assert deletion_planned.is_deletion_planned()
    assert not deletion_planned.is_deleted()
    assert not deletion_planned.is_deleted(now=now.add(days=30))
    assert deletion_planned.is_deleted(now=now.add(days=31))
    assert deletion_planned.is_deleted(now=now.add(days=32))

    deletion_planned = RealmArchivingConfiguration.deletion_planned(now)
    assert deletion_planned.str == "DELETION_PLANNED"
    assert deletion_planned.deletion_date == now
    assert deletion_planned.is_deletion_planned()
    assert deletion_planned.is_deleted()

    for config in (available, archived, deletion_planned):
        certificate = RealmArchivingCertificate(
            author=alice.device_id,
            timestamp=now,
            realm_id=realm,
            configuration=config,
        )
        assert certificate.author == alice.device_id
        assert certificate.timestamp == now
        assert certificate.realm_id == realm
        assert certificate.configuration == config

        signed = certificate.dump_and_sign(alice.signing_key)
        unsecure_certificate = RealmArchivingCertificate.unsecure_load(signed)
        assert unsecure_certificate == certificate
        verified_certificate = RealmArchivingCertificate.verify_and_load(
            signed,
            alice.verify_key,
            expected_author=alice.device_id,
            expected_realm=realm,
        )
        assert verified_certificate == certificate


@pytest.fixture
def realm_update_archiving_helper(next_timestamp):
    async def _realm_update_archiving(ws, author, realm_id, configuration, timestamp=None):
        certif = RealmArchivingCertificate(
            author=author.device_id,
            timestamp=timestamp or next_timestamp(),
            realm_id=realm_id,
            configuration=configuration,
        ).dump_and_sign(author.signing_key)
        return await realm_update_archiving(ws, certif, check_rep=False)

    return _realm_update_archiving


@pytest.mark.trio
async def test_realm_update_archiving_ok(
    alice_ws,
    alice: LocalDevice,
    realm: RealmID,
    realm_update_archiving_helper,
    backend,
):
    now = DateTime.now()
    available = RealmArchivingConfiguration.available()
    archived = RealmArchivingConfiguration.archived()
    deletion_planned = RealmArchivingConfiguration.deletion_planned(now.add(days=31))

    for _ in range(3):
        for configuration in (available, archived, deletion_planned):
            with backend.event_bus.listen() as spy:
                rep = await realm_update_archiving_helper(
                    alice_ws,
                    alice,
                    realm,
                    configuration,
                )
                assert isinstance(rep, RealmUpdateArchivingRepOk)
                await spy.wait_with_timeout(BackendEvent.REALM_ARCHIVING_UPDATED)
            (event,) = spy.events
            assert event.kwargs == {
                "author": alice.device_id,
                "configuration": configuration,
                "organization_id": alice.organization_id,
                "realm_id": realm,
            }


@pytest.mark.trio
async def test_realm_update_archiving_not_allowed(
    bob_ws,
    bob: LocalDevice,
    realm: RealmID,
    realm_update_archiving_helper,
):
    available = RealmArchivingConfiguration.available()

    rep = await realm_update_archiving_helper(
        bob_ws,
        bob,
        realm,
        available,
    )
    assert isinstance(rep, RealmUpdateArchivingRepNotAllowed)


@pytest.mark.trio
async def test_realm_update_archiving_invalid_certification(
    alice_ws,
    bob: LocalDevice,
    realm: RealmID,
    realm_update_archiving_helper,
):
    available = RealmArchivingConfiguration.available()

    rep = await realm_update_archiving_helper(
        alice_ws,
        bob,
        realm,
        available,
    )
    assert isinstance(rep, RealmUpdateArchivingRepInvalidCertification)


@pytest.mark.trio
async def test_realm_update_archiving_realm_not_found(
    alice_ws,
    alice: LocalDevice,
    realm_update_archiving_helper,
):
    available = RealmArchivingConfiguration.available()

    rep = await realm_update_archiving_helper(
        alice_ws,
        alice,
        RealmID.new(),
        available,
    )
    assert isinstance(rep, RealmUpdateArchivingRepNotFound)


@pytest.mark.trio
async def test_realm_update_archiving_require_greater_timestamp(
    alice_ws,
    alice: LocalDevice,
    realm: RealmID,
    realm_update_archiving_helper,
):
    now = DateTime.now()
    available = RealmArchivingConfiguration.available()

    rep = await realm_update_archiving_helper(
        alice_ws,
        alice,
        realm,
        available,
        timestamp=now,
    )
    assert isinstance(rep, RealmUpdateArchivingRepOk)
    rep = await realm_update_archiving_helper(
        alice_ws,
        alice,
        realm,
        available,
        timestamp=now,
    )
    assert isinstance(rep, RealmUpdateArchivingRepRequireGreaterTimestamp)
    assert rep.strictly_greater_than == now


@pytest.mark.trio
async def test_realm_update_archiving_bad_timestamp(
    alice_ws,
    alice: LocalDevice,
    realm: RealmID,
    realm_update_archiving_helper,
):
    now = DateTime.now()
    available = RealmArchivingConfiguration.available()

    rep = await realm_update_archiving_helper(
        alice_ws,
        alice,
        realm,
        available,
        timestamp=now.subtract(days=1),
    )
    assert isinstance(rep, RealmUpdateArchivingRepBadTimestamp)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert now <= rep.backend_timestamp <= now.add(seconds=1)
    assert rep.client_timestamp == now.subtract(days=1)


@pytest.mark.trio
async def test_realm_update_archiving_realm_deleted(
    alice_ws,
    alice: LocalDevice,
    deleted_realm: RealmID,
    realm_update_archiving_helper,
):
    DateTime.now()
    available = RealmArchivingConfiguration.available()

    rep = await realm_update_archiving_helper(
        alice_ws,
        alice,
        deleted_realm,
        available,
    )
    assert isinstance(rep, RealmUpdateArchivingRepRealmDeleted)


@pytest.mark.trio
async def test_realm_update_archiving_archiving_period_too_short(
    alice_ws,
    alice: LocalDevice,
    realm: RealmID,
    realm_update_archiving_helper,
):
    now = DateTime.now()
    deletion_planned = RealmArchivingConfiguration.deletion_planned(now.add(days=10))

    rep = await realm_update_archiving_helper(
        alice_ws,
        alice,
        realm,
        deletion_planned,
    )
    assert isinstance(rep, RealmUpdateArchivingRepArchivingPeriodTooShort)
