# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import auto
from hashlib import sha256
from typing import Literal

from parsec._parsec import (
    AsyncEnrollmentAcceptPayload,
    AsyncEnrollmentID,
    AsyncEnrollmentSubmitPayload,
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    PkiSignatureAlgorithm,
    UserCertificate,
    VerifyKey,
    X509Certificate,
    anonymous_cmds,
    authenticated_cmds,
)
from parsec.api import api
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.client_context import (
    AnonymousClientContext,
    AuthenticatedClientContext,
)
from parsec.logging import get_logger
from parsec.types import BadOutcome, BadOutcomeEnum

logger = get_logger()


# Maximum number of certificates a trustchain can contains (excluding leaf and root).
# We follow OpenSSL's default here, which should be plenty for most use-cases.
# (see https://docs.openssl.org/3.2/man3/SSL_CTX_set_verify/)
MAX_X509_INTERMEDIATE_CERTIFICATES_DEPTH = 100


@dataclass(slots=True)
class AsyncEnrollmentEmailAlreadySubmitted(BadOutcome):
    submitted_on: DateTime


class AsyncEnrollmentSubmitBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    EMAIL_ALREADY_ENROLLED = auto()
    ID_ALREADY_USED = auto()


class AsyncEnrollmentSubmitValidateBadOutcome(BadOutcomeEnum):
    INVALID_SUBMIT_PAYLOAD = auto()
    INVALID_SUBMIT_PAYLOAD_SIGNATURE = auto()  # TODO: server-side validation not yet implemented
    INVALID_DER_X509_CERTIFICATE = auto()
    INVALID_X509_TRUSTCHAIN = auto()


class AsyncEnrollmentInfoBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    ENROLLMENT_NOT_FOUND = auto()


class AsyncEnrollmentListBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()


class AsyncEnrollmentRejectBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    ENROLLMENT_NOT_FOUND = auto()
    ENROLLMENT_NO_LONGER_AVAILABLE = auto()


class AsyncEnrollmentAcceptBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    ENROLLMENT_NOT_FOUND = auto()
    ENROLLMENT_NO_LONGER_AVAILABLE = auto()
    ACTIVE_USERS_LIMIT_REACHED = auto()
    USER_ALREADY_EXISTS = auto()
    HUMAN_HANDLE_ALREADY_TAKEN = auto()
    # Submit payload for this enrollment has been signed using a different
    # identity system that the provided accept payload
    SUBMIT_AND_ACCEPT_IDENTITY_SYSTEMS_MISMATCH = auto()


class AsyncEnrollmentAcceptValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()
    CERTIFICATES_TIMESTAMP_MISMATCH = auto()
    CERTIFICATES_USER_ID_MISMATCH = auto()
    CERTIFICATES_INVALID_REDACTED = auto()
    CERTIFICATES_REDACTED_MISMATCH = auto()
    INVALID_ACCEPT_PAYLOAD = auto()
    INVALID_ACCEPT_PAYLOAD_SIGNATURE = auto()  # TODO: server-side validation not yet implemented
    INVALID_DER_X509_CERTIFICATE = auto()  # TODO: server-side validation not yet implemented
    INVALID_X509_TRUSTCHAIN = auto()  # TODO: server-side validation not yet implemented


type AsyncEnrollmentPayloadSignature = (
    AsyncEnrollmentPayloadSignaturePKI | AsyncEnrollmentPayloadSignatureOpenBao
)


@dataclass(slots=True)
class AsyncEnrollmentPayloadSignaturePKI:
    signature: bytes
    author_der_x509_certificate: bytes
    algorithm: PkiSignatureAlgorithm
    intermediate_der_x509_certificates: list[bytes]


@dataclass(slots=True)
class AsyncEnrollmentPayloadSignatureOpenBao:
    signature: str
    author_openbao_entity_id: str


type AsyncEnrollmentInfo = (
    AsyncEnrollmentInfoSubmitted
    | AsyncEnrollmentInfoRejected
    | AsyncEnrollmentInfoCancelled
    | AsyncEnrollmentInfoAccepted
)


@dataclass(slots=True)
class AsyncEnrollmentInfoSubmitted:
    submitted_on: DateTime


@dataclass(slots=True)
class AsyncEnrollmentInfoRejected:
    rejected_on: DateTime
    submitted_on: DateTime


