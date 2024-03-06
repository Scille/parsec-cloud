# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    DeviceID,
    RevokedUserCertificate,
    UserProfile,
    UserUpdateCertificate,
    authenticated_cmds,
)
from parsec.events import EventUserUpdated
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_user_update_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    now = DateTime.now()
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.device_id.user_id,
        new_profile=UserProfile.ADMIN,
    )

    expected_dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    expected_dump[coolorg.bob.device_id.user_id].current_profile = UserProfile.ADMIN

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.user_update(
            user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.v4.user_update.RepOk()

        await spy.wait_event_occurred(
            EventUserUpdated(
                organization_id=coolorg.organization_id,
                user_id=coolorg.bob.device_id.user_id,
                new_profile=UserProfile.ADMIN,
            )
        )

    dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    assert dump == expected_dump


async def test_authenticated_user_update_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    now = DateTime.now()
    certif = UserUpdateCertificate(
        author=coolorg.bob.device_id,
        timestamp=now,
        user_id=coolorg.bob.device_id.user_id,
        new_profile=UserProfile.ADMIN,
    )

    expected_dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    expected_dump[coolorg.bob.device_id.user_id].current_profile = UserProfile.STANDARD

    rep = await coolorg.bob.user_update(
        user_update_certificate=certif.dump_and_sign(coolorg.bob.signing_key)
    )
    assert rep == authenticated_cmds.v4.user_update.RepAuthorNotAllowed()

    dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    assert dump == expected_dump


async def test_authenticated_user_update_user_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    now = DateTime.now()
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=DeviceID("mike@new_dev").user_id,
        new_profile=UserProfile.ADMIN,
    )

    rep = await coolorg.alice.user_update(
        user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.v4.user_update.RepUserNotFound()


async def test_authenticated_user_update_user_revoked(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # 1) Before updating user Bob, let's revoke it

    t1 = DateTime.now()
    certif1 = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        user_id=coolorg.bob.device_id.user_id,
    )

    outcome = await backend.user.revoke_user(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif1.dump_and_sign(coolorg.alice.signing_key),
    )

    assert isinstance(outcome, RevokedUserCertificate)

    # 2) Try to update user Bob (which is now revoked)

    now = DateTime.now()
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.device_id.user_id,
        new_profile=UserProfile.ADMIN,
    )

    rep = await coolorg.alice.user_update(
        user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.v4.user_update.RepUserRevoked()


async def test_authenticated_user_update_user_no_changes(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    now = DateTime.now()
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.device_id.user_id,
        new_profile=UserProfile.STANDARD,
    )

    rep = await coolorg.alice.user_update(
        user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.v4.user_update.RepUserNoChanges()


@pytest.mark.parametrize(
    "kind",
    (
        "user_update_certificate",
        "user_update_certificate_not_author_user_id",
        "user_update_certificate_author_device_mismatch",
        "user_update_certificate_author_update_itself",
    ),
)
async def test_authenticated_user_update_invalid_certificate(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    now = DateTime.now()

    match kind:
        case "user_update_certificate":
            certif = b"<dummy>"
        case "user_update_certificate_not_author_user_id":
            certif = UserUpdateCertificate(
                author=coolorg.bob.device_id,
                timestamp=now,
                user_id=coolorg.bob.device_id.user_id,
                new_profile=UserProfile.ADMIN,
            ).dump_and_sign(coolorg.alice.signing_key)
        case "user_update_certificate_author_device_mismatch":
            certif = UserUpdateCertificate(
                author=DeviceID("alice@dev2"),
                timestamp=now,
                user_id=coolorg.bob.device_id.user_id,
                new_profile=UserProfile.ADMIN,
            ).dump_and_sign(coolorg.alice.signing_key)
        case "user_update_certificate_author_update_itself":
            certif = UserUpdateCertificate(
                author=coolorg.alice.device_id,
                timestamp=now,
                user_id=coolorg.alice.device_id.user_id,
                new_profile=UserProfile.STANDARD,
            ).dump_and_sign(coolorg.alice.signing_key)
        case _:
            raise Exception(f"Test not implemented for kind: {kind}")

    rep = await coolorg.alice.user_update(user_update_certificate=certif)
    assert rep == authenticated_cmds.v4.user_update.RepInvalidCertificate()


async def test_authenticated_user_update_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    t0 = DateTime.now().subtract(seconds=3600)
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=t0,
        user_id=coolorg.bob.device_id.user_id,
        new_profile=UserProfile.ADMIN,
    )

    rep = await coolorg.alice.user_update(
        user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
    )

    assert isinstance(rep, authenticated_cmds.v4.user_update.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


@pytest.mark.parametrize(
    "timestamp_offset",
    (pytest.param(0, id="same_timestamp"), pytest.param(1, id="previous_timestamp")),
)
async def test_authenticated_user_update_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_offset: int,
) -> None:
    now = DateTime.now()
    user_update_timestamp = now.subtract(seconds=timestamp_offset)

    # 1) Create a certificate in the organization

    existing_certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.device_id.user_id,
        new_profile=UserProfile.ADMIN,
    ).dump_and_sign(coolorg.alice.signing_key)

    await backend.user.update_user(
        now=now,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_update_certificate=existing_certif,
    )

    # 2) Create revoke user certificate where timestamp is clashing
    #    with the previous certificate

    new_certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=user_update_timestamp,
        user_id=coolorg.bob.device_id.user_id,
        new_profile=UserProfile.STANDARD,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.user_update(user_update_certificate=new_certif)
    assert rep == authenticated_cmds.v4.user_update.RepRequireGreaterTimestamp(
        strictly_greater_than=now
    )
