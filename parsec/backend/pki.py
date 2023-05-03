# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any, List, cast

import attr

from parsec._parsec import (
    DateTime,
    EnrollmentID,
    OrganizationID,
    UserProfile,
    anonymous_cmds,
    authenticated_cmds,
)
from parsec.api.data import DataError, PkiEnrollmentAnswerPayload, PkiEnrollmentSubmitPayload
from parsec.backend.client_context import AnonymousClientContext, AuthenticatedClientContext
from parsec.backend.user_type import (
    CertificateValidationError,
    Device,
    User,
    validate_new_user_certificates,
)
from parsec.backend.utils import api
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

    @api
    async def apiv2_pki_enrollment_submit(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.v2.pki_enrollment_submit.Req,
    ) -> anonymous_cmds.v2.pki_enrollment_submit.Rep:
        # `pki_enrollment_submit` command is strictly similar between APIv2 and v4+
        # (from client point of view, server may provide an old APIv2 where this
        # command is not available)
        return await self.api_pki_enrollment_submit(client_ctx, req)  # type: ignore

    @api
    async def api_pki_enrollment_submit(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.pki_enrollment_submit.Req,
    ) -> anonymous_cmds.latest.pki_enrollment_submit.Rep:
        try:
            PkiEnrollmentSubmitPayload.load(req.submit_payload)

        except DataError as exc:
            return anonymous_cmds.latest.pki_enrollment_submit.RepInvalidPayloadData(str(exc))

        submitted_on = DateTime.now()
        try:
            await self.submit(
                organization_id=client_ctx.organization_id,
                enrollment_id=cast(EnrollmentID, req.enrollment_id),
                force=req.force,
                submitter_der_x509_certificate=req.submitter_der_x509_certificate,
                submit_payload_signature=req.submit_payload_signature,
                submit_payload=req.submit_payload,
                submitted_on=submitted_on,
                submitter_der_x509_certificate_email=req.submitter_der_x509_certificate_email,
            )
            return anonymous_cmds.latest.pki_enrollment_submit.RepOk(submitted_on)

        except PkiEnrollmentCertificateAlreadySubmittedError as exc:
            return anonymous_cmds.latest.pki_enrollment_submit.RepAlreadySubmitted(exc.submitted_on)

        except PkiEnrollmentIdAlreadyUsedError:
            return anonymous_cmds.latest.pki_enrollment_submit.RepIdAlreadyUsed()

        except PkiEnrollmentEmailAlreadyUsedError:
            return anonymous_cmds.latest.pki_enrollment_submit.RepEmailAlreadyUsed()

        except PkiEnrollmentAlreadyEnrolledError:
            return anonymous_cmds.latest.pki_enrollment_submit.RepAlreadyEnrolled()

    @api
    async def apiv2_pki_enrollment_info(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.v2.pki_enrollment_info.Req,
    ) -> anonymous_cmds.v2.pki_enrollment_info.Rep:
        # `pki_enrollment_info` command is strictly similar between APIv2 and v4+
        # (from client point of view, server may provide an old APIv2 where this
        # command is not available)
        return await self.api_pki_enrollment_info(client_ctx, req)  # type: ignore

    @api
    async def api_pki_enrollment_info(
        self, client_ctx: AnonymousClientContext, req: anonymous_cmds.latest.pki_enrollment_info.Req
    ) -> anonymous_cmds.latest.pki_enrollment_info.Rep:
        try:
            info = await self.info(
                organization_id=client_ctx.organization_id,
                enrollment_id=cast(EnrollmentID, req.enrollment_id),
            )
            rep: anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatus
            if isinstance(info, PkiEnrollmentInfoSubmitted):
                rep = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusSubmitted(
                    info.submitted_on
                )
            elif isinstance(info, PkiEnrollmentInfoAccepted):
                rep = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusAccepted(
                    info.submitted_on,
                    info.accepted_on,
                    info.accepter_der_x509_certificate,
                    info.accept_payload_signature,
                    info.accept_payload,
                )
            elif isinstance(info, PkiEnrollmentInfoRejected):
                rep = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusRejected(
                    info.submitted_on, info.rejected_on
                )
            elif isinstance(info, PkiEnrollmentInfoCancelled):
                rep = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusCancelled(
                    info.submitted_on, info.cancelled_on
                )
            else:
                assert False

        except PkiEnrollmentNotFoundError as exc:
            return anonymous_cmds.latest.pki_enrollment_info.RepNotFound(str(exc))

        return anonymous_cmds.latest.pki_enrollment_info.RepOk(rep)

    @api
    async def apiv2_pki_enrollment_list(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.v2.pki_enrollment_list.Req,
    ) -> authenticated_cmds.v2.pki_enrollment_list.Rep:
        # `pki_enrollment_list` command is strictly similar between APIv2 and v4+
        # (from client point of view, server may provide an old APIv2 where this
        # command is not available)
        return await self.api_pki_enrollment_list(client_ctx, req)  # type: ignore

    @api
    async def api_pki_enrollment_list(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.pki_enrollment_list.Req,
    ) -> authenticated_cmds.latest.pki_enrollment_list.Rep:
        if client_ctx.profile != UserProfile.ADMIN:
            return authenticated_cmds.latest.pki_enrollment_list.RepNotAllowed(
                f"User `{client_ctx.device_id.user_id.str}` is not admin"
            )
        enrollments = await self.list(organization_id=client_ctx.organization_id)
        return authenticated_cmds.latest.pki_enrollment_list.RepOk(
            [
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

    @api
    async def apiv2_pki_enrollment_reject(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.v2.pki_enrollment_reject.Req,
    ) -> authenticated_cmds.v2.pki_enrollment_reject.Rep:
        # `pki_enrollment_reject` command is strictly similar between APIv2 and v4+
        # (from client point of view, server may provide an old APIv2 where this
        # command is not available)
        return await self.api_pki_enrollment_reject(client_ctx, req)  # type: ignore

    @api
    async def api_pki_enrollment_reject(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.pki_enrollment_reject.Req,
    ) -> authenticated_cmds.latest.pki_enrollment_reject.Rep:
        if client_ctx.profile != UserProfile.ADMIN:
            return authenticated_cmds.latest.pki_enrollment_reject.RepNotAllowed(
                f"User `{client_ctx.device_id.user_id.str}` is not admin"
            )
        try:
            await self.reject(
                organization_id=client_ctx.organization_id,
                enrollment_id=req.enrollment_id,
                rejected_on=DateTime.now(),
            )
            return authenticated_cmds.latest.pki_enrollment_reject.RepOk()

        except PkiEnrollmentNotFoundError as exc:
            return authenticated_cmds.latest.pki_enrollment_reject.RepNotFound(str(exc))

        except PkiEnrollmentNoLongerAvailableError as exc:
            return authenticated_cmds.latest.pki_enrollment_reject.RepNoLongerAvailable(str(exc))

    @api
    async def apiv2_pki_enrollment_accept(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.v2.pki_enrollment_accept.Req,
    ) -> authenticated_cmds.v2.pki_enrollment_accept.Rep:
        # `pki_enrollment_accept` command is strictly similar between APIv2 and v4+
        # (from client point of view, server may provide an old APIv2 where this
        # command is not available)
        return await self.api_pki_enrollment_accept(client_ctx, req)  # type: ignore

    @api
    async def api_pki_enrollment_accept(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.pki_enrollment_accept.Req,
    ) -> authenticated_cmds.latest.pki_enrollment_accept.Rep:
        if client_ctx.profile != UserProfile.ADMIN:
            return authenticated_cmds.latest.pki_enrollment_accept.RepNotAllowed(
                f"User `{client_ctx.device_id.user_id.str}` is not admin",
            )

        try:
            PkiEnrollmentAnswerPayload.load(req.accept_payload)

        except DataError as exc:
            return authenticated_cmds.latest.pki_enrollment_accept.RepInvalidPayloadData(str(exc))

        try:
            user, first_device = validate_new_user_certificates(
                expected_author=client_ctx.device_id,
                author_verify_key=client_ctx.verify_key,
                device_certificate=req.device_certificate,
                user_certificate=req.user_certificate,
                redacted_user_certificate=req.redacted_user_certificate,
                redacted_device_certificate=req.redacted_device_certificate,
            )

        except CertificateValidationError as exc:
            return authenticated_cmds.latest.pki_enrollment_accept.RepInvalidCertification(
                exc.reason
            )

        try:
            await self.accept(
                organization_id=client_ctx.organization_id,
                enrollment_id=req.enrollment_id,
                accepter_der_x509_certificate=req.accepter_der_x509_certificate,
                accept_payload_signature=req.accept_payload_signature,
                accept_payload=req.accept_payload,
                accepted_on=DateTime.now(),
                user=user,
                first_device=first_device,
            )
            return authenticated_cmds.latest.pki_enrollment_accept.RepOk()

        except PkiEnrollmentNotFoundError as exc:
            return authenticated_cmds.latest.pki_enrollment_accept.RepNotFound(str(exc))

        except PkiEnrollmentNoLongerAvailableError as exc:
            return authenticated_cmds.latest.pki_enrollment_accept.RepNoLongerAvailable(str(exc))

        except PkiEnrollmentAlreadyExistError as exc:
            return authenticated_cmds.latest.pki_enrollment_accept.RepAlreadyExists(str(exc))

        except PkiEnrollmentActiveUsersLimitReached:
            return authenticated_cmds.latest.pki_enrollment_accept.RepActiveUsersLimitReached()

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
