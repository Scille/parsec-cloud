# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    RevokedUserCertificate,
    SigningKey,
    authenticated_cmds,
)
from parsec.events import EventCommonCertificate
from tests.common import (
    AuthenticatedRpcClient,
    Backend,
    CoolorgRpcClients,
    MinimalorgRpcClients,
    TestbedBackend,
)

NEW_ALICE_DEVICE_ID = DeviceID("alice@new_dev")
NEW_ALICE_SIGNING_KEY = SigningKey.generate()


def generate_new_alice_device_certificates(
    author: AuthenticatedRpcClient,
    timestamp: DateTime,
    device_id=NEW_ALICE_DEVICE_ID,
    verify_key=NEW_ALICE_SIGNING_KEY.verify_key,
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


async def test_authenticated_device_create_ok(
    minimalorg: MinimalorgRpcClients,
    testbed: TestbedBackend,
    backend: Backend,
) -> None:
    expected_dump = await testbed.backend.user.test_dump_current_users(minimalorg.organization_id)
    expected_dump[minimalorg.alice.user_id].devices.append(NEW_ALICE_DEVICE_ID.device_name)

    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_alice_device_certificates(
        minimalorg.alice, t1
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.device_create(
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert rep == authenticated_cmds.v4.device_create.RepOk()

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
        device_id=NEW_ALICE_DEVICE_ID,
        signing_key=NEW_ALICE_SIGNING_KEY,
    )
    rep = await alice2_rpc.ping(ping="hello")
    assert rep == authenticated_cmds.v4.ping.RepOk(pong="hello")


@pytest.mark.parametrize(
    "kind",
    (
        "certificate",
        "redacted_certificate",
        "user_id_from_another_user",
        "not_author_user_id",
        "author_mismatch",
        "timestamp_mismatch",
        "user_id_mismatch",
        "device_id_mismatch",
        "verify_key_mismatch",
        "not_redacted",
    ),
)
async def test_authenticated_device_create_invalid_certificate(
    coolorg: CoolorgRpcClients,
    kind: str,
) -> None:
    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_alice_device_certificates(
        coolorg.alice, t1
    )

    match kind:
        case "certificate":
            device_certificate = b"<dummy>"
        case "redacted_certificate":
            redacted_device_certificate = b"<dummy>"
        case "user_id_from_another_user":
            (
                device_certificate,
                redacted_device_certificate,
            ) = generate_new_alice_device_certificates(
                coolorg.alice, t1, device_id=DeviceID(f"{coolorg.bob.user_id}@new_device")
            )
        case "not_author_user_id":
            (
                device_certificate,
                redacted_device_certificate,
            ) = generate_new_alice_device_certificates(
                coolorg.alice, t1, device_id=DeviceID("unknown@new_device")
            )
        case "author_mismatch":
            _, redacted_device_certificate = generate_new_alice_device_certificates(
                coolorg.alice, t1, author_device_id=DeviceID("alice@dev2")
            )
        case "timestamp_mismatch":
            _, redacted_device_certificate = generate_new_alice_device_certificates(
                coolorg.alice, t1.subtract(seconds=1)
            )
        case "user_id_mismatch":
            _, redacted_device_certificate = generate_new_alice_device_certificates(
                coolorg.alice, t1, device_id=DeviceID(f"dummy@{NEW_ALICE_DEVICE_ID.device_name}")
            )
        case "device_id_mismatch":
            _, redacted_device_certificate = generate_new_alice_device_certificates(
                coolorg.alice, t1, device_id=DeviceID(f"{NEW_ALICE_DEVICE_ID.user_id}@dummy")
            )
        case "verify_key_mismatch":
            _, redacted_device_certificate = generate_new_alice_device_certificates(
                coolorg.alice, t1, verify_key=SigningKey.generate().verify_key
            )
        case "not_redacted":
            redacted_device_certificate = device_certificate
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.device_create(
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.device_create.RepInvalidCertificate()


async def test_authenticated_device_create_device_already_exists(
    coolorg: CoolorgRpcClients,
) -> None:
    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_alice_device_certificates(
        coolorg.alice, t1, device_id=DeviceID("alice@dev2")
    )

    rep = await coolorg.alice.device_create(
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.device_create.RepDeviceAlreadyExists()


async def test_authenticated_device_create_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
) -> None:
    t0 = DateTime.now().subtract(seconds=3600)
    device_certificate, redacted_device_certificate = generate_new_alice_device_certificates(
        coolorg.alice, t0
    )

    rep = await coolorg.alice.device_create(
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert isinstance(rep, authenticated_cmds.v4.device_create.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


@pytest.mark.parametrize("kind", ("same_timestamp", "smaller_timestamp"))
async def test_authenticated_device_create_require_greater_timestamp(
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

    device_certificate, redacted_device_certificate = generate_new_alice_device_certificates(
        coolorg.alice, t2
    )
    rep = await coolorg.alice.device_create(
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.v4.device_create.RepRequireGreaterTimestamp(
        strictly_greater_than=t1
    )
