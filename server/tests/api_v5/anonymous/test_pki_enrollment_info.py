# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    PkiEnrollmentAnswerPayload,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PkiSignatureAlgorithm,
    PrivateKey,
    SigningKey,
    UserProfile,
    anonymous_cmds,
)
from parsec.components.pki import PkiTrustchainError, parse_pki_cert
from tests.api_v5.authenticated.test_user_create import (
    NEW_MIKE_DEVICE_ID,
    NEW_MIKE_DEVICE_LABEL,
    NEW_MIKE_HUMAN_HANDLE,
    NEW_MIKE_USER_ID,
    generate_new_mike_device_certificates,
    generate_new_mike_user_certificates,
)
from tests.common import Backend, CoolorgRpcClients, Enrollment, HttpCommonErrorsTester
from tests.common.pki import TestPki


@pytest.mark.parametrize("kind", ("submitted", "accepted", "cancelled", "rejected"))
async def test_anonymous_pki_enrollment_info_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
    test_pki: TestPki,
) -> None:
    enrollment_id = PKIEnrollmentID.new()
    submitted_on = DateTime.now()
    submit_payload = PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
    ).dump()
    outcome = await backend.pki.submit(
        now=submitted_on,
        organization_id=coolorg.organization_id,
        enrollment_id=enrollment_id,
        force=False,
        submitter_human_handle=NEW_MIKE_HUMAN_HANDLE,
        submitter_trustchain=[
            parse_pki_cert(test_pki.cert["bob"].der_certificate),
            parse_pki_cert(test_pki.root["black_mesa"].der_certificate),
        ],
        submit_payload_signature=b"<mike submit payload signature>",
        submit_payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        submit_payload=submit_payload,
    )
    assert outcome is None

    match kind:
        case "submitted":
            expected_unit = (
                anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusSubmitted(
                    submitted_on=submitted_on
                )
            )

        case "accepted":
            accepted_on = submitted_on.add(seconds=1)

            u_certif, redacted_u_certif = generate_new_mike_user_certificates(
                author=coolorg.alice, timestamp=accepted_on
            )
            d_certif, redacted_d_certif = generate_new_mike_device_certificates(
                author=coolorg.alice, timestamp=accepted_on
            )

            accept_payload = PkiEnrollmentAnswerPayload(
                user_id=NEW_MIKE_USER_ID,
                device_id=NEW_MIKE_DEVICE_ID,
                device_label=NEW_MIKE_DEVICE_LABEL,
                profile=UserProfile.STANDARD,
                root_verify_key=coolorg.root_verify_key,
            ).dump()

            # we need to use build_trustchain here, as it adds the signed_by fields (as opposed to parse_pki_cert)
            trustchain = await backend.pki.build_trustchain(
                test_pki.cert["alice"].der_certificate,
                [test_pki.root["black_mesa"].der_certificate],
            )

            match trustchain:
                case PkiTrustchainError():
                    assert False
                case _:
                    pass

            outcome = await backend.pki.accept(
                now=accepted_on,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                enrollment_id=enrollment_id,
                payload=accept_payload,
                payload_signature=b"<alice accept payload signature>",
                payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                accepter_trustchain=trustchain,
                submitter_user_certificate=u_certif,
                submitter_redacted_user_certificate=redacted_u_certif,
                submitter_device_certificate=d_certif,
                submitter_redacted_device_certificate=redacted_d_certif,
            )
            assert isinstance(outcome, tuple)

            expected_unit = (
                anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusAccepted(
                    submitted_on=submitted_on,
                    accepted_on=accepted_on,
                    accepter_der_x509_certificate=test_pki.cert["alice"].der_certificate,
                    accepter_intermediate_der_x509_certificates=[
                        test_pki.root["black_mesa"].der_certificate,
                    ],
                    accept_payload_signature=b"<alice accept payload signature>",
                    accept_payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                    accept_payload=accept_payload,
                )
            )

        case "cancelled":
            cancelled_on = submitted_on.add(seconds=1)

            # Submit a new enrollment for this X509 certificate to cancel the previous one

            new_enrollment_id = PKIEnrollmentID.new()
            submit_payload = PkiEnrollmentSubmitPayload(
                verify_key=SigningKey.generate().verify_key,
                public_key=PrivateKey.generate().public_key,
                device_label=DeviceLabel("Dev1"),
            ).dump()
            outcome = await backend.pki.submit(
                now=cancelled_on,
                organization_id=coolorg.organization_id,
                enrollment_id=new_enrollment_id,
                force=True,
                submitter_human_handle=NEW_MIKE_HUMAN_HANDLE,
                submitter_trustchain=[parse_pki_cert(test_pki.cert["bob"].der_certificate)],
                submit_payload_signature=b"<mike submit payload signature>",
                submit_payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                submit_payload=submit_payload,
            )
            assert outcome is None

            expected_unit = (
                anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusCancelled(
                    submitted_on=submitted_on,
                    cancelled_on=cancelled_on,
                )
            )

        case "rejected":
            rejected_on = submitted_on.add(seconds=1)

            outcome = await backend.pki.reject(
                now=rejected_on,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                enrollment_id=enrollment_id,
            )
            assert outcome is None

            expected_unit = (
                anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusRejected(
                    submitted_on=submitted_on,
                    rejected_on=rejected_on,
                )
            )

        case unknown:
            assert False, unknown

    rep = await coolorg.anonymous.pki_enrollment_info(enrollment_id=enrollment_id)
    assert rep == anonymous_cmds.latest.pki_enrollment_info.RepOk(unit=expected_unit)


