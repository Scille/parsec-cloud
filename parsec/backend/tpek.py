# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from parsec.api.data.certif import TpekDerServiceEncryptionKeyCertificateContent
from parsec.api.protocol.types import OrganizationID, UserProfile
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.utils import ClientType, api, catch_protocol_errors
from parsec.event_bus import EventBus
from parsec.api.protocol.tpek import tpek_register_service_serializer


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
        # Check signature
        await self._verify_and_store_tpek_der_signature(
            tpek_certificate, client_ctx.organization_id
        )
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

    def _verify_and_store_tpek_der_signature(
        self,
        tpek_certificate: TpekDerServiceEncryptionKeyCertificateContent,
        organization_id: OrganizationID,
    ):
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()
