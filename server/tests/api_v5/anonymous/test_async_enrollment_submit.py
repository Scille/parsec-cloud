# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
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
    AsyncEnrollmentInfoSubmitted,
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


@pytest.mark.parametrize("kind", ("openbao", "pki"))
async def test_anonymous_async_enrollment_submit_ok(
    backend: Backend,
    minimalorg: MinimalorgRpcClients,
    kind: str,
) -> None:
    match kind:
        case "openbao":
            submit_payload_signature = anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignatureOpenBao(
                signature="vault:v1:PCMqlBeeP3qSZF5ABMhV04/wYSIOCBaBAcvQtvtzffTDnRB0uTeD0gdUds/fRSZrJcwDIg4qy7OIatadeW/xfQ==",
                submitter_openbao_entity_alias_id="33cc217a-5464-4a62-b19b-5bb217153357",
            )

        case "pki":
            submit_payload_signature = (
                anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignaturePKI(
                    signature=b"<submit_payload_signature>",
                    submitter_der_x509_certificate=b"<submitter_der_x509_certificate>",
                    algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                    intermediate_der_x509_certificates=[],
                )
            )

        case unknown:
            assert False, unknown

    enrollment_id = AsyncEnrollmentID.new()

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.anonymous.async_enrollment_submit(
            enrollment_id=enrollment_id,
            force=False,
            submit_payload=AsyncEnrollmentSubmitPayload(
                verify_key=SigningKey.generate().verify_key,
                public_key=PrivateKey.generate().public_key,
                requested_device_label=DeviceLabel("Dev1"),
                requested_human_handle=HumanHandle(
                    label="Philip", email=EmailAddress("philip@example.com")
                ),
            ).dump(),
            submit_payload_signature=submit_payload_signature,
        )
        assert isinstance(rep, anonymous_cmds.latest.async_enrollment_submit.RepOk)

        await spy.wait_event_occurred(
            EventAsyncEnrollment(organization_id=minimalorg.organization_id)
        )

    info = await backend.async_enrollment.info(minimalorg.organization_id, enrollment_id)
    assert isinstance(info, AsyncEnrollmentInfoSubmitted)
    assert info.submitted_on == rep.submitted_on


async def test_anonymous_async_enrollment_submit_email_already_enrolled(
    minimalorg: MinimalorgRpcClients,
) -> None:
    rep = await minimalorg.anonymous.async_enrollment_submit(
        enrollment_id=AsyncEnrollmentID.new(),
        force=False,
        submit_payload=AsyncEnrollmentSubmitPayload(
            verify_key=SigningKey.generate().verify_key,
            public_key=PrivateKey.generate().public_key,
            requested_device_label=DeviceLabel("Dev1"),
            requested_human_handle=HumanHandle(
                label="Philip", email=minimalorg.alice.human_handle.email
            ),
        ).dump(),
        submit_payload_signature=anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignatureOpenBao(
            signature="vault:v1:PCMqlBeeP3qSZF5ABMhV04/wYSIOCBaBAcvQtvtzffTDnRB0uTeD0gdUds/fRSZrJcwDIg4qy7OIatadeW/xfQ==",
            submitter_openbao_entity_alias_id="33cc217a-5464-4a62-b19b-5bb217153357",
        ),
    )
    assert rep == anonymous_cmds.latest.async_enrollment_submit.RepEmailAlreadyEnrolled()


async def test_anonymous_async_enrollment_submit_email_already_submitted(
    backend: Backend,
    minimalorg: MinimalorgRpcClients,
) -> None:
    previous_submitted_on = DateTime(2010, 1, 1)
    _, previous_submit_payload = await submit_for_mike(
        backend,
        minimalorg.organization_id,
        AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
        submitted_on=previous_submitted_on,
    )

    rep = await minimalorg.anonymous.async_enrollment_submit(
        enrollment_id=AsyncEnrollmentID.new(),
        force=False,
        submit_payload=AsyncEnrollmentSubmitPayload(
            verify_key=SigningKey.generate().verify_key,
            public_key=PrivateKey.generate().public_key,
            requested_device_label=DeviceLabel("Dev1"),
            requested_human_handle=HumanHandle(
                label="Philip", email=previous_submit_payload.requested_human_handle.email
            ),
        ).dump(),
        submit_payload_signature=anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignatureOpenBao(
            signature="vault:v1:PCMqlBeeP3qSZF5ABMhV04/wYSIOCBaBAcvQtvtzffTDnRB0uTeD0gdUds/fRSZrJcwDIg4qy7OIatadeW/xfQ==",
            submitter_openbao_entity_alias_id="33cc217a-5464-4a62-b19b-5bb217153357",
        ),
    )
    assert rep == anonymous_cmds.latest.async_enrollment_submit.RepEmailAlreadySubmitted(
        submitted_on=previous_submitted_on
    )


