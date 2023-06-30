# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import (
    ClientType,
    DataError,
    DeviceID,
    InviteShamirRecoveryRevealRep,
    InviteShamirRecoveryRevealRepNotFound,
    InviteShamirRecoveryRevealRepOk,
    InviteShamirRecoveryRevealReq,
    OrganizationID,
    ShamirRecoveryOthersListRep,
    ShamirRecoveryOthersListRepNotAllowed,
    ShamirRecoveryOthersListRepOk,
    ShamirRecoveryOthersListReq,
    ShamirRecoverySelfInfoRep,
    ShamirRecoverySelfInfoRepOk,
    ShamirRecoverySelfInfoReq,
    ShamirRecoverySetup,
    ShamirRecoverySetupRep,
    ShamirRecoverySetupRepInvalidData,
    ShamirRecoverySetupRepOk,
    ShamirRecoverySetupReq,
    ShamirRevealToken,
    UserProfile,
    VerifyKey,
)
from parsec.backend.client_context import AuthenticatedClientContext, InvitedClientContext
from parsec.backend.utils import api, api_typed_msg_adapter, catch_protocol_errors


class BaseShamirComponent:
    @api("shamir_recovery_others_list")
    @catch_protocol_errors
    @api_typed_msg_adapter(ShamirRecoveryOthersListReq, ShamirRecoveryOthersListRep)
    async def api_shamir_recovery_others_list(
        self, client_ctx: AuthenticatedClientContext, req: ShamirRecoveryOthersListReq
    ) -> ShamirRecoveryOthersListRep:
        if client_ctx.profile != UserProfile.ADMIN:
            return ShamirRecoveryOthersListRepNotAllowed()

        result = await self.recovery_others_list(client_ctx.organization_id, client_ctx.device_id)
        brief_certificates, share_certificates = zip(*result)

        return ShamirRecoveryOthersListRepOk(brief_certificates, share_certificates)

    @api("shamir_recovery_self_info")
    @catch_protocol_errors
    @api_typed_msg_adapter(ShamirRecoverySelfInfoReq, ShamirRecoverySelfInfoRep)
    async def api_recovery_self_info(
        self, client_ctx: AuthenticatedClientContext, req: ShamirRecoverySelfInfoReq
    ) -> ShamirRecoverySelfInfoRep:
        self_info = await self.recovery_self_info(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
        )

        return ShamirRecoverySelfInfoRepOk(self_info)

    @api("shamir_recovery_setup")
    @catch_protocol_errors
    @api_typed_msg_adapter(ShamirRecoverySetupReq, ShamirRecoverySetupRep)
    async def api_shamir_recovery_setup(
        self, client_ctx: AuthenticatedClientContext, req: ShamirRecoverySetupReq
    ) -> ShamirRecoverySetupRep:
        try:
            await self.recovery_setup(
                organization_id=client_ctx.organization_id,
                author=client_ctx.device_id,
                author_verify_key=client_ctx.verify_key,
                setup=req.setup,
            )
        except DataError:
            return ShamirRecoverySetupRepInvalidData()

        return ShamirRecoverySetupRepOk()

    @api("invite_shamir_recovery_reveal", client_types=[ClientType.INVITED])
    @catch_protocol_errors
    @api_typed_msg_adapter(InviteShamirRecoveryRevealReq, InviteShamirRecoveryRevealRep)
    async def api_invite_shamir_recovery_reveal(
        self, client_ctx: InvitedClientContext, req: InviteShamirRecoveryRevealReq
    ) -> InviteShamirRecoveryRevealRep:
        ciphered_data = await self.recovery_reveal(reveal_token=req.reveal_token)

        if ciphered_data is None:
            return InviteShamirRecoveryRevealRepNotFound()

        return InviteShamirRecoveryRevealRepOk(ciphered_data)

    async def recovery_others_list(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> list[tuple[bytes, bytes]]:
        raise NotImplementedError()

    async def recovery_self_info(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> bytes | None:
        raise NotImplementedError()

    async def recovery_setup(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        setup: ShamirRecoverySetup | None,
    ) -> None:
        raise NotImplementedError()

    async def recovery_reveal(
        self,
        reveal_token: ShamirRevealToken,
    ) -> bytes | None:
        raise NotImplementedError()
