# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from dataclasses import dataclass

import pytest

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    EnrollmentID,
    HumanHandle,
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentSubmitPayload,
    PrivateKey,
    RevokedUserCertificate,
    SigningKey,
    UserCertificate,
    UserID,
    UserProfile,
    anonymous_cmds,
)
from tests.common import Backend, CoolorgRpcClients


@dataclass
class Enrollment:
    enrollment_id: EnrollmentID
    submitter_der_x509_certificate: bytes
    submitter_der_x509_certificate_email: str
    submit_payload_signature: bytes
    submit_payload: bytes
    submitted_on: DateTime


@pytest.fixture(scope="session")
def submit_payload() -> bytes:
    return PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        requested_device_label=DeviceLabel("Dev1"),
    ).dump()


@pytest.fixture
async def existing_enrollment(coolorg: CoolorgRpcClients, submit_payload: bytes) -> Enrollment:
    enrollment_id = EnrollmentID.new()
    submitter_der_x509_certificate = b"<mike der x509 certificate>"
    submitter_der_x509_certificate_email = "mike@example.invalid"
    submit_payload_signature = b"<mike submit payload signature>"

    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=submitter_der_x509_certificate,
        submitter_der_x509_certificate_email=submitter_der_x509_certificate_email,
        submit_payload_signature=submit_payload_signature,
        submit_payload=submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.v4.pki_enrollment_submit.RepOk)

    return Enrollment(
        enrollment_id=enrollment_id,
        submitter_der_x509_certificate=submitter_der_x509_certificate,
        submitter_der_x509_certificate_email=submitter_der_x509_certificate_email,
        submit_payload_signature=submit_payload_signature,
        submit_payload=submit_payload,
        submitted_on=rep.submitted_on,
    )


async def test_anonymous_pki_enrollment_submit_ok(
    coolorg: CoolorgRpcClients, submit_payload: bytes
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=b"<philip der x509 certificate>",
        submitter_der_x509_certificate_email="philip@example.invalid",
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.v4.pki_enrollment_submit.RepOk)


async def test_anonymous_pki_enrollment_submit_ok_with_force(
    coolorg: CoolorgRpcClients, existing_enrollment: Enrollment, submit_payload: bytes
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=EnrollmentID.new(),
        force=True,
        submitter_der_x509_certificate=existing_enrollment.submitter_der_x509_certificate,
        submitter_der_x509_certificate_email=existing_enrollment.submitter_der_x509_certificate_email,
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.v4.pki_enrollment_submit.RepOk)


async def test_anonymous_pki_enrollment_submit_ok_with_email_from_revoked_user(
    coolorg: CoolorgRpcClients, backend: Backend, submit_payload: bytes
) -> None:
    t1 = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        user_id=coolorg.bob.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)
    outcome = await backend.user.revoke_user(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif,
    )
    assert isinstance(outcome, RevokedUserCertificate)

    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=b"<bob der x509 certificate>",
        submitter_der_x509_certificate_email=coolorg.bob.human_handle.email,
        submit_payload_signature=b"<bob submit payload signature>",
        submit_payload=submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.v4.pki_enrollment_submit.RepOk)


async def test_anonymous_pki_enrollment_submit_ok_with_cancelled_enrollment(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    existing_enrollment: Enrollment,
) -> None:
    t1 = DateTime.now()
    outcome = await backend.pki.reject(
        now=t1,
        author=coolorg.alice.device_id,
        organization_id=coolorg.organization_id,
        enrollment_id=existing_enrollment.enrollment_id,
    )
    assert outcome is None

    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=existing_enrollment.submitter_der_x509_certificate,
        submitter_der_x509_certificate_email=existing_enrollment.submitter_der_x509_certificate_email,
        submit_payload_signature=existing_enrollment.submit_payload_signature,
        submit_payload=existing_enrollment.submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.v4.pki_enrollment_submit.RepOk)


async def test_anonymous_pki_enrollment_submit_x509_certificate_already_submitted(
    coolorg: CoolorgRpcClients,
    existing_enrollment: Enrollment,
    submit_payload: bytes,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=existing_enrollment.submitter_der_x509_certificate,
        submitter_der_x509_certificate_email="philip@example.invalid",
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=submit_payload,
    )
    assert rep == anonymous_cmds.v4.pki_enrollment_submit.RepX509CertificateAlreadySubmitted(
        submitted_on=existing_enrollment.submitted_on
    )


