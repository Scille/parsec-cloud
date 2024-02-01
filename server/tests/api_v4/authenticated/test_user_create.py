# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    HumanHandle,
    PrivateKey,
    RevokedUserCertificate,
    SigningKey,
    UserCertificate,
    UserID,
    UserProfile,
    authenticated_cmds,
)
from parsec.components.user import UserDump
from parsec.events import EventCommonCertificate
from tests.common import (
    AuthenticatedRpcClient,
    Backend,
    CoolorgRpcClients,
    MinimalorgRpcClients,
    TestbedBackend,
)

NEW_MIKE_DEVICE_ID = DeviceID("mike@new_dev")
NEW_MIKE_HUMAN_HANDLE = HumanHandle(email="mike@ifd.invalid", label="Mike")
NEW_MIKE_SIGNING_KEY = SigningKey.generate()
NEW_MIKE_PRIVATE_KEY = PrivateKey.generate()


def generate_new_mike_device_certificates(
    author: AuthenticatedRpcClient,
    timestamp: DateTime,
    device_id=NEW_MIKE_DEVICE_ID,
    verify_key=NEW_MIKE_SIGNING_KEY.verify_key,
    author_device_id=None,
) -> tuple[bytes, bytes]:
    author_device_id = author_device_id or author.device_id

    raw_device_certificate = DeviceCertificate(
        author=author_device_id,
        timestamp=timestamp,
        device_id=device_id,
        device_label=DeviceLabel("New device"),
        verify_key=verify_key,
    ).dump_and_sign(author.signing_key)

    raw_redacted_device_certificate = DeviceCertificate(
        author=author_device_id,
        timestamp=timestamp,
        device_id=device_id,
        device_label=None,
        verify_key=verify_key,
    ).dump_and_sign(author.signing_key)

    return raw_device_certificate, raw_redacted_device_certificate


def generate_new_mike_user_certificates(
    author: AuthenticatedRpcClient,
    timestamp: DateTime,
    user_id=NEW_MIKE_DEVICE_ID.user_id,
    human_handle=NEW_MIKE_HUMAN_HANDLE,
    public_key=NEW_MIKE_PRIVATE_KEY.public_key,
    profile=UserProfile.STANDARD,
    author_device_id=None,
) -> tuple[bytes, bytes]:
    author_device_id = author_device_id or author.device_id

    raw_user_certificate = UserCertificate(
        author=author_device_id,
        timestamp=timestamp,
        user_id=user_id,
        human_handle=human_handle,
        profile=profile,
        public_key=public_key,
    ).dump_and_sign(author.signing_key)

    raw_redacted_user_certificate = UserCertificate(
        author=author_device_id,
        timestamp=timestamp,
        user_id=user_id,
        human_handle=None,
        profile=profile,
        public_key=public_key,
    ).dump_and_sign(author.signing_key)

    return raw_user_certificate, raw_redacted_user_certificate


async def test_authenticated_user_create_ok(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    testbed: TestbedBackend,
) -> None:
    expected_dump = await testbed.backend.user.test_dump_current_users(minimalorg.organization_id)
    expected_dump[NEW_MIKE_DEVICE_ID.user_id] = UserDump(
        user_id=NEW_MIKE_DEVICE_ID.user_id,
        devices=[NEW_MIKE_DEVICE_ID.device_name],
        current_profile=UserProfile.STANDARD,
        is_revoked=False,
    )

    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        minimalorg.alice, t1
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        minimalorg.alice, t1
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.user_create(
            user_certificate=user_certificate,
            redacted_user_certificate=redacted_user_certificate,
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert rep == authenticated_cmds.v4.user_create.RepOk()

        await spy.wait_event_occurred(
            EventCommonCertificate(
                organization_id=minimalorg.organization_id,
                timestamp=t1,
            )
        )

    dump = await testbed.backend.user.test_dump_current_users(minimalorg.organization_id)
    assert dump == expected_dump

    # Now alice dev2 can connect
    alice2_rpc = AuthenticatedRpcClient(
        raw_client=minimalorg.raw_client,
        organization_id=minimalorg.organization_id,
        device_id=NEW_MIKE_DEVICE_ID,
        signing_key=NEW_MIKE_SIGNING_KEY,
    )
    rep = await alice2_rpc.ping(ping="hello")
    assert rep == authenticated_cmds.v4.ping.RepOk(pong="hello")


async def test_authenticated_user_create_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        coolorg.bob, t1
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        coolorg.bob, t1
    )

    rep = await coolorg.bob.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.user_create.RepAuthorNotAllowed()


async def test_authenticated_user_create_active_users_limit_reached(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    await backend.organization.update(
        minimalorg.organization_id, active_users_limit=ActiveUsersLimit.from_maybe_int(1)
    )
    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        minimalorg.alice, t1
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        minimalorg.alice, t1
    )

    rep = await minimalorg.alice.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.user_create.RepActiveUsersLimitReached()


