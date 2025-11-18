# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from dataclasses import dataclass

import pytest

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    DevicePurpose,
    EmailAddress,
    HumanHandle,
    PkiEnrollmentAnswerPayload,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PkiSignatureAlgorithm,
    PrivateKey,
    PrivateKeyAlgorithm,
    RevokedUserCertificate,
    SigningKey,
    SigningKeyAlgorithm,
    UserCertificate,
    UserID,
    UserProfile,
    anonymous_cmds,
)
from parsec.components.pki import PkiEnrollmentInfoCancelled, PkiEnrollmentInfoSubmitted
from parsec.events import EventPinged, EventPkiEnrollment
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester


@dataclass
class Enrollment:
    enrollment_id: PKIEnrollmentID
    submitter_der_x509_certificate: bytes
    submitter_intermediate_der_x509_certificates: list[bytes]
    submitter_der_x509_certificate_email: EmailAddress
    submit_payload_signature: bytes
    submit_payload: bytes
    submitted_on: DateTime


@pytest.fixture(scope="session")
def submit_payload() -> bytes:
    return PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
        human_handle=HumanHandle(label="Alice", email=EmailAddress("alice@example.invalid")),
    ).dump()


@pytest.fixture
async def existing_enrollment(
    coolorg: CoolorgRpcClients, backend: Backend, submit_payload: bytes
) -> Enrollment:
    enrollment_id = PKIEnrollmentID.new()
    submitter_der_x509_certificate = b"<mike der x509 certificate>"
    submitter_intermediate_der_x509_certificates = []
    submitter_der_x509_certificate_email = EmailAddress("mike@example.invalid")
    submit_payload_signature = b"<mike submit payload signature>"

    with backend.event_bus.spy() as spy:
        rep = await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=enrollment_id,
            force=False,
            der_x509_certificate=submitter_der_x509_certificate,
            intermediate_der_x509_certificates=submitter_intermediate_der_x509_certificates,
            payload_signature=submit_payload_signature,
            payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            payload=submit_payload,
        )
        assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepOk)

        await spy.wait_event_occurred(
            EventPkiEnrollment(
                organization_id=coolorg.organization_id,
            )
        )
    return Enrollment(
        enrollment_id=enrollment_id,
        submitter_der_x509_certificate=submitter_der_x509_certificate,
        submitter_intermediate_der_x509_certificates=submitter_intermediate_der_x509_certificates,
        submitter_der_x509_certificate_email=submitter_der_x509_certificate_email,
        submit_payload_signature=submit_payload_signature,
        submit_payload=submit_payload,
        submitted_on=rep.submitted_on,
    )


async def test_anonymous_pki_enrollment_submit_ok(
    coolorg: CoolorgRpcClients, backend: Backend, submit_payload: bytes
) -> None:
    enrollment_id = PKIEnrollmentID.new()
    with backend.event_bus.spy() as spy:
        rep = await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=enrollment_id,
            force=False,
            der_x509_certificate=b"<philip der x509 certificate>",
            intermediate_der_x509_certificates=[],
            payload_signature=b"<philip submit payload signature>",
            payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            payload=submit_payload,
        )
        assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepOk)

        await spy.wait_event_occurred(
            EventPkiEnrollment(
                organization_id=coolorg.organization_id,
            )
        )

    # The new enrollment is expected to start in a submitted state
    outcome = await backend.pki.info(
        organization_id=coolorg.organization_id, enrollment_id=enrollment_id
    )
    assert isinstance(outcome, PkiEnrollmentInfoSubmitted)


async def test_anonymous_pki_enrollment_submit_ok_with_force(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    existing_enrollment: Enrollment,
    submit_payload: bytes,
) -> None:
    enrollment_id = PKIEnrollmentID.new()
    with backend.event_bus.spy() as spy:
        rep = await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=enrollment_id,
            force=True,
            der_x509_certificate=existing_enrollment.submitter_der_x509_certificate,
            intermediate_der_x509_certificates=[],
            payload_signature=b"<philip submit payload signature>",
            payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            payload=submit_payload,
        )
        assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepOk)

        # Force the dispatch of an unrelated event to ensure there is no pki event
        # that are still in-flight.
        await coolorg.alice.ping(ping="ping")
        await spy.wait_event_occurred(
            EventPinged(
                organization_id=coolorg.organization_id,
                ping="ping",
            )
        )

        # Note we get only a single event, even if there is two enrollments modified
        # (i.e. the newly created one and the old one being cancelled).
        pki_events = [
            e
            for e in spy.events
            if isinstance(e, EventPkiEnrollment) and e.organization_id == coolorg.organization_id
        ]
        assert len(pki_events) == 1

    # Now th old enrollment should have been cancelled...
    outcome = await backend.pki.info(
        organization_id=coolorg.organization_id, enrollment_id=existing_enrollment.enrollment_id
    )
    assert isinstance(outcome, PkiEnrollmentInfoCancelled)

    # ...and the new one in a submitted state
    outcome = await backend.pki.info(
        organization_id=coolorg.organization_id, enrollment_id=enrollment_id
    )
    assert isinstance(outcome, PkiEnrollmentInfoSubmitted)


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
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=b"<bob der x509 certificate>",
        intermediate_der_x509_certificates=[],
        payload_signature=b"<bob submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepOk)


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
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=existing_enrollment.submitter_der_x509_certificate,
        intermediate_der_x509_certificates=[],
        payload_signature=existing_enrollment.submit_payload_signature,
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=existing_enrollment.submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepOk)


