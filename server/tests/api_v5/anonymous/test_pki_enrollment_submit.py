# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PkiSignatureAlgorithm,
    PrivateKey,
    RevokedUserCertificate,
    SigningKey,
    anonymous_cmds,
    authenticated_cmds,
)
from parsec.components.pki import (
    PkiEnrollmentInfoCancelled,
    PkiEnrollmentInfoSubmitted,
)
from parsec.events import EventPinged, EventPkiEnrollment
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, TestPki
from tests.common.pki import PkiEnrollment, accept_pki_enrollment, sign_message
from tests.common.utils import generate_new_device_certificates, generate_new_user_certificates


async def test_anonymous_pki_enrollment_submit_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    submit_payload: bytes,
    test_pki: TestPki,
    backend_with_test_pki_roots,
) -> None:
    enrollment_id = PKIEnrollmentID.new()
    with backend.event_bus.spy() as spy:
        sign_algo, payload_signature = sign_message(test_pki.cert["bob"].key, submit_payload)
        rep = await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=enrollment_id,
            force=False,
            der_x509_certificate=test_pki.cert["bob"].certificate.der,
            intermediate_der_x509_certificates=[],
            payload_signature=payload_signature,
            payload_signature_algorithm=sign_algo,
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
    existing_pki_enrollment: PkiEnrollment,
    submit_payload: bytes,
    backend_with_test_pki_roots,
    test_pki: TestPki,
) -> None:
    enrollment_id = PKIEnrollmentID.new()
    with backend.event_bus.spy() as spy:
        sign_algo, payload_signature = sign_message(test_pki.cert["bob"].key, submit_payload)
        rep = await coolorg.anonymous.pki_enrollment_submit(
            enrollment_id=enrollment_id,
            force=True,
            der_x509_certificate=existing_pki_enrollment.submitter_trustchain[0].content,
            intermediate_der_x509_certificates=[],
            payload_signature=payload_signature,
            payload_signature_algorithm=sign_algo,
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
        organization_id=coolorg.organization_id, enrollment_id=existing_pki_enrollment.enrollment_id
    )
    assert isinstance(outcome, PkiEnrollmentInfoCancelled)

    # ...and the new one in a submitted state
    outcome = await backend.pki.info(
        organization_id=coolorg.organization_id, enrollment_id=enrollment_id
    )
    assert isinstance(outcome, PkiEnrollmentInfoSubmitted)


async def test_anonymous_pki_enrollment_submit_ok_with_email_from_revoked_user(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    submit_payload: bytes,
    test_pki: TestPki,
    backend_with_test_pki_roots,
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

    sign_algo, payload_signature = sign_message(test_pki.cert["bob"].key, submit_payload)
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].certificate.der,
        intermediate_der_x509_certificates=[],
        payload_signature=payload_signature,
        payload_signature_algorithm=sign_algo,
        payload=submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepOk)


async def test_anonymous_pki_enrollment_submit_ok_with_cancelled_enrollment(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    existing_pki_enrollment: PkiEnrollment,
    backend_with_test_pki_roots,
) -> None:
    t1 = DateTime.now()
    outcome = await backend.pki.reject(
        now=t1,
        author=coolorg.alice.device_id,
        organization_id=coolorg.organization_id,
        enrollment_id=existing_pki_enrollment.enrollment_id,
    )
    assert outcome is None

    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=existing_pki_enrollment.submitter_trustchain[0].content,
        intermediate_der_x509_certificates=[],
        payload_signature=existing_pki_enrollment.submit_payload_signature,
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=existing_pki_enrollment.submit_payload,
    )
    assert isinstance(rep, anonymous_cmds.latest.pki_enrollment_submit.RepOk)


async def test_anonymous_pki_enrollment_submit_already_submitted(
    coolorg: CoolorgRpcClients,
    existing_pki_enrollment: PkiEnrollment,
    submit_payload: bytes,
    backend_with_test_pki_roots,
    test_pki: TestPki,
) -> None:
    sign_algo, payload_signature = sign_message(test_pki.cert["bob"].key, submit_payload)
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=existing_pki_enrollment.submitter_trustchain[0].content,
        intermediate_der_x509_certificates=[],
        payload_signature=payload_signature,
        payload_signature_algorithm=sign_algo,
        payload=submit_payload,
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepAlreadySubmitted(
        submitted_on=existing_pki_enrollment.submitted_on
    )