@pytest.mark.parametrize(
    "kind",
    (
        "device_certificate",
        "redacted_device_certificate",
        "user_certificate",
        "redacted_user_certificate",
        "user_certif_user_id_from_another_user",
        "user_certif_not_author_user_id",
        "user_certif_author_mismatch",
        "user_certif_timestamp_mismatch",
        "user_certif_profile_mismatch",
        "user_certif_user_id_mismatch",
        "user_certif_public_key_mismatch",
        "user_certif_not_redacted",
        "device_certif_user_id_from_another_user",
        "device_certif_not_author_user_id",
        "device_certif_author_mismatch",
        "device_certif_timestamp_mismatch",
        "device_certif_user_id_mismatch",
        "device_certif_device_id_mismatch",
        "device_certif_verify_key_mismatch",
        "device_certif_not_redacted",
    ),
)
async def test_authenticated_user_create_invalid_certificate(
    coolorg: CoolorgRpcClients,
    kind: str,
) -> None:
    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        coolorg.alice, t1
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        coolorg.alice, t1
    )

    match kind:
        case "device_certificate":
            device_certificate = b"<dummy>"
        case "redacted_device_certificate":
            redacted_device_certificate = b"<dummy>"
        case "user_certificate":
            user_certificate = b"<dummy>"
        case "redacted_user_certificate":
            redacted_user_certificate = b"<dummy>"

        case "user_certif_user_id_from_another_user":
            user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1, user_id=coolorg.bob.user_id
            )
        case "user_certif_not_author_user_id":
            user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1, user_id=UserID("unknown")
            )
        case "user_certif_author_mismatch":
            _, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1, author_device_id=DeviceID("alice@dev2")
            )
        case "user_certif_timestamp_mismatch":
            _, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1.subtract(seconds=1)
            )
        case "user_certif_user_id_mismatch":
            _, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1, user_id=UserID("dummy")
            )
        case "user_certif_profile_mismatch":
            _, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1, profile=UserProfile.ADMIN
            )
        case "user_certif_public_key_mismatch":
            _, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1, public_key=PrivateKey.generate().public_key
            )
        case "user_certif_not_redacted":
            redacted_user_certificate = user_certificate

        case "device_certif_user_id_from_another_user":
            device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, device_id=DeviceID(f"{coolorg.bob.user_id}@new_device")
            )
        case "device_certif_not_author_user_id":
            device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, device_id=DeviceID("unknown@new_device")
            )
        case "device_certif_author_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, author_device_id=DeviceID("alice@dev2")
            )
        case "device_certif_timestamp_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1.subtract(seconds=1)
            )
        case "device_certif_user_id_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, device_id=DeviceID(f"dummy@{NEW_MIKE_DEVICE_ID.device_name}")
            )
        case "device_certif_device_id_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, device_id=DeviceID(f"{NEW_MIKE_DEVICE_ID.user_id}@dummy")
            )
        case "device_certif_verify_key_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, verify_key=SigningKey.generate().verify_key
            )
        case "device_certif_not_redacted":
            redacted_device_certificate = device_certificate

        case unknown:
            assert False, unknown

    rep = await coolorg.alice.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.user_create.RepInvalidCertificate()


async def test_authenticated_user_create_user_already_exists(
    coolorg: CoolorgRpcClients,
) -> None:
    t1 = DateTime.now()
    device_id = DeviceID("alice@new_dev")
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        coolorg.alice, t1, device_id=device_id
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        coolorg.alice, t1, user_id=device_id.user_id
    )
    rep = await coolorg.alice.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.user_create.RepUserAlreadyExists()


async def test_authenticated_user_create_human_handle_already_exists(
    minimalorg: MinimalorgRpcClients,
) -> None:
    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        minimalorg.alice, t1
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        minimalorg.alice, t1, human_handle=minimalorg.alice.human_handle
    )
    rep = await minimalorg.alice.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.user_create.RepHumanHandleAlreadyTaken()


async def test_authenticated_user_create_device_timestamp_out_of_ballpark(
    minimalorg: MinimalorgRpcClients,
) -> None:
    t0 = DateTime.now().subtract(seconds=3600)
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        minimalorg.alice, t0
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        minimalorg.alice, t0
    )

    rep = await minimalorg.alice.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert isinstance(rep, authenticated_cmds.v4.user_create.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


@pytest.mark.parametrize("kind", ("same_timestamp", "smaller_timestamp"))
async def test_authenticated_user_create_device_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    t1 = DateTime.now()
    match kind:
        case "same_timestamp":
            t2 = t1
        case "smaller_timestamp":
            t2 = t1.subtract(seconds=1)
        case unknown:
            assert False, unknown

    # 1) Create a new certificate in the organization

    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.mallory.device_id.user_id,
        timestamp=t1,
    )
    await backend.user.revoke_user(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )

    # 2) Our device creation's timestamp is clashing with the previous certificate

    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        coolorg.alice, t2
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        coolorg.alice, t2
    )
    rep = await coolorg.alice.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.user_create.RepRequireGreaterTimestamp(
        strictly_greater_than=t1
    )