@dataclass(slots=True)
class AsyncEnrollmentInfoCancelled:
    cancelled_on: DateTime
    submitted_on: DateTime


@dataclass(slots=True)
class AsyncEnrollmentInfoAccepted:
    submitted_on: DateTime
    accepted_on: DateTime
    accept_payload: bytes
    accept_payload_signature: AsyncEnrollmentPayloadSignature


@dataclass(slots=True)
class AsyncEnrollmentListItem:
    enrollment_id: AsyncEnrollmentID
    submitted_on: DateTime
    submit_payload: bytes
    submit_payload_signature: AsyncEnrollmentPayloadSignature


# TODO: `X509Certificate` is error prone since it's validation is currently lazy
# (see https://github.com/Scille/parsec-cloud/issues/12012).
@dataclass(slots=True, frozen=True)
class X509Certificate2:
    der: bytes
    issuer: bytes
    subject: bytes
    sha256_fingerprint: bytes

    @classmethod
    def from_der(cls, der: bytes) -> X509Certificate2:
        certificate = X509Certificate.from_der(der)
        return X509Certificate2(
            der=der,
            # Raises `PkiInvalidCertificateDER` which is a subclass of `ValueError`
            issuer=certificate.issuer(),
            subject=certificate.subject(),
            sha256_fingerprint=sha256(der).digest(),
        )

    def __repr__(self) -> str:
        return f"{self.__class__.__qualname__}(subject={self.subject}, issuer={self.issuer})"


def _validate_x509_trustchain(
    leaf: bytes, intermediates: list[bytes]
) -> (
    list[X509Certificate2]
    | Literal["INVALID_DER_X509_CERTIFICATE"]
    | Literal["INVALID_X509_TRUSTCHAIN"]
):
    # Validate format for each x509 certificate
    try:
        author_der_x509_certificate = X509Certificate2.from_der(leaf)
        intermediate_der_x509_certificates = [
            X509Certificate2.from_der(certif) for certif in intermediates
        ]
    except ValueError:
        return "INVALID_DER_X509_CERTIFICATE"
    x509_trustchain = [author_der_x509_certificate, *intermediate_der_x509_certificates]

    # Validate the x509 trustchain:
    # - Should not be longer than `MAX_X509_INTERMEDIATE_CERTIFICATES_DEPTH` items (excluding leaf and root)
    # - Each intermediate certificate should be used exactly once

    if len(intermediate_der_x509_certificates) > MAX_X509_INTERMEDIATE_CERTIFICATES_DEPTH:
        return "INVALID_X509_TRUSTCHAIN"

    current = author_der_x509_certificate
    while True:
        try:
            i, current = next(
                (i, c)
                for i, c in enumerate(intermediate_der_x509_certificates)
                if c.subject == current.issuer
            )
            intermediate_der_x509_certificates.pop(i)
        except StopIteration:  # Parent not found, we should have reached the root
            if intermediate_der_x509_certificates:
                # Unused intermediate certificates is not allowed!
                return "INVALID_X509_TRUSTCHAIN"
            break

    return x509_trustchain


def async_enrollment_submit_validate(
    now: DateTime,
    submit_payload: bytes,
    submit_payload_signature: AsyncEnrollmentPayloadSignature,
) -> (
    tuple[AsyncEnrollmentSubmitPayload, list[X509Certificate2] | None]
    | AsyncEnrollmentSubmitValidateBadOutcome
):
    try:
        p_data = AsyncEnrollmentSubmitPayload.load(submit_payload)
    except ValueError:
        return AsyncEnrollmentSubmitValidateBadOutcome.INVALID_SUBMIT_PAYLOAD

    if isinstance(submit_payload_signature, AsyncEnrollmentPayloadSignaturePKI):
        match _validate_x509_trustchain(
            submit_payload_signature.author_der_x509_certificate,
            submit_payload_signature.intermediate_der_x509_certificates,
        ):
            case list() as x509_trustchain:
                pass
            case "INVALID_DER_X509_CERTIFICATE":
                return AsyncEnrollmentSubmitValidateBadOutcome.INVALID_DER_X509_CERTIFICATE
            case "INVALID_X509_TRUSTCHAIN":
                return AsyncEnrollmentSubmitValidateBadOutcome.INVALID_X509_TRUSTCHAIN
    else:
        x509_trustchain = None

    # TODO: Payload signature is currently not validated.
    #       Note this has no big impact on security since the client always
    #       validate the payload signature on its end.

    return (p_data, x509_trustchain)