@pytest.mark.parametrize("existing_enrollment_rejected", (False, True))
async def test_anonymous_pki_enrollment_submit_enrollment_id_already_used(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    existing_enrollment: Enrollment,
    existing_enrollment_rejected: bool,
    submit_payload: bytes,
) -> None:
    if existing_enrollment_rejected:
        t1 = DateTime.now()
        outcome = await backend.pki.reject(
            now=t1,
            author=coolorg.alice.device_id,
            organization_id=coolorg.organization_id,
            enrollment_id=existing_enrollment.enrollment_id,
        )
        assert outcome is None

    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=existing_enrollment.enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<philip der x509 certificate>",
        submitter_der_x509_certificate_email="philip@example.invalid",
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=submit_payload,
    )
    assert rep == anonymous_cmds.v4.pki_enrollment_submit.RepEnrollmentIdAlreadyUsed()


async def test_anonymous_pki_enrollment_submit_email_already_enrolled(
    coolorg: CoolorgRpcClients,
    submit_payload: bytes,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=b"<philip der x509 certificate>",
        submitter_der_x509_certificate_email=coolorg.alice.human_handle.email,
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=submit_payload,
    )
    assert rep == anonymous_cmds.v4.pki_enrollment_submit.RepEmailAlreadyEnrolled()


async def test_anonymous_pki_enrollment_submit_already_enrolled(
    backend: Backend,
    coolorg: CoolorgRpcClients,
    existing_enrollment: Enrollment,
    submit_payload: bytes,
) -> None:
    t1 = DateTime.now()

    u_certif = UserCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        user_id=UserID("mike"),
        human_handle=HumanHandle(
            email=existing_enrollment.submitter_der_x509_certificate_email, label="Mike"
        ),
        public_key=PrivateKey.generate().public_key,
        profile=UserProfile.STANDARD,
    )
    user_certificate = u_certif.dump_and_sign(coolorg.alice.signing_key)

    redacted_user_certificate = UserCertificate(
        author=u_certif.author,
        timestamp=u_certif.timestamp,
        user_id=u_certif.user_id,
        human_handle=None,
        public_key=u_certif.public_key,
        profile=u_certif.profile,
    ).dump_and_sign(coolorg.alice.signing_key)

    d_certif = DeviceCertificate(
        author=coolorg.alice.device_id,
        device_id=DeviceID("mike@dev1"),
        timestamp=t1,
        device_label=DeviceLabel("Dev1"),
        verify_key=SigningKey.generate().verify_key,
    )
    device_certificate = d_certif.dump_and_sign(coolorg.alice.signing_key)

    redacted_device_certificate = DeviceCertificate(
        author=d_certif.author,
        device_id=d_certif.device_id,
        timestamp=d_certif.timestamp,
        device_label=None,
        verify_key=d_certif.verify_key,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.pki.accept(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        enrollment_id=existing_enrollment.enrollment_id,
        accept_payload=PkiEnrollmentAnswerPayload(
            device_id=d_certif.device_id,
            human_handle=u_certif.human_handle,
            profile=u_certif.profile,
            device_label=d_certif.device_label,
            root_verify_key=coolorg.root_verify_key,
        ).dump(),
        accept_payload_signature=b"<accept payload signature>",
        accepter_der_x509_certificate=b"<accepter der x509 certificate>",
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    assert isinstance(outcome, tuple)

    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=existing_enrollment.submitter_der_x509_certificate,
        submitter_der_x509_certificate_email=existing_enrollment.submitter_der_x509_certificate_email,
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=submit_payload,
    )
    assert rep == anonymous_cmds.v4.pki_enrollment_submit.RepAlreadyEnrolled()


async def test_anonymous_pki_enrollment_submit_invalid_payload_data(
    coolorg: CoolorgRpcClients
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=b"<philip der x509 certificate>",
        submitter_der_x509_certificate_email="philip@example.invalid",
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=b"<dummy data>",
    )
    assert isinstance(rep, anonymous_cmds.v4.pki_enrollment_submit.RepInvalidPayloadData)
