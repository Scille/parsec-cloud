# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


from uuid import UUID

from pendulum import DateTime
from parsec.api.data.pki import PkiEnrollmentRequest
from parsec.api.protocol.handshake import HandshakeType
from parsec.api.protocol.pki import (
    pki_enrollment_get_requests_serializer,
    pki_enrollment_reply_serializer,
    pki_enrollment_request_serializer,
    pki_enrollment_get_reply_serializer,
)
from parsec.api.protocol.types import OrganizationID, UserProfile
from parsec.backend.utils import api, catch_protocol_errors


class PkiCertificateError(Exception):
    pass


class PkiCertificateAlreadyRequestedError(PkiCertificateError):
    def __init__(self, timestamp, *args, **kwargs):
        self.request_timestamp = timestamp
        PkiCertificateError.__init__(self, *args, **kwargs)


class PkiCertificateAlreadyEnrolledError(PkiCertificateError):
    def __init__(self, timestamp, *args, **kwargs):
        self.reply_timestamp = timestamp
        PkiCertificateError.__init__(self, *args, **kwargs)


class PkiCertificateEmailAlreadyAttributedError(PkiCertificateError):
    pass


class PkiCertificateNotFoundError(PkiCertificateError):
    pass


class PkiCertificateRequestNotFoundError(PkiCertificateError):
    pass


class BasePkiCertificateComponent:
    @catch_protocol_errors
    async def api_pki_enrollment_request(self, msg, organization_id: OrganizationID):
        msg = pki_enrollment_request_serializer.req_load(msg)
        certifiate_id = msg["certificate_id"]
        request = msg["request"]
        request_id = msg["request_id"]
        force_flag = msg["force_flag"]
        try:
            result = await self.pki_enrollment_request(
                organization_id, certifiate_id, request_id, request, force_flag
            )
            return pki_enrollment_request_serializer.rep_dump({"status": "ok", "timestamp": result})
        except PkiCertificateAlreadyRequestedError as err:
            return pki_enrollment_request_serializer.rep_dump(
                {"status": "already_requested", "timestamp": err.request_timestamp}
            )
        except PkiCertificateAlreadyEnrolledError as err:
            return pki_enrollment_request_serializer.rep_dump(
                {"status": "already_enrolled", "timestamp": err.reply_timestamp}
            )
        except PkiCertificateEmailAlreadyAttributedError:
            return pki_enrollment_request_serializer.rep_dump(
                {"status": "email_already_attributed"}
            )

    async def pki_enrollment_request(
        self,
        organization_id: OrganizationID,
        certificate_id: bytes,
        request_id: UUID,
        request_object: PkiEnrollmentRequest,
        force_flag: bool = False,
    ) -> DateTime:
        raise NotImplementedError()

    @api("pki_enrollment_get_requests", handshake_types=[HandshakeType.AUTHENTICATED])
    @catch_protocol_errors
    async def api_pki_enrollment_get_requests(self, client_ctx, msg):
        if client_ctx.profile != UserProfile.ADMIN:
            return {
                "status": "not_allowed",
                "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
            }
        msg = pki_enrollment_get_requests_serializer.req_load(msg)
        requested_pki_cert = await self.pki_enrollment_get_requests()
        return pki_enrollment_get_requests_serializer.rep_dump({"requests": requested_pki_cert})

    @api("pki_enrollment_reply", handshake_types=[HandshakeType.AUTHENTICATED])
    @catch_protocol_errors
    async def api_pki_enrollment_reply(self, client_ctx, msg):
        if client_ctx.profile != UserProfile.ADMIN:
            return {
                "status": "not_allowed",
                "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
            }
        msg = pki_enrollment_reply_serializer.req_load(msg)
        certificate_id = msg["certificate_id"]
        request_id = msg["request_id"]
        user_id = msg["user_id"]
        reply = msg["reply"]
        try:
            timestamp = await self.pki_enrollment_reply(
                certificate_id, request_id, reply, client_ctx.human_handle, user_id
            )
            return pki_enrollment_reply_serializer.rep_dump(
                {"status": "ok", "timestamp": timestamp}
            )
        except PkiCertificateNotFoundError:
            return pki_enrollment_reply_serializer.rep_dump({"status": "certificate not found"})
        except PkiCertificateRequestNotFoundError:
            return pki_enrollment_reply_serializer.rep_dump({"status": "request not found"})

    @catch_protocol_errors
    async def api_pki_enrollment_get_reply(self, msg):
        msg = pki_enrollment_get_reply_serializer.req_load(msg)
        certificate_id = msg["certificate_id"]
        request_id = msg["request_id"]
        try:
            (
                reply,
                reply_timestamp,
                request_timestamp,
                user_id,
                admin_user,
            ) = await self.pki_enrollment_get_reply(certificate_id, request_id)
            if not reply:
                return pki_enrollment_get_reply_serializer.rep_dump(
                    {"status": "pending", "timestamp": request_timestamp}
                )
            if user_id:
                return pki_enrollment_get_reply_serializer.rep_dump(
                    {"status": "already enrolled on other device", "timestamp": reply_timestamp}
                )
            else:
                return pki_enrollment_get_reply_serializer.rep_dump(
                    {
                        "status": "ok",
                        "reply": reply,
                        "timestamp": reply_timestamp,
                        "admin_human_handle": admin_user,
                    }
                )
        except PkiCertificateNotFoundError:
            return pki_enrollment_get_reply_serializer.rep_dump({"status": "certificate not found"})
        except PkiCertificateRequestNotFoundError:
            return pki_enrollment_get_reply_serializer.rep_dump({"status": "request not found"})
