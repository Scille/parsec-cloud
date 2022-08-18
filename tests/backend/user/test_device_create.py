# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.backend.backend_events import BackendEvent
import pytest
from parsec._parsec import DateTime

from parsec.api.data import DeviceCertificate
from parsec.api.protocol import UserProfile
from parsec.backend.user import INVITATION_VALIDITY, Device

from tests.common import freeze_time, customize_fixtures
from tests.backend.common import device_create, ping


@pytest.fixture
def alice_nd(local_device_factory, alice):
    return local_device_factory(f"{alice.user_id}@new_device")


@pytest.mark.trio
@customize_fixtures(
    alice_profile=UserProfile.OUTSIDER
)  # Any profile is be allowed to create new devices
@pytest.mark.parametrize("with_labels", [False, True])
async def test_device_create_ok(
    backend_asgi_app, backend_authenticated_ws_factory, alice_ws, alice, alice_nd, with_labels
):
    now = DateTime.now()
    device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        device_label=alice_nd.device_label,
        verify_key=alice_nd.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)
    if not with_labels:
        device_certificate = redacted_device_certificate
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(alice.signing_key)

    with backend_asgi_app.backend.event_bus.listen() as spy:
        rep = await device_create(
            alice_ws,
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert rep == {"status": "ok"}

        # No guarantees this event occurs before the command's return
        await spy.wait_with_timeout(
            BackendEvent.DEVICE_CREATED,
            {
                "organization_id": alice_nd.organization_id,
                "device_id": alice_nd.device_id,
                "device_certificate": device_certificate,
                "encrypted_answer": b"",
            },
        )

    # Make sure the new device can connect now
    async with backend_authenticated_ws_factory(backend_asgi_app, alice_nd) as sock:
        rep = await ping(sock, ping="Hello world !")
        assert rep == {"status": "ok", "pong": "Hello world !"}

    # Check the resulting data in the backend
    _, backend_device = await backend_asgi_app.backend.user.get_user_with_device(
        alice_nd.organization_id, alice_nd.device_id
    )
    assert backend_device == Device(
        device_id=alice_nd.device_id,
        device_label=alice_nd.device_label if with_labels else None,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
        device_certifier=alice.device_id,
        created_on=now,
    )


@pytest.mark.trio
async def test_device_create_invalid_certified(alice_ws, alice, bob, alice_nd):
    now = DateTime.now()
    good_device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        device_label=None,  # Can be used as regular and redacted certificate
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(alice.signing_key)
    bad_device_certificate = DeviceCertificate(
        author=bob.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        device_label=None,  # Can be used as regular and redacted certificate
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(bob.signing_key)

    rep = await device_create(
        alice_ws,
        device_certificate=bad_device_certificate,
        redacted_device_certificate=good_device_certificate,
    )
    assert rep == {
        "status": "invalid_certification",
        "reason": "Invalid certification data (Signature was forged or corrupt).",
    }

    # Same for the redacted part

    rep = await device_create(
        alice_ws,
        device_certificate=good_device_certificate,
        redacted_device_certificate=bad_device_certificate,
    )
    assert rep == {
        "status": "invalid_certification",
        "reason": "Invalid certification data (Signature was forged or corrupt).",
    }


@pytest.mark.trio
async def test_device_create_already_exists(alice_ws, alice, alice2):
    now = DateTime.now()
    device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=alice2.device_id,
        device_label=None,
        verify_key=alice2.verify_key,
    ).dump_and_sign(alice.signing_key)

    rep = await device_create(
        alice_ws,
        device_certificate=device_certificate,
        redacted_device_certificate=device_certificate,
    )
    assert rep == {
        "status": "already_exists",
        "reason": f"Device `{alice2.device_id}` already exists",
    }


@pytest.mark.trio
async def test_device_create_not_own_user(bob_ws, bob, alice_nd):
    now = DateTime.now()
    device_certificate = DeviceCertificate(
        author=bob.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        device_label=None,
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(bob.signing_key)

    rep = await device_create(
        bob_ws,
        device_certificate=device_certificate,
        redacted_device_certificate=device_certificate,
    )
    assert rep == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}


@pytest.mark.trio
async def test_device_create_certify_too_old(alice_ws, alice, alice_nd):
    now = DateTime(2000, 1, 1)
    device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        device_label=None,
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(alice.signing_key)

    with freeze_time(now.add(seconds=INVITATION_VALIDITY + 1)):
        rep = await device_create(
            alice_ws,
            device_certificate=device_certificate,
            redacted_device_certificate=device_certificate,
        )
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid timestamp in certification.",
        }


@pytest.mark.trio
async def test_device_create_bad_redacted_device_certificate(alice_ws, alice, alice_nd):
    now = DateTime.now()
    device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        device_label=alice_nd.device_label,
        verify_key=alice_nd.verify_key,
    )
    good_redacted_device_certificate = device_certificate.evolve(device_label=None)
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    for bad_redacted_device_certificate in (
        good_redacted_device_certificate.evolve(timestamp=now.add(seconds=1)),
        good_redacted_device_certificate.evolve(device_id=alice.device_id),
        good_redacted_device_certificate.evolve(verify_key=alice.verify_key),
    ):
        rep = await device_create(
            alice_ws,
            device_certificate=device_certificate,
            redacted_device_certificate=bad_redacted_device_certificate.dump_and_sign(
                alice.signing_key
            ),
        )
        assert rep == {
            "status": "invalid_data",
            "reason": "Redacted Device certificate differs from Device certificate.",
        }

    # Finally just make sure good was really good
    rep = await device_create(
        alice_ws,
        device_certificate=device_certificate,
        redacted_device_certificate=good_redacted_device_certificate.dump_and_sign(
            alice.signing_key
        ),
    )
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_redacted_certificates_cannot_contain_sensitive_data(alice_ws, alice, alice_nd):
    now = DateTime.now()
    device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        device_label=alice_nd.device_label,
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(alice.signing_key)

    with freeze_time(now):
        rep = await device_create(
            alice_ws,
            device_certificate=device_certificate,
            redacted_device_certificate=device_certificate,
        )
        assert rep == {
            "status": "invalid_data",
            "reason": "Redacted Device certificate must not contain a device_label field.",
        }
