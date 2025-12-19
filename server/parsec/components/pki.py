# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections.abc import Iterable
from dataclasses import dataclass
from enum import auto
from hashlib import sha256

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    HumanHandle,
    OrganizationID,
    PKIEnrollmentID,
    PkiSignatureAlgorithm,
    UserCertificate,
    VerifyKey,
    X509Certificate,
    X509CertificateInformation,
    anonymous_cmds,
    authenticated_cmds,
)
from parsec.api import api
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.client_context import AnonymousClientContext, AuthenticatedClientContext
from parsec.config import BackendConfig
from parsec.logging import get_logger
from parsec.types import BadOutcomeEnum

logger = get_logger()


class PkiEnrollmentAcceptValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()
    TIMESTAMP_MISMATCH = auto()
    USER_ID_MISMATCH = auto()
    INVALID_REDACTED = auto()
    REDACTED_MISMATCH = auto()


def pki_enrollment_accept_validate(
    now: DateTime,
    expected_author: DeviceID,
    author_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> (
    tuple[UserCertificate, DeviceCertificate]
    | TimestampOutOfBallpark
    | PkiEnrollmentAcceptValidateBadOutcome
):
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
        return PkiEnrollmentAcceptValidateBadOutcome.INVALID_CERTIFICATE

    if u_data.timestamp != d_data.timestamp:
        return PkiEnrollmentAcceptValidateBadOutcome.TIMESTAMP_MISMATCH

    match timestamps_in_the_ballpark(u_data.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    if u_data.user_id != d_data.user_id:
        return PkiEnrollmentAcceptValidateBadOutcome.USER_ID_MISMATCH

    if not ru_data.is_redacted:
        return PkiEnrollmentAcceptValidateBadOutcome.INVALID_REDACTED

    if not rd_data.is_redacted:
        return PkiEnrollmentAcceptValidateBadOutcome.INVALID_REDACTED

    if not u_data.redacted_compare(ru_data):
        return PkiEnrollmentAcceptValidateBadOutcome.REDACTED_MISMATCH

    if not d_data.redacted_compare(rd_data):
        return PkiEnrollmentAcceptValidateBadOutcome.REDACTED_MISMATCH

    return u_data, d_data


@dataclass(slots=True)
class PkiEnrollmentInfoSubmitted:
    enrollment_id: PKIEnrollmentID
    submitted_on: DateTime


@dataclass(slots=True)
class PkiEnrollmentInfoAccepted:
    enrollment_id: PKIEnrollmentID
    submitted_on: DateTime
    accepted_on: DateTime
    accepter_der_x509_certificate: bytes
    accepter_intermediate_der_x509_certificates: list[bytes]
    accept_payload_signature: bytes
    accept_payload_signature_algorithm: PkiSignatureAlgorithm
    accept_payload: bytes


@dataclass(slots=True)
class PkiEnrollmentInfoRejected:
    enrollment_id: PKIEnrollmentID
    submitted_on: DateTime
    rejected_on: DateTime


@dataclass(slots=True)
class PkiEnrollmentInfoCancelled:
    enrollment_id: PKIEnrollmentID
    submitted_on: DateTime
    cancelled_on: DateTime


@dataclass(slots=True)
class PkiEnrollmentListItem:
    enrollment_id: PKIEnrollmentID
    submitted_on: DateTime
    der_x509_certificate: bytes
    intermediate_der_x509_certificates: list[bytes]
    payload_signature: bytes
    payload_signature_algorithm: PkiSignatureAlgorithm
    payload: bytes


PkiEnrollmentInfo = (
    PkiEnrollmentInfoSubmitted
    | PkiEnrollmentInfoAccepted
    | PkiEnrollmentInfoRejected
    | PkiEnrollmentInfoCancelled
)


@dataclass(slots=True)
class PkiCertificate:
    fingerprint_sha256: bytes
    # fingerprint of the cert of which subject is current cert issuer
    # cannot be computed from current cert only
    signed_by: bytes | None
    content: bytes
    subject: bytes
    issuer: bytes

    def __repr__(self) -> str:
        fingerprint = self.fingerprint_sha256.hex()
        signed_by = bytes.hex(self.signed_by) if self.signed_by else None
        return f"{self.__class__.__qualname__}({fingerprint=}, {signed_by=})"


def parse_pki_cert(cert: bytes, signed_by: bytes | None = None) -> PkiCertificate:
    parsed = X509Certificate.from_der(cert)
    fingerprint = get_sha256_fingerprint_from_cert(cert)
    return PkiCertificate(
        fingerprint_sha256=fingerprint,
        signed_by=signed_by,
        content=cert,
        subject=parsed.subject(),
        issuer=parsed.issuer(),
    )


def get_sha256_fingerprint_from_cert(cert: bytes) -> bytes:
    return sha256(cert).digest()


class PkiTrustchainError(BadOutcomeEnum):
    TrustchainTooLong = auto()
    ParseError = auto()


# TODO choose real value (and maybe move that to a better place)
# should this be a configurable value ?
# look up what's an expected value
MAX_INTERMEDIATE_CERTIFICATES_DEPTH = 100


@dataclass(slots=True)
class PkiEnrollmentSubmitX509CertificateAlreadySubmitted:
    submitted_on: DateTime


class PkiEnrollmentSubmitBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    ENROLLMENT_ALREADY_EXISTS = auto()
    ENROLLMENT_ID_ALREADY_USED = auto()
    X509_CERTIFICATE_ALREADY_ENROLLED = auto()
    USER_EMAIL_ALREADY_ENROLLED = auto()
    INVALID_SUBMIT_PAYLOAD = auto()
    INVALID_X509_TRUSTCHAIN = auto()
    INVALID_DER_X509_CERTIFICATE = auto()
    INVALID_PAYLOAD_SIGNATURE = auto()


class PkiEnrollmentInfoBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    ENROLLMENT_NOT_FOUND = auto()
    CERTIFICATE_NOT_FOUND = auto()


class PkiEnrollmentListBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    CERTIFICATE_NOT_FOUND = auto()


class PkiEnrollmentRejectBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    ENROLLMENT_NOT_FOUND = auto()
    ENROLLMENT_NO_LONGER_AVAILABLE = auto()


class PkiEnrollmentAcceptBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    ENROLLMENT_NOT_FOUND = auto()
    ENROLLMENT_NO_LONGER_AVAILABLE = auto()
    USER_ALREADY_EXISTS = auto()
    HUMAN_HANDLE_ALREADY_TAKEN = auto()
    ACTIVE_USERS_LIMIT_REACHED = auto()
    INVALID_ACCEPT_PAYLOAD = auto()
    INVALID_PAYLOAD_SIGNATURE = auto()
    INVALID_X509_TRUSTCHAIN = auto()
    INVALID_DER_X509_CERTIFICATE = auto()


class BasePkiEnrollmentComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    async def submit(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        enrollment_id: PKIEnrollmentID,
        force: bool,
        submitter_human_handle: HumanHandle,
        submitter_trustchain: list[PkiCertificate],
        submit_payload_signature: bytes,
        submit_payload_signature_algorithm: PkiSignatureAlgorithm,
        submit_payload: bytes,
    ) -> None | PkiEnrollmentSubmitBadOutcome | PkiEnrollmentSubmitX509CertificateAlreadySubmitted:
        raise NotImplementedError

    async def info(
        self,
        organization_id: OrganizationID,
        enrollment_id: PKIEnrollmentID,
    ) -> PkiEnrollmentInfo | PkiEnrollmentInfoBadOutcome:
        raise NotImplementedError

    async def list(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> list[PkiEnrollmentListItem] | PkiEnrollmentListBadOutcome:
        raise NotImplementedError

    async def reject(
        self,
        now: DateTime,
        author: DeviceID,
        organization_id: OrganizationID,
        enrollment_id: PKIEnrollmentID,
    ) -> None | PkiEnrollmentRejectBadOutcome:
        raise NotImplementedError

    async def accept(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        enrollment_id: PKIEnrollmentID,
        payload: bytes,
        payload_signature: bytes,
        payload_signature_algorithm: PkiSignatureAlgorithm,
        accepter_trustchain: list[PkiCertificate],
        submitter_user_certificate: bytes,
        submitter_redacted_user_certificate: bytes,
        submitter_device_certificate: bytes,
        submitter_redacted_device_certificate: bytes,
    ) -> (
        tuple[UserCertificate, DeviceCertificate]
        | PkiEnrollmentAcceptValidateBadOutcome
        | PkiEnrollmentAcceptBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def build_trustchain(
        self,
        leaf: bytes,
        intermediate_certificates: Iterable[bytes],
    ) -> list[PkiCertificate] | PkiTrustchainError:
        # TODO shortcut if the rest of the trustchain is already in DB

        # Parse leaf cert
        try:
            current = parse_pki_cert(leaf)
        except ValueError:
            return PkiTrustchainError.ParseError
        # Parse intermediate -> HashMap (subject, cert)
        try:
            intermediate = {
                cert.subject: cert for cert in map(parse_pki_cert, intermediate_certificates)
            }
        except ValueError:
            return PkiTrustchainError.ParseError

        trustchain = {current.subject: current}

        for _ in range(MAX_INTERMEDIATE_CERTIFICATES_DEPTH):
            # check circular trustchain (allows to stop iteration)
            if current.issuer in trustchain.keys():
                return list(trustchain.values())

            # check if there is a cert where cert.subject == current.issuer
            match intermediate.get(current.issuer):
                case None:
                    # TODO check in database
                    # Otherwise we're at the end of what we can check
                    return list(trustchain.values())

                case cert:
                    current.signed_by = cert.fingerprint_sha256
                    trustchain[cert.subject] = cert
                    current = cert

        return PkiTrustchainError.TrustchainTooLong

    @api
    async def api_pki_enrollment_info(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.pki_enrollment_info.Req,
    ) -> anonymous_cmds.latest.pki_enrollment_info.Rep:
        outcome = await self.info(
            organization_id=client_ctx.organization_id,
            enrollment_id=req.enrollment_id,
        )
        match outcome:
            case PkiEnrollmentInfoSubmitted() as info:
                unit = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusSubmitted(
                    submitted_on=info.submitted_on,
                )
            case PkiEnrollmentInfoAccepted() as info:
                unit = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusAccepted(
                    submitted_on=info.submitted_on,
                    accepted_on=info.accepted_on,
                    accepter_der_x509_certificate=info.accepter_der_x509_certificate,
                    accepter_intermediate_der_x509_certificates=info.accepter_intermediate_der_x509_certificates,
                    accept_payload_signature=info.accept_payload_signature,
                    accept_payload_signature_algorithm=info.accept_payload_signature_algorithm,
                    accept_payload=info.accept_payload,
                )
            case PkiEnrollmentInfoRejected() as info:
                unit = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusRejected(
                    submitted_on=info.submitted_on,
                    rejected_on=info.rejected_on,
                )
            case PkiEnrollmentInfoCancelled() as info:
                unit = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusCancelled(
                    submitted_on=info.submitted_on,
                    cancelled_on=info.cancelled_on,
                )
            case PkiEnrollmentInfoBadOutcome.ENROLLMENT_NOT_FOUND:
                return anonymous_cmds.latest.pki_enrollment_info.RepEnrollmentNotFound()
            case PkiEnrollmentInfoBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case PkiEnrollmentInfoBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case PkiEnrollmentInfoBadOutcome.CERTIFICATE_NOT_FOUND:
                return anonymous_cmds.latest.pki_enrollment_info.RepUnknownStatus(
                    "cert not found", None
                )

        return anonymous_cmds.latest.pki_enrollment_info.RepOk(unit)

    @api
    async def api_pki_enrollment_submit(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.pki_enrollment_submit.Req,
    ) -> anonymous_cmds.latest.pki_enrollment_submit.Rep:
        now = DateTime.now()

        try:
            cert_info = X509CertificateInformation.load_der(req.der_x509_certificate)
            human_handle = cert_info.human_handle()
        except ValueError as e:
            logger.info("Invalid certificate", exc_info=e)
            return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidDerX509Certificate()

        match await self.build_trustchain(
            req.der_x509_certificate, req.intermediate_der_x509_certificates
        ):
            case list() as trustchain:
                pass
            case PkiTrustchainError.ParseError:
                return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidDerX509Certificate()
            case PkiTrustchainError.TrustchainTooLong:
                return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidX509Trustchain()

        outcome = await self.submit(
            now=now,
            organization_id=client_ctx.organization_id,
            enrollment_id=req.enrollment_id,
            force=req.force,
            submitter_human_handle=human_handle,
            submitter_trustchain=trustchain,
            submit_payload_signature=req.payload_signature,
            submit_payload_signature_algorithm=req.payload_signature_algorithm,
            submit_payload=req.payload,
        )
        match outcome:
            case None:
                return anonymous_cmds.latest.pki_enrollment_submit.RepOk(submitted_on=now)
            case PkiEnrollmentSubmitBadOutcome.ENROLLMENT_ALREADY_EXISTS:
                return anonymous_cmds.latest.pki_enrollment_submit.RepAlreadyEnrolled()
            case PkiEnrollmentSubmitX509CertificateAlreadySubmitted() as error:
                return anonymous_cmds.latest.pki_enrollment_submit.RepAlreadySubmitted(
                    submitted_on=error.submitted_on
                )
            case PkiEnrollmentSubmitBadOutcome.X509_CERTIFICATE_ALREADY_ENROLLED:
                return anonymous_cmds.latest.pki_enrollment_submit.RepAlreadyEnrolled()
            case PkiEnrollmentSubmitBadOutcome.USER_EMAIL_ALREADY_ENROLLED:
                return anonymous_cmds.latest.pki_enrollment_submit.RepEmailAlreadyUsed()
            case PkiEnrollmentSubmitBadOutcome.ENROLLMENT_ID_ALREADY_USED:
                return anonymous_cmds.latest.pki_enrollment_submit.RepIdAlreadyUsed()
            case PkiEnrollmentSubmitBadOutcome.INVALID_SUBMIT_PAYLOAD:
                return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidPayload()
            case PkiEnrollmentSubmitBadOutcome.INVALID_PAYLOAD_SIGNATURE:
                return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidPayloadSignature()
            case PkiEnrollmentSubmitBadOutcome.INVALID_X509_TRUSTCHAIN:
                return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidX509Trustchain()
            case PkiEnrollmentSubmitBadOutcome.INVALID_DER_X509_CERTIFICATE:
                return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidDerX509Certificate()
            case PkiEnrollmentSubmitBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case PkiEnrollmentSubmitBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()

    @api
    async def api_pki_enrollment_list(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.pki_enrollment_list.Req,
    ) -> authenticated_cmds.latest.pki_enrollment_list.Rep:
        outcome = await self.list(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
        )
        match outcome:
            case list() as enrollments:
                return authenticated_cmds.latest.pki_enrollment_list.RepOk(
                    enrollments=[
                        authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem(
                            e.enrollment_id,
                            e.submitted_on,
                            e.der_x509_certificate,
                            e.intermediate_der_x509_certificates,
                            e.payload_signature,
                            e.payload_signature_algorithm,
                            e.payload,
                        )
                        for e in enrollments
                    ]
                )
            case PkiEnrollmentListBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.pki_enrollment_list.RepAuthorNotAllowed()
            case PkiEnrollmentListBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case PkiEnrollmentListBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case PkiEnrollmentListBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case PkiEnrollmentListBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            case PkiEnrollmentListBadOutcome.CERTIFICATE_NOT_FOUND:
                # TODO add not found error to protocol ?
                return authenticated_cmds.latest.pki_enrollment_list.RepUnknownStatus(
                    "leaf not found", None
                )

    @api
    async def api_pki_enrollment_reject(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.pki_enrollment_reject.Req,
    ) -> authenticated_cmds.latest.pki_enrollment_reject.Rep:
        outcome = await self.reject(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            enrollment_id=req.enrollment_id,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.pki_enrollment_reject.RepOk()
            case PkiEnrollmentRejectBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE:
                return (
                    authenticated_cmds.latest.pki_enrollment_reject.RepEnrollmentNoLongerAvailable()
                )
            case PkiEnrollmentRejectBadOutcome.ENROLLMENT_NOT_FOUND:
                return authenticated_cmds.latest.pki_enrollment_reject.RepEnrollmentNotFound()
            case PkiEnrollmentRejectBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.pki_enrollment_reject.RepAuthorNotAllowed()
            case PkiEnrollmentRejectBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case PkiEnrollmentRejectBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case PkiEnrollmentRejectBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case PkiEnrollmentRejectBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_pki_enrollment_accept(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.pki_enrollment_accept.Req,
    ) -> authenticated_cmds.latest.pki_enrollment_accept.Rep:
        trustchain = await self.build_trustchain(
            req.accepter_der_x509_certificate,
            req.accepter_intermediate_der_x509_certificates,
        )
        match trustchain:
            case PkiTrustchainError.TrustchainTooLong:
                return authenticated_cmds.latest.pki_enrollment_accept.RepInvalidX509Trustchain()

            case PkiTrustchainError.ParseError:
                return (
                    authenticated_cmds.latest.pki_enrollment_accept.RepInvalidDerX509Certificate()
                )
            case trustchain:
                pass

        outcome = await self.accept(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            enrollment_id=req.enrollment_id,
            payload=req.payload,
            payload_signature=req.payload_signature,
            payload_signature_algorithm=req.payload_signature_algorithm,
            accepter_trustchain=trustchain,
            submitter_device_certificate=req.submitter_device_certificate,
            submitter_user_certificate=req.submitter_user_certificate,
            submitter_redacted_device_certificate=req.submitter_redacted_device_certificate,
            submitter_redacted_user_certificate=req.submitter_redacted_user_certificate,
        )
        match outcome:
            case (_, _):
                return authenticated_cmds.latest.pki_enrollment_accept.RepOk()
            case PkiEnrollmentAcceptBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE:
                return (
                    authenticated_cmds.latest.pki_enrollment_accept.RepEnrollmentNoLongerAvailable()
                )
            case PkiEnrollmentAcceptBadOutcome.USER_ALREADY_EXISTS:
                return authenticated_cmds.latest.pki_enrollment_accept.RepUserAlreadyExists()
            case PkiEnrollmentAcceptBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN:
                return authenticated_cmds.latest.pki_enrollment_accept.RepHumanHandleAlreadyTaken()
            case PkiEnrollmentAcceptBadOutcome.ACTIVE_USERS_LIMIT_REACHED:
                return authenticated_cmds.latest.pki_enrollment_accept.RepActiveUsersLimitReached()
            case PkiEnrollmentAcceptBadOutcome.ENROLLMENT_NOT_FOUND:
                return authenticated_cmds.latest.pki_enrollment_accept.RepEnrollmentNotFound()
            case PkiEnrollmentAcceptBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.pki_enrollment_accept.RepAuthorNotAllowed()
            case PkiEnrollmentAcceptBadOutcome.INVALID_ACCEPT_PAYLOAD:
                return authenticated_cmds.latest.pki_enrollment_accept.RepInvalidPayload()
            case PkiEnrollmentAcceptBadOutcome.INVALID_X509_TRUSTCHAIN:
                return authenticated_cmds.latest.pki_enrollment_accept.RepInvalidX509Trustchain()
            case PkiEnrollmentAcceptBadOutcome.INVALID_DER_X509_CERTIFICATE:
                return (
                    authenticated_cmds.latest.pki_enrollment_accept.RepInvalidDerX509Certificate()
                )
            case PkiEnrollmentAcceptBadOutcome.INVALID_PAYLOAD_SIGNATURE:
                return authenticated_cmds.latest.pki_enrollment_accept.RepInvalidPayloadSignature()

            # TODO: https://github.com/Scille/parsec-cloud/issues/11648
            case PkiEnrollmentAcceptValidateBadOutcome():
                raise NotImplementedError()
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.pki_enrollment_accept.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.pki_enrollment_accept.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case PkiEnrollmentAcceptBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case PkiEnrollmentAcceptBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case PkiEnrollmentAcceptBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case PkiEnrollmentAcceptBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
