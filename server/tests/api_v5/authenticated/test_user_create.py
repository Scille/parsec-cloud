# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    DeviceID,
    DeviceLabel,
    EmailAddress,
    HumanHandle,
    PrivateKey,
    PrivateKeyAlgorithm,
    RevokedUserCertificate,
    SigningKey,
    SigningKeyAlgorithm,
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
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    TestbedBackend,
    bob_becomes_admin_and_changes_alice,
)
from tests.common.utils import generate_new_device_certificates, generate_new_user_certificates

NEW_MIKE_USER_ID = UserID.new()
NEW_MIKE_DEVICE_ID = DeviceID.new()
NEW_MIKE_HUMAN_HANDLE = HumanHandle(email=EmailAddress("mike@ifd.invalid"), label="Mike")
NEW_MIKE_DEVICE_LABEL = DeviceLabel("New device")
NEW_MIKE_SIGNING_KEY = SigningKey.generate()
NEW_MIKE_PRIVATE_KEY = PrivateKey.generate()


def generate_new_mike_device_certificates(
    author: AuthenticatedRpcClient,
    timestamp: DateTime,
    user_id=NEW_MIKE_USER_ID,
    device_id=NEW_MIKE_DEVICE_ID,
    verify_key=NEW_MIKE_SIGNING_KEY.verify_key,
    author_device_id: DeviceID | None = None,
) -> tuple[bytes, bytes]:
    certs = generate_new_device_certificates(
        timestamp=timestamp,
        user_id=user_id,
        device_id=device_id,
        verify_key=verify_key,
        device_label=NEW_MIKE_DEVICE_LABEL,
        algorithm=SigningKeyAlgorithm.ED25519,
        author_device_id=author_device_id or author.device_id,
        author_signing_key=author.signing_key,
    )
    return certs.signed_certificate, certs.signed_redacted_certificate


def generate_new_mike_user_certificates(
    author: AuthenticatedRpcClient,
    timestamp: DateTime,
    user_id=NEW_MIKE_USER_ID,
    human_handle=NEW_MIKE_HUMAN_HANDLE,
    public_key=NEW_MIKE_PRIVATE_KEY.public_key,
    profile=UserProfile.STANDARD,
    author_device_id: DeviceID | None = None,
) -> tuple[bytes, bytes]:
    certs = generate_new_user_certificates(
        timestamp=timestamp,
        user_id=user_id,
        human_handle=human_handle,
        public_key=public_key,
        profile=profile,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        author_device_id=author_device_id or author.device_id,
        author_signing_key=author.signing_key,
    )
    return certs.signed_certificate, certs.signed_redacted_certificate


