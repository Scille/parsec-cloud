# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    DeviceLabel,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PrivateKey,
    SigningKey,
    anonymous_cmds,
)
from tests.api_v5.anonymous.test_pki_enrollment_submit import PkiEnrollment
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester
from tests.common.pki import TestPki, accept_pki_enrollment, start_pki_enrollment


@pytest.mark.parametrize("kind", ("submitted", "accepted", "cancelled", "rejected"))
async def test_anonymous_pki_enrollment_info_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
    test_pki: TestPki,
    existing_pki_enrollment: PkiEnrollment,
) -> None:
    match kind:
        case "submitted":
            expected_unit = (
                anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusSubmitted(
                    submitted_on=existing_pki_enrollment.submitted_on
                )
            )

        case "accepted":
            accepted_on = existing_pki_enrollment.submitted_on.add(seconds=1)

            accepted = await accept_pki_enrollment(
                accepted_on,
                backend,
                coolorg.organization_id,
                coolorg.alice,
                test_pki.cert["alice"],
                [],
                coolorg.root_verify_key,
                existing_pki_enrollment,
            )

            expected_unit = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusAccepted(
                submitted_on=existing_pki_enrollment.submitted_on,
                accepted_on=accepted_on,
                accepter_der_x509_certificate=accepted.accepter_trustchain[0].content,
                accepter_intermediate_der_x509_certificates=list(
                    map(lambda x: x.content, accepted.accepter_trustchain[1:])
                ),
                accept_payload_signature=accepted.accepter_payload_signature,
                accept_payload_signature_algorithm=accepted.accepter_payload_signature_algorithm,
                accept_payload=accepted.accepter_payload,
            )

        case "cancelled":
            cancelled_on = existing_pki_enrollment.submitted_on.add(seconds=1)

            # Submit a new enrollment for this X509 certificate to cancel the previous one

            submit_payload = PkiEnrollmentSubmitPayload(
                verify_key=SigningKey.generate().verify_key,
                public_key=PrivateKey.generate().public_key,
                device_label=DeviceLabel("Dev1"),
            ).dump()
            await start_pki_enrollment(
                cancelled_on,
                backend,
                coolorg.organization_id,
                test_pki.cert["bob"],
                [],
                submit_payload,
                force=True,
            )

            expected_unit = (
                anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusCancelled(
                    submitted_on=existing_pki_enrollment.submitted_on,
                    cancelled_on=cancelled_on,
                )
            )

        case "rejected":
            rejected_on = existing_pki_enrollment.submitted_on.add(seconds=1)

            outcome = await backend.pki.reject(
                now=rejected_on,
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                enrollment_id=existing_pki_enrollment.enrollment_id,
            )
            assert outcome is None

            expected_unit = (
                anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusRejected(
                    submitted_on=existing_pki_enrollment.submitted_on,
                    rejected_on=rejected_on,
                )
            )

        case unknown:
            assert False, unknown

    rep = await coolorg.anonymous.pki_enrollment_info(
        enrollment_id=existing_pki_enrollment.enrollment_id
    )
    assert rep == anonymous_cmds.latest.pki_enrollment_info.RepOk(unit=expected_unit)


async def test_anonymous_pki_enrollment_info_enrollment_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.anonymous.pki_enrollment_info(enrollment_id=PKIEnrollmentID.new())
    assert rep == anonymous_cmds.latest.pki_enrollment_info.RepEnrollmentNotFound()


async def test_anonymous_pki_enrollment_info_http_common_errors(
    coolorg: CoolorgRpcClients,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
    existing_pki_enrollment: PkiEnrollment,
) -> None:
    async def do():
        await coolorg.anonymous.pki_enrollment_info(
            enrollment_id=existing_pki_enrollment.enrollment_id
        )

    await anonymous_http_common_errors_tester(do)
