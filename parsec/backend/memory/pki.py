# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
from typing import Dict, List, Optional, Tuple
from uuid import UUID

import attr

import pendulum
from pendulum.datetime import DateTime
from parsec.api.data.pki import PkiEnrollmentReply, PkiEnrollmentRequest
from parsec.api.protocol.types import HumanHandle, OrganizationID
from parsec.backend.memory.user import MemoryUserComponent
from parsec.backend.pki import (
    BasePkiCertificateComponent,
    PkiCertificateAlreadyEnrolledError,
    PkiCertificateAlreadyRequestedError,
    PkiCertificateEmailAlreadyAttributedError,
    PkiCertificateNotFoundError,
    PkiCertificateRequestNotFoundError,
    PkiEnrollementReplyBundle,
)


@attr.s
class PkiCertificate:
    certificate_id: bytes = attr.ib()
    request_id: UUID = attr.ib()
    request_object: PkiEnrollmentRequest = attr.ib()
    request_timestamp: DateTime = attr.ib()
    reply_user_id: Optional[str] = attr.ib(default=None)
    reply_timestamp: Optional[DateTime] = attr.ib(default=None)
    reply_object: Optional[PkiEnrollmentReply] = attr.ib(default=None)
    reply_admin_user: Optional[HumanHandle] = attr.ib(default=None)


class MemoryPkiCertificateComponent(BasePkiCertificateComponent):
    def __init__(self, send_event):
        self._send_event = send_event
        self._pki_certificates: Dict[bytes, PkiCertificate] = {}
        self._user_component: MemoryUserComponent = None

    def register_components(self, **other_components):
        self._user_component = other_components["user"]

    async def pki_enrollment_request(
        self,
        organization_id: OrganizationID,
        certificate_id: bytes,
        request_id: UUID,
        request_object: PkiEnrollmentRequest,
        force_flag: bool = False,
    ) -> DateTime:

        humans, _ = await self._user_component.find_humans(
            organization_id=organization_id, query=request_object.requested_human_handle.email
        )
        if humans and not humans[0].revoked:
            raise PkiCertificateEmailAlreadyAttributedError(
                f"email f{request_object.requested_human_handle} already attributed"
            )
        # Revoked user certificate can be overwritten
        elif not humans:
            existing_certificate = self._pki_certificates.get(certificate_id, None)
            if existing_certificate:
                if existing_certificate.reply_object and existing_certificate.reply_user_id:
                    raise PkiCertificateAlreadyEnrolledError(
                        existing_certificate.request_timestamp,
                        f"Certificate {str(certificate_id)} already attributed",
                    )

                if not force_flag:
                    raise PkiCertificateAlreadyRequestedError(
                        existing_certificate.reply_timestamp,
                        f"Certificate {str(certificate_id)} already used in request {request_id}",
                    )

        new_pki_certificate = PkiCertificate(
            certificate_id=certificate_id,
            request_id=request_id,
            request_timestamp=pendulum.now(),
            request_object=request_object,
            reply_user_id=None,
            reply_timestamp=None,
            reply_object=None,
        )
        self._pki_certificates[certificate_id] = new_pki_certificate
        return new_pki_certificate.request_timestamp

    async def pki_enrollment_get_requests(self) -> List[Tuple[bytes, UUID, PkiEnrollmentRequest]]:
        return [
            (
                pki_certificate.certificate_id,
                pki_certificate.request_id,
                pki_certificate.request_object,
            )
            for pki_certificate in self._pki_certificates.values()
            if pki_certificate.reply_admin_user is None
        ]

    async def pki_enrollrment_reply_reject_user(
        self, pki_reply_bundle: PkiEnrollementReplyBundle
    ) -> DateTime:
        return await self._pki_enrollment_reply(pki_reply_bundle)

    async def pki_enrollment_reply_approve_user(
        self, pki_reply_bundle: PkiEnrollementReplyBundle, client_ctx, msg
    ) -> DateTime:
        await self._user_component._api_user_create(client_ctx, msg, pki_reply_bundle)
        timestamp = self._pki_certificates[pki_reply_bundle.certificate_id].reply_timestamp
        assert timestamp
        return timestamp

    async def _pki_enrollment_reply(self, pki_reply_bundle: PkiEnrollementReplyBundle) -> DateTime:
        try:
            pki_certificate = self._pki_certificates[pki_reply_bundle.certificate_id]
        except KeyError:
            raise PkiCertificateNotFoundError(
                f"Certificate {str(pki_reply_bundle.certificate_id)} not found"
            )
        if pki_certificate.request_id != pki_reply_bundle.request_id:
            raise PkiCertificateRequestNotFoundError(
                f"Request {pki_reply_bundle.request_id} not found for certificate {str(pki_reply_bundle.certificate_id)}"
            )

        pki_certificate.reply_object = pki_reply_bundle.reply_object
        pki_certificate.reply_user_id = pki_reply_bundle.reply_user_id
        pki_certificate.reply_timestamp = pendulum.now()
        pki_certificate.reply_admin_user = pki_reply_bundle.reply_admin_user
        return pki_certificate.reply_timestamp

    async def pki_enrollment_get_reply(
        self, certificate_id, request_id
    ) -> Tuple[
        Optional[PkiEnrollmentReply],
        Optional[DateTime],
        DateTime,
        Optional[str],
        Optional[HumanHandle],
    ]:
        try:
            pki_certificate = self._pki_certificates[certificate_id]
        except KeyError:
            raise PkiCertificateNotFoundError(f"Certificate {certificate_id} not found")
        if pki_certificate.request_id != request_id:
            raise PkiCertificateRequestNotFoundError(
                f"Request {request_id} not found for certificate {certificate_id}"
            )
        return (
            pki_certificate.reply_object,
            pki_certificate.reply_timestamp,
            pki_certificate.request_timestamp,
            pki_certificate.reply_user_id,
            pki_certificate.reply_admin_user,
        )
