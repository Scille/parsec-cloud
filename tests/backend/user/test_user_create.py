# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import pendulum

from parsec.backend.user import INVITATION_VALIDITY, User, Device
from parsec.api.data import UserCertificateContent, DeviceCertificateContent, UserProfile
from parsec.api.protocol import DeviceID, DeviceLabel

from tests.common import customize_fixtures, freeze_time
from tests.backend.common import user_get, user_create


@pytest.mark.trio
@pytest.mark.parametrize(
    "profile,with_labels", [(profile, profile != UserProfile.STANDARD) for profile in UserProfile]
)
async def test_user_create_ok(
    backend_asgi_app,
    backend_authenticated_ws_factory,
    alice_ws,
    alice,
    mallory,
    profile,
    with_labels,
):
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=profile,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)
    if not with_labels:
        user_certificate = redacted_user_certificate
        device_certificate = redacted_device_certificate

    user_certificate = user_certificate.dump_and_sign(alice.signing_key)
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    redacted_user_certificate = redacted_user_certificate.dump_and_sign(alice.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(alice.signing_key)

    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == {"status": "ok"}

    # Make sure mallory can connect now
    async with backend_authenticated_ws_factory(backend_asgi_app, mallory) as sock:
        rep = await user_get(sock, user_id=mallory.user_id)
        assert rep["status"] == "ok"

    # Check the resulting data in the backend
    backend_user, backend_device = await backend_asgi_app.backend.user.get_user_with_device(
        mallory.organization_id, mallory.device_id
    )

    assert backend_user == User(
        user_id=mallory.user_id,
        human_handle=mallory.human_handle if with_labels else None,
        profile=profile,
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        user_certifier=alice.device_id,
        created_on=now,
    )
    assert backend_device == Device(
        device_id=mallory.device_id,
        device_label=mallory.device_label if with_labels else None,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
        device_certifier=alice.device_id,
        created_on=now,
    )


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_user_create_nok_active_users_limit_reached(
    backend_asgi_app,
    backend_data_binder_factory,
    backend_authenticated_ws_factory,
    coolorg,
    alice,
    mallory,
):
    # Ensure there is only one user in the organization...
    binder = backend_data_binder_factory(backend_asgi_app.backend)
    await binder.bind_organization(coolorg, alice)
    # ...so our active user limit has just been reached
    await backend_asgi_app.backend.organization.update(alice.organization_id, active_users_limit=1)

    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=None,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=None,
        verify_key=mallory.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    user_certificate = user_certificate.dump_and_sign(alice.signing_key)
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    redacted_user_certificate = redacted_user_certificate.dump_and_sign(alice.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(alice.signing_key)

    async with backend_authenticated_ws_factory(backend_asgi_app, alice) as sock:
        rep = await user_create(
            sock,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert rep == {"status": "active_users_limit_reached"}

        # Now correct the limit, and ensure the user can be created

        await backend_asgi_app.backend.organization.update(
            alice.organization_id, active_users_limit=2
        )

        rep = await user_create(
            sock,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_create_invalid_certificate(alice_ws, alice, bob, mallory):
    now = pendulum.now()
    good_user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    good_device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)
    bad_user_certificate = UserCertificateContent(
        author=bob.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(bob.signing_key)
    bad_device_certificate = DeviceCertificateContent(
        author=bob.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    ).dump_and_sign(bob.signing_key)

    for cu, cd in [
        (good_user_certificate, bad_device_certificate),
        (bad_user_certificate, good_device_certificate),
        (bad_user_certificate, bad_device_certificate),
    ]:
        rep = await user_create(
            alice_ws,
            user_certificate=cu,
            device_certificate=cd,
            redacted_user_certificate=good_user_certificate,
            redacted_device_certificate=good_device_certificate,
        )
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid certification data (Signature was forged or corrupt).",
        }

    # Same thing for the redacted part
    for cu, cd in [
        (good_user_certificate, bad_device_certificate),
        (bad_user_certificate, good_device_certificate),
        (bad_user_certificate, bad_device_certificate),
    ]:
        rep = await user_create(
            alice_ws,
            user_certificate=good_user_certificate,
            device_certificate=good_device_certificate,
            redacted_user_certificate=cu,
            redacted_device_certificate=cd,
        )
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid certification data (Signature was forged or corrupt).",
        }


@pytest.mark.trio
async def test_user_create_not_matching_user_device(alice_ws, alice, bob, mallory):
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=bob.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)

    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=user_certificate,
        redacted_device_certificate=device_certificate,
    )
    assert rep == {
        "status": "invalid_data",
        "reason": "Device and User must have the same user ID.",
    }


