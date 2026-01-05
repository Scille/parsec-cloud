# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from dataclasses import dataclass
from typing import Any

import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    AsyncEnrollmentAcceptPayload,
    AsyncEnrollmentID,
    AsyncEnrollmentSubmitPayload,
    DateTime,
    DeviceID,
    DeviceLabel,
    EmailAddress,
    HumanHandle,
    PkiSignatureAlgorithm,
    PrivateKey,
    RevokedUserCertificate,
    SigningKey,
    UserID,
    UserProfile,
    authenticated_cmds,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentInfoAccepted,
    AsyncEnrollmentPayloadSignature,
    AsyncEnrollmentPayloadSignatureOpenBao,
    AsyncEnrollmentPayloadSignaturePKI,
)
from parsec.events import EventAsyncEnrollment, EventCommonCertificate
from tests.common import (
    AuthenticatedRpcClient,
    Backend,
    CoolorgRpcClients,
    DeviceCertificates,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    TestPki,
    UserCertificates,
    generate_new_device_certificates,
    generate_new_user_certificates,
)


@dataclass(slots=True)
class Enrollment:
    enrollment_id: AsyncEnrollmentID
    submit_payload: AsyncEnrollmentSubmitPayload
    user_certificates: UserCertificates
    device_certificates: DeviceCertificates
    accept_payload: AsyncEnrollmentAcceptPayload
    accept_payload_signature: (
        authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignatureOpenBao
    )


async def submit_for_mike(
    backend: Backend,
    org: CoolorgRpcClients | MinimalorgRpcClients,
    submit_payload_signature: AsyncEnrollmentPayloadSignature | None = None,
    submitted_on: DateTime | None = None,
    certificates_timestamp: DateTime | None = None,
    accepter: AuthenticatedRpcClient | None = None,
    generate_new_user_certificates_kwargs: dict[str, Any] = {},
    generate_new_device_certificates_kwargs: dict[str, Any] = {},
) -> Enrollment:
    organization_id = org.organization_id
    accepter = accepter or org.alice
    enrollment_id = AsyncEnrollmentID.new()
    submitted_on = submitted_on or DateTime.now()
    certificates_timestamp = certificates_timestamp or submitted_on

    submit_payload_signature = submit_payload_signature or AsyncEnrollmentPayloadSignatureOpenBao(
        signature="vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==",
        author_openbao_entity_id="d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
    )
    submit_payload = AsyncEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        requested_device_label=DeviceLabel("Dev1"),
        requested_human_handle=HumanHandle(
            label="Mike", email=EmailAddress("mike@example.invalid")
        ),
    )

    with backend.event_bus.spy() as spy:
        outcome = await backend.async_enrollment.submit(
            now=submitted_on,
            organization_id=organization_id,
            enrollment_id=enrollment_id,
            force=True,
            submit_payload=submit_payload.dump(),
            submit_payload_signature=submit_payload_signature,
        )
        assert outcome is None
        await spy.wait_event_occurred(EventAsyncEnrollment(organization_id=organization_id))

    user_certificates = generate_new_user_certificates(
        **{
            "timestamp": certificates_timestamp,
            "human_handle": submit_payload.requested_human_handle,
            "public_key": submit_payload.public_key,
            "author_device_id": accepter.device_id,
            "author_signing_key": accepter.signing_key,
            **generate_new_user_certificates_kwargs,
        }
    )

    device_certificates = generate_new_device_certificates(
        **{
            "timestamp": certificates_timestamp,
            "user_id": user_certificates.certificate.user_id,
            "device_label": submit_payload.requested_device_label,
            "verify_key": submit_payload.verify_key,
            "author_device_id": accepter.device_id,
            "author_signing_key": accepter.signing_key,
            **generate_new_device_certificates_kwargs,
        }
    )

    accept_payload = AsyncEnrollmentAcceptPayload(
        user_id=user_certificates.certificate.user_id,
        device_id=device_certificates.certificate.device_id,
        device_label=device_certificates.certificate.device_label,
        human_handle=user_certificates.certificate.human_handle,
        profile=user_certificates.certificate.profile,
        root_verify_key=org.root_verify_key,
    )

    return Enrollment(
        enrollment_id=enrollment_id,
        submit_payload=submit_payload,
        user_certificates=user_certificates,
        device_certificates=device_certificates,
        accept_payload=accept_payload,
        accept_payload_signature=authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignatureOpenBao(
            signature="vault:v1:kqrnMiRBFelGqTq7J4bmlhkGun09HshMIfOeGVoA8WZEEHkBlqoWQV+rI/WlBItUjRhBKVVm2PIigshKA7Cb+Q==",
            accepter_openbao_entity_id="81b533c6-e41a-4533-9d50-3188cb88edd8",
        ),
    )


