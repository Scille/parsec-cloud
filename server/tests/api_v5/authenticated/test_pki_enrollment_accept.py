# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import TypedDict

import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    DeviceID,
    DeviceLabel,
    EnrollmentID,
    HumanHandle,
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentSubmitPayload,
    PrivateKey,
    RevokedUserCertificate,
    SigningKey,
    UserID,
    UserProfile,
    authenticated_cmds,
)
from tests.api_v5.authenticated.test_user_create import (
    NEW_MIKE_DEVICE_ID,
    NEW_MIKE_DEVICE_LABEL,
    NEW_MIKE_HUMAN_HANDLE,
    NEW_MIKE_USER_ID,
    generate_new_mike_device_certificates,
    generate_new_mike_user_certificates,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    bob_becomes_admin_and_changes_alice,
)


@pytest.fixture
async def enrollment_id(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> EnrollmentID:
    enrollment_id = EnrollmentID.new()
    submitted_on = DateTime.now()
    submit_payload = PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        requested_device_label=DeviceLabel("Dev1"),
    ).dump()
    outcome = await backend.pki.submit(
        now=submitted_on,
        organization_id=coolorg.organization_id,
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<philip der x509 certificate>",
        submitter_der_x509_certificate_email="philip@example.invalid",
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=submit_payload,
    )
    assert outcome is None

    return enrollment_id


class AcceptParams(TypedDict):
    enrollment_id: EnrollmentID
    accept_payload: bytes
    accept_payload_signature: bytes
    accepter_der_x509_certificate: bytes
    enrollment_id: EnrollmentID
    device_certificate: bytes
    user_certificate: bytes
    redacted_device_certificate: bytes
    redacted_user_certificate: bytes


def generate_accept_params(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
    now: DateTime | None = None,
    user_id: UserID = NEW_MIKE_USER_ID,
    device_id: DeviceID = NEW_MIKE_DEVICE_ID,
    device_label: DeviceLabel = NEW_MIKE_DEVICE_LABEL,
    human_handle: HumanHandle = NEW_MIKE_HUMAN_HANDLE,
    profile: UserProfile = UserProfile.STANDARD,
) -> AcceptParams:
    now = now or DateTime.now()

    u_certif, redacted_u_certif = generate_new_mike_user_certificates(
        author=coolorg.alice,
        timestamp=now,
        user_id=user_id,
        human_handle=human_handle,
        profile=profile,
    )
    d_certif, redacted_d_certif = generate_new_mike_device_certificates(
        author=coolorg.alice,
        timestamp=now,
        user_id=user_id,
        device_id=device_id,
    )

    accept_payload = PkiEnrollmentAnswerPayload(
        user_id=user_id,
        device_id=device_id,
        device_label=device_label,
        human_handle=human_handle,
        profile=profile,
        root_verify_key=coolorg.root_verify_key,
    ).dump()

    return {
        "enrollment_id": enrollment_id,
        "accept_payload": accept_payload,
        "accept_payload_signature": b"<alice accept payload signature>",
        "accepter_der_x509_certificate": b"<alice der x509 certificate>",
        "user_certificate": u_certif,
        "redacted_user_certificate": redacted_u_certif,
        "device_certificate": d_certif,
        "redacted_device_certificate": redacted_d_certif,
    }


async def test_authenticated_pki_enrollment_accept_ok(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
) -> None:
    rep = await coolorg.alice.pki_enrollment_accept(
        **generate_accept_params(coolorg, enrollment_id)
    )
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepOk()


