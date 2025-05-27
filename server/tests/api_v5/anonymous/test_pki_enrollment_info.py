# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    EmailAddress,
    EnrollmentID,
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentSubmitPayload,
    PrivateKey,
    SigningKey,
    UserProfile,
    anonymous_cmds,
)
from tests.api_v5.authenticated.test_user_create import (
    NEW_MIKE_DEVICE_ID,
    NEW_MIKE_DEVICE_LABEL,
    NEW_MIKE_HUMAN_HANDLE,
    NEW_MIKE_USER_ID,
    generate_new_mike_device_certificates,
    generate_new_mike_user_certificates,
)
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester


@pytest.mark.parametrize("kind", ("submitted", "accepted", "cancelled", "rejected"))
async def test_anonymous_pki_enrollment_info_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
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
        submitter_der_x509_certificate=b"<mike der x509 certificate>",
        submitter_der_x509_certificate_email=EmailAddress("mike@example.invalid"),
        submit_payload_signature=b"<mike submit payload signature>",
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
                human_handle=NEW_MIKE_HUMAN_HANDLE,
                profile=UserProfile.STANDARD,
                root_verify_key=coolorg.root_verify_key,
            ).dump()

            outcome = await backend.pki.accept(
                now=accepted_on,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                author_verify_key=coolorg.alice.signing_key.verify_key,
                enrollment_id=enrollment_id,
                accept_payload=accept_payload,
                accept_payload_signature=b"<alice accept payload signature>",
                accepter_der_x509_certificate=b"<alice der x509 certificate>",
                user_certificate=u_certif,
                redacted_user_certificate=redacted_u_certif,
                device_certificate=d_certif,
                redacted_device_certificate=redacted_d_certif,
            )
            assert isinstance(outcome, tuple)

            expected_unit = (
                anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusAccepted(
                    submitted_on=submitted_on,
                    accepted_on=accepted_on,
                    accepter_der_x509_certificate=b"<alice der x509 certificate>",
                    accept_payload_signature=b"<alice accept payload signature>",
                    accept_payload=accept_payload,
                )
            )

        case "cancelled":
            cancelled_on = submitted_on.add(seconds=1)

            # Submit a new enrollment for this X509 certificate to cancel the previous one

            new_enrollment_id = EnrollmentID.new()
            submit_payload = PkiEnrollmentSubmitPayload(
                verify_key=SigningKey.generate().verify_key,
                public_key=PrivateKey.generate().public_key,
                requested_device_label=DeviceLabel("Dev1"),
            ).dump()
            outcome = await backend.pki.submit(
                now=cancelled_on,
                organization_id=coolorg.organization_id,
                enrollment_id=new_enrollment_id,
                force=True,
                submitter_der_x509_certificate=b"<mike der x509 certificate>",
                submitter_der_x509_certificate_email=EmailAddress("mike@example.invalid"),
                submit_payload_signature=b"<mike submit payload signature>",
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

    rep = await coolorg.anonymous.pki_enrollment_info(enrollment_id=enrollment_id)
    assert rep == anonymous_cmds.latest.pki_enrollment_info.RepOk(unit=expected_unit)


async def test_anonymous_pki_enrollment_info_enrollment_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_info(enrollment_id=EnrollmentID.new())
    assert rep == anonymous_cmds.latest.pki_enrollment_info.RepEnrollmentNotFound()


async def test_anonymous_pki_enrollment_info_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
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
        submitter_der_x509_certificate=b"<mike der x509 certificate>",
        submitter_der_x509_certificate_email=EmailAddress("mike@example.invalid"),
        submit_payload_signature=b"<mike submit payload signature>",
        submit_payload=submit_payload,
    )
    assert outcome is None

    async def do():
        await coolorg.anonymous.pki_enrollment_info(enrollment_id=enrollment_id)

    await anonymous_http_common_errors_tester(do)
