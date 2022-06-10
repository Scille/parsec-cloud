# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from uuid import UUID
import oscrypto
from parsec.api.data.certif import TpekDerServiceEncryptionKeyCertificateContent
from parsec.api.protocol.types import OrganizationID, UserProfile
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.utils import ClientType, api, catch_protocol_errors
from parsec.event_bus import EventBus
from parsec.api.protocol.tpek import TpekServiceType, tpek_register_service_serializer

from oscrypto.asymmetric import PublicKey as DerPublicKey


class TpekError(Exception):
    pass


class TpekSignatureError(TpekError):
    pass


class BaseTpekComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    @api("tpek_register_service", client_types=[ClientType.AUTHENTICATED])
    @catch_protocol_errors
    async def api_tpek_register_service(
        self, client_ctx: AuthenticatedClientContext, msg: dict
    ) -> dict:
        if client_ctx.profile != UserProfile.ADMIN:
            return {
                "status": "not_allowed",
                "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
            }

        msg = tpek_register_service_serializer.req_load(msg)
        tpek_certificate = TpekDerServiceEncryptionKeyCertificateContent.verify_and_load(
            msg["service_certificate"],
            author_verify_key=client_ctx.verify_key,
            expected_author=client_ctx.device_id,
        )
        try:
            await self.register_service(
                client_ctx.organization_id,
                msg["service_id"],
                msg["service_type"],
                tpek_certificate.encryption_key,
                tpek_certificate.signed_encryption_key,
                msg["service_certificate"],
            )
        except TpekSignatureError:
            return {"status": "signature_error", "reason": "Bad tpek signature"}
        return tpek_register_service_serializer.rep_dump({"status": "ok"})

    # @api("tpek_list_services", client_types=[ClientType.AUTHENTICATED])
    # @catch_protocol_errors
    # async def api_tpek_list_services(
    #     self, client_ctx: AuthenticatedClientContext, msg: dict
    # ) -> dict:
    #     if client_ctx.profile != UserProfile.ADMIN:
    #         return {
    #             "status": "not_allowed",
    #             "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
    #         }

    async def register_service(
        self,
        organization_id: OrganizationID,
        service_id: UUID,
        service_type: TpekServiceType,
        tpek_certificate_encryption_key: DerPublicKey,
        tpek_certificate_signed_encryption_key: bytes,
        tpek_certificate: bytes,
    ):
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()


def verify_tpek_der_signature(
    tpek_verify_key: DerPublicKey, signed_data: bytes, data: bytes, hash_algorithm: str = "sha1"
):
    """
    Raises:
        TpekSignatureError
    """
    try:
        oscrypto.asymetric.rsa_pkcs1v15_verify(tpek_verify_key, signed_data, data, hash_algorithm)
    except oscrypto.errors.SignatureError:
        raise TpekSignatureError()
