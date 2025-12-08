# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    AsyncEnrollmentID,
    AsyncEnrollmentSubmitPayload,
    DateTime,
    DeviceLabel,
    EmailAddress,
    HumanHandle,
    OrganizationID,
    PrivateKey,
    SigningKey,
    authenticated_cmds,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentInfoRejected,
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentPayloadSignatureOpenBao,
)
from parsec.events import EventAsyncEnrollment
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
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
    outcome = await backend.async_enrollment.submit(
        now=submitted_on,
        organization_id=organization_id,
        enrollment_id=enrollment_id,
        force=True,
        submit_payload=submit_payload.dump(),
        submit_payload_signature=submit_payload_signature,
    )
    assert outcome is None

    return (enrollment_id, submit_payload)


async def test_authenticated_async_enrollment_reject_ok(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    enrollment_id, _ = await submit_for_mike(
        backend,
        minimalorg.organization_id,
        AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.async_enrollment_reject(enrollment_id=enrollment_id)
        assert rep == authenticated_cmds.latest.async_enrollment_reject.RepOk()

        await spy.wait_event_occurred(
            EventAsyncEnrollment(organization_id=minimalorg.organization_id)
        )

    info = await backend.async_enrollment.info(
        organization_id=minimalorg.organization_id, enrollment_id=enrollment_id
    )
    assert isinstance(info, AsyncEnrollmentInfoRejected)
    assert info.submitted_on < info.rejected_on


async def test_authenticated_async_enrollment_reject_enrollment_not_found(
    minimalorg: MinimalorgRpcClients,
) -> None:
    rep = await minimalorg.alice.async_enrollment_reject(enrollment_id=AsyncEnrollmentID.new())
    assert rep == authenticated_cmds.latest.async_enrollment_reject.RepEnrollmentNotFound()


async def test_authenticated_async_enrollment_reject_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    enrollment_id, _ = await submit_for_mike(
        backend,
        coolorg.organization_id,
        AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
    )
    rep = await coolorg.bob.async_enrollment_reject(enrollment_id=enrollment_id)
    assert rep == authenticated_cmds.latest.async_enrollment_reject.RepAuthorNotAllowed()


async def test_authenticated_async_enrollment_reject_enrollment_no_longer_available(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    enrollment_id, _ = await submit_for_mike(
        backend,
        coolorg.organization_id,
        AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
    )
    outcome = await backend.async_enrollment.reject(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        enrollment_id=enrollment_id,
    )
    assert outcome is None

    rep = await coolorg.alice.async_enrollment_reject(enrollment_id=enrollment_id)
    assert rep == authenticated_cmds.latest.async_enrollment_reject.RepEnrollmentNoLongerAvailable()


async def test_authenticated_async_enrollment_reject_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    enrollment_id, _ = await submit_for_mike(
        backend,
        coolorg.organization_id,
        AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
    )

    async def do():
        await coolorg.alice.async_enrollment_reject(enrollment_id=enrollment_id)

    await anonymous_http_common_errors_tester(do)