async def test_anonymous_pki_enrollment_submit_already_submitted(
    coolorg: CoolorgRpcClients,
    existing_enrollment: Enrollment,
    submit_payload: bytes,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=existing_enrollment.submitter_der_x509_certificate,
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepAlreadySubmitted(
        submitted_on=existing_enrollment.submitted_on
    )


@pytest.mark.parametrize("existing_enrollment_rejected", (False, True))
async def test_anonymous_pki_enrollment_submit_id_already_used(
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
        der_x509_certificate=b"<philip der x509 certificate>",
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepIdAlreadyUsed()


async def test_anonymous_pki_enrollment_submit_email_already_used(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=b"<philip der x509 certificate>",
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=PkiEnrollmentSubmitPayload(
            verify_key=SigningKey.generate().verify_key,
            public_key=PrivateKey.generate().public_key,
            device_label=DeviceLabel("Dev1"),
            human_handle=coolorg.alice.human_handle,
        ).dump(),
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepEmailAlreadyUsed()


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
        user_id=UserID.new(),
        human_handle=HumanHandle(
            email=existing_enrollment.submitter_der_x509_certificate_email,
            label="Mike",
        ),
        public_key=PrivateKey.generate().public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        profile=UserProfile.STANDARD,
    )
    user_certificate = u_certif.dump_and_sign(coolorg.alice.signing_key)

    redacted_user_certificate = UserCertificate(
        author=u_certif.author,
        timestamp=u_certif.timestamp,
        user_id=u_certif.user_id,
        human_handle=None,
        public_key=u_certif.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        profile=u_certif.profile,
    ).dump_and_sign(coolorg.alice.signing_key)

    d_certif = DeviceCertificate(
        author=coolorg.alice.device_id,
        purpose=DevicePurpose.STANDARD,
        user_id=u_certif.user_id,
        device_id=DeviceID.new(),
        timestamp=t1,
        device_label=DeviceLabel("Dev1"),
        verify_key=SigningKey.generate().verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    )
    device_certificate = d_certif.dump_and_sign(coolorg.alice.signing_key)

    redacted_device_certificate = DeviceCertificate(
        author=d_certif.author,
        purpose=d_certif.purpose,
        user_id=u_certif.user_id,
        device_id=d_certif.device_id,
        timestamp=d_certif.timestamp,
        device_label=None,
        verify_key=d_certif.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.pki.accept(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        enrollment_id=existing_enrollment.enrollment_id,
        payload=PkiEnrollmentAnswerPayload(
            user_id=u_certif.user_id,
            device_id=d_certif.device_id,
            human_handle=u_certif.human_handle,
            profile=u_certif.profile,
            device_label=d_certif.device_label,
            root_verify_key=coolorg.root_verify_key,
        ).dump(),
        payload_signature=b"<accept payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        accepter_der_x509_certificate=b"<accepter der x509 certificate>",
        submitter_user_certificate=user_certificate,
        submitter_redacted_user_certificate=redacted_user_certificate,
        submitter_device_certificate=device_certificate,
        submitter_redacted_device_certificate=redacted_device_certificate,
    )
    assert isinstance(outcome, tuple)

    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=existing_enrollment.submitter_der_x509_certificate,
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepAlreadyEnrolled()


async def test_anonymous_pki_enrollment_submit_invalid_payload(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=b"<philip der x509 certificate>",
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=b"<dummy data>",
    )
    assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepInvalidPayload)


async def test_anonymous_pki_enrollment_submit_http_common_errors(
    coolorg: CoolorgRpcClients,
    submit_payload: bytes,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=PKIEnrollmentID.new(),
            force=False,
            der_x509_certificate=b"<philip der x509 certificate>",
            intermediate_der_x509_certificates=[],
            payload_signature=b"<philip submit payload signature>",
            payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            payload=submit_payload,
        )

    await anonymous_http_common_errors_tester(do)