async def test_anonymous_async_enrollment_submit_id_already_used(
    backend: Backend,
    minimalorg: MinimalorgRpcClients,
) -> None:
    enrollement_id, _ = await submit_for_mike(
        backend,
        minimalorg.organization_id,
        AsyncEnrollmentPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            author_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
    )

    rep = await minimalorg.anonymous.async_enrollment_submit(
        enrollment_id=enrollement_id,
        force=False,
        submit_payload=AsyncEnrollmentSubmitPayload(
            verify_key=SigningKey.generate().verify_key,
            public_key=PrivateKey.generate().public_key,
            requested_device_label=DeviceLabel("Dev1"),
            requested_human_handle=HumanHandle(
                label="Philip", email=EmailAddress("philip@example.invalid")
            ),
        ).dump(),
        submit_payload_signature=anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignatureOpenBao(
            signature="vault:v1:PCMqlBeeP3qSZF5ABMhV04/wYSIOCBaBAcvQtvtzffTDnRB0uTeD0gdUds/fRSZrJcwDIg4qy7OIatadeW/xfQ==",
            submitter_openbao_entity_alias_id="33cc217a-5464-4a62-b19b-5bb217153357",
        ),
    )
    assert rep == anonymous_cmds.latest.async_enrollment_submit.RepIdAlreadyUsed()


async def test_anonymous_async_enrollment_submit_invalid_submit_payload(
    minimalorg: MinimalorgRpcClients,
) -> None:
    rep = await minimalorg.anonymous.async_enrollment_submit(
        enrollment_id=AsyncEnrollmentID.new(),
        force=False,
        submit_payload=b"<dummy>",
        submit_payload_signature=anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            submitter_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
    )
    assert rep == anonymous_cmds.latest.async_enrollment_submit.RepInvalidSubmitPayload()


@pytest.mark.skip(reason="Server-side identity validation not implemented yet")
async def test_anonymous_async_enrollment_submit_invalid_submit_payload_signature(
    minimalorg: MinimalorgRpcClients,
) -> None:
    rep = await minimalorg.anonymous.async_enrollment_submit(
        enrollment_id=AsyncEnrollmentID.new(),
        force=False,
        submit_payload=AsyncEnrollmentSubmitPayload(
            verify_key=SigningKey.generate().verify_key,
            public_key=PrivateKey.generate().public_key,
            requested_device_label=DeviceLabel("Dev1"),
            requested_human_handle=HumanHandle(
                label="Mike", email=EmailAddress("mike@example.invalid")
            ),
        ).dump(),
        submit_payload_signature=anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignatureOpenBao(
            signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
            submitter_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
        ),
    )
    assert rep == anonymous_cmds.latest.async_enrollment_submit.RepInvalidSubmitPayloadSignature()


async def test_anonymous_async_enrollment_submit_http_common_errors(
    coolorg: CoolorgRpcClients,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.anonymous.async_enrollment_submit(
            enrollment_id=AsyncEnrollmentID.new(),
            force=False,
            submit_payload=AsyncEnrollmentSubmitPayload(
                verify_key=SigningKey.generate().verify_key,
                public_key=PrivateKey.generate().public_key,
                requested_device_label=DeviceLabel("Dev1"),
                requested_human_handle=HumanHandle(
                    label="Mike", email=EmailAddress("mike@example.invalid")
                ),
            ).dump(),
            submit_payload_signature=anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignatureOpenBao(
                signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
                submitter_openbao_entity_alias_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
            ),
        )

    await anonymous_http_common_errors_tester(do)
