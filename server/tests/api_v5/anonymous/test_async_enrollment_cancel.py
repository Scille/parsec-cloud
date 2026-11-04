# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    AsyncEnrollmentID,
    DateTime,
    anonymous_cmds,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentPayloadSignatureOpenBao,
)
from parsec.events import EventAsyncEnrollment
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
)

from .test_async_enrollment_submit import submit_for_mike


async def test_anonymous_async_enrollment_cancel_ok(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
        signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
        author_openbao_entity_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
    )

    enrollment_id, _ = await submit_for_mike(
        backend,
        minimalorg.organization_id,
        submit_payload_signature,
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.anonymous.async_enrollment_cancel(enrollment_id=enrollment_id)
        assert rep == anonymous_cmds.latest.async_enrollment_cancel.RepOk()
        await spy.wait_event_occurred(
            EventAsyncEnrollment(organization_id=minimalorg.organization_id)
        )

    # Verify the enrollment is now cancelled
    info_rep = await minimalorg.anonymous.async_enrollment_info(enrollment_id=enrollment_id)
    assert isinstance(info_rep, anonymous_cmds.latest.async_enrollment_info.RepOk)
    assert isinstance(
        info_rep.unit, anonymous_cmds.latest.async_enrollment_info.InfoStatusCancelled
    )


async def test_anonymous_async_enrollment_cancel_enrollment_not_found(
    minimalorg: MinimalorgRpcClients,
) -> None:
    rep = await minimalorg.anonymous.async_enrollment_cancel(enrollment_id=AsyncEnrollmentID.new())
    assert rep == anonymous_cmds.latest.async_enrollment_cancel.RepEnrollmentNotFound()


@pytest.mark.parametrize("kind", ("cancelled", "rejected"))
async def test_anonymous_async_enrollment_cancel_enrollment_no_longer_available(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
        signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
        author_openbao_entity_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
    )

    enrollment_id, _ = await submit_for_mike(
        backend,
        minimalorg.organization_id,
        submit_payload_signature,
    )

    match kind:
        case "cancelled":
            outcome = await backend.async_enrollment.cancel(
                now=DateTime.now(),
                organization_id=minimalorg.organization_id,
                enrollment_id=enrollment_id,
            )
            assert outcome is None

        case "rejected":
            outcome = await backend.async_enrollment.reject(
                now=DateTime.now(),
                organization_id=minimalorg.organization_id,
                author=minimalorg.alice.device_id,
                enrollment_id=enrollment_id,
            )
            assert outcome is None

        case unknown:
            assert False, unknown

    rep = await minimalorg.anonymous.async_enrollment_cancel(enrollment_id=enrollment_id)
    assert rep == anonymous_cmds.latest.async_enrollment_cancel.RepEnrollmentNoLongerAvailable()


async def test_anonymous_async_enrollment_cancel_http_common_errors(
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
        await coolorg.anonymous.async_enrollment_cancel(enrollment_id=enrollment_id)

    await anonymous_http_common_errors_tester(do)