async def test_authenticated_user_create_ok(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    testbed: TestbedBackend,
) -> None:
    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        minimalorg.alice, t1
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        minimalorg.alice, t1
    )

    expected_dump = await testbed.backend.user.test_dump_current_users(minimalorg.organization_id)
    expected_dump[NEW_MIKE_USER_ID] = UserDump(
        user_id=NEW_MIKE_USER_ID,
        devices=[NEW_MIKE_DEVICE_ID],
        profile_history=[(t1, UserProfile.STANDARD)],
        created_on=t1,
        human_handle=NEW_MIKE_HUMAN_HANDLE,
        revoked_on=None,
        frozen=False,
    )
    expected_topics = await backend.organization.test_dump_topics(minimalorg.organization_id)
    expected_topics.common = t1

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.user_create(
            user_certificate=user_certificate,
            redacted_user_certificate=redacted_user_certificate,
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        assert rep == authenticated_cmds.latest.user_create.RepOk()

        await spy.wait_event_occurred(
            EventCommonCertificate(
                organization_id=minimalorg.organization_id,
                timestamp=t1,
            )
        )

    dump = await testbed.backend.user.test_dump_current_users(minimalorg.organization_id)
    assert dump == expected_dump
    topics = await backend.organization.test_dump_topics(minimalorg.organization_id)
    assert topics == expected_topics

    # Now alice dev2 can connect
    alice2_rpc = AuthenticatedRpcClient(
        raw_client=minimalorg.raw_client,
        organization_id=minimalorg.organization_id,
        user_id=NEW_MIKE_USER_ID,
        device_id=NEW_MIKE_DEVICE_ID,
        signing_key=NEW_MIKE_SIGNING_KEY,
    )
    rep = await alice2_rpc.ping(ping="hello")
    assert rep == authenticated_cmds.latest.ping.RepOk(pong="hello")


@pytest.mark.parametrize("kind", ("as_outsider", "as_standard", "no_longer_allowed"))
async def test_authenticated_user_create_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "as_outsider":
            author = coolorg.bob

        case "as_standard":
            author = coolorg.bob

        case "no_longer_allowed":
            await bob_becomes_admin_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_profile=UserProfile.STANDARD
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        author, t1
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(author, t1)

    rep = await author.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.latest.user_create.RepAuthorNotAllowed()


async def test_authenticated_user_create_active_users_limit_reached(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    await backend.organization.update(
        now=DateTime.now(),
        id=minimalorg.organization_id,
        active_users_limit=ActiveUsersLimit.from_maybe_int(1),
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
    assert rep == authenticated_cmds.latest.user_create.RepActiveUsersLimitReached()


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
                coolorg.alice, t1, user_id=UserID.new()
            )
        case "user_certif_author_mismatch":
            _, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1, author_device_id=DeviceID.test_from_nickname("alice@dev2")
            )
        case "user_certif_timestamp_mismatch":
            _, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1.subtract(seconds=1)
            )
        case "user_certif_user_id_mismatch":
            _, redacted_user_certificate = generate_new_mike_user_certificates(
                coolorg.alice, t1, user_id=UserID.new()
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
                coolorg.alice, t1, user_id=coolorg.bob.user_id
            )
        case "device_certif_not_author_user_id":
            device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, user_id=UserID.new()
            )
        case "device_certif_author_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, author_device_id=DeviceID.test_from_nickname("alice@dev2")
            )
        case "device_certif_timestamp_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1.subtract(seconds=1)
            )
        case "device_certif_user_id_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, user_id=UserID.new()
            )
        case "device_certif_device_id_mismatch":
            _, redacted_device_certificate = generate_new_mike_device_certificates(
                coolorg.alice, t1, device_id=DeviceID.new()
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
    assert rep == authenticated_cmds.latest.user_create.RepInvalidCertificate()


@pytest.mark.parametrize(
    "kind",
    (
        "conflicting_user_id",
        "conflicting_device_id",
    ),
)
async def test_authenticated_user_create_user_already_exists(
    coolorg: CoolorgRpcClients,
    kind: str,
) -> None:
    user_id = NEW_MIKE_USER_ID
    device_id = NEW_MIKE_DEVICE_ID
    match kind:
        case "conflicting_user_id":
            user_id = coolorg.alice.user_id
        case "conflicting_device_id":
            device_id = coolorg.alice.device_id
        case unknown:
            assert False, unknown

    t1 = DateTime.now()
    device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
        coolorg.alice, t1, user_id=user_id, device_id=device_id
    )
    user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
        coolorg.alice, t1, user_id=user_id
    )
    rep = await coolorg.alice.user_create(
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert rep == authenticated_cmds.latest.user_create.RepUserAlreadyExists()


async def test_authenticated_user_create_human_handle_already_taken(
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
    assert rep == authenticated_cmds.latest.user_create.RepHumanHandleAlreadyTaken()


async def test_authenticated_user_create_timestamp_out_of_ballpark(
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
    assert isinstance(rep, authenticated_cmds.latest.user_create.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


@pytest.mark.parametrize("kind", ("same_timestamp", "smaller_timestamp"))
async def test_authenticated_user_create_require_greater_timestamp(
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
        user_id=coolorg.mallory.user_id,
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
    assert rep == authenticated_cmds.latest.user_create.RepRequireGreaterTimestamp(
        strictly_greater_than=t1
    )


async def test_authenticated_user_create_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        now = DateTime.now()

        device_certificate, redacted_device_certificate = generate_new_mike_device_certificates(
            coolorg.alice, now
        )
        user_certificate, redacted_user_certificate = generate_new_mike_user_certificates(
            coolorg.alice, now
        )

        await coolorg.alice.user_create(
            user_certificate=user_certificate,
            redacted_user_certificate=redacted_user_certificate,
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )

    await authenticated_http_common_errors_tester(do)