async def test_anonymous_pki_enrollment_info_enrollment_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_info(enrollment_id=PKIEnrollmentID.new())
    assert rep == anonymous_cmds.latest.pki_enrollment_info.RepEnrollmentNotFound()


async def test_anonymous_pki_enrollment_info_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
    test_pki: TestPki,
) -> None:
    enrollment_id = PKIEnrollmentID.new()
    submitted_on = DateTime.now()
    submit_payload = PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
    ).dump()
    outcome = await backend.pki.submit(
        now=submitted_on,
        organization_id=coolorg.organization_id,
        enrollment_id=enrollment_id,
        force=False,
        submitter_human_handle=NEW_MIKE_HUMAN_HANDLE,
        submitter_trustchain=[parse_pki_cert(test_pki.cert["bob"].der_certificate)],
        submit_payload_signature=b"<mike submit payload signature>",
        submit_payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        submit_payload=submit_payload,
    )
    assert outcome is None

    async def do():
        await coolorg.anonymous.pki_enrollment_info(enrollment_id=enrollment_id)

    await anonymous_http_common_errors_tester(do)


# Not tested if postgre because the database consistency is explicitly enforced
async def test_anonymous_pki_enrollment_info_certificate_not_found(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    existing_enrollment: Enrollment,
    test_pki: TestPki,
    monkeypatch: pytest.MonkeyPatch,
    xfail_if_postgresql,
) -> None:
    # Accept enrollment (as this error only happens when we try to
    # look for the certs while re building the trustchain)
    accepted_on = existing_enrollment.submitted_on.add(seconds=1)

    u_certif, redacted_u_certif = generate_new_mike_user_certificates(
        author=coolorg.alice, timestamp=accepted_on
    )
    d_certif, redacted_d_certif = generate_new_mike_device_certificates(
        author=coolorg.alice, timestamp=accepted_on
    )
    accept_payload = PkiEnrollmentAnswerPayload(
        user_id=NEW_MIKE_USER_ID,
        device_id=NEW_MIKE_DEVICE_ID,
        device_label=NEW_MIKE_DEVICE_LABEL,
        profile=UserProfile.STANDARD,
        root_verify_key=coolorg.root_verify_key,
    ).dump()
    # we need to use build_trustchain here, as it adds the signed_by fields (as opposed to parse_pki_cert)
    trustchain = await backend.pki.build_trustchain(
        test_pki.cert["alice"].der_certificate,
        [test_pki.root["black_mesa"].der_certificate],
    )
    match trustchain:
        case PkiTrustchainError():
            assert False
        case _:
            pass
    outcome = await backend.pki.accept(
        now=accepted_on,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        enrollment_id=existing_enrollment.enrollment_id,
        payload=accept_payload,
        payload_signature=b"<alice accept payload signature>",
        payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        accepter_trustchain=trustchain,
        submitter_user_certificate=u_certif,
        submitter_redacted_user_certificate=redacted_u_certif,
        submitter_device_certificate=d_certif,
        submitter_redacted_device_certificate=redacted_d_certif,
    )
    assert isinstance(outcome, tuple)

    # DB corruption
    async def _mocked_get_cert(*args, **kwargs):
        return None

    monkeypatch.setattr(
        "parsec.components.memory.datamodel.MemoryOrganization.get_cert", _mocked_get_cert
    )
    rep = await coolorg.anonymous.pki_enrollment_info(
        enrollment_id=existing_enrollment.enrollment_id
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_info.RepCertificateNotFound()
