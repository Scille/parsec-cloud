# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import attr
from typing import List
from uuid import UUID
from parsec._parsec import DateTime

from parsec.api.data import DataError, PkiEnrollmentSubmitPayload, PkiEnrollmentAcceptPayload
from parsec.api.protocol import (
    OrganizationID,
    UserProfile,
    PkiEnrollmentStatus,
    pki_enrollment_submit_serializer,
    pki_enrollment_info_serializer,
    pki_enrollment_list_serializer,
    pki_enrollment_reject_serializer,
    pki_enrollment_accept_serializer,
)
from parsec.backend.user_type import (
    User,
    Device,
    validate_new_user_certificates,
    CertificateValidationError,
)
from parsec.backend.client_context import AuthenticatedClientContext, AnonymousClientContext
from parsec.backend.utils import api, catch_protocol_errors, ClientType
from parsec.event_bus import EventBus


class PkiEnrollmentError(Exception):
    pass


class PkiEnrollmentAlreadyEnrolledError(PkiEnrollmentError):
    def __init__(self, accepted_on, *args, **kwargs):
        self.accepted_on = accepted_on
        PkiEnrollmentError.__init__(self, *args, **kwargs)


class PkiEnrollmentNotFoundError(PkiEnrollmentError):
    pass


class PkiEnrollmentAlreadyExistError(PkiEnrollmentError):
    pass


class PkiEnrollmentActiveUsersLimitReached(PkiEnrollmentError):
    pass


class PkiEnrollmentCertificateAlreadySubmittedError(PkiEnrollmentError):
    def __init__(self, submitted_on, *args, **kwargs):
        self.submitted_on = submitted_on
        PkiEnrollmentError.__init__(self, *args, **kwargs)


class PkiEnrollmentIdAlreadyUsedError(PkiEnrollmentError):
    pass


class PkiEnrollementEmailAlreadyUsedError(PkiEnrollmentError):
    pass


class PkiEnrollmentNoLongerAvailableError(PkiEnrollmentError):
    pass


class PkiEnrollmentRequestNotFoundError(PkiEnrollmentError):
    pass


class PkiEnrollmentReplacedError(PkiEnrollmentError):  # TODO
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentInfo:
    enrollment_id: UUID


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
    enrollment_id: UUID
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    submit_payload: bytes


class BasePkiEnrollmentComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    @api("pki_enrollment_submit", client_types=[ClientType.ANONYMOUS])
    @catch_protocol_errors
    async def api_pki_enrollment_submit(
        self, client_ctx: AnonymousClientContext, msg: dict
    ) -> dict:
        msg = pki_enrollment_submit_serializer.req_load(msg)

        try:
            PkiEnrollmentSubmitPayload.load(msg["submit_payload"])

        except DataError as exc:
            return {"status": "invalid_payload_data", "reason": str(exc)}

        submitted_on = DateTime.now()
        try:
            await self.submit(
                organization_id=client_ctx.organization_id,
                enrollment_id=msg["enrollment_id"],
                force=msg["force"],
                submitter_der_x509_certificate=msg["submitter_der_x509_certificate"],
                submit_payload_signature=msg["submit_payload_signature"],
                submit_payload=msg["submit_payload"],
                submitted_on=submitted_on,
                submitter_der_x509_certificate_email=msg["submitter_der_x509_certificate_email"],
            )
            rep = {"status": "ok", "submitted_on": submitted_on}

        except PkiEnrollmentCertificateAlreadySubmittedError as exc:
            rep = {"status": "already_submitted", "submitted_on": exc.submitted_on}

        except PkiEnrollmentIdAlreadyUsedError:
            rep = {"status": "enrollment_id_already_used"}

        except PkiEnrollementEmailAlreadyUsedError:
            rep = {"status": "email_already_used"}

        except PkiEnrollmentAlreadyEnrolledError:
            rep = {"status": "already_enrolled"}

        return pki_enrollment_submit_serializer.rep_dump(rep)

    @api("pki_enrollment_info", client_types=[ClientType.ANONYMOUS])
    @catch_protocol_errors
    async def api_pki_enrollment_info(self, client_ctx: AnonymousClientContext, msg: dict) -> dict:
        msg = pki_enrollment_info_serializer.req_load(msg)
        try:
            info = await self.info(
                organization_id=client_ctx.organization_id, enrollment_id=msg["enrollment_id"]
            )
            if isinstance(info, PkiEnrollmentInfoSubmitted):
                rep = {
                    "enrollment_status": PkiEnrollmentStatus.SUBMITTED,
                    "submitted_on": info.submitted_on,
                }
            elif isinstance(info, PkiEnrollmentInfoAccepted):
                rep = {
                    "enrollment_status": PkiEnrollmentStatus.ACCEPTED,
                    "submitted_on": info.submitted_on,
                    "accepted_on": info.accepted_on,
                    "accepter_der_x509_certificate": info.accepter_der_x509_certificate,
                    "accept_payload_signature": info.accept_payload_signature,
                    "accept_payload": info.accept_payload,
                }
            elif isinstance(info, PkiEnrollmentInfoRejected):
                rep = {
                    "enrollment_status": PkiEnrollmentStatus.REJECTED,
                    "submitted_on": info.submitted_on,
                    "rejected_on": info.rejected_on,
                }
            elif isinstance(info, PkiEnrollmentInfoCancelled):
                rep = {
                    "enrollment_status": PkiEnrollmentStatus.CANCELLED,
                    "submitted_on": info.submitted_on,
                    "cancelled_on": info.cancelled_on,
                }
            else:
                assert False

        except PkiEnrollmentNotFoundError as exc:
            rep = {"status": "not_found", "reason": str(exc)}

        return pki_enrollment_info_serializer.rep_dump(rep)

    @api("pki_enrollment_list")
    @catch_protocol_errors
    async def api_pki_enrollment_list(
        self, client_ctx: AuthenticatedClientContext, msg: dict
    ) -> dict:
        if client_ctx.profile != UserProfile.ADMIN:
            return {
                "status": "not_allowed",
                "reason": f"User `{client_ctx.device_id.user_id.str}` is not admin",
            }
        msg = pki_enrollment_list_serializer.req_load(msg)
        enrollments = await self.list(organization_id=client_ctx.organization_id)
        return pki_enrollment_list_serializer.rep_dump(
            {
                "enrollments": [
                    {
                        "enrollment_id": e.enrollment_id,
                        "submitted_on": e.submitted_on,
                        "submitter_der_x509_certificate": e.submitter_der_x509_certificate,
                        "submit_payload_signature": e.submit_payload_signature,
                        "submit_payload": e.submit_payload,
                    }
                    for e in enrollments
                ]
            }
        )

    @api("pki_enrollment_reject")
    @catch_protocol_errors
    async def api_pki_enrollment_reject(
        self, client_ctx: AuthenticatedClientContext, msg: dict
    ) -> dict:
        if client_ctx.profile != UserProfile.ADMIN:
            return {
                "status": "not_allowed",
                "reason": f"User `{client_ctx.device_id.user_id.str}` is not admin",
            }
        msg = pki_enrollment_reject_serializer.req_load(msg)
        try:
            await self.reject(
                organization_id=client_ctx.organization_id,
                enrollment_id=msg["enrollment_id"],
                rejected_on=DateTime.now(),
            )
            rep = {"status": "ok"}

        except PkiEnrollmentNotFoundError as exc:
            rep = {"status": "not_found", "reason": str(exc)}

        except PkiEnrollmentNoLongerAvailableError as exc:
            rep = {"status": "no_longer_available", "reason": str(exc)}

        return pki_enrollment_reject_serializer.rep_dump(rep)

    @api("pki_enrollment_accept")
    @catch_protocol_errors
    async def api_pki_enrollment_accept(
        self, client_ctx: AuthenticatedClientContext, msg: dict
    ) -> dict:
        if client_ctx.profile != UserProfile.ADMIN:
            return {
                "status": "not_allowed",
                "reason": f"User `{client_ctx.device_id.user_id.str}` is not admin",
            }
        msg = pki_enrollment_accept_serializer.req_load(msg)

        try:
            PkiEnrollmentAcceptPayload.load(msg["accept_payload"])

        except DataError as exc:
            return {"status": "invalid_payload_data", "reason": str(exc)}

        try:
            user, first_device = validate_new_user_certificates(
                expected_author=client_ctx.device_id,
                author_verify_key=client_ctx.verify_key,
                device_certificate=msg["device_certificate"],
                user_certificate=msg["user_certificate"],
                redacted_user_certificate=msg["redacted_user_certificate"],
                redacted_device_certificate=msg["redacted_device_certificate"],
            )

        except CertificateValidationError as exc:
            return pki_enrollment_accept_serializer.rep_dump(
                {"status": exc.status, "reason": exc.reason}
            )

        try:
            await self.accept(
                organization_id=client_ctx.organization_id,
                enrollment_id=msg["enrollment_id"],
                accepter_der_x509_certificate=msg["accepter_der_x509_certificate"],
                accept_payload_signature=msg["accept_payload_signature"],
                accept_payload=msg["accept_payload"],
                accepted_on=DateTime.now(),
                user=user,
                first_device=first_device,
            )
            rep = {"status": "ok"}

        except PkiEnrollmentNotFoundError as exc:
            rep = {"status": "not_found", "reason": str(exc)}

        except PkiEnrollmentNoLongerAvailableError as exc:
            rep = {"status": "no_longer_available", "reason": str(exc)}

        except PkiEnrollmentAlreadyExistError as exc:
            rep = {"status": "already_exists", "reason": str(exc)}

        except PkiEnrollmentActiveUsersLimitReached:
            rep = {"status": "active_users_limit_reached"}

        return pki_enrollment_accept_serializer.rep_dump(rep)

    async def submit(
        self,
        organization_id: OrganizationID,
        enrollment_id: UUID,
        force: bool,
        submitter_der_x509_certificate: bytes,
        submitter_der_x509_certificate_email: str,
        submit_payload_signature: bytes,
        submit_payload: bytes,
        submitted_on: DateTime,
    ) -> None:
        """
        Raises:
            PkiEnrollmentCertificateAlreadySubmittedError
            PkiEnrollmentAlreadyEnrolledError
            PkiEnrollmentIdAlreadyUsedError
            PkiEnrollementEmailAlreadyUsedError
        """
        raise NotImplementedError()

    async def info(self, organization_id: OrganizationID, enrollment_id: UUID) -> PkiEnrollmentInfo:
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
        self, organization_id: OrganizationID, enrollment_id: UUID, rejected_on: DateTime
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
        enrollment_id: UUID,
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
