# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from uuid import UUID
from parsec.api.data.base import DataError
from parsec.api.data.tpek import TpekDerServiceEncryptionKey

from parsec.api.protocol.types import OrganizationID
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.utils import ClientType, api, catch_protocol_errors
from parsec.event_bus import EventBus
from parsec.api.protocol.tpek import TpekServiceType, tpek_register_service_serializer

from parsec.tpek_crypto import DerPublicKey, TpekCryptoSignatureError, verify_tpek


class TpekError(Exception):
    pass


class TpekSignatureError(TpekError):
    pass


class BaseTpekComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    @api("tpek_register_service", client_types=[ClientType.ANONYMOUS])
    @catch_protocol_errors
    async def api_tpek_register_service(
        self, client_ctx: AuthenticatedClientContext, msg: dict
    ) -> dict:

        msg = tpek_register_service_serializer.req_load(msg)
        tpek_encryption_key_payload = msg["tpek_der_payload"]
        try:
            # Ensure tpek_encrypotion_payload is loadable
            TpekDerServiceEncryptionKey.load(tpek_encryption_key_payload)
        except DataError as exc:
            return {"status": "invalid_der_payload", "reason": str(exc)}

        try:
            await self.register_service(
                client_ctx.organization_id,
                msg["service_id"],
                msg["service_type"],
                tpek_encryption_key_payload,
                msg["tpek_der_payload_signature"],
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
        tpek_encryption_key: bytes,
        tpek_encryption_key_signature: bytes,
    ):
        """
        Raises:
            TpekSignatureError
        """
        raise NotImplementedError()


def verify_tpek_der_signature(tpek_verify_key: DerPublicKey, signed_data: bytes, data: bytes):
    """
    Raises:
        TpekSignatureError
    """
    try:
        verify_tpek(tpek_verify_key, data, signed_data)
    except TpekCryptoSignatureError:
        raise TpekSignatureError()
