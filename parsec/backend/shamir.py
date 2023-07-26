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
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryOthersListRep,
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
    ShamirRecoveryShareCertificate,
    ShamirRevealToken,
    UserID,
    VerifyKey,
)
from parsec.backend.client_context import AuthenticatedClientContext, InvitedClientContext
from parsec.backend.utils import api, api_typed_msg_adapter, catch_protocol_errors

# Helpers


def verify_certificates(
    setup: ShamirRecoverySetup,
    author: DeviceID,
    author_verify_key: VerifyKey,
) -> tuple[ShamirRecoveryBriefCertificate, dict[UserID, bytes]]:
    share_certificates: dict[UserID, bytes] = {}
    brief_certificate = ShamirRecoveryBriefCertificate.verify_and_load(
        setup.brief,
        author_verify_key,
        expected_author=author,
    )
    for raw_share in setup.shares:
        share_certificate = ShamirRecoveryShareCertificate.verify_and_load(
            raw_share, author_verify_key, expected_author=author
        )
        if share_certificate.recipient not in brief_certificate.per_recipient_shares:
            raise DataError(
                f"Recipient {share_certificate.recipient.str} does not in appear in the brief certificate"
            )
        if share_certificate.recipient in share_certificates:
            raise DataError(f"Recipient {share_certificate.recipient.str} appears more than once")
        if share_certificate.recipient == author.user_id:
            raise DataError(f"Author {author.user_id} included themselves in the recipients")
        share_certificates[share_certificate.recipient] = raw_share
    delta = set(brief_certificate.per_recipient_shares) - set(share_certificates)
    if delta:
        missing = ", ".join(user_id.str for user_id in delta)
        raise DataError(f"The following shares are missing: {missing}")
    return brief_certificate, share_certificates


class BaseShamirComponent:
    @api("shamir_recovery_others_list")
    @catch_protocol_errors
    @api_typed_msg_adapter(ShamirRecoveryOthersListReq, ShamirRecoveryOthersListRep)
    async def api_shamir_recovery_others_list(
        self, client_ctx: AuthenticatedClientContext, req: ShamirRecoveryOthersListReq
    ) -> ShamirRecoveryOthersListRep:
        result = await self.recovery_others_list(client_ctx.organization_id, client_ctx.device_id)

        # Unzip
        brief_certificates = []
        share_certificates = []
        for brief_certificate, share_certificate in result:
            brief_certificates.append(brief_certificate)
            share_certificates.append(share_certificate)

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
            if req.setup is None:
                await self.remove_recovery_setup(
                    organization_id=client_ctx.organization_id,
                    author=client_ctx.device_id,
                )
            else:
                brief_certificate, raw_share_certificates = verify_certificates(
                    req.setup,
                    author=client_ctx.device_id,
                    author_verify_key=client_ctx.verify_key,
                )
                await self.add_recovery_setup(
                    organization_id=client_ctx.organization_id,
                    author=client_ctx.device_id,
                    setup=req.setup,
                    brief_certificate=brief_certificate,
                    raw_share_certificates=raw_share_certificates,
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
        ciphered_data = await self.recovery_reveal(
            client_ctx.organization_id, reveal_token=req.reveal_token
        )

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

    async def remove_recovery_setup(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> None:
        raise NotImplementedError

    async def add_recovery_setup(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        setup: ShamirRecoverySetup,
        brief_certificate: ShamirRecoveryBriefCertificate,
        raw_share_certificates: dict[UserID, bytes],
    ) -> None:
        raise NotImplementedError

    async def recovery_reveal(
        self,
        organization_id: OrganizationID,
        reveal_token: ShamirRevealToken,
    ) -> bytes | None:
        raise NotImplementedError()
