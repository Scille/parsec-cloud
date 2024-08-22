# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from dataclasses import dataclass
from enum import auto

from parsec._parsec import (
    DateTime,
    DeviceID,
    InvitationToken,
    OrganizationID,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryShareCertificate,
    UserID,
    VerifyKey,
    authenticated_cmds,
)
from parsec.api import api
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.client_context import AuthenticatedClientContext
from parsec.types import BadOutcome, BadOutcomeEnum


class BaseShamirComponent:
    @api
    async def api_shamir_recovery_setup(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.shamir_recovery_setup.Req,
    ) -> authenticated_cmds.latest.shamir_recovery_setup.Rep:
        if req.setup is None:
            await self.remove_recovery_setup(client_ctx.organization_id, client_ctx.user_id)
            return authenticated_cmds.latest.shamir_recovery_setup.RepOk()
        else:
            match await self.add_recovery_setup(
                now=DateTime.now(),
                organization_id=client_ctx.organization_id,
                author=client_ctx.user_id,
                device=client_ctx.device_id,
                author_verify_key=client_ctx.device_verify_key,
                setup_ciphered_data=req.setup.ciphered_data,
                setup_reveal_token=req.setup.reveal_token,
                setup_brief=req.setup.brief,
                setup_shares=req.setup.shares,
            ):
                case None:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepOk()

                case ShamirAddOrDeleteRecoverySetupStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                    client_ctx.organization_not_found_abort()
                case ShamirAddOrDeleteRecoverySetupStoreBadOutcome.ORGANIZATION_EXPIRED:
                    client_ctx.organization_expired_abort()
                case ShamirAddOrDeleteRecoverySetupStoreBadOutcome.AUTHOR_NOT_FOUND:
                    client_ctx.author_not_found_abort()
                case ShamirAddOrDeleteRecoverySetupStoreBadOutcome.AUTHOR_REVOKED:
                    client_ctx.author_revoked_abort()
                case ShamirSetupAlreadyExistsBadOutcome() as error:
                    return (
                        authenticated_cmds.latest.shamir_recovery_setup.RepShamirSetupAlreadyExists(
                            error.last_shamir_certificate_timestamp
                        )
                    )

                case ShamirAddRecoverySetupValidateBadOutcome.BRIEF_INVALID_DATA:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepBriefInvalidData()
                case ShamirAddRecoverySetupValidateBadOutcome.SHARE_INVALID_DATA:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepShareInvalidData()
                case ShamirAddRecoverySetupValidateBadOutcome.SHARE_RECIPIENT_NOT_IN_BRIEF:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepShareRecipientNotInBrief()
                case ShamirAddRecoverySetupValidateBadOutcome.DUPLICATE_SHARE_FOR_RECIPIENT:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepDuplicateShareForRecipient()
                case ShamirAddRecoverySetupValidateBadOutcome.AUTHOR_INCLUDED_AS_RECIPIENT:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepAuthorIncludedAsRecipient()
                case ShamirAddRecoverySetupValidateBadOutcome.MISSING_SHARE_FOR_RECIPIENT:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepMissingShareForRecipient()
                case ShamirAddRecoverySetupValidateBadOutcome.SHARE_INCONSISTENT_TIMESTAMP:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepShareInconsistentTimestamp()

                case ShamirInvalidRecipientBadOutcome() as error:
                    return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidRecipient(
                        error.user_id
                    )

                case TimestampOutOfBallpark() as error:
                    return (
                        authenticated_cmds.latest.shamir_recovery_setup.RepTimestampOutOfBallpark(
                            server_timestamp=error.server_timestamp,
                            client_timestamp=error.client_timestamp,
                            ballpark_client_early_offset=error.ballpark_client_early_offset,
                            ballpark_client_late_offset=error.ballpark_client_late_offset,
                        )
                    )
                case RequireGreaterTimestamp() as error:
                    return (
                        authenticated_cmds.latest.shamir_recovery_setup.RepRequireGreaterTimestamp(
                            strictly_greater_than=error.strictly_greater_than
                        )
                    )

    async def remove_recovery_setup(
        self,
        organization_id: OrganizationID,
        author: UserID,
    ) -> None | ShamirAddOrDeleteRecoverySetupStoreBadOutcome:
        raise NotImplementedError

    async def add_recovery_setup(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: UserID,
        device: DeviceID,
        author_verify_key: VerifyKey,
        setup_ciphered_data: bytes,
        setup_reveal_token: InvitationToken,
        setup_brief: bytes,
        setup_shares: list[bytes],
    ) -> (
        None
        | ShamirAddOrDeleteRecoverySetupStoreBadOutcome
        | ShamirAddRecoverySetupValidateBadOutcome
        | ShamirSetupAlreadyExistsBadOutcome
        | ShamirInvalidRecipientBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError


class ShamirAddRecoverySetupValidateBadOutcome(BadOutcomeEnum):
    BRIEF_INVALID_DATA = auto()
    SHARE_INVALID_DATA = auto()
    SHARE_RECIPIENT_NOT_IN_BRIEF = auto()
    DUPLICATE_SHARE_FOR_RECIPIENT = auto()
    AUTHOR_INCLUDED_AS_RECIPIENT = auto()
    MISSING_SHARE_FOR_RECIPIENT = auto()
    SHARE_INCONSISTENT_TIMESTAMP = auto()


class ShamirAddOrDeleteRecoverySetupStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


@dataclass(slots=True)
class ShamirSetupAlreadyExistsBadOutcome(BadOutcome):
    last_shamir_certificate_timestamp: DateTime


@dataclass(slots=True)
class ShamirInvalidRecipientBadOutcome(BadOutcome):
    user_id: UserID


# Check internal consistency of certificate
def shamir_add_recovery_setup_validate(
    now: DateTime,
    author: DeviceID,
    user_id: UserID,
    author_verify_key: VerifyKey,
    setup_brief: bytes,
    setup_shares: list[bytes],
) -> (
    tuple[ShamirRecoveryBriefCertificate, dict[UserID, bytes]]
    | ShamirAddRecoverySetupValidateBadOutcome
    | TimestampOutOfBallpark
):
    share_certificates: dict[UserID, bytes] = {}
    try:
        brief_certificate = ShamirRecoveryBriefCertificate.verify_and_load(
            setup_brief, author_verify_key, expected_author=author
        )
    except ValueError:
        return ShamirAddRecoverySetupValidateBadOutcome.BRIEF_INVALID_DATA

    match timestamps_in_the_ballpark(brief_certificate.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass
    for raw_share in setup_shares:
        try:
            share_certificate = ShamirRecoveryShareCertificate.verify_and_load(
                raw_share, author_verify_key, expected_author=author, expected_recipient=None
            )
        except ValueError:
            return ShamirAddRecoverySetupValidateBadOutcome.SHARE_INVALID_DATA

        if share_certificate.timestamp != brief_certificate.timestamp:
            return ShamirAddRecoverySetupValidateBadOutcome.SHARE_INCONSISTENT_TIMESTAMP
        # share recipient not in brief
        if share_certificate.recipient not in brief_certificate.per_recipient_shares:
            return ShamirAddRecoverySetupValidateBadOutcome.SHARE_RECIPIENT_NOT_IN_BRIEF
        # this recipient already has a share
        if share_certificate.recipient in share_certificates:
            return ShamirAddRecoverySetupValidateBadOutcome.DUPLICATE_SHARE_FOR_RECIPIENT
        # user included themselves as a share recipient
        if share_certificate.recipient == user_id:
            return ShamirAddRecoverySetupValidateBadOutcome.AUTHOR_INCLUDED_AS_RECIPIENT
        share_certificates[share_certificate.recipient] = raw_share
    delta = set(brief_certificate.per_recipient_shares) - set(share_certificates)
    # some recipient specified in brief has no share
    if delta:
        return ShamirAddRecoverySetupValidateBadOutcome.MISSING_SHARE_FOR_RECIPIENT
    return brief_certificate, share_certificates