@pytest.mark.parametrize("existing_pki_enrollment_rejected", (False, True))
async def test_anonymous_pki_enrollment_submit_id_already_used(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    existing_pki_enrollment: PkiEnrollment,
    existing_pki_enrollment_rejected: bool,
    submit_payload: bytes,
    test_pki: TestPki,
    backend_with_test_pki_roots,
) -> None:
    if existing_pki_enrollment_rejected:
        t1 = DateTime.now()
        outcome = await backend.pki.reject(
            now=t1,
            author=coolorg.alice.device_id,
            organization_id=coolorg.organization_id,
            enrollment_id=existing_pki_enrollment.enrollment_id,
        )
        assert outcome is None

    sign_algo, payload_signature = sign_message(test_pki.cert["bob"].key, submit_payload)
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=existing_pki_enrollment.enrollment_id,
        force=False,
        der_x509_certificate=test_pki.cert["bob"].certificate.der,
        intermediate_der_x509_certificates=[],
        payload_signature=payload_signature,
        payload_signature_algorithm=sign_algo,
        payload=submit_payload,
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepIdAlreadyUsed()


async def test_anonymous_pki_enrollment_submit_email_already_used(
    coolorg: CoolorgRpcClients,
    test_pki: TestPki,
    backend_with_test_pki_roots,
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
    submit_payload = PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
    ).dump()
    sign_algo, payload_signature = sign_message(test_pki.cert["bob"].key, submit_payload)
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].certificate.der,
        intermediate_der_x509_certificates=[],
        payload_signature=payload_signature,
        payload_signature_algorithm=sign_algo,
        payload=submit_payload,
    )

    # 3. ...
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepEmailAlreadyUsed()


async def test_anonymous_pki_enrollment_submit_already_enrolled(
    backend: Backend,
    coolorg: CoolorgRpcClients,
    existing_pki_enrollment: PkiEnrollment,
    submit_payload: bytes,
    test_pki: TestPki,
    backend_with_test_pki_roots,
) -> None:
    t1 = DateTime.now()
    await accept_pki_enrollment(
        t1,
        backend,
        coolorg.organization_id,
        coolorg.alice,
        test_pki.cert["alice"],
        [],
        coolorg.root_verify_key,
        existing_pki_enrollment,
    )

    sign_algo, payload_signature = sign_message(test_pki.cert["bob"].key, submit_payload)
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=existing_pki_enrollment.submitter_trustchain[0].content,
        intermediate_der_x509_certificates=[],
        payload_signature=payload_signature,
        payload_signature_algorithm=sign_algo,
        payload=submit_payload,
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepAlreadyEnrolled()


async def test_anonymous_pki_enrollment_submit_invalid_payload(
    coolorg: CoolorgRpcClients,
    test_pki: TestPki,
    backend_with_test_pki_roots,
) -> None:
    submit_payload = b"<dummy data>"
    sign_algo, payload_signature = sign_message(test_pki.cert["bob"].key, submit_payload)
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].certificate.der,
        intermediate_der_x509_certificates=[],
        payload_signature=payload_signature,
        payload_signature_algorithm=sign_algo,
        payload=submit_payload,
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
            der_x509_certificate=test_pki.cert["bob"].certificate.der,
            intermediate_der_x509_certificates=[],
            payload_signature=b"<philip submit payload signature>",
            payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            payload=submit_payload,
        )

    await anonymous_http_common_errors_tester(do)


async def test_anonymous_pki_enrollment_submit_invalid_der_x509_certificate(
    coolorg: CoolorgRpcClients,
    submit_payload: bytes,
    backend_with_test_pki_roots,
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
        der_x509_certificate=test_pki.cert["bob"].certificate.der,
        intermediate_der_x509_certificates=[b"not a valid certificate"],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )

    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepInvalidDerX509Certificate()


async def test_anonymous_pki_enrollment_submit_invalid_payload_signature(
    coolorg: CoolorgRpcClients,
    submit_payload: bytes,
    test_pki: TestPki,
    backend_with_test_pki_roots,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].certificate.der,
        intermediate_der_x509_certificates=[],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )

    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepInvalidPayloadSignature()


@pytest.mark.parametrize("kind", ("trust_chain_too_complex", "no_trusted_anchor"))
async def test_anonymous_pki_enrollment_submit_invalid_x509_trustchain(
    coolorg: CoolorgRpcClients,
    submit_payload: bytes,
    test_pki: TestPki,
    monkeypatch: pytest.MonkeyPatch,
    backend: Backend,
    kind: str,
) -> None:
    from parsec.components import pki

    match kind:
        case "trust_chain_too_complex":
            monkeypatch.setattr(pki, "MAX_INTERMEDIATE_CERTIFICATES_DEPTH", 0)
        case "no_trusted_anchor":
            backend.pki._config.x509_trust_anchor = []
        case _:
            assert False
    rep = await coolorg.anonymous.pki_enrollment_submit(
        enrollment_id=PKIEnrollmentID.new(),
        force=False,
        der_x509_certificate=test_pki.cert["bob"].certificate.der,
        intermediate_der_x509_certificates=[test_pki.root["black_mesa"].certificate.der],
        payload_signature=b"<philip submit payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        payload=submit_payload,
    )

    assert rep == anonymous_cmds.latest.pki_enrollment_submit.RepInvalidX509Trustchain()
