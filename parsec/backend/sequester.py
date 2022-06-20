# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from uuid import UUID
from parsec.api.data.base import DataError
from parsec.api.data.sequester import SequesterDerServiceEncryptionKey

from parsec.api.protocol.types import OrganizationID
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.utils import ClientType, api, catch_protocol_errors
from parsec.event_bus import EventBus
from parsec.api.protocol.sequester import (
    SequesterServiceType,
    sequester_register_service_serializer,
)

from parsec.sequester_crypto import DerPublicKey, SequesterCryptoSignatureError, verify_sequester


class SequesterError(Exception):
    pass


class SequesterSignatureError(SequesterError):
    pass


class BaseSequesterComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    @api("sequester_register_service", client_types=[ClientType.ANONYMOUS])
    @catch_protocol_errors
    async def api_sequester_register_service(
        self, client_ctx: AuthenticatedClientContext, msg: dict
    ) -> dict:

        msg = sequester_register_service_serializer.req_load(msg)
        sequester_encryption_key_payload = msg["sequester_der_payload"]
        try:
            # Ensure sequester_encrypotion_payload is loadable
            SequesterDerServiceEncryptionKey.load(sequester_encryption_key_payload)
        except DataError as exc:
            return {"status": "invalid_der_payload", "reason": str(exc)}

        try:
            await self.register_service(
                client_ctx.organization_id,
                msg["service_id"],
                msg["service_type"],
                sequester_encryption_key_payload,
                msg["sequester_der_payload_signature"],
            )
        except SequesterSignatureError:
            return {"status": "signature_error", "reason": "Bad sequester signature"}
        return sequester_register_service_serializer.rep_dump({"status": "ok"})

    # @api("sequester_list_services", client_types=[ClientType.AUTHENTICATED])
    # @catch_protocol_errors
    # async def api_sequester_list_services(
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
        service_type: SequesterServiceType,
        sequester_encryption_key: bytes,
        sequester_encryption_key_signature: bytes,
    ):
        """
        Raises:
            SequesterSignatureError
        """
        raise NotImplementedError()


def verify_sequester_der_signature(
    sequester_verify_key: DerPublicKey, signed_data: bytes, data: bytes
):
    """
    Raises:
        SequesterSignatureError
    """
    try:
        verify_sequester(sequester_verify_key, data, signed_data)
    except SequesterCryptoSignatureError:
        raise SequesterSignatureError()