async def test_authenticated_pki_enrollment_accept_active_users_limit_reached(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    enrollment_id: EnrollmentID,
) -> None:
    outcome = await backend.organization.update(
        now=DateTime.now(),
        id=coolorg.organization_id,
        active_users_limit=ActiveUsersLimit.limited_to(3),
    )
    assert outcome is None
    rep = await coolorg.alice.pki_enrollment_accept(
        **generate_accept_params(coolorg, enrollment_id)
    )
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepActiveUsersLimitReached()


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_pki_enrollment_accept_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    enrollment_id: EnrollmentID,
    kind: str,
) -> None:
    match kind:
        case "never_allowed":
            author = coolorg.bob

        case "no_longer_allowed":
            await bob_becomes_admin_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_profile=UserProfile.STANDARD
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    rep = await author.pki_enrollment_accept(**generate_accept_params(coolorg, enrollment_id))
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepAuthorNotAllowed()


async def test_authenticated_pki_enrollment_accept_enrollment_no_longer_available(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    enrollment_id: EnrollmentID,
) -> None:
    outcome = await backend.pki.reject(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        enrollment_id=enrollment_id,
    )
    assert outcome is None

    rep = await coolorg.alice.pki_enrollment_accept(
        **generate_accept_params(coolorg, enrollment_id)
    )
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepEnrollmentNoLongerAvailable()


async def test_authenticated_pki_enrollment_accept_enrollment_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.pki_enrollment_accept(
        **generate_accept_params(coolorg, EnrollmentID.new())
    )
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepEnrollmentNotFound()


async def test_authenticated_pki_enrollment_accept_human_handle_already_taken(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
) -> None:
    rep = await coolorg.alice.pki_enrollment_accept(
        **generate_accept_params(coolorg, enrollment_id, human_handle=coolorg.bob.human_handle)
    )
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepHumanHandleAlreadyTaken()


async def test_authenticated_pki_enrollment_accept_invalid_certificate(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
) -> None:
    params = generate_accept_params(coolorg, enrollment_id)
    params["user_certificate"] = b"<dummy>"
    rep = await coolorg.alice.pki_enrollment_accept(**params)
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepInvalidCertificate()


async def test_authenticated_pki_enrollment_accept_invalid_payload_data(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
) -> None:
    params = generate_accept_params(coolorg, enrollment_id)
    params["accept_payload"] = b"<dummy>"
    rep = await coolorg.alice.pki_enrollment_accept(**params)
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepInvalidPayloadData()


async def test_authenticated_pki_enrollment_accept_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    rep = await coolorg.alice.pki_enrollment_accept(
        **generate_accept_params(coolorg, enrollment_id, now=timestamp_out_of_ballpark)
    )
    assert isinstance(
        rep, authenticated_cmds.latest.pki_enrollment_accept.RepTimestampOutOfBallpark
    )
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


async def test_authenticated_pki_enrollment_accept_user_already_exists(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
) -> None:
    rep = await coolorg.alice.pki_enrollment_accept(
        **generate_accept_params(coolorg, enrollment_id, user_id=coolorg.bob.user_id)
    )
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepUserAlreadyExists()


@pytest.mark.parametrize("kind", ("same_timestamp", "previous_timestamp"))
async def test_authenticated_pki_enrollment_accept_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    enrollment_id: EnrollmentID,
    kind: str,
) -> None:
    now = DateTime.now()
    match kind:
        case "same_timestamp":
            accepted_timestamp = now
        case "previous_timestamp":
            accepted_timestamp = now.subtract(seconds=1)
        case unknown:
            assert False, unknown

    # 1) Create a certificate in the organization

    revoked_user_certificate = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    await backend.user.revoke_user(
        organization_id=coolorg.organization_id,
        now=now,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=revoked_user_certificate,
    )

    # 2) Do the PKI accept with a certificate which timestamp is clashing
    #    with the previous certificate

    rep = await coolorg.alice.pki_enrollment_accept(
        **generate_accept_params(coolorg, enrollment_id, now=accepted_timestamp)
    )
    assert rep == authenticated_cmds.latest.pki_enrollment_accept.RepRequireGreaterTimestamp(
        strictly_greater_than=now
    )


async def test_authenticated_pki_enrollment_accept_http_common_errors(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.pki_enrollment_accept(**generate_accept_params(coolorg, enrollment_id))

    await authenticated_http_common_errors_tester(do)