def async_enrollment_accept_validate(
    now: DateTime,
    expected_author: DeviceID,
    author_verify_key: VerifyKey,
    accept_payload: bytes,
    accept_payload_signature: AsyncEnrollmentPayloadSignature,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> (
    tuple[
        AsyncEnrollmentAcceptPayload,
        UserCertificate,
        DeviceCertificate,
        list[X509Certificate2] | None,
    ]
    | TimestampOutOfBallpark
    | AsyncEnrollmentAcceptValidateBadOutcome
):
    try:
        p_data = AsyncEnrollmentAcceptPayload.load(accept_payload)
    except ValueError:
        return AsyncEnrollmentAcceptValidateBadOutcome.INVALID_ACCEPT_PAYLOAD

    if isinstance(accept_payload_signature, AsyncEnrollmentPayloadSignaturePKI):
        match _validate_x509_trustchain(
            accept_payload_signature.author_der_x509_certificate,
            accept_payload_signature.intermediate_der_x509_certificates,
        ):
            case list() as x509_trustchain:
                pass
            case "INVALID_DER_X509_CERTIFICATE":
                return AsyncEnrollmentAcceptValidateBadOutcome.INVALID_DER_X509_CERTIFICATE
            case "INVALID_X509_TRUSTCHAIN":
                return AsyncEnrollmentAcceptValidateBadOutcome.INVALID_X509_TRUSTCHAIN
    else:
        x509_trustchain = None

    # TODO: Payload signature is currently not validated.
    #       Note this has no big impact on security since the client always
    #       validate the payload signature on its end.

    try:
        d_data = DeviceCertificate.verify_and_load(
            device_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )
        u_data = UserCertificate.verify_and_load(
            user_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )
        ru_data = UserCertificate.verify_and_load(
            redacted_user_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )
        rd_data = DeviceCertificate.verify_and_load(
            redacted_device_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )

    except ValueError:
        return AsyncEnrollmentAcceptValidateBadOutcome.INVALID_CERTIFICATE

    if u_data.timestamp != d_data.timestamp:
        return AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_TIMESTAMP_MISMATCH

    match timestamps_in_the_ballpark(u_data.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    if u_data.user_id != d_data.user_id:
        return AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_USER_ID_MISMATCH

    if not ru_data.is_redacted:
        return AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_INVALID_REDACTED

    if not rd_data.is_redacted:
        return AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_INVALID_REDACTED

    if not u_data.redacted_compare(ru_data):
        return AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_REDACTED_MISMATCH

    if not d_data.redacted_compare(rd_data):
        return AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_REDACTED_MISMATCH

    return p_data, u_data, d_data, x509_trustchain


class BaseAsyncEnrollmentComponent:
    async def submit(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        enrollment_id: AsyncEnrollmentID,
        force: bool,
        submit_payload: bytes,
        submit_payload_signature: AsyncEnrollmentPayloadSignature,
    ) -> (
        None
        | AsyncEnrollmentSubmitValidateBadOutcome
        | AsyncEnrollmentEmailAlreadySubmitted
        | AsyncEnrollmentSubmitBadOutcome
    ):
        raise NotImplementedError

    async def info(
        self,
        organization_id: OrganizationID,
        enrollment_id: AsyncEnrollmentID,
    ) -> AsyncEnrollmentInfo | AsyncEnrollmentInfoBadOutcome:
        raise NotImplementedError

    async def list(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> list[AsyncEnrollmentListItem] | AsyncEnrollmentListBadOutcome:
        raise NotImplementedError

    async def reject(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        enrollment_id: AsyncEnrollmentID,
    ) -> None | AsyncEnrollmentRejectBadOutcome:
        raise NotImplementedError

    async def accept(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        enrollment_id: AsyncEnrollmentID,
        accept_payload: bytes,
        accept_payload_signature: AsyncEnrollmentPayloadSignature,
        submitter_user_certificate: bytes,
        submitter_redacted_user_certificate: bytes,
        submitter_device_certificate: bytes,
        submitter_redacted_device_certificate: bytes,
    ) -> (
        tuple[UserCertificate, DeviceCertificate]
        | AsyncEnrollmentAcceptValidateBadOutcome
        | AsyncEnrollmentAcceptBadOutcome
        | RequireGreaterTimestamp
        | TimestampOutOfBallpark
    ):
        raise NotImplementedError

    @api
    async def api_async_enrollment_submit(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.async_enrollment_submit.Req,
    ) -> anonymous_cmds.latest.async_enrollment_submit.Rep:
        match req.submit_payload_signature:
            case (
                anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignaturePKI() as signature
            ):
                submit_payload_signature = AsyncEnrollmentPayloadSignaturePKI(
                    signature=signature.signature,
                    author_der_x509_certificate=signature.submitter_der_x509_certificate,
                    algorithm=signature.algorithm,
                    intermediate_der_x509_certificates=signature.intermediate_der_x509_certificates,
                )
            case (
                anonymous_cmds.latest.async_enrollment_submit.SubmitPayloadSignatureOpenBao() as signature
            ):
                submit_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
                    signature=signature.signature,
                    author_openbao_entity_id=signature.submitter_openbao_entity_id,
                )
            case unknown:
                assert False, unknown

        now = DateTime.now()
        outcome = await self.submit(
            now=now,
            organization_id=client_ctx.organization_id,
            enrollment_id=req.enrollment_id,
            force=req.force,
            submit_payload=req.submit_payload,
            submit_payload_signature=submit_payload_signature,
        )
        match outcome:
            case None:
                return anonymous_cmds.latest.async_enrollment_submit.RepOk(submitted_on=now)
            case AsyncEnrollmentEmailAlreadySubmitted() as error:
                return anonymous_cmds.latest.async_enrollment_submit.RepEmailAlreadySubmitted(
                    submitted_on=error.submitted_on
                )
            case AsyncEnrollmentSubmitBadOutcome.EMAIL_ALREADY_ENROLLED:
                return anonymous_cmds.latest.async_enrollment_submit.RepEmailAlreadyEnrolled()
            case AsyncEnrollmentSubmitBadOutcome.ID_ALREADY_USED:
                return anonymous_cmds.latest.async_enrollment_submit.RepIdAlreadyUsed()
            case AsyncEnrollmentSubmitValidateBadOutcome.INVALID_SUBMIT_PAYLOAD:
                return anonymous_cmds.latest.async_enrollment_submit.RepInvalidSubmitPayload()
            case AsyncEnrollmentSubmitValidateBadOutcome.INVALID_SUBMIT_PAYLOAD_SIGNATURE:
                return (
                    anonymous_cmds.latest.async_enrollment_submit.RepInvalidSubmitPayloadSignature()
                )
            case AsyncEnrollmentSubmitValidateBadOutcome.INVALID_DER_X509_CERTIFICATE:
                return anonymous_cmds.latest.async_enrollment_submit.RepInvalidDerX509Certificate()
            case AsyncEnrollmentSubmitValidateBadOutcome.INVALID_X509_TRUSTCHAIN:
                return anonymous_cmds.latest.async_enrollment_submit.RepInvalidX509Trustchain()
            case AsyncEnrollmentSubmitBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case AsyncEnrollmentSubmitBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()

    @api
    async def api_async_enrollment_info(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.async_enrollment_info.Req,
    ) -> anonymous_cmds.latest.async_enrollment_info.Rep:
        outcome = await self.info(
            organization_id=client_ctx.organization_id,
            enrollment_id=req.enrollment_id,
        )
        match outcome:
            case AsyncEnrollmentInfoSubmitted() as info:
                return anonymous_cmds.latest.async_enrollment_info.RepOk(
                    unit=anonymous_cmds.latest.async_enrollment_info.InfoStatusSubmitted(
                        submitted_on=info.submitted_on,
                    )
                )
            case AsyncEnrollmentInfoRejected() as info:
                return anonymous_cmds.latest.async_enrollment_info.RepOk(
                    unit=anonymous_cmds.latest.async_enrollment_info.InfoStatusRejected(
                        submitted_on=info.submitted_on,
                        rejected_on=info.rejected_on,
                    )
                )
            case AsyncEnrollmentInfoCancelled() as info:
                return anonymous_cmds.latest.async_enrollment_info.RepOk(
                    unit=anonymous_cmds.latest.async_enrollment_info.InfoStatusCancelled(
                        submitted_on=info.submitted_on,
                        cancelled_on=info.cancelled_on,
                    )
                )
            case AsyncEnrollmentInfoAccepted() as info:
                match info.accept_payload_signature:
                    case AsyncEnrollmentPayloadSignaturePKI() as signature:
                        accept_payload_signature = anonymous_cmds.latest.async_enrollment_info.AcceptPayloadSignaturePKI(
                            signature=signature.signature,
                            algorithm=signature.algorithm,
                            accepter_der_x509_certificate=signature.author_der_x509_certificate,
                            intermediate_der_x509_certificates=signature.intermediate_der_x509_certificates,
                        )
                    case AsyncEnrollmentPayloadSignatureOpenBao() as signature:
                        accept_payload_signature = anonymous_cmds.latest.async_enrollment_info.AcceptPayloadSignatureOpenBao(
                            signature=signature.signature,
                            accepter_openbao_entity_id=signature.author_openbao_entity_id,
                        )
                return anonymous_cmds.latest.async_enrollment_info.RepOk(
                    unit=anonymous_cmds.latest.async_enrollment_info.InfoStatusAccepted(
                        submitted_on=info.submitted_on,
                        accepted_on=info.accepted_on,
                        accept_payload=info.accept_payload,
                        accept_payload_signature=accept_payload_signature,
                    )
                )
            case AsyncEnrollmentInfoBadOutcome.ENROLLMENT_NOT_FOUND:
                return anonymous_cmds.latest.async_enrollment_info.RepEnrollmentNotFound()
            case AsyncEnrollmentInfoBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case AsyncEnrollmentInfoBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()

    @api
    async def api_async_enrollment_list(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.async_enrollment_list.Req,
    ) -> authenticated_cmds.latest.async_enrollment_list.Rep:
        outcome = await self.list(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
        )
        match outcome:
            case list() as enrollments:
                cooked_enrollments = []
                for enrollment in enrollments:
                    match enrollment.submit_payload_signature:
                        case AsyncEnrollmentPayloadSignaturePKI() as signature:
                            submit_payload_signature = authenticated_cmds.latest.async_enrollment_list.SubmitPayloadSignaturePKI(
                                signature=signature.signature,
                                algorithm=signature.algorithm,
                                submitter_der_x509_certificate=signature.author_der_x509_certificate,
                                intermediate_der_x509_certificates=signature.intermediate_der_x509_certificates,
                            )
                        case AsyncEnrollmentPayloadSignatureOpenBao() as signature:
                            submit_payload_signature = authenticated_cmds.latest.async_enrollment_list.SubmitPayloadSignatureOpenBao(
                                signature=signature.signature,
                                submitter_openbao_entity_id=signature.author_openbao_entity_id,
                            )
                    cooked_enrollments.append(
                        authenticated_cmds.latest.async_enrollment_list.Enrollment(
                            enrollment_id=enrollment.enrollment_id,
                            submitted_on=enrollment.submitted_on,
                            submit_payload=enrollment.submit_payload,
                            submit_payload_signature=submit_payload_signature,
                        )
                    )
                return authenticated_cmds.latest.async_enrollment_list.RepOk(
                    enrollments=cooked_enrollments
                )
            case AsyncEnrollmentListBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.async_enrollment_list.RepAuthorNotAllowed()
            case AsyncEnrollmentListBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case AsyncEnrollmentListBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case AsyncEnrollmentListBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case AsyncEnrollmentListBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_async_enrollment_reject(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.async_enrollment_reject.Req,
    ) -> authenticated_cmds.latest.async_enrollment_reject.Rep:
        outcome = await self.reject(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            enrollment_id=req.enrollment_id,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.async_enrollment_reject.RepOk()
            case AsyncEnrollmentRejectBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.async_enrollment_reject.RepAuthorNotAllowed()
            case AsyncEnrollmentRejectBadOutcome.ENROLLMENT_NOT_FOUND:
                return authenticated_cmds.latest.async_enrollment_reject.RepEnrollmentNotFound()
            case AsyncEnrollmentRejectBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE:
                return authenticated_cmds.latest.async_enrollment_reject.RepEnrollmentNoLongerAvailable()
            case AsyncEnrollmentRejectBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case AsyncEnrollmentRejectBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case AsyncEnrollmentRejectBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case AsyncEnrollmentRejectBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_async_enrollment_accept(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.async_enrollment_accept.Req,
    ) -> authenticated_cmds.latest.async_enrollment_accept.Rep:
        match req.accept_payload_signature:
            case (
                authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignaturePKI() as signature
            ):
                accept_payload_signature = AsyncEnrollmentPayloadSignaturePKI(
                    signature=signature.signature,
                    author_der_x509_certificate=signature.accepter_der_x509_certificate,
                    algorithm=signature.algorithm,
                    intermediate_der_x509_certificates=signature.intermediate_der_x509_certificates,
                )
            case (
                authenticated_cmds.latest.async_enrollment_accept.AcceptPayloadSignatureOpenBao() as signature
            ):
                accept_payload_signature = AsyncEnrollmentPayloadSignatureOpenBao(
                    signature=signature.signature,
                    author_openbao_entity_id=signature.accepter_openbao_entity_id,
                )
            case unknown:
                assert False, unknown

        outcome = await self.accept(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            enrollment_id=req.enrollment_id,
            accept_payload=req.accept_payload,
            accept_payload_signature=accept_payload_signature,
            submitter_user_certificate=req.submitter_user_certificate,
            submitter_redacted_user_certificate=req.submitter_redacted_user_certificate,
            submitter_device_certificate=req.submitter_device_certificate,
            submitter_redacted_device_certificate=req.submitter_redacted_device_certificate,
        )
        match outcome:
            case (_, _):
                return authenticated_cmds.latest.async_enrollment_accept.RepOk()
            case AsyncEnrollmentAcceptBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.async_enrollment_accept.RepAuthorNotAllowed()
            case AsyncEnrollmentAcceptBadOutcome.ENROLLMENT_NOT_FOUND:
                return authenticated_cmds.latest.async_enrollment_accept.RepEnrollmentNotFound()
            case (
                AsyncEnrollmentAcceptValidateBadOutcome.INVALID_CERTIFICATE
                | AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_TIMESTAMP_MISMATCH
                | AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_USER_ID_MISMATCH
                | AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_INVALID_REDACTED
                | AsyncEnrollmentAcceptValidateBadOutcome.CERTIFICATES_REDACTED_MISMATCH
            ):
                return authenticated_cmds.latest.async_enrollment_accept.RepInvalidCertificate()
            case AsyncEnrollmentAcceptValidateBadOutcome.INVALID_ACCEPT_PAYLOAD:
                return authenticated_cmds.latest.async_enrollment_accept.RepInvalidAcceptPayload()
            case AsyncEnrollmentAcceptBadOutcome.SUBMIT_AND_ACCEPT_IDENTITY_SYSTEMS_MISMATCH:
                return authenticated_cmds.latest.async_enrollment_accept.RepSubmitAndAcceptIdentitySystemsMismatch()
            case AsyncEnrollmentAcceptValidateBadOutcome.INVALID_ACCEPT_PAYLOAD_SIGNATURE:
                return authenticated_cmds.latest.async_enrollment_accept.RepInvalidAcceptPayloadSignature()
            case AsyncEnrollmentAcceptValidateBadOutcome.INVALID_DER_X509_CERTIFICATE:
                return (
                    authenticated_cmds.latest.async_enrollment_accept.RepInvalidDerX509Certificate()
                )
            case AsyncEnrollmentAcceptValidateBadOutcome.INVALID_X509_TRUSTCHAIN:
                return authenticated_cmds.latest.async_enrollment_accept.RepInvalidX509Trustchain()
            case AsyncEnrollmentAcceptBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE:
                return authenticated_cmds.latest.async_enrollment_accept.RepEnrollmentNoLongerAvailable()
            case AsyncEnrollmentAcceptBadOutcome.ACTIVE_USERS_LIMIT_REACHED:
                return (
                    authenticated_cmds.latest.async_enrollment_accept.RepActiveUsersLimitReached()
                )
            case AsyncEnrollmentAcceptBadOutcome.USER_ALREADY_EXISTS:
                return authenticated_cmds.latest.async_enrollment_accept.RepUserAlreadyExists()
            case AsyncEnrollmentAcceptBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN:
                return (
                    authenticated_cmds.latest.async_enrollment_accept.RepHumanHandleAlreadyTaken()
                )
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.async_enrollment_accept.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.async_enrollment_accept.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case AsyncEnrollmentAcceptBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case AsyncEnrollmentAcceptBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case AsyncEnrollmentAcceptBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case AsyncEnrollmentAcceptBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
