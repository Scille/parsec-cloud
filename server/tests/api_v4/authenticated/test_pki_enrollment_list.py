# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    EnrollmentID,
    PkiEnrollmentSubmitPayload,
    PrivateKey,
    SigningKey,
    UserProfile,
    authenticated_cmds,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    bob_becomes_admin_and_changes_alice,
)


@pytest.mark.parametrize("kind", ("empty", "items"))
async def test_authenticated_pki_enrollment_list_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "empty":
            expected_enrollments = []

        case "items":
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

            expected_enrollments = [
                authenticated_cmds.v4.pki_enrollment_list.PkiEnrollmentListItem(
                    enrollment_id=enrollment_id,
                    submit_payload=submit_payload,
                    submit_payload_signature=b"<philip submit payload signature>",
                    submitted_on=submitted_on,
                    submitter_der_x509_certificate=b"<philip der x509 certificate>",
                )
            ]

        case unknown:
            assert False, unknown

    rep = await coolorg.alice.pki_enrollment_list()
    assert rep == authenticated_cmds.v4.pki_enrollment_list.RepOk(enrollments=expected_enrollments)


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_pki_enrollment_list_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
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

    rep = await author.pki_enrollment_list()
    assert rep == authenticated_cmds.v4.pki_enrollment_list.RepAuthorNotAllowed()


async def test_authenticated_pki_enrollment_list_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.pki_enrollment_list()

    await authenticated_http_common_errors_tester(do)
