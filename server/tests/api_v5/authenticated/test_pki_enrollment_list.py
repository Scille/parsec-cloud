# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    EmailAddress,
    HumanHandle,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PrivateKey,
    SigningKey,
    UserProfile,
    authenticated_cmds,
)
from tests.api_v5.authenticated.test_pki_enrollment_accept import generate_accept_params
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    bob_becomes_admin_and_changes_alice,
)


async def test_authenticated_pki_enrollment_list_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    # 1) Check with no enrollments available

    rep = await coolorg.alice.pki_enrollment_list()
    assert rep == authenticated_cmds.latest.pki_enrollment_list.RepOk(enrollments=[])

    # 2) Add enrollments

    expected_enrollments: list[
        authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem
    ] = []
    for i in range(4):
        enrollment_id = PKIEnrollmentID.new()
        submitted_on = DateTime.now()
        submit_payload = PkiEnrollmentSubmitPayload(
            verify_key=SigningKey.generate().verify_key,
            public_key=PrivateKey.generate().public_key,
            device_label=DeviceLabel("Dev1"),
            human_handle=HumanHandle(
                label=f"User{i}", email=EmailAddress(f"user{i}@example.invalid")
            ),
        ).dump()
        expected_enrollment_item = (
            authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                der_x509_certificate=f"<user{i} der x509 certificate>".encode(),
                payload_signature=f"<user{i} submit payload signature>".encode(),
                payload=submit_payload,
            )
        )
        outcome = await backend.pki.submit(
            now=submitted_on,
            organization_id=coolorg.organization_id,
            enrollment_id=enrollment_id,
            force=False,
            submitter_der_x509_certificate=expected_enrollment_item.der_x509_certificate,
            submit_payload_signature=expected_enrollment_item.payload_signature,
            submit_payload=expected_enrollment_item.payload,
        )
        assert outcome is None
        expected_enrollments.append(expected_enrollment_item)

    rep = await coolorg.alice.pki_enrollment_list()
    assert rep == authenticated_cmds.latest.pki_enrollment_list.RepOk(
        enrollments=expected_enrollments
    )

    # 3) Also ensure `ACCEPTED/CANCELLED/REJECTED` enrollments are ignored

    to_accept = expected_enrollments.pop()
    outcome = await backend.pki.accept(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        **generate_accept_params(coolorg, to_accept.enrollment_id),
    )
    assert isinstance(outcome, tuple)

    to_reject = expected_enrollments.pop()
    outcome = await backend.pki.reject(
        now=DateTime.now(),
        author=coolorg.alice.device_id,
        organization_id=coolorg.organization_id,
        enrollment_id=to_reject.enrollment_id,
    )
    assert outcome is None

    to_cancel = expected_enrollments.pop()
    canceller_enrollment_id = PKIEnrollmentID.new()
    canceller_submitted_on = DateTime.now()
    canceller_submit_payload = PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
        human_handle=HumanHandle(
            label="Canceller", email=EmailAddress("canceller@example.invalid")
        ),
    ).dump()
    canceller_expected_enrollment_item = (
        authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem(
            enrollment_id=canceller_enrollment_id,
            submitted_on=canceller_submitted_on,
            der_x509_certificate=to_cancel.der_x509_certificate,
            payload_signature=b"<canceller submit payload signature>",
            payload=canceller_submit_payload,
        )
    )
    outcome = await backend.pki.submit(
        now=canceller_submitted_on,
        organization_id=coolorg.organization_id,
        enrollment_id=canceller_enrollment_id,
        force=True,
        submitter_der_x509_certificate=canceller_expected_enrollment_item.der_x509_certificate,
        submit_payload_signature=canceller_expected_enrollment_item.payload_signature,
        submit_payload=canceller_expected_enrollment_item.payload,
    )
    assert outcome is None
    expected_enrollments.append(canceller_expected_enrollment_item)

    rep = await coolorg.alice.pki_enrollment_list()
    assert rep == authenticated_cmds.latest.pki_enrollment_list.RepOk(
        enrollments=expected_enrollments
    )


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
    assert rep == authenticated_cmds.latest.pki_enrollment_list.RepAuthorNotAllowed()


async def test_authenticated_pki_enrollment_list_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.pki_enrollment_list()

    await authenticated_http_common_errors_tester(do)
