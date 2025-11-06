# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    EmailAddress,
    EnrollmentID,
    HumanHandle,
    PkiEnrollmentSubmitPayload,
    PrivateKey,
    SigningKey,
    UserProfile,
    authenticated_cmds,
)
from parsec.events import EventPkiEnrollment
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    bob_becomes_admin_and_changes_alice,
)


@pytest.fixture
async def enrollment_id(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> EnrollmentID:
    enrollment_id = EnrollmentID.new()
    submitted_on = DateTime.now()
    submit_payload = PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
        human_handle=HumanHandle(label="Alice", email=EmailAddress("alice@example.invalid")),
    ).dump()
    outcome = await backend.pki.submit(
        now=submitted_on,
        organization_id=coolorg.organization_id,
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<philip der x509 certificate>",
        submit_payload_signature=b"<philip submit payload signature>",
        submit_payload=submit_payload,
    )
    assert outcome is None

    return enrollment_id


async def test_authenticated_pki_enrollment_reject_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    enrollment_id: EnrollmentID,
) -> None:
    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.pki_enrollment_reject(enrollment_id=enrollment_id)
        assert rep == authenticated_cmds.latest.pki_enrollment_reject.RepOk()

        await spy.wait_event_occurred(
            EventPkiEnrollment(
                organization_id=coolorg.organization_id,
            )
        )


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_pki_enrollment_reject_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, enrollment_id: EnrollmentID, kind: str
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

    rep = await author.pki_enrollment_reject(enrollment_id=enrollment_id)
    assert rep == authenticated_cmds.latest.pki_enrollment_reject.RepAuthorNotAllowed()


async def test_authenticated_pki_enrollment_reject_enrollment_no_longer_available(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    enrollment_id: EnrollmentID,
) -> None:
    outcome = await backend.pki.reject(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        enrollment_id=enrollment_id,
    )
    assert outcome is None

    rep = await coolorg.alice.pki_enrollment_reject(enrollment_id=enrollment_id)
    assert rep == authenticated_cmds.latest.pki_enrollment_reject.RepEnrollmentNoLongerAvailable()


async def test_authenticated_pki_enrollment_reject_enrollment_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.pki_enrollment_reject(enrollment_id=EnrollmentID.new())
    assert rep == authenticated_cmds.latest.pki_enrollment_reject.RepEnrollmentNotFound()


async def test_authenticated_pki_enrollment_reject_http_common_errors(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.pki_enrollment_reject(enrollment_id=enrollment_id)

    await authenticated_http_common_errors_tester(do)
