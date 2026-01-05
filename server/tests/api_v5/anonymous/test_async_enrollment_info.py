# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    AsyncEnrollmentAcceptPayload,
    AsyncEnrollmentID,
    AsyncEnrollmentSubmitPayload,
    DateTime,
    DeviceLabel,
    EmailAddress,
    HumanHandle,
    OrganizationID,
    PkiSignatureAlgorithm,
    PrivateKey,
    SigningKey,
    anonymous_cmds,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentPayloadSignatureOpenBao,
    AsyncEnrollmentPayloadSignaturePKI,
)
from parsec.events import EventAsyncEnrollment
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    TestPki,
    generate_new_device_certificates,
    generate_new_user_certificates,
)


async def submit_for_mike(
    backend: Backend,
    organization_id: OrganizationID,
    submit_payload_signature: AsyncEnrollmentPayloadSignature,
    submitted_on: DateTime | None = None,
) -> tuple[AsyncEnrollmentID, AsyncEnrollmentSubmitPayload]:
    enrollment_id = AsyncEnrollmentID.new()
    submitted_on = submitted_on or DateTime.now()
    submit_payload = AsyncEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        requested_device_label=DeviceLabel("Dev1"),
        requested_human_handle=HumanHandle(
            label="Mike", email=EmailAddress("mike@example.invalid")
        ),
    )

    with backend.event_bus.spy() as spy:
        outcome = await backend.async_enrollment.submit(
            now=submitted_on,
            organization_id=organization_id,
            enrollment_id=enrollment_id,
            force=True,
            submit_payload=submit_payload.dump(),
            submit_payload_signature=submit_payload_signature,
        )
        assert outcome is None
        await spy.wait_event_occurred(EventAsyncEnrollment(organization_id=organization_id))

    return (enrollment_id, submit_payload)


