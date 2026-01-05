# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
    authenticated_cmds,
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


async def submit_for(
    backend: Backend,
    organization_id: OrganizationID,
    email: str,
    submit_payload_signature: AsyncEnrollmentPayloadSignature,
    submitted_on: DateTime | None = None,
) -> tuple[AsyncEnrollmentID, AsyncEnrollmentSubmitPayload, bytes]:
    enrollment_id = AsyncEnrollmentID.new()
    submitted_on = submitted_on or DateTime.now()
    submit_payload = AsyncEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        requested_device_label=DeviceLabel("Dev1"),
        requested_human_handle=HumanHandle(
            label=email.split("@", 1)[0].capitalize(), email=EmailAddress(email)
        ),
    )
    submit_payload_raw = submit_payload.dump()

    with backend.event_bus.spy() as spy:
        outcome = await backend.async_enrollment.submit(
            now=submitted_on,
            organization_id=organization_id,
            enrollment_id=enrollment_id,
            force=True,
            submit_payload=submit_payload_raw,
            submit_payload_signature=submit_payload_signature,
        )
        assert outcome is None
        await spy.wait_event_occurred(EventAsyncEnrollment(organization_id=organization_id))

    return (enrollment_id, submit_payload, submit_payload_raw)


async def test_authenticated_async_enrollment_list_ok(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    test_pki: TestPki,
) -> None:
    # Rejected enrollment, should be ignored

    rejected_enrollment_id, _, _ = await submit_for(
        backend=backend,
        organization_id=minimalorg.organization_id,
        email="godfrey@example.invalid",
        submit_payload_signature=AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:PCMqlBeeP3qSZF5ABMhV04/wYSIOCBaBAcvQtvtzffTDnRB0uTeD0gdUds/fRSZrJcwDIg4qy7OIatadeW/xfQ==",
            author_openbao_entity_id="89c6c299-c501-4292-b446-787810322a28",
        ),
        submitted_on=DateTime(2010, 1, 1),
    )
    outcome = await backend.async_enrollment.reject(
        now=DateTime(2010, 1, 2),
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
        enrollment_id=rejected_enrollment_id,
    )
    assert outcome is None

    # Accepted enrollment, should be ignored

    accepted_enrollment_id, philip_submit_payload, _ = await submit_for(
        backend=backend,
        organization_id=minimalorg.organization_id,
        email="blacky@example.invalid",
        submit_payload_signature=AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:36vrY0YuWU/RK9HcxmV3DePU/uEQP7IhPj0+XrerO9YqfUNpusYUQgL/WdLbZblZv/A79yhuJL471fUMCvOQ6w==",
            author_openbao_entity_id="4ba1ff68-34f4-480f-9453-48016b53aeec",
        ),
        submitted_on=DateTime(2011, 1, 1),
    )
    certificates_generated_on = DateTime(2011, 1, 2)
    accepted_on = certificates_generated_on.add(seconds=1)
    blacky_user_certificates = generate_new_user_certificates(
        timestamp=certificates_generated_on,
        human_handle=philip_submit_payload.requested_human_handle,
        public_key=philip_submit_payload.public_key,
        author_device_id=minimalorg.alice.device_id,
        author_signing_key=minimalorg.alice.signing_key,
    )

    blacky_device_certificates = generate_new_device_certificates(
        timestamp=certificates_generated_on,
        user_id=blacky_user_certificates.certificate.user_id,
        device_label=philip_submit_payload.requested_device_label,
        verify_key=philip_submit_payload.verify_key,
        author_device_id=minimalorg.alice.device_id,
        author_signing_key=minimalorg.alice.signing_key,
    )

    accept_payload = AsyncEnrollmentAcceptPayload(
        user_id=blacky_user_certificates.certificate.user_id,
        device_id=blacky_device_certificates.certificate.device_id,
        device_label=blacky_device_certificates.certificate.device_label,
        human_handle=blacky_user_certificates.certificate.human_handle,
        profile=blacky_user_certificates.certificate.profile,
        root_verify_key=minimalorg.root_verify_key,
    ).dump()

    accept_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
        signature="vault:v1:kqrnMiRBFelGqTq7J4bmlhkGun09HshMIfOeGVoA8WZEEHkBlqoWQV+rI/WlBItUjRhBKVVm2PIigshKA7Cb+Q==",
        author_openbao_entity_id="33cc217a-5464-4a62-b19b-5bb217153357",
    )
    outcome = await backend.async_enrollment.accept(
        now=accepted_on,
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
        author_verify_key=minimalorg.alice.signing_key.verify_key,
        enrollment_id=accepted_enrollment_id,
        accept_payload=accept_payload,
        accept_payload_signature=accept_payload_signature,
        submitter_user_certificate=blacky_user_certificates.signed_certificate,
        submitter_redacted_user_certificate=blacky_user_certificates.signed_redacted_certificate,
        submitter_device_certificate=blacky_device_certificates.signed_certificate,
        submitter_redacted_device_certificate=blacky_device_certificates.signed_redacted_certificate,
    )
    assert isinstance(outcome, tuple)

    # Cancelled enrollment, should be ignored

    await submit_for(
        backend=backend,
        organization_id=minimalorg.organization_id,
        email="mike@example.invalid",
        submit_payload_signature=AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
        submitted_on=DateTime(2012, 1, 1),
    )

    # Overwrite mike's previous enrollment, this one should be part of the list

    mike_submitted_on = DateTime(2012, 1, 2)
    mike_submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
        signature="vault:v1:LWWedDmTTaxTZQu97MxFw58NPeI1x+03AqeEQTPEoRRHIlZVO49nY5o89FU7F1NhLnicHDfv/5wpJmZ31MKx9g==",
        author_openbao_entity_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
    )
    mike_enrollment_id, _, mike_submit_payload_raw = await submit_for(
        backend=backend,
        organization_id=minimalorg.organization_id,
        email="mike@example.invalid",
        submit_payload_signature=mike_submit_payload_signature,
        submitted_on=mike_submitted_on,
    )

    # Enrollment request with no prior request for this email, should be part of the list

    philip_submitted_on = DateTime(2013, 1, 1)
    philip_submit_payload_signature = AsyncEnrollmentPayloadSignaturePKI(
        signature=b"<submit_payload_signature>",
        algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        author_der_x509_certificate=test_pki.cert["mallory-sign"].certificate.der,
        intermediate_der_x509_certificates=[
            test_pki.intermediate["glados_dev_team"].certificate.der,
        ],
    )
    philip_enrollment_id, _, philip_submit_payload_raw = await submit_for(
        backend=backend,
        organization_id=minimalorg.organization_id,
        email="philip@example.invalid",
        submit_payload_signature=philip_submit_payload_signature,
        submitted_on=philip_submitted_on,
    )

    # Actual test!

    rep = await minimalorg.alice.async_enrollment_list()
    assert rep == authenticated_cmds.latest.async_enrollment_list.RepOk(
        enrollments=[
            authenticated_cmds.latest.async_enrollment_list.Enrollment(
                enrollment_id=mike_enrollment_id,
                submitted_on=mike_submitted_on,
                submit_payload=mike_submit_payload_raw,
                submit_payload_signature=authenticated_cmds.latest.async_enrollment_list.SubmitPayloadSignatureOpenBao(
                    signature=mike_submit_payload_signature.signature,
                    submitter_openbao_entity_id=mike_submit_payload_signature.author_openbao_entity_id,
                ),
            ),
            authenticated_cmds.latest.async_enrollment_list.Enrollment(
                enrollment_id=philip_enrollment_id,
                submitted_on=philip_submitted_on,
                submit_payload=philip_submit_payload_raw,
                submit_payload_signature=authenticated_cmds.latest.async_enrollment_list.SubmitPayloadSignaturePKI(
                    signature=philip_submit_payload_signature.signature,
                    algorithm=philip_submit_payload_signature.algorithm,
                    submitter_der_x509_certificate=philip_submit_payload_signature.author_der_x509_certificate,
                    intermediate_der_x509_certificates=philip_submit_payload_signature.intermediate_der_x509_certificates,
                ),
            ),
        ]
    )


async def test_authenticated_async_enrollment_list_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.bob.async_enrollment_list()
    assert rep == authenticated_cmds.latest.async_enrollment_list.RepAuthorNotAllowed()


async def test_authenticated_async_enrollment_list_http_common_errors(
    coolorg: CoolorgRpcClients,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.async_enrollment_list()

    await anonymous_http_common_errors_tester(do)
