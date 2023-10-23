# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    DeviceID,
    DeviceLabel,
    UserProfile,
    authenticated_cmds,
)
from parsec.api.data import DeviceCertificate, UserCertificate
from parsec.api.protocol import (
    ApiV2V3_UserGetRepOk,
    UserCreateRepActiveUsersLimitReached,
    UserCreateRepAlreadyExists,
    UserCreateRepInvalidCertification,
    UserCreateRepInvalidData,
    UserCreateRepNotAllowed,
    UserCreateRepOk,
)
from parsec.backend.user import INVITATION_VALIDITY, Device, User
from tests.backend.common import apiv2v3_user_get, user_create
from tests.common import customize_fixtures, freeze_time


@pytest.mark.trio
@pytest.mark.parametrize("profile", UserProfile.VALUES)
async def test_user_create_ok(
    backend_asgi_app,
    backend_authenticated_ws_factory,
    alice_ws,
    alice,
    mallory,
    profile,
):
    now = DateTime.now()
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=profile,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificate(
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

    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert isinstance(rep, UserCreateRepOk)

    # Make sure mallory can connect now
    async with backend_authenticated_ws_factory(backend_asgi_app, mallory) as sock:
        rep = await apiv2v3_user_get(sock, user_id=mallory.user_id)
        assert isinstance(rep, ApiV2V3_UserGetRepOk)

    # Check the resulting data in the backend
    backend_user, backend_device = await backend_asgi_app.backend.user.get_user_with_device(
        mallory.organization_id, mallory.device_id
    )

    assert backend_user == User(
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        initial_profile=profile,
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        user_certifier=alice.device_id,
        created_on=now,
    )
    assert backend_device == Device(
        device_id=mallory.device_id,
        device_label=mallory.device_label,
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
    await backend_asgi_app.backend.organization.update(
        alice.organization_id, active_users_limit=ActiveUsersLimit.LimitedTo(1)
    )

    now = DateTime.now()
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificate(
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

    async with backend_authenticated_ws_factory(backend_asgi_app, alice) as sock:
        rep = await user_create(
            sock,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert isinstance(rep, UserCreateRepActiveUsersLimitReached)

        # Now correct the limit, and ensure the user can be created

        await backend_asgi_app.backend.organization.update(
            alice.organization_id, active_users_limit=ActiveUsersLimit.LimitedTo(2)
        )

        rep = await user_create(
            sock,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert isinstance(rep, UserCreateRepOk)


@pytest.mark.trio
async def test_user_create_invalid_certificate(alice_ws, alice, bob, mallory):
    now = DateTime.now()
    good_user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    good_device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)
    bad_user_certificate = UserCertificate(
        author=bob.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(bob.signing_key)
    bad_device_certificate = DeviceCertificate(
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
        assert isinstance(rep, UserCreateRepInvalidCertification)

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
        assert isinstance(rep, UserCreateRepInvalidCertification)


@pytest.mark.trio
async def test_user_redacted_non_redacted_mixed_up(
    alice_ws,
    alice,
    mallory,
):
    now = DateTime.now()
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificate(
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

    kwargs = {
        "user_certificate": user_certificate,
        "device_certificate": device_certificate,
        "redacted_user_certificate": redacted_user_certificate,
        "redacted_device_certificate": redacted_device_certificate,
    }
    for field, value in [
        ("user_certificate", redacted_user_certificate),
        ("device_certificate", redacted_device_certificate),
        ("redacted_user_certificate", user_certificate),
        ("redacted_device_certificate", device_certificate),
    ]:
        rep = await user_create(alice_ws, **{**kwargs, field: value})
        assert isinstance(rep, UserCreateRepInvalidData)


@pytest.mark.trio
async def test_user_create_not_matching_user_device(alice_ws, alice, bob, mallory):
    now = DateTime.now()
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificate(
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
    assert isinstance(rep, UserCreateRepInvalidData)


@pytest.mark.trio
async def test_user_create_bad_redacted_device_certificate(alice_ws, alice, mallory):
    now = DateTime.now()
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    redacted_user_certificate = redacted_user_certificate.dump_and_sign(alice.signing_key)
    user_certificate = user_certificate.dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificate(
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
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=bad_redacted_device_certificate.dump_and_sign(
                alice.signing_key
            ),
        )
        assert isinstance(rep, UserCreateRepInvalidData)

    # Missing redacted certificate is not allowed as well
    # We should not be able to build an invalid request
    # Generated from Python implementation (Parsec v2.11.1+dev)
    # Content:
    #   cmd: "user_create"
    #   device_certificate: hex!("666f6f626172")
    #   redacted_device_certificate: None
    #   redacted_user_certificate: hex!("666f6f626172")
    #   user_certificate: hex!("666f6f626172")
    #
    raw_req = bytes.fromhex(
        "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
        "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c0b9"
        "72656461637465645f757365725f6365727469666963617465c406666f6f626172b0757365"
        "725f6365727469666963617465c406666f6f626172"
    )
    await alice_ws.send(raw_req)
    raw_rep = await alice_ws.receive()
    rep = authenticated_cmds.latest.user_create.Rep.load(raw_rep)
    assert isinstance(rep, authenticated_cmds.latest.user_create.RepUnknownStatus)
    assert rep.status == "invalid_msg_format"

    # Finally just make sure good was really good
    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=good_redacted_device_certificate.dump_and_sign(
            alice.signing_key
        ),
    )
    assert isinstance(rep, UserCreateRepOk)


@pytest.mark.trio
async def test_user_create_bad_redacted_user_certificate(alice_ws, alice, mallory):
    now = DateTime.now()
    device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        device_label=mallory.device_label,
        verify_key=mallory.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(alice.signing_key)
    device_certificate = device_certificate.dump_and_sign(alice.signing_key)
    user_certificate = UserCertificate(
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
            redacted_device_certificate=redacted_device_certificate,
        )
        assert isinstance(rep, UserCreateRepInvalidData)

    # Missing redacted certificate is not allowed as well
    # We should not be able to build an invalid request
    # Generated from Python implementation (Parsec v2.11.1+dev)
    # Content:
    #   cmd: "user_create"
    #   device_certificate: hex!("666f6f626172")
    #   redacted_device_certificate: hex!("666f6f626172")
    #   redacted_user_certificate: None
    #   user_certificate: hex!("666f6f626172")
    #
    raw_req = bytes.fromhex(
        "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
        "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c406"
        "666f6f626172b972656461637465645f757365725f6365727469666963617465c0b0757365"
        "725f6365727469666963617465c406666f6f626172"
    )
    await alice_ws.send(raw_req)
    raw_rep = await alice_ws.receive()
    rep = authenticated_cmds.latest.user_create.Rep.load(raw_rep)
    assert isinstance(rep, authenticated_cmds.latest.user_create.RepUnknownStatus)
    assert rep.status == "invalid_msg_format"

    # Finally just make sure good was really good
    rep = await user_create(
        alice_ws,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=good_redacted_user_certificate.dump_and_sign(alice.signing_key),
        redacted_device_certificate=redacted_device_certificate,
    )
    assert isinstance(rep, UserCreateRepOk)


@pytest.mark.trio
async def test_user_create_already_exists(alice_ws, alice, bob):
    now = DateTime.now()
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=bob.user_id,
        human_handle=alice.human_handle,
        public_key=bob.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=bob.device_id,
        device_label=alice.device_label,
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
    assert isinstance(rep, UserCreateRepAlreadyExists)


@pytest.mark.trio
async def test_user_create_human_handle_already_exists(alice_ws, alice, bob):
    now = DateTime.now()
    bob2_device_id = DeviceID("bob2@dev1")
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=bob2_device_id.user_id,
        human_handle=bob.human_handle,
        public_key=bob.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificate(
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
    assert isinstance(rep, UserCreateRepAlreadyExists)


@pytest.mark.trio
async def test_user_create_human_handle_with_revoked_previous_one(
    alice_ws, alice, bob, backend_data_binder
):
    # First revoke bob
    await backend_data_binder.bind_revocation(user_id=bob.user_id, certifier=alice)

    # Now recreate another user with bob's human handle
    now = DateTime.now()
    bob2_device_id = DeviceID("bob2@dev1")
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=bob2_device_id.user_id,
        human_handle=bob.human_handle,
        public_key=bob.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificate(
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
    assert isinstance(rep, UserCreateRepOk)


@pytest.mark.trio
async def test_user_create_not_matching_certified_on(alice_ws, alice, mallory):
    date1 = DateTime(2000, 1, 1)
    date2 = date1.add(seconds=1)
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=date1,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificate(
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
        assert isinstance(rep, UserCreateRepInvalidData)


@pytest.mark.trio
async def test_user_create_certificate_too_old(alice_ws, alice, mallory):
    too_old = DateTime(2000, 1, 1)
    now = too_old.add(seconds=INVITATION_VALIDITY + 1)
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=too_old,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificate(
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
        assert isinstance(rep, UserCreateRepInvalidCertification)


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
    assert isinstance(rep, UserCreateRepNotAllowed)


@pytest.mark.trio
async def test_redacted_certificates_cannot_contain_sensitive_data(alice_ws, alice, mallory):
    now = DateTime.now()
    user_certificate = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        human_handle=mallory.human_handle,
        public_key=mallory.public_key,
        profile=UserProfile.STANDARD,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    device_certificate = DeviceCertificate(
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
        assert isinstance(rep, UserCreateRepInvalidData)

        rep = await user_create(
            alice_ws,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=device_certificate,
        )
        assert isinstance(rep, UserCreateRepInvalidData)