@pytest.mark.parametrize("kind", ("openbao", "pki"))
async def test_authenticated_async_enrollment_accept_ok(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    test_pki: TestPki,
    kind: str,
) -> None:
    match kind:
        case "openbao":
            submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
                signature="vault:v1:PCMqlBeeP3qSZF5ABMhV04/wYSIOCBaBAcvQtvtzffTDnRB0uTeD0gdUds/fRSZrJcwDIg4qy7OIatadeW/xfQ==",
                author_openbao_entity_id="33cc217a-5464-4a62-b19b-5bb217153357",
            )
            accept_payload_signature = authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignatureOpenBao(
                signature="vault:v1:kqrnMiRBFelGqTq7J4bmlhkGun09HshMIfOeGVoA8WZEEHkBlqoWQV+rI/WlBItUjRhBKVVm2PIigshKA7Cb+Q==",
                accepter_openbao_entity_id="81b533c6-e41a-4533-9d50-3188cb88edd8",
            )

        case "pki":
            submit_payload_signature = AsyncEnrollmentPayloadSignaturePKI(
                signature=b"<submit_payload_signature>",
                algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                author_der_x509_certificate=test_pki.cert["bob"].certificate.der,
                intermediate_der_x509_certificates=[],
            )
            accept_payload_signature = (
                authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignaturePKI(
                    signature=b"<accept_payload_signature>",
                    algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                    accepter_der_x509_certificate=test_pki.cert["mallory-sign"].certificate.der,
                    intermediate_der_x509_certificates=[
                        test_pki.intermediate["glados_dev_team"].certificate.der,
                    ],
                )
            )

        case unknown:
            assert False, unknown

    enrollment = await submit_for_mike(
        backend,
        minimalorg,
        submit_payload_signature,
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.async_enrollment_accept(
            enrollment_id=enrollment.enrollment_id,
            submitter_user_certificate=enrollment.user_certificates.signed_certificate,
            submitter_device_certificate=enrollment.device_certificates.signed_certificate,
            submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
            submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
            accept_payload=enrollment.accept_payload.dump(),
            accept_payload_signature=accept_payload_signature,
        )
        assert rep == authenticated_cmds.latest.async_enrollment_accept.RepOk()

        await spy.wait_event_occurred(
            EventCommonCertificate(
                organization_id=minimalorg.organization_id,
                timestamp=enrollment.user_certificates.certificate.timestamp,
            )
        )
        await spy.wait_event_occurred(
            EventAsyncEnrollment(organization_id=minimalorg.organization_id)
        )

    info = await backend.async_enrollment.info(
        organization_id=minimalorg.organization_id, enrollment_id=enrollment.enrollment_id
    )
    assert isinstance(info, AsyncEnrollmentInfoAccepted)
    assert info.submitted_on < info.accepted_on


async def test_authenticated_async_enrollment_accept_active_users_limit_reached(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    await backend.organization.update(
        now=DateTime.now(),
        id=minimalorg.organization_id,
        active_users_limit=ActiveUsersLimit.from_maybe_int(1),
    )

    enrollment = await submit_for_mike(
        backend,
        minimalorg,
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepActiveUsersLimitReached()


async def test_authenticated_async_enrollment_accept_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    enrollment = await submit_for_mike(
        backend,
        coolorg,
        accepter=coolorg.bob,
    )

    rep = await coolorg.bob.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepAuthorNotAllowed()


async def test_authenticated_async_enrollment_accept_enrollment_not_found(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    enrollment = await submit_for_mike(
        backend,
        minimalorg,
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=AsyncEnrollmentID.new(),
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepEnrollmentNotFound()


async def test_authenticated_async_enrollment_accept_enrollment_no_longer_available(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    enrollment = await submit_for_mike(
        backend,
        minimalorg,
    )

    outcome = await backend.async_enrollment.reject(
        now=DateTime.now(),
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
        enrollment_id=enrollment.enrollment_id,
    )
    assert outcome is None

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepEnrollmentNoLongerAvailable()


@pytest.mark.parametrize(
    "kind",
    (
        "conflicting_user_id",
        "conflicting_device_id",
    ),
)
async def test_authenticated_async_enrollment_accept_user_already_exists(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    user_id = UserID.new()
    device_id = DeviceID.new()
    match kind:
        case "conflicting_user_id":
            user_id = minimalorg.alice.user_id
        case "conflicting_device_id":
            device_id = minimalorg.alice.device_id
        case unknown:
            assert False, unknown

    enrollment = await submit_for_mike(
        backend,
        minimalorg,
        generate_new_user_certificates_kwargs={"user_id": user_id},
        generate_new_device_certificates_kwargs={"device_id": device_id},
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepUserAlreadyExists()


async def test_authenticated_async_enrollment_accept_human_handle_already_taken(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    enrollment = await submit_for_mike(
        backend,
        minimalorg,
        generate_new_user_certificates_kwargs={"human_handle": minimalorg.alice.human_handle},
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepHumanHandleAlreadyTaken()


async def test_authenticated_async_enrollment_accept_invalid_accept_payload(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    enrollment = await submit_for_mike(
        backend,
        minimalorg,
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=b"<dummy>",
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepInvalidAcceptPayload()


async def test_authenticated_async_enrollment_accept_submit_and_accept_identity_systems_mismatch(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    # Submit and accept signatures are supposed to be of the same type!
    submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
        signature="vault:v1:PCMqlBeeP3qSZF5ABMhV04/wYSIOCBaBAcvQtvtzffTDnRB0uTeD0gdUds/fRSZrJcwDIg4qy7OIatadeW/xfQ==",
        author_openbao_entity_id="33cc217a-5464-4a62-b19b-5bb217153357",
    )
    accept_payload_signature = (
        authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignaturePKI(
            signature=b"<accept_payload_signature>",
            accepter_der_x509_certificate=b"<accepter_der_x509_certificate>",
            algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            intermediate_der_x509_certificates=[],
        )
    )

    enrollment = await submit_for_mike(
        backend,
        minimalorg,
        submit_payload_signature=submit_payload_signature,
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=accept_payload_signature,
    )
    assert (
        rep
        == authenticated_cmds.latest.async_enrollment_accept.RepSubmitAndAcceptIdentitySystemsMismatch()
    )


@pytest.mark.skip(reason="Server-side identity validation not implemented yet")
async def test_authenticated_async_enrollment_accept_invalid_accept_payload_signature(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    # Submit and accept signatures are supposed to be of the same type!
    submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
        signature="vault:v1:PCMqlBeeP3qSZF5ABMhV04/wYSIOCBaBAcvQtvtzffTDnRB0uTeD0gdUds/fRSZrJcwDIg4qy7OIatadeW/xfQ==",
        author_openbao_entity_id="33cc217a-5464-4a62-b19b-5bb217153357",
    )
    accept_payload_signature = (
        authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignaturePKI(
            signature=b"<accept_payload_signature>",
            accepter_der_x509_certificate=b"<accepter_der_x509_certificate>",
            algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            intermediate_der_x509_certificates=[],
        )
    )

    enrollment = await submit_for_mike(
        backend,
        minimalorg,
        submit_payload_signature=submit_payload_signature,
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=accept_payload_signature,
    )
    assert (
        rep == authenticated_cmds.latest.async_enrollment_accept.RepInvalidAcceptPayloadSignature()
    )


@pytest.mark.parametrize("kind", ("bad_leaf", "bad_intermediate"))
async def test_authenticated_async_enrollment_accept_invalid_der_x509_certificate(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    test_pki: TestPki,
    kind: str,
) -> None:
    match kind:
        case "bad_leaf":
            accepter_der_x509_certificate = b"<dummy>"
            intermediate_der_x509_certificates = []

        case "bad_intermediate":
            accepter_der_x509_certificate = test_pki.cert["alice"].certificate.der
            intermediate_der_x509_certificates = [b"<dummy>"]

        case unknown:
            assert False, unknown

    enrollment = await submit_for_mike(
        backend,
        minimalorg,
        submit_payload_signature=AsyncEnrollmentPayloadSignaturePKI(
            signature=b"<submit_payload_signature>",
            algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            author_der_x509_certificate=test_pki.cert["bob"].certificate.der,
            intermediate_der_x509_certificates=[],
        ),
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignaturePKI(
            signature=b"<accept_payload_signature>",
            algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            accepter_der_x509_certificate=accepter_der_x509_certificate,
            intermediate_der_x509_certificates=intermediate_der_x509_certificates,
        ),
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepInvalidDerX509Certificate()


@pytest.mark.parametrize("kind", ("too_many_intermediate", "unused_intermediate"))
async def test_authenticated_async_enrollment_accept_invalid_x509_trustchain(
    monkeypatch: pytest.MonkeyPatch,
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    test_pki: TestPki,
    kind: str,
) -> None:
    match kind:
        case "too_many_intermediate":
            # Mallory is signed by Glados Dev Team intermediary certificate, so this is all good...
            accepter_der_x509_certificate = test_pki.cert["mallory-sign"].certificate.der
            intermediate_der_x509_certificates = [
                test_pki.intermediate["glados_dev_team"].certificate.der
            ]
            # ...but we patch the server to consider even one intermediary certificate is too much!
            monkeypatch.setattr(
                "parsec.components.async_enrollment.MAX_X509_INTERMEDIATE_CERTIFICATES_DEPTH", 0
            )

        case "unused_intermediate":
            # Unlike Mallory, Alice is not signed by Glados Dev Team intermediary certificate
            accepter_der_x509_certificate = test_pki.cert["alice"].certificate.der
            intermediate_der_x509_certificates = [
                test_pki.intermediate["glados_dev_team"].certificate.der,
            ]

        case unknown:
            assert False, unknown

    enrollment = await submit_for_mike(
        backend,
        minimalorg,
        submit_payload_signature=AsyncEnrollmentPayloadSignaturePKI(
            signature=b"<submit_payload_signature>",
            algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            author_der_x509_certificate=test_pki.cert["bob"].certificate.der,
            intermediate_der_x509_certificates=[],
        ),
    )

    rep = await minimalorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignaturePKI(
            signature=b"<accept_payload_signature>",
            algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            accepter_der_x509_certificate=accepter_der_x509_certificate,
            intermediate_der_x509_certificates=intermediate_der_x509_certificates,
        ),
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepInvalidX509Trustchain()


@pytest.mark.parametrize(
    "kind",
    (
        "device_certificate",
        "redacted_device_certificate",
        "user_certificate",
        "redacted_user_certificate",
        "user_certif_user_id_from_another_user",
        "user_certif_not_author_user_id",
        "user_certif_author_mismatch",
        "user_certif_timestamp_mismatch",
        "user_certif_profile_mismatch",
        "user_certif_user_id_mismatch",
        "user_certif_public_key_mismatch",
        "user_certif_not_redacted",
        "device_certif_user_id_from_another_user",
        "device_certif_not_author_user_id",
        "device_certif_author_mismatch",
        "device_certif_timestamp_mismatch",
        "device_certif_user_id_mismatch",
        "device_certif_device_id_mismatch",
        "device_certif_verify_key_mismatch",
        "device_certif_not_redacted",
    ),
)
async def test_authenticated_async_enrollment_accept_invalid_certificate(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    enrollment = await submit_for_mike(
        backend,
        coolorg,
    )
    accept_args = {
        "enrollment_id": enrollment.enrollment_id,
        "submitter_user_certificate": enrollment.user_certificates.signed_certificate,
        "submitter_device_certificate": enrollment.device_certificates.signed_certificate,
        "submitter_redacted_user_certificate": enrollment.user_certificates.signed_redacted_certificate,
        "submitter_redacted_device_certificate": enrollment.device_certificates.signed_redacted_certificate,
        "accept_payload": enrollment.accept_payload.dump(),
        "accept_payload_signature": enrollment.accept_payload_signature,
    }

    match kind:
        case "device_certificate":
            accept_args["submitter_device_certificate"] = b"<dummy>"

        case "redacted_device_certificate":
            accept_args["submitter_redacted_device_certificate"] = b"<dummy>"

        case "user_certificate":
            accept_args["submitter_user_certificate"] = b"<dummy>"

        case "redacted_user_certificate":
            accept_args["submitter_redacted_user_certificate"] = b"<dummy>"

        case "user_certif_user_id_from_another_user":
            certificates = generate_new_user_certificates(
                timestamp=enrollment.user_certificates.certificate.timestamp,
                user_id=coolorg.bob.user_id,
                human_handle=enrollment.user_certificates.certificate.human_handle,
                profile=enrollment.user_certificates.certificate.profile,
                public_key=enrollment.user_certificates.certificate.public_key,
                algorithm=enrollment.user_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_user_certificate"] = certificates.signed_certificate
            accept_args["submitter_redacted_user_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "user_certif_not_author_user_id":
            certificates = generate_new_user_certificates(
                timestamp=enrollment.user_certificates.certificate.timestamp,
                user_id=UserID.new(),  # Mismatch with the device certificate
                human_handle=enrollment.user_certificates.certificate.human_handle,
                profile=enrollment.user_certificates.certificate.profile,
                public_key=enrollment.user_certificates.certificate.public_key,
                algorithm=enrollment.user_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_user_certificate"] = certificates.signed_certificate
            accept_args["submitter_redacted_user_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "user_certif_author_mismatch":
            certificates = generate_new_user_certificates(
                timestamp=enrollment.user_certificates.certificate.timestamp,
                user_id=enrollment.user_certificates.certificate.user_id,
                human_handle=enrollment.user_certificates.certificate.human_handle,
                profile=enrollment.user_certificates.certificate.profile,
                public_key=enrollment.user_certificates.certificate.public_key,
                algorithm=enrollment.user_certificates.certificate.algorithm,
                author_device_id=DeviceID.test_from_nickname("alice@dev2"),
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_user_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "user_certif_timestamp_mismatch":
            certificates = generate_new_user_certificates(
                timestamp=enrollment.user_certificates.certificate.timestamp.subtract(seconds=1),
                user_id=enrollment.user_certificates.certificate.user_id,
                human_handle=enrollment.user_certificates.certificate.human_handle,
                profile=enrollment.user_certificates.certificate.profile,
                public_key=enrollment.user_certificates.certificate.public_key,
                algorithm=enrollment.user_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_user_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "user_certif_profile_mismatch":
            certificates = generate_new_user_certificates(
                timestamp=enrollment.user_certificates.certificate.timestamp,
                user_id=enrollment.user_certificates.certificate.user_id,
                human_handle=enrollment.user_certificates.certificate.human_handle,
                profile=UserProfile.OUTSIDER,
                public_key=enrollment.user_certificates.certificate.public_key,
                algorithm=enrollment.user_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_user_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "user_certif_user_id_mismatch":
            certificates = generate_new_user_certificates(
                timestamp=enrollment.user_certificates.certificate.timestamp,
                user_id=UserID.new(),
                human_handle=enrollment.user_certificates.certificate.human_handle,
                profile=enrollment.user_certificates.certificate.profile,
                public_key=enrollment.user_certificates.certificate.public_key,
                algorithm=enrollment.user_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_user_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "user_certif_public_key_mismatch":
            certificates = generate_new_user_certificates(
                timestamp=enrollment.user_certificates.certificate.timestamp,
                user_id=enrollment.user_certificates.certificate.user_id,
                human_handle=enrollment.user_certificates.certificate.human_handle,
                profile=enrollment.user_certificates.certificate.profile,
                public_key=PrivateKey.generate().public_key,
                algorithm=enrollment.user_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_user_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "user_certif_not_redacted":
            accept_args["submitter_redacted_user_certificate"] = accept_args[
                "submitter_user_certificate"
            ]

        case "device_certif_user_id_from_another_user":
            certificates = generate_new_device_certificates(
                timestamp=enrollment.device_certificates.certificate.timestamp,
                user_id=coolorg.bob.user_id,
                device_id=enrollment.device_certificates.certificate.device_id,
                device_label=enrollment.device_certificates.certificate.device_label,
                verify_key=enrollment.device_certificates.certificate.verify_key,
                purpose=enrollment.device_certificates.certificate.purpose,
                algorithm=enrollment.device_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_device_certificate"] = certificates.signed_certificate
            accept_args["submitter_redacted_device_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "device_certif_not_author_user_id":
            certificates = generate_new_device_certificates(
                timestamp=enrollment.device_certificates.certificate.timestamp,
                user_id=UserID.new(),  # Mismatch with the user certificate
                device_id=enrollment.device_certificates.certificate.device_id,
                device_label=enrollment.device_certificates.certificate.device_label,
                verify_key=enrollment.device_certificates.certificate.verify_key,
                purpose=enrollment.device_certificates.certificate.purpose,
                algorithm=enrollment.device_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_device_certificate"] = certificates.signed_certificate
            accept_args["submitter_redacted_device_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "device_certif_author_mismatch":
            certificates = generate_new_device_certificates(
                timestamp=enrollment.device_certificates.certificate.timestamp,
                user_id=enrollment.device_certificates.certificate.user_id,
                device_id=enrollment.device_certificates.certificate.device_id,
                device_label=enrollment.device_certificates.certificate.device_label,
                verify_key=enrollment.device_certificates.certificate.verify_key,
                purpose=enrollment.device_certificates.certificate.purpose,
                algorithm=enrollment.device_certificates.certificate.algorithm,
                author_device_id=DeviceID.test_from_nickname("alice@dev2"),
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_device_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "device_certif_timestamp_mismatch":
            certificates = generate_new_device_certificates(
                timestamp=enrollment.user_certificates.certificate.timestamp.subtract(seconds=1),
                user_id=enrollment.device_certificates.certificate.user_id,
                device_id=enrollment.device_certificates.certificate.device_id,
                device_label=enrollment.device_certificates.certificate.device_label,
                verify_key=enrollment.device_certificates.certificate.verify_key,
                purpose=enrollment.device_certificates.certificate.purpose,
                algorithm=enrollment.device_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_device_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "device_certif_user_id_mismatch":
            certificates = generate_new_device_certificates(
                timestamp=enrollment.device_certificates.certificate.timestamp,
                user_id=UserID.new(),
                device_id=enrollment.device_certificates.certificate.device_id,
                device_label=enrollment.device_certificates.certificate.device_label,
                verify_key=enrollment.device_certificates.certificate.verify_key,
                purpose=enrollment.device_certificates.certificate.purpose,
                algorithm=enrollment.device_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_device_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "device_certif_device_id_mismatch":
            certificates = generate_new_device_certificates(
                timestamp=enrollment.device_certificates.certificate.timestamp,
                user_id=enrollment.device_certificates.certificate.user_id,
                device_id=DeviceID.new(),
                device_label=enrollment.device_certificates.certificate.device_label,
                verify_key=enrollment.device_certificates.certificate.verify_key,
                purpose=enrollment.device_certificates.certificate.purpose,
                algorithm=enrollment.device_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_device_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "device_certif_verify_key_mismatch":
            certificates = generate_new_device_certificates(
                timestamp=enrollment.device_certificates.certificate.timestamp,
                user_id=enrollment.device_certificates.certificate.user_id,
                device_id=enrollment.device_certificates.certificate.device_id,
                device_label=enrollment.device_certificates.certificate.device_label,
                verify_key=SigningKey.generate().verify_key,
                purpose=enrollment.device_certificates.certificate.purpose,
                algorithm=enrollment.device_certificates.certificate.algorithm,
                author_device_id=coolorg.alice.device_id,
                author_signing_key=coolorg.alice.signing_key,
            )
            accept_args["submitter_redacted_device_certificate"] = (
                certificates.signed_redacted_certificate
            )

        case "device_certif_not_redacted":
            accept_args["submitter_redacted_device_certificate"] = accept_args[
                "submitter_device_certificate"
            ]

        case unknown:
            assert False, unknown

    rep = await coolorg.alice.async_enrollment_accept(**accept_args)
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepInvalidCertificate()


@pytest.mark.parametrize("kind", ("same_timestamp", "smaller_timestamp"))
async def test_authenticated_async_enrollment_accept_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    t1 = DateTime.now()
    t0 = t1.subtract(days=1)
    match kind:
        case "same_timestamp":
            t2 = t1
        case "smaller_timestamp":
            t2 = t1.subtract(seconds=1)
        case unknown:
            assert False, unknown

    # 1) Create a new certificate in the organization

    certif = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.mallory.user_id,
        timestamp=t1,
    )
    await backend.user.revoke_user(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )

    # 2) Our user creation's timestamp is clashing with the previous certificate

    enrollment = await submit_for_mike(
        backend,
        coolorg,
        submitted_on=t0,
        certificates_timestamp=t2,
    )

    rep = await coolorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert rep == authenticated_cmds.latest.async_enrollment_accept.RepRequireGreaterTimestamp(
        strictly_greater_than=t1,
    )


async def test_authenticated_async_enrollment_accept_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    t0 = DateTime.now().subtract(seconds=3600)

    enrollment = await submit_for_mike(
        backend,
        coolorg,
        submitted_on=t0,
        certificates_timestamp=t0,
    )

    rep = await coolorg.alice.async_enrollment_accept(
        enrollment_id=enrollment.enrollment_id,
        submitter_user_certificate=enrollment.user_certificates.signed_certificate,
        submitter_device_certificate=enrollment.device_certificates.signed_certificate,
        submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
        submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
        accept_payload=enrollment.accept_payload.dump(),
        accept_payload_signature=enrollment.accept_payload_signature,
    )
    assert isinstance(
        rep, authenticated_cmds.latest.async_enrollment_accept.RepTimestampOutOfBallpark
    )
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


async def test_authenticated_async_enrollment_accept_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    enrollment = await submit_for_mike(
        backend,
        coolorg,
    )

    async def do():
        await coolorg.alice.async_enrollment_accept(
            enrollment_id=enrollment.enrollment_id,
            submitter_user_certificate=enrollment.user_certificates.signed_certificate,
            submitter_device_certificate=enrollment.device_certificates.signed_certificate,
            submitter_redacted_user_certificate=enrollment.user_certificates.signed_redacted_certificate,
            submitter_redacted_device_certificate=enrollment.device_certificates.signed_redacted_certificate,
            accept_payload=enrollment.accept_payload.dump(),
            accept_payload_signature=enrollment.accept_payload_signature,
        )

    await anonymous_http_common_errors_tester(do)
