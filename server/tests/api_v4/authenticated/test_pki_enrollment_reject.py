# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    EnrollmentID,
    PkiEnrollmentSubmitPayload,
    PrivateKey,
    SigningKey,
    authenticated_cmds,
)
from tests.common import Backend, CoolorgRpcClients


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

    return enrollment_id


async def test_authenticated_pki_enrollment_reject_ok(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
) -> None:
    rep = await coolorg.alice.pki_enrollment_reject(enrollment_id=enrollment_id)
    assert rep == authenticated_cmds.v4.pki_enrollment_reject.RepOk()


async def test_authenticated_pki_enrollment_reject_author_not_allowed(
    coolorg: CoolorgRpcClients,
    enrollment_id: EnrollmentID,
) -> None:
    rep = await coolorg.bob.pki_enrollment_reject(enrollment_id=enrollment_id)
    assert rep == authenticated_cmds.v4.pki_enrollment_reject.RepAuthorNotAllowed()


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
    assert rep == authenticated_cmds.v4.pki_enrollment_reject.RepEnrollmentNoLongerAvailable()


async def test_authenticated_pki_enrollment_reject_enrollment_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.pki_enrollment_reject(enrollment_id=EnrollmentID.new())
    assert rep == authenticated_cmds.v4.pki_enrollment_reject.RepEnrollmentNotFound()