@pytest.mark.trio
async def test_user_create_bad_redacted_device_certificate(alice_ws, alice, mallory):
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=None,  # Can be used as regular and redacted certificate
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    )
    good_redacted_device_certificate = device_certificate.evolve(device_label=None)
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    for bad_redacted_device_certificate in (
        good_redacted_device_certificate.evolve(timestamp=now.add(seconds=1)),
        good_redacted_device_certificate.evolve(device_id=alice.device_id),
        good_redacted_device_certificate.evolve(verify_key=alice.verify_key),
    ):
        rep = await user_create(
            alice_ws,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=user_certificate,
            redacted_device_certificate=bad_redacted_device_certificate.dump_and_sign(
                alice.signing_key
            ),
        )
        assert rep == {
            "status": "invalid_data",
            "reason": "Redacted Device certificate differs from Device certificate.",
        }

    # Missing redacted certificate is not allowed as well
    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=user_certificate,
        redacted_device_certificate=None,
    )
    assert rep == {
        "status": "bad_message",
        "reason": "Invalid message.",
        "errors": {"redacted_device_certificate": ["Missing data for required field."]},
    }

    # Finally just make sure good was really good
    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=user_certificate,
        redacted_device_certificate=good_redacted_device_certificate.dump_and_sign(
            alice.signing_key
        ),
    )
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_create_bad_redacted_user_certificate(alice_ws, alice, mallory):
    now = pendulum.now()
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=None,  # Can be used as regular and redacted certificate
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    )
    good_redacted_user_certificate = user_certificate.evolve(human_handle=None)
    user_certificate = user_certificate.dump_and_sign(alice.signing_key)
    for bad_redacted_user_certificate in (
        good_redacted_user_certificate.evolve(timestamp=now.add(seconds=1)),
        good_redacted_user_certificate.evolve(user_id=alice.user_id),
        good_redacted_user_certificate.evolve(public_key=alice.public_key),
        good_redacted_user_certificate.evolve(profile=UserProfile.OUTSIDER),
    ):
        rep = await user_create(
            alice_ws,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=bad_redacted_user_certificate.dump_and_sign(
                alice.signing_key
            ),
            redacted_device_certificate=device_certificate,
        )
        assert rep == {
            "status": "invalid_data",
            "reason": "Redacted User certificate differs from User certificate.",
        }

    # Missing redacted certificate is not allowed as well
    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=None,
        redacted_device_certificate=device_certificate,
    )
    assert rep == {
        "status": "bad_message",
        "reason": "Invalid message.",
        "errors": {"redacted_user_certificate": ["Missing data for required field."]},
    }

    # Finally just make sure good was really good
    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=good_redacted_user_certificate.dump_and_sign(alice.signing_key),
        redacted_device_certificate=device_certificate,
    )
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_create_already_exists(alice_ws, alice, bob):
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=bob.user_id,
        human_handle=None,
        public_key=bob.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=bob.device_id,
        device_label=None,
        verify_key=bob.verify_key,
    ).dump_and_sign(alice.signing_key)

    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=user_certificate,
        redacted_device_certificate=device_certificate,
    )
    assert rep == {"status": "already_exists", "reason": f"User `{bob.user_id}` already exists"}


@pytest.mark.trio
async def test_user_create_human_handle_already_exists(alice_ws, alice, bob):
    now = pendulum.now()
    bob2_device_id = DeviceID("bob2@dev1")
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=bob2_device_id.user_id,
        human_handle=bob.human_handle,
        public_key=bob.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=bob2_device_id,
        device_label=DeviceLabel("dev2"),
        verify_key=bob.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    user_certificate = user_certificate.dump_and_sign(alice.signing_key)
    redacted_user_certificate = redacted_user_certificate.dump_and_sign(alice.signing_key)
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(alice.signing_key)

    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == {
        "status": "already_exists",
        "reason": f"Human handle `{bob.human_handle}` already corresponds to a non-revoked user",
    }


@pytest.mark.trio
async def test_user_create_human_handle_with_revoked_previous_one(
    alice_ws, alice, bob, backend_data_binder
):
    # First revoke bob
    await backend_data_binder.bind_revocation(user_id=bob.user_id, certifier=alice)

    # Now recreate another user with bob's human handle
    now = pendulum.now()
    bob2_device_id = DeviceID("bob2@dev1")
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=bob2_device_id.user_id,
        human_handle=bob.human_handle,
        public_key=bob.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=bob2_device_id,
        device_label=bob.device_label,  # Device label doesn't have to be unique
        verify_key=bob.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    user_certificate = user_certificate.dump_and_sign(alice.signing_key)
    redacted_user_certificate = redacted_user_certificate.dump_and_sign(alice.signing_key)
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(alice.signing_key)

    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_create_not_matching_certified_on(alice_ws, alice, mallory):
    date1 = pendulum.datetime(2000, 1, 1)
    date2 = date1.add(seconds=1)
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=date1,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=date2,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)
    with freeze_time(date1):
        rep = await user_create(
            alice_ws,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=user_certificate,
            redacted_device_certificate=device_certificate,
        )
        assert rep == {
            "status": "invalid_data",
            "reason": "Device and User certificates must have the same timestamp.",
        }


@pytest.mark.trio
async def test_user_create_certificate_too_old(alice_ws, alice, mallory):
    too_old = pendulum.datetime(2000, 1, 1)
    now = too_old.add(seconds=INVITATION_VALIDITY + 1)
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=too_old,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=too_old,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)

    with freeze_time(now):
        rep = await user_create(
            alice_ws,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=user_certificate,
            redacted_device_certificate=device_certificate,
        )
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid timestamp in certificate.",
        }


@pytest.mark.trio
async def test_user_create_author_not_admin(backend_asgi_app, bob_ws):
    # No need for valid certificate given given access right should be
    # checked before payload deserialization
    rep = await user_create(
        bob_ws,
        user_certificate=b"<user_certificate>",
        device_certificate=b"<device_certificate>",
        redacted_user_certificate=b"<redacted_user_certificate>",
        redacted_device_certificate=b"<redacted_device_certificate>",
    )
    assert rep == {"status": "not_allowed", "reason": "User `bob` is not admin"}


@pytest.mark.trio
async def test_redacted_certificates_cannot_contain_sensitive_data(alice_ws, alice, mallory):
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    user_certificate = user_certificate.dump_and_sign(alice.signing_key)
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    redacted_user_certificate = redacted_user_certificate.dump_and_sign(alice.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(alice.signing_key)

    with freeze_time(now):
        rep = await user_create(
            alice_ws,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert rep == {
            "status": "invalid_data",
            "reason": "Redacted User certificate must not contain a human_handle field.",
        }

        rep = await user_create(
            alice_ws,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=device_certificate,
        )
        assert rep == {
            "status": "invalid_data",
            "reason": "Redacted Device certificate must not contain a device_label field.",
        }
