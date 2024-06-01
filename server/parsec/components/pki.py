# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import auto

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    EnrollmentID,
    OrganizationID,
    UserCertificate,
    VerifyKey,
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
from parsec.types import BadOutcomeEnum


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
    enrollment_id: EnrollmentID
    submitted_on: DateTime


@dataclass(slots=True)
class PkiEnrollmentInfoAccepted:
    enrollment_id: EnrollmentID
    submitted_on: DateTime
    accepted_on: DateTime
    accepter_der_x509_certificate: bytes
    accept_payload_signature: bytes
    accept_payload: bytes


@dataclass(slots=True)
class PkiEnrollmentInfoRejected:
    enrollment_id: EnrollmentID
    submitted_on: DateTime
    rejected_on: DateTime


@dataclass(slots=True)
class PkiEnrollmentInfoCancelled:
    enrollment_id: EnrollmentID
    submitted_on: DateTime
    cancelled_on: DateTime


@dataclass(slots=True)
class PkiEnrollmentListItem:
    enrollment_id: EnrollmentID
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    submit_payload: bytes


PkiEnrollmentInfo = (
    PkiEnrollmentInfoSubmitted
    | PkiEnrollmentInfoAccepted
    | PkiEnrollmentInfoRejected
    | PkiEnrollmentInfoCancelled
)


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


class PkiEnrollmentInfoBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    ENROLLMENT_NOT_FOUND = auto()


class PkiEnrollmentListBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()


class PkiEnrollmentRejectBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    ENROLLMENT_NOT_FOUND = auto()
    ENROLLMENT_NO_LONGER_AVAILABLE = auto()


class PkiEnrollmentAcceptStoreBadOutcome(BadOutcomeEnum):
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


class BasePkiEnrollmentComponent:
    async def submit(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        enrollment_id: EnrollmentID,
        force: bool,
        submitter_der_x509_certificate: bytes,
        submitter_der_x509_certificate_email: str,
        submit_payload_signature: bytes,
        submit_payload: bytes,
    ) -> None | PkiEnrollmentSubmitBadOutcome | PkiEnrollmentSubmitX509CertificateAlreadySubmitted:
        raise NotImplementedError

    async def info(
        self,
        organization_id: OrganizationID,
        enrollment_id: EnrollmentID,
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
        enrollment_id: EnrollmentID,
    ) -> None | PkiEnrollmentRejectBadOutcome:
        raise NotImplementedError

    async def accept(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        enrollment_id: EnrollmentID,
        accept_payload: bytes,
        accept_payload_signature: bytes,
        accepter_der_x509_certificate: bytes,
        user_certificate: bytes,
        redacted_user_certificate: bytes,
        device_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> (
        tuple[UserCertificate, DeviceCertificate]
        | PkiEnrollmentAcceptValidateBadOutcome
        | PkiEnrollmentAcceptStoreBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

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
                    accept_payload_signature=info.accept_payload_signature,
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

        return anonymous_cmds.latest.pki_enrollment_info.RepOk(unit)

    @api
    async def api_pki_enrollment_submit(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.pki_enrollment_submit.Req,
    ) -> anonymous_cmds.latest.pki_enrollment_submit.Rep:
        now = DateTime.now()
        outcome = await self.submit(
            now=now,
            organization_id=client_ctx.organization_id,
            enrollment_id=req.enrollment_id,
            force=req.force,
            submitter_der_x509_certificate=req.submitter_der_x509_certificate,
            submitter_der_x509_certificate_email=req.submitter_der_x509_certificate_email,
            submit_payload_signature=req.submit_payload_signature,
            submit_payload=req.submit_payload,
        )
        match outcome:
            case None:
                return anonymous_cmds.latest.pki_enrollment_submit.RepOk(submitted_on=now)
            case PkiEnrollmentSubmitBadOutcome.ENROLLMENT_ALREADY_EXISTS:
                return anonymous_cmds.latest.pki_enrollment_submit.RepAlreadyEnrolled()
            case PkiEnrollmentSubmitX509CertificateAlreadySubmitted() as error:
                return (
                    anonymous_cmds.latest.pki_enrollment_submit.RepX509CertificateAlreadySubmitted(
                        submitted_on=error.submitted_on
                    )
                )
            case PkiEnrollmentSubmitBadOutcome.X509_CERTIFICATE_ALREADY_ENROLLED:
                return anonymous_cmds.latest.pki_enrollment_submit.RepAlreadyEnrolled()
            case PkiEnrollmentSubmitBadOutcome.USER_EMAIL_ALREADY_ENROLLED:
                return anonymous_cmds.latest.pki_enrollment_submit.RepEmailAlreadyEnrolled()
            case PkiEnrollmentSubmitBadOutcome.ENROLLMENT_ID_ALREADY_USED:
                return anonymous_cmds.latest.pki_enrollment_submit.RepEnrollmentIdAlreadyUsed()
            case PkiEnrollmentSubmitBadOutcome.INVALID_SUBMIT_PAYLOAD:
                return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidPayloadData()
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
                            e.submit_payload,
                            e.submit_payload_signature,
                            e.submitted_on,
                            e.submitter_der_x509_certificate,
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
        outcome = await self.accept(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            enrollment_id=req.enrollment_id,
            accept_payload=req.accept_payload,
            accept_payload_signature=req.accept_payload_signature,
            accepter_der_x509_certificate=req.accepter_der_x509_certificate,
            device_certificate=req.device_certificate,
            user_certificate=req.user_certificate,
            redacted_device_certificate=req.redacted_device_certificate,
            redacted_user_certificate=req.redacted_user_certificate,
        )
        match outcome:
            case (_, _):
                return authenticated_cmds.latest.pki_enrollment_accept.RepOk()
            case PkiEnrollmentAcceptStoreBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE:
                return (
                    authenticated_cmds.latest.pki_enrollment_accept.RepEnrollmentNoLongerAvailable()
                )
            case PkiEnrollmentAcceptStoreBadOutcome.USER_ALREADY_EXISTS:
                return authenticated_cmds.latest.pki_enrollment_accept.RepUserAlreadyExists()
            case PkiEnrollmentAcceptStoreBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN:
                return authenticated_cmds.latest.pki_enrollment_accept.RepHumanHandleAlreadyTaken()
            case PkiEnrollmentAcceptStoreBadOutcome.ACTIVE_USERS_LIMIT_REACHED:
                return authenticated_cmds.latest.pki_enrollment_accept.RepActiveUsersLimitReached()
            case PkiEnrollmentAcceptStoreBadOutcome.ENROLLMENT_NOT_FOUND:
                return authenticated_cmds.latest.pki_enrollment_accept.RepEnrollmentNotFound()
            case PkiEnrollmentAcceptStoreBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.pki_enrollment_accept.RepAuthorNotAllowed()
            case PkiEnrollmentAcceptStoreBadOutcome.INVALID_ACCEPT_PAYLOAD:
                return authenticated_cmds.latest.pki_enrollment_accept.RepInvalidPayloadData()
            case PkiEnrollmentAcceptValidateBadOutcome():
                return authenticated_cmds.latest.pki_enrollment_accept.RepInvalidCertificate()
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
            case PkiEnrollmentAcceptStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case PkiEnrollmentAcceptStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case PkiEnrollmentAcceptStoreBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case PkiEnrollmentAcceptStoreBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
