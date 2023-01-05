# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any, List, cast

import attr

from parsec._parsec import (
    ClientType,
    DateTime,
    EnrollmentID,
    PkiEnrollmentAcceptRep,
    PkiEnrollmentAcceptRepActiveUsersLimitReached,
    PkiEnrollmentAcceptRepAlreadyExists,
    PkiEnrollmentAcceptRepInvalidCertification,
    PkiEnrollmentAcceptRepInvalidPayloadData,
    PkiEnrollmentAcceptRepNoLongerAvailable,
    PkiEnrollmentAcceptRepNotAllowed,
    PkiEnrollmentAcceptRepNotFound,
    PkiEnrollmentAcceptRepOk,
    PkiEnrollmentAcceptReq,
    PkiEnrollmentInfoRep,
    PkiEnrollmentInfoRepNotFound,
    PkiEnrollmentInfoRepOk,
    PkiEnrollmentInfoReq,
    PkiEnrollmentInfoStatus,
    PkiEnrollmentListRep,
    PkiEnrollmentListRepNotAllowed,
    PkiEnrollmentListRepOk,
    PkiEnrollmentListReq,
    PkiEnrollmentRejectRep,
    PkiEnrollmentRejectRepNoLongerAvailable,
    PkiEnrollmentRejectRepNotAllowed,
    PkiEnrollmentRejectRepNotFound,
    PkiEnrollmentRejectRepOk,
    PkiEnrollmentRejectReq,
    PkiEnrollmentSubmitRep,
    PkiEnrollmentSubmitRepAlreadyEnrolled,
    PkiEnrollmentSubmitRepAlreadySubmitted,
    PkiEnrollmentSubmitRepEmailAlreadyUsed,
    PkiEnrollmentSubmitRepIdAlreadyUsed,
    PkiEnrollmentSubmitRepInvalidPayloadData,
    PkiEnrollmentSubmitRepOk,
    PkiEnrollmentSubmitReq,
)
from parsec._parsec import PkiEnrollmentListItem as RustPkiEnrollmentListItem
from parsec.api.data import DataError, PkiEnrollmentAnswerPayload, PkiEnrollmentSubmitPayload
from parsec.api.protocol import OrganizationID, UserProfile
from parsec.backend.client_context import AnonymousClientContext, AuthenticatedClientContext
from parsec.backend.user_type import (
    CertificateValidationError,
    Device,
    User,
    validate_new_user_certificates,
)
from parsec.backend.utils import api, api_typed_msg_adapter, catch_protocol_errors
from parsec.event_bus import EventBus


class PkiEnrollmentError(Exception):
    pass


class PkiEnrollmentAlreadyEnrolledError(PkiEnrollmentError):
    def __init__(self, accepted_on: DateTime, *args: Any, **kwargs: Any):
        self.accepted_on = accepted_on
        PkiEnrollmentError.__init__(self, *args, **kwargs)


class PkiEnrollmentNotFoundError(PkiEnrollmentError):
    pass


class PkiEnrollmentAlreadyExistError(PkiEnrollmentError):
    pass


class PkiEnrollmentActiveUsersLimitReached(PkiEnrollmentError):
    pass


class PkiEnrollmentCertificateAlreadySubmittedError(PkiEnrollmentError):
    def __init__(self, submitted_on: DateTime, *args: Any, **kwargs: Any):
        self.submitted_on = submitted_on
        PkiEnrollmentError.__init__(self, *args, **kwargs)


class PkiEnrollmentIdAlreadyUsedError(PkiEnrollmentError):
    pass


class PkiEnrollmentEmailAlreadyUsedError(PkiEnrollmentError):
    pass


class PkiEnrollmentNoLongerAvailableError(PkiEnrollmentError):
    pass


class PkiEnrollmentRequestNotFoundError(PkiEnrollmentError):
    pass


class PkiEnrollmentReplacedError(PkiEnrollmentError):  # TODO
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentInfo:
    enrollment_id: EnrollmentID


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentInfoSubmitted(PkiEnrollmentInfo):
    submitted_on: DateTime


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentInfoAccepted(PkiEnrollmentInfo):
    submitted_on: DateTime
    accepted_on: DateTime
    accepter_der_x509_certificate: bytes
    accept_payload_signature: bytes
    accept_payload: bytes


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentInfoRejected(PkiEnrollmentInfo):
    submitted_on: DateTime
    rejected_on: DateTime


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentInfoCancelled(PkiEnrollmentInfo):
    submitted_on: DateTime
    cancelled_on: DateTime


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentListItem:
    enrollment_id: EnrollmentID
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    submit_payload: bytes


class BasePkiEnrollmentComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    @api("pki_enrollment_submit", client_types=[ClientType.ANONYMOUS])
    @catch_protocol_errors
    @api_typed_msg_adapter(PkiEnrollmentSubmitReq, PkiEnrollmentSubmitRep)
    async def api_pki_enrollment_submit(
        self, client_ctx: AnonymousClientContext, msg: PkiEnrollmentSubmitReq
    ) -> PkiEnrollmentSubmitRep:
        try:
            PkiEnrollmentSubmitPayload.load(msg.submit_payload)

        except DataError as exc:
            return PkiEnrollmentSubmitRepInvalidPayloadData(str(exc))

        submitted_on = DateTime.now()
        try:
            await self.submit(
                organization_id=client_ctx.organization_id,
                enrollment_id=cast(EnrollmentID, msg.enrollment_id),
                force=msg.force,
                submitter_der_x509_certificate=msg.submitter_der_x509_certificate,
                submit_payload_signature=msg.submit_payload_signature,
                submit_payload=msg.submit_payload,
                submitted_on=submitted_on,
                submitter_der_x509_certificate_email=msg.submitter_der_x509_certificate_email,
            )
            return PkiEnrollmentSubmitRepOk(submitted_on)

        except PkiEnrollmentCertificateAlreadySubmittedError as exc:
            return PkiEnrollmentSubmitRepAlreadySubmitted(exc.submitted_on)

        except PkiEnrollmentIdAlreadyUsedError:
            return PkiEnrollmentSubmitRepIdAlreadyUsed()

        except PkiEnrollmentEmailAlreadyUsedError:
            return PkiEnrollmentSubmitRepEmailAlreadyUsed()

        except PkiEnrollmentAlreadyEnrolledError:
            return PkiEnrollmentSubmitRepAlreadyEnrolled()

    @api("pki_enrollment_info", client_types=[ClientType.ANONYMOUS])
    @catch_protocol_errors
    @api_typed_msg_adapter(PkiEnrollmentInfoReq, PkiEnrollmentInfoRep)
    async def api_pki_enrollment_info(
        self, client_ctx: AnonymousClientContext, msg: PkiEnrollmentInfoReq
    ) -> PkiEnrollmentInfoRep:
        try:
            info = await self.info(
                organization_id=client_ctx.organization_id,
                enrollment_id=cast(EnrollmentID, msg.enrollment_id),
            )
            rep: PkiEnrollmentInfoStatus
            if isinstance(info, PkiEnrollmentInfoSubmitted):
                rep = PkiEnrollmentInfoStatus.Submitted(info.submitted_on)
            elif isinstance(info, PkiEnrollmentInfoAccepted):
                rep = PkiEnrollmentInfoStatus.Accepted(
                    info.submitted_on,
                    info.accepted_on,
                    info.accepter_der_x509_certificate,
                    info.accept_payload_signature,
                    info.accept_payload,
                )
            elif isinstance(info, PkiEnrollmentInfoRejected):
                rep = PkiEnrollmentInfoStatus.Rejected(info.submitted_on, info.rejected_on)
            elif isinstance(info, PkiEnrollmentInfoCancelled):
                rep = PkiEnrollmentInfoStatus.Cancelled(info.submitted_on, info.cancelled_on)
            else:
                assert False

        except PkiEnrollmentNotFoundError as exc:
            return PkiEnrollmentInfoRepNotFound(str(exc))

        return PkiEnrollmentInfoRepOk(rep)

    @api("pki_enrollment_list")
    @catch_protocol_errors
    @api_typed_msg_adapter(PkiEnrollmentListReq, PkiEnrollmentListRep)
    async def api_pki_enrollment_list(
        self, client_ctx: AuthenticatedClientContext, _: PkiEnrollmentListReq
    ) -> PkiEnrollmentListRep:
        if client_ctx.profile != UserProfile.ADMIN:
            return PkiEnrollmentListRepNotAllowed(
                f"User `{client_ctx.device_id.user_id.str}` is not admin"
            )
        enrollments = await self.list(organization_id=client_ctx.organization_id)
        return PkiEnrollmentListRepOk(
            [
                RustPkiEnrollmentListItem(
                    e.enrollment_id,
                    e.submit_payload,
                    e.submit_payload_signature,
                    e.submitted_on,
                    e.submitter_der_x509_certificate,
                )
                for e in enrollments
            ]
        )

    @api("pki_enrollment_reject")
    @catch_protocol_errors
    @api_typed_msg_adapter(PkiEnrollmentRejectReq, PkiEnrollmentRejectRep)
    async def api_pki_enrollment_reject(
        self, client_ctx: AuthenticatedClientContext, msg: PkiEnrollmentRejectReq
    ) -> PkiEnrollmentRejectRep:
        if client_ctx.profile != UserProfile.ADMIN:
            return PkiEnrollmentRejectRepNotAllowed(
                f"User `{client_ctx.device_id.user_id.str}` is not admin"
            )
        try:
            await self.reject(
                organization_id=client_ctx.organization_id,
                enrollment_id=msg.enrollment_id,
                rejected_on=DateTime.now(),
            )
            return PkiEnrollmentRejectRepOk()

        except PkiEnrollmentNotFoundError as exc:
            return PkiEnrollmentRejectRepNotFound(str(exc))

        except PkiEnrollmentNoLongerAvailableError as exc:
            return PkiEnrollmentRejectRepNoLongerAvailable(str(exc))

    @api("pki_enrollment_accept")
    @catch_protocol_errors
    @api_typed_msg_adapter(PkiEnrollmentAcceptReq, PkiEnrollmentAcceptRep)
    async def api_pki_enrollment_accept(
        self, client_ctx: AuthenticatedClientContext, msg: PkiEnrollmentAcceptReq
    ) -> PkiEnrollmentAcceptRep:
        if client_ctx.profile != UserProfile.ADMIN:
            return PkiEnrollmentAcceptRepNotAllowed(
                f"User `{client_ctx.device_id.user_id.str}` is not admin",
            )

        try:
            PkiEnrollmentAnswerPayload.load(msg.accept_payload)

        except DataError as exc:
            return PkiEnrollmentAcceptRepInvalidPayloadData(str(exc))

        try:
            user, first_device = validate_new_user_certificates(
                expected_author=client_ctx.device_id,
                author_verify_key=client_ctx.verify_key,
                device_certificate=msg.device_certificate,
                user_certificate=msg.user_certificate,
                redacted_user_certificate=msg.redacted_user_certificate,
                redacted_device_certificate=msg.redacted_device_certificate,
            )

        except CertificateValidationError as exc:
            return PkiEnrollmentAcceptRepInvalidCertification(exc.reason)

        try:
            await self.accept(
                organization_id=client_ctx.organization_id,
                enrollment_id=msg.enrollment_id,
                accepter_der_x509_certificate=msg.accepter_der_x509_certificate,
                accept_payload_signature=msg.accept_payload_signature,
                accept_payload=msg.accept_payload,
                accepted_on=DateTime.now(),
                user=user,
                first_device=first_device,
            )
            return PkiEnrollmentAcceptRepOk()

        except PkiEnrollmentNotFoundError as exc:
            return PkiEnrollmentAcceptRepNotFound(str(exc))

        except PkiEnrollmentNoLongerAvailableError as exc:
            return PkiEnrollmentAcceptRepNoLongerAvailable(str(exc))

        except PkiEnrollmentAlreadyExistError as exc:
            return PkiEnrollmentAcceptRepAlreadyExists(str(exc))

        except PkiEnrollmentActiveUsersLimitReached:
            return PkiEnrollmentAcceptRepActiveUsersLimitReached(None)

    async def submit(
        self,
        organization_id: OrganizationID,
        enrollment_id: EnrollmentID,
        force: bool,
        submitter_der_x509_certificate: bytes,
        submitter_der_x509_certificate_email: str | None,
        submit_payload_signature: bytes,
        submit_payload: bytes,
        submitted_on: DateTime,
    ) -> None:
        """
        Raises:
            PkiEnrollmentCertificateAlreadySubmittedError
            PkiEnrollmentAlreadyEnrolledError
            PkiEnrollmentIdAlreadyUsedError
            PkiEnrollmentEmailAlreadyUsedError
        """
        raise NotImplementedError()

    async def info(
        self, organization_id: OrganizationID, enrollment_id: EnrollmentID
    ) -> PkiEnrollmentInfo:
        """
        Raises:
            PkiEnrollmentNotFoundError
        """
        raise NotImplementedError()

    async def list(self, organization_id: OrganizationID) -> List[PkiEnrollmentListItem]:
        """
        Raises: Nothing !
        """
        raise NotImplementedError()

    async def reject(
        self,
        organization_id: OrganizationID,
        enrollment_id: EnrollmentID,
        rejected_on: DateTime,
    ) -> None:
        """
        Raises:
            PkiEnrollmentNotFoundError
            PkiEnrollmentNoLongerAvailableError
        """
        raise NotImplementedError()

    async def accept(
        self,
        organization_id: OrganizationID,
        enrollment_id: EnrollmentID,
        accepter_der_x509_certificate: bytes,
        accept_payload_signature: bytes,
        accept_payload: bytes,
        accepted_on: DateTime,
        user: User,
        first_device: Device,
    ) -> None:
        """
        Raises:
            PkiEnrollmentNotFoundError
            PkiEnrollmentNoLongerAvailableError
            PkiEnrollmentAlreadyExistError
            PkiEnrollmentActiveUsersLimitReached
        """
        raise NotImplementedError()
