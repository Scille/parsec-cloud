# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from parsec._parsec import DateTime

from parsec.api.data import RealmRoleCertificateContent, UserProfile
from parsec.api.protocol import RealmID, RealmRole
from parsec.backend.backend_events import BackendEvent
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET

from tests.common import freeze_time, customize_fixtures
from tests.backend.test_events import events_subscribe
from tests.backend.common import realm_create


async def _test_create_ok(backend, device, ws):
    await events_subscribe(ws)

    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    certif = RealmRoleCertificateContent.build_realm_root_certif(
        author=device.device_id, timestamp=DateTime.now(), realm_id=realm_id
    ).dump_and_sign(device.signing_key)
    with backend.event_bus.listen() as spy:
        rep = await realm_create(ws, certif)
        assert rep == {"status": "ok"}
        await spy.wait_with_timeout(BackendEvent.REALM_ROLES_UPDATED)


@pytest.mark.trio
async def test_create_ok(backend, alice, alice_ws):
    await _test_create_ok(backend, alice, alice_ws)


@pytest.mark.trio
@customize_fixtures(alice_profile=UserProfile.OUTSIDER)
async def test_create_allowed_for_outsider(backend, alice, alice_ws):
    await _test_create_ok(backend, alice, alice_ws)


@pytest.mark.trio
async def test_create_invalid_certif(bob, alice_ws):
    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    certif = RealmRoleCertificateContent.build_realm_root_certif(
        author=bob.device_id, timestamp=DateTime.now(), realm_id=realm_id
    ).dump_and_sign(bob.signing_key)
    rep = await realm_create(alice_ws, certif)
    assert rep == {
        "status": "invalid_certification",
        "reason": "Invalid certification data (Signature was forged or corrupt).",
    }


@pytest.mark.trio
async def test_create_certif_not_self_signed(alice, bob, alice_ws):
    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    certif = RealmRoleCertificateContent(
        author=alice.device_id,
        timestamp=DateTime.now(),
        realm_id=realm_id,
        user_id=bob.user_id,
        role=RealmRole.OWNER,
    ).dump_and_sign(alice.signing_key)
    rep = await realm_create(alice_ws, certif)
    assert rep == {
        "status": "invalid_data",
        "reason": "Initial realm role certificate must be self-signed.",
    }


@pytest.mark.trio
async def test_create_certif_role_not_owner(alice, alice_ws):
    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    certif = RealmRoleCertificateContent(
        author=alice.device_id,
        timestamp=DateTime.now(),
        realm_id=realm_id,
        user_id=alice.user_id,
        role=RealmRole.MANAGER,
    ).dump_and_sign(alice.signing_key)
    rep = await realm_create(alice_ws, certif)
    assert rep == {
        "status": "invalid_data",
        "reason": "Initial realm role certificate must set OWNER role.",
    }


@pytest.mark.trio
async def test_create_certif_too_old(alice, alice_ws):
    now = DateTime.now()

    # Generate a certificate

    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    certif = RealmRoleCertificateContent.build_realm_root_certif(
        author=alice.device_id, timestamp=now, realm_id=realm_id
    ).dump_and_sign(alice.signing_key)

    # Create a realm a tiny bit too late

    later = now.add(seconds=BALLPARK_CLIENT_LATE_OFFSET)
    with freeze_time(later):
        rep = await realm_create(alice_ws, certif)
    assert rep == {
        "status": "bad_timestamp",
        "backend_timestamp": later,
        "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
        "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
        "client_timestamp": now,
    }

    #  Create a realm late but right before the deadline

    later = now.add(seconds=BALLPARK_CLIENT_LATE_OFFSET, microseconds=-1)
    with freeze_time(later):
        rep = await realm_create(alice_ws, certif)
    assert rep["status"] == "ok"

    # Generate a new certificate

    realm_id = RealmID.from_hex("C0000000000000000000000000000001")
    certif = RealmRoleCertificateContent.build_realm_root_certif(
        author=alice.device_id, timestamp=now, realm_id=realm_id
    ).dump_and_sign(alice.signing_key)

    # Create a realm a tiny bit too soon

    sooner = now.subtract(seconds=BALLPARK_CLIENT_EARLY_OFFSET)
    with freeze_time(sooner):
        rep = await realm_create(alice_ws, certif)
    assert rep == {
        "status": "bad_timestamp",
        "backend_timestamp": sooner,
        "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
        "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
        "client_timestamp": now,
    }

    # Create a realm soon but after the limit

    sooner = now.subtract(seconds=BALLPARK_CLIENT_EARLY_OFFSET, microseconds=-1)
    with freeze_time(sooner):
        rep = await realm_create(alice_ws, certif)
    assert rep["status"] == "ok"


@pytest.mark.trio
async def test_create_realm_already_exists(alice, alice_ws, realm):
    certif = RealmRoleCertificateContent.build_realm_root_certif(
        author=alice.device_id, timestamp=DateTime.now(), realm_id=realm
    ).dump_and_sign(alice.signing_key)
    rep = await realm_create(alice_ws, certif)
    assert rep == {"status": "already_exists"}