@pytest.mark.parametrize(
    "kind", ("submitted", "cancelled", "rejected", "accepted_pki", "accepted_openbao")
)
async def test_anonymous_async_enrollment_info_ok(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    test_pki: TestPki,
    kind: str,
):
    if kind.endswith("_pki"):
        submit_payload_signature = AsyncEnrollmentPayloadSignaturePKI(
            signature=b"<submit_payload_signature>",
            algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            author_der_x509_certificate=test_pki.cert["mallory-sign"].certificate.der,
            intermediate_der_x509_certificates=[
                test_pki.intermediate["glados_dev_team"].certificate.der,
            ],
        )
    else:
        submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        )

    submitted_on = DateTime.now()
    enrollment_id, submit_payload = await submit_for_mike(
        backend,
        minimalorg.organization_id,
        submit_payload_signature,
        submitted_on,
    )

    match kind:
        case "submitted":
            expected_unit = anonymous_cmds.latest.async_enrollment_info.InfoStatusSubmitted(
                submitted_on=submitted_on
            )

        case "cancelled":
            cancelled_on = submitted_on.add(seconds=1)

            # Submit a new enrollment with the same email to cancel the previous one

            await submit_for_mike(
                backend,
                minimalorg.organization_id,
                AsyncEnrollmentPayloadSignaturePKI(
                    signature=b"<signature>",
                    algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                    author_der_x509_certificate=test_pki.cert["mallory-sign"].certificate.der,
                    intermediate_der_x509_certificates=[
                        test_pki.intermediate["glados_dev_team"].certificate.der,
                    ],
                ),
                cancelled_on,
            )

            expected_unit = anonymous_cmds.latest.async_enrollment_info.InfoStatusCancelled(
                submitted_on=submitted_on,
                cancelled_on=cancelled_on,
            )

        case "rejected":
            rejected_on = submitted_on.add(seconds=1)

            outcome = await backend.async_enrollment.reject(
                now=rejected_on,
                organization_id=minimalorg.organization_id,
                author=minimalorg.alice.device_id,
                enrollment_id=enrollment_id,
            )
            assert outcome is None

            expected_unit = anonymous_cmds.latest.async_enrollment_info.InfoStatusRejected(
                submitted_on=submitted_on,
                rejected_on=rejected_on,
            )

        case "accepted_openbao":
            certificates_generated_on = submitted_on.add(seconds=1)
            accepted_on = submitted_on.add(seconds=1)

            mike_user_certificates = generate_new_user_certificates(
                timestamp=certificates_generated_on,
                human_handle=submit_payload.requested_human_handle,
                public_key=submit_payload.public_key,
                author_device_id=minimalorg.alice.device_id,
                author_signing_key=minimalorg.alice.signing_key,
            )

            mike_device_certificates = generate_new_device_certificates(
                timestamp=certificates_generated_on,
                user_id=mike_user_certificates.certificate.user_id,
                device_label=submit_payload.requested_device_label,
                verify_key=submit_payload.verify_key,
                author_device_id=minimalorg.alice.device_id,
                author_signing_key=minimalorg.alice.signing_key,
            )

            accept_payload = AsyncEnrollmentAcceptPayload(
                user_id=mike_user_certificates.certificate.user_id,
                device_id=mike_device_certificates.certificate.device_id,
                device_label=mike_device_certificates.certificate.device_label,
                human_handle=mike_user_certificates.certificate.human_handle,
                profile=mike_user_certificates.certificate.profile,
                root_verify_key=minimalorg.root_verify_key,
            ).dump()

            accept_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
                signature="vault:v1:LWWedDmTTaxTZQu97MxFw58NPeI1x+03AqeEQTPEoRRHIlZVO49nY5o89FU7F1NhLnicHDfv/5wpJmZ31MKx9g==",
                author_openbao_entity_id="3ff5aa1d-b8a1-44e5-b5f2-ff6fa0a8cb0a",
            )
            outcome = await backend.async_enrollment.accept(
                now=accepted_on,
                organization_id=minimalorg.organization_id,
                author=minimalorg.alice.device_id,
                author_verify_key=minimalorg.alice.signing_key.verify_key,
                enrollment_id=enrollment_id,
                accept_payload=accept_payload,
                accept_payload_signature=accept_payload_signature,
                submitter_user_certificate=mike_user_certificates.signed_certificate,
                submitter_redacted_user_certificate=mike_user_certificates.signed_redacted_certificate,
                submitter_device_certificate=mike_device_certificates.signed_certificate,
                submitter_redacted_device_certificate=mike_device_certificates.signed_redacted_certificate,
            )
            assert isinstance(outcome, tuple)

            expected_unit = anonymous_cmds.latest.async_enrollment_info.InfoStatusAccepted(
                submitted_on=submitted_on,
                accepted_on=accepted_on,
                accept_payload=accept_payload,
                accept_payload_signature=anonymous_cmds.latest.async_enrollment_info.AcceptPayloadSignatureOpenBao(
                    signature=accept_payload_signature.signature,
                    accepter_openbao_entity_id=accept_payload_signature.author_openbao_entity_id,
                ),
            )

        case "accepted_pki":
            certificates_generated_on = submitted_on.add(seconds=1)
            accepted_on = submitted_on.add(seconds=1)

            mike_user_certificates = generate_new_user_certificates(
                timestamp=certificates_generated_on,
                human_handle=submit_payload.requested_human_handle,
                public_key=submit_payload.public_key,
                author_device_id=minimalorg.alice.device_id,
                author_signing_key=minimalorg.alice.signing_key,
            )

            mike_device_certificates = generate_new_device_certificates(
                timestamp=certificates_generated_on,
                user_id=mike_user_certificates.certificate.user_id,
                device_label=submit_payload.requested_device_label,
                verify_key=submit_payload.verify_key,
                author_device_id=minimalorg.alice.device_id,
                author_signing_key=minimalorg.alice.signing_key,
            )

            accept_payload = AsyncEnrollmentAcceptPayload(
                user_id=mike_user_certificates.certificate.user_id,
                device_id=mike_device_certificates.certificate.device_id,
                device_label=mike_device_certificates.certificate.device_label,
                human_handle=mike_user_certificates.certificate.human_handle,
                profile=mike_user_certificates.certificate.profile,
                root_verify_key=minimalorg.root_verify_key,
            ).dump()

            accept_payload_signature = AsyncEnrollmentPayloadSignaturePKI(
                signature=b"<accept_payload_signature>",
                algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                author_der_x509_certificate=test_pki.cert["mallory-sign"].certificate.der,
                intermediate_der_x509_certificates=[
                    test_pki.intermediate["glados_dev_team"].certificate.der,
                ],
            )
            outcome = await backend.async_enrollment.accept(
                now=accepted_on,
                organization_id=minimalorg.organization_id,
                author=minimalorg.alice.device_id,
                author_verify_key=minimalorg.alice.signing_key.verify_key,
                enrollment_id=enrollment_id,
                accept_payload=accept_payload,
                accept_payload_signature=accept_payload_signature,
                submitter_user_certificate=mike_user_certificates.signed_certificate,
                submitter_redacted_user_certificate=mike_user_certificates.signed_redacted_certificate,
                submitter_device_certificate=mike_device_certificates.signed_certificate,
                submitter_redacted_device_certificate=mike_device_certificates.signed_redacted_certificate,
            )
            assert isinstance(outcome, tuple)

            expected_unit = anonymous_cmds.latest.async_enrollment_info.InfoStatusAccepted(
                submitted_on=submitted_on,
                accepted_on=accepted_on,
                accept_payload=accept_payload,
                accept_payload_signature=anonymous_cmds.latest.async_enrollment_info.AcceptPayloadSignaturePKI(
                    signature=accept_payload_signature.signature,
                    algorithm=accept_payload_signature.algorithm,
                    accepter_der_x509_certificate=accept_payload_signature.author_der_x509_certificate,
                    intermediate_der_x509_certificates=accept_payload_signature.intermediate_der_x509_certificates,
                ),
            )

        case unknown:
            assert False, unknown

    rep = await minimalorg.anonymous.async_enrollment_info(enrollment_id=enrollment_id)
    assert rep == anonymous_cmds.latest.async_enrollment_info.RepOk(unit=expected_unit)


async def test_anonymous_async_enrollment_info_enrollment_not_found(
    minimalorg: MinimalorgRpcClients,
) -> None:
    rep = await minimalorg.anonymous.async_enrollment_info(enrollment_id=AsyncEnrollmentID.new())
    assert rep == anonymous_cmds.latest.async_enrollment_info.RepEnrollmentNotFound()


async def test_anonymous_async_enrollment_info_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    enrollment_id, _ = await submit_for_mike(
        backend,
        coolorg.organization_id,
        AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
    )

    async def do():
        await coolorg.anonymous.async_enrollment_info(enrollment_id=enrollment_id)

    await anonymous_http_common_errors_tester(do)
