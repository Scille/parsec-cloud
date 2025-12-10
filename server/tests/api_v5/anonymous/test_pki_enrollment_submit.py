# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from dataclasses import dataclass

import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    EmailAddress,
    HumanHandle,
    PkiEnrollmentAnswerPayload,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PkiSignatureAlgorithm,
    PrivateKey,
    RevokedUserCertificate,
    SigningKey,
    anonymous_cmds,
    authenticated_cmds,
)
from parsec.components.pki import PkiEnrollmentInfoCancelled, PkiEnrollmentInfoSubmitted
from parsec.events import EventPinged, EventPkiEnrollment
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, TestPki
from tests.common.utils import generate_new_device_certificates, generate_new_user_certificates


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
    ).dump()


@pytest.fixture
async def existing_enrollment(
    coolorg: CoolorgRpcClients, backend: Backend, submit_payload: bytes, test_pki: TestPki
) -> Enrollment:
    enrollment_id = PKIEnrollmentID.new()
    submitter_der_x509_certificate = test_pki.cert["bob"].der_certificate
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
    coolorg: CoolorgRpcClients, backend: Backend, submit_payload: bytes, test_pki: TestPki
) -> None:
    enrollment_id = PKIEnrollmentID.new()
    with backend.event_bus.spy() as spy:
        rep = await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=enrollment_id,
            force=False,
            der_x509_certificate=test_pki.cert["bob"].der_certificate,
            intermediate_der_x509_certificates=[test_pki.root["black_mesa"].der_certificate],
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
    coolorg: CoolorgRpcClients, backend: Backend, submit_payload: bytes, test_pki: TestPki
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
        der_x509_certificate=test_pki.cert["bob"].der_certificate,
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
    test_pki: TestPki,
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
        der_x509_certificate=test_pki.cert["bob"].der_certificate,
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepIdAlreadyUsed()


async def test_anonymous_pki_enrollment_submit_email_already_used(
    coolorg: CoolorgRpcClients, test_pki: TestPki
) -> None:
    # 1. Create a user that use the same email as the submitter certificate
    submitter_cert = test_pki.cert["bob"]
    user_certs = generate_new_user_certificates(
        timestamp=DateTime.now(),
        human_handle=submitter_cert.cert_info.human_handle(),
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )
    dev_certs = generate_new_device_certificates(
        timestamp=user_certs.certificate.timestamp,
        user_id=user_certs.certificate.user_id,
        device_label=DeviceLabel("Bob dev"),
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )
    user_creation_rep = await coolorg.alice.user_create(
        user_certs.signed_certificate,
        dev_certs.signed_certificate,
        user_certs.signed_redacted_certificate,
        dev_certs.signed_redacted_certificate,
    )
    assert user_creation_rep == authenticated_cmds.latest.user_create.RepOk()

    # 2. Create a submit request using a certificate using an email for an already known org's user
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].der_certificate,
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=PkiEnrollmentSubmitPayload(
            verify_key=SigningKey.generate().verify_key,
            public_key=PrivateKey.generate().public_key,
            device_label=DeviceLabel("Dev1"),
        ).dump(),
    )

    # 3. ...
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepEmailAlreadyUsed()


async def test_anonymous_pki_enrollment_submit_already_enrolled(
    backend: Backend,
    coolorg: CoolorgRpcClients,
    existing_enrollment: Enrollment,
    submit_payload: bytes,
    test_pki: TestPki,
) -> None:
    t1 = DateTime.now()
    user_certificates = generate_new_user_certificates(
        timestamp=t1,
        human_handle=HumanHandle(
            email=existing_enrollment.submitter_der_x509_certificate_email,
            label="Mike",
        ),
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )

    device_certificates = generate_new_device_certificates(
        timestamp=t1,
        user_id=user_certificates.certificate.user_id,
        device_label=DeviceLabel("Dev1"),
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )

    outcome = await backend.pki.accept(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        enrollment_id=existing_enrollment.enrollment_id,
        payload=PkiEnrollmentAnswerPayload(
            user_id=user_certificates.certificate.user_id,
            device_id=device_certificates.certificate.device_id,
            profile=user_certificates.certificate.profile,
            device_label=device_certificates.certificate.device_label,
            root_verify_key=coolorg.root_verify_key,
        ).dump(),
        payload_signature=b"<accept payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        accepter_der_x509_certificate=test_pki.cert["alice"].der_certificate,
        accepter_intermediate_der_x509_certificates=[],
        submitter_user_certificate=user_certificates.signed_certificate,
        submitter_redacted_user_certificate=user_certificates.signed_redacted_certificate,
        submitter_device_certificate=device_certificates.signed_certificate,
        submitter_redacted_device_certificate=device_certificates.signed_redacted_certificate,
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
    coolorg: CoolorgRpcClients, test_pki: TestPki
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].der_certificate,
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
    test_pki: TestPki,
) -> None:
    async def do():
        await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=PKIEnrollmentID.new(),
            force=False,
            der_x509_certificate=test_pki.cert["bob"].der_certificate,
            intermediate_der_x509_certificates=[],
            payload_signature=b"<philip submit payload signature>",
            payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            payload=submit_payload,
        )

    await anonymous_http_common_errors_tester(do)


async def test_anonymous_pki_enrollment_submit_invalid_der_x509_certificate(
    coolorg: CoolorgRpcClients, submit_payload: bytes
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=b"not a valid certificate",
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )

    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepInvalidDerX509Certificate()


async def test_anonymous_pki_enrollment_submit_invalid_der_x509_certificate_in_trustchain(
    coolorg: CoolorgRpcClients,
    submit_payload: bytes,
    test_pki: TestPki,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].der_certificate,
        intermediate_der_x509_certificates=[b"not a valid certificate"],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )

    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepInvalidDerX509Certificate()


@pytest.mark.xfail(reason="TODO: https://github.com/Scille/parsec-cloud/issues/11648")
async def test_anonymous_pki_enrollment_submit_invalid_payload_signature() -> None:
    raise NotImplementedError


async def test_anonymous_pki_enrollment_submit_invalid_x509_trustchain(
    coolorg: CoolorgRpcClients,
    submit_payload: bytes,
    test_pki: TestPki,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    from parsec.components import pki

    monkeypatch.setattr(pki, "MAX_INTERMEDIATE_CERTIFICATES_DEPTH", 0)
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].der_certificate,
        intermediate_der_x509_certificates=[test_pki.root["black_mesa"].der_certificate],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )

    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepInvalidX509Trustchain()
