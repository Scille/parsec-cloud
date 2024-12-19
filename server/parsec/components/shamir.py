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
    ShamirRecoveryDeletionCertificate,
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


class ShamirSetupValidateBadOutcome(BadOutcomeEnum):
    BRIEF_CORRUPTED = auto()
    SHARE_CORRUPTED = auto()
    SHARE_RECIPIENT_NOT_IN_BRIEF = auto()
    DUPLICATE_SHARE_FOR_RECIPIENT = auto()
    AUTHOR_INCLUDED_AS_RECIPIENT = auto()
    MISSING_SHARE_FOR_RECIPIENT = auto()
    SHARE_INCONSISTENT_TIMESTAMP = auto()
    USER_ID_MUST_BE_SELF = auto()


class ShamirSetupStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    RECIPIENT_NOT_FOUND = auto()


@dataclass(slots=True)
class ShamirSetupAlreadyExistsBadOutcome(BadOutcome):
    last_shamir_recovery_certificate_timestamp: DateTime


@dataclass(slots=True)
class ShamirSetupRevokedRecipientBadOutcome(BadOutcome):
    last_common_certificate_timestamp: DateTime


class ShamirDeleteValidateBadOutcome(BadOutcomeEnum):
    CORRUPTED = auto()
    USER_ID_MUST_BE_SELF = auto()


class ShamirDeleteStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    SETUP_NOT_FOUND = auto()
    RECIPIENTS_MISMATCH = auto()


@dataclass(slots=True)
class ShamirDeleteSetupAlreadyDeletedBadOutcome(BadOutcome):
    last_shamir_recovery_certificate_timestamp: DateTime


# Check internal consistency of certificate
def shamir_setup_validate(
    now: DateTime,
    expected_author_device_id: DeviceID,
    expected_author_user_id: UserID,
    author_verify_key: VerifyKey,
    shamir_recovery_brief_certificate: bytes,
    shamir_recovery_share_certificates: list[bytes],
) -> (
    tuple[
        ShamirRecoveryBriefCertificate, dict[UserID, tuple[bytes, ShamirRecoveryShareCertificate]]
    ]
    | ShamirSetupValidateBadOutcome
    | TimestampOutOfBallpark
):
    shares: dict[UserID, tuple[bytes, ShamirRecoveryShareCertificate]] = {}
    try:
        brief = ShamirRecoveryBriefCertificate.verify_and_load(
            shamir_recovery_brief_certificate,
            author_verify_key,
            expected_author=expected_author_device_id,
        )
    except ValueError:
        return ShamirSetupValidateBadOutcome.BRIEF_CORRUPTED

    match timestamps_in_the_ballpark(brief.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    if brief.user_id != expected_author_user_id:
        return ShamirSetupValidateBadOutcome.USER_ID_MUST_BE_SELF

    # Some recipient specified in brief has no share
    if len(brief.per_recipient_shares) != len(shamir_recovery_share_certificates):
        return ShamirSetupValidateBadOutcome.MISSING_SHARE_FOR_RECIPIENT

    for raw_share in shamir_recovery_share_certificates:
        try:
            share_certificate = ShamirRecoveryShareCertificate.verify_and_load(
                raw_share,
                author_verify_key,
                expected_author=expected_author_device_id,
                expected_recipient=None,
            )
        except ValueError:
            return ShamirSetupValidateBadOutcome.SHARE_CORRUPTED

        if share_certificate.timestamp != brief.timestamp:
            return ShamirSetupValidateBadOutcome.SHARE_INCONSISTENT_TIMESTAMP
        # Share recipient not in brief
        if share_certificate.recipient not in brief.per_recipient_shares:
            return ShamirSetupValidateBadOutcome.SHARE_RECIPIENT_NOT_IN_BRIEF
        # This recipient already has a share
        if share_certificate.recipient in shares:
            return ShamirSetupValidateBadOutcome.DUPLICATE_SHARE_FOR_RECIPIENT
        # User included themselves as a share recipient
        if share_certificate.recipient == expected_author_user_id:
            return ShamirSetupValidateBadOutcome.AUTHOR_INCLUDED_AS_RECIPIENT
        shares[share_certificate.recipient] = (raw_share, share_certificate)

    return brief, shares


# Check internal consistency of certificate
def shamir_delete_validate(
    now: DateTime,
    expected_author_device_id: DeviceID,
    expected_author_user_id: UserID,
    author_verify_key: VerifyKey,
    shamir_recovery_deletion_certificate: bytes,
) -> ShamirRecoveryDeletionCertificate | ShamirDeleteValidateBadOutcome | TimestampOutOfBallpark:
    try:
        deletion = ShamirRecoveryDeletionCertificate.verify_and_load(
            shamir_recovery_deletion_certificate,
            author_verify_key,
            expected_author=expected_author_device_id,
        )
    except ValueError:
        return ShamirDeleteValidateBadOutcome.CORRUPTED

    match timestamps_in_the_ballpark(deletion.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    if deletion.setup_to_delete_user_id != expected_author_user_id:
        return ShamirDeleteValidateBadOutcome.USER_ID_MUST_BE_SELF

    return deletion


class BaseShamirComponent:
    @api
    async def api_shamir_recovery_setup(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.shamir_recovery_setup.Req,
    ) -> authenticated_cmds.latest.shamir_recovery_setup.Rep:
        match await self.setup(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            ciphered_data=req.ciphered_data,
            reveal_token=req.reveal_token,
            shamir_recovery_brief_certificate=req.shamir_recovery_brief_certificate,
            shamir_recovery_share_certificates=req.shamir_recovery_share_certificates,
        ):
            case ShamirRecoveryBriefCertificate():
                return authenticated_cmds.latest.shamir_recovery_setup.RepOk()

            case ShamirSetupAlreadyExistsBadOutcome() as error:
                return (
                    authenticated_cmds.latest.shamir_recovery_setup.RepShamirRecoveryAlreadyExists(
                        error.last_shamir_recovery_certificate_timestamp
                    )
                )
            case ShamirSetupStoreBadOutcome.RECIPIENT_NOT_FOUND:
                return authenticated_cmds.latest.shamir_recovery_setup.RepRecipientNotFound()
            case ShamirSetupRevokedRecipientBadOutcome() as error:
                return authenticated_cmds.latest.shamir_recovery_setup.RepRevokedRecipient(
                    last_common_certificate_timestamp=error.last_common_certificate_timestamp
                )
            case ShamirSetupValidateBadOutcome.BRIEF_CORRUPTED:
                return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidCertificateBriefCorrupted()
            case ShamirSetupValidateBadOutcome.SHARE_CORRUPTED:
                return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidCertificateShareCorrupted()
            case ShamirSetupValidateBadOutcome.SHARE_RECIPIENT_NOT_IN_BRIEF:
                return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidCertificateShareRecipientNotInBrief()
            case ShamirSetupValidateBadOutcome.DUPLICATE_SHARE_FOR_RECIPIENT:
                return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidCertificateDuplicateShareForRecipient()
            case ShamirSetupValidateBadOutcome.AUTHOR_INCLUDED_AS_RECIPIENT:
                return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidCertificateAuthorIncludedAsRecipient()
            case ShamirSetupValidateBadOutcome.MISSING_SHARE_FOR_RECIPIENT:
                return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidCertificateMissingShareForRecipient()
            case ShamirSetupValidateBadOutcome.SHARE_INCONSISTENT_TIMESTAMP:
                return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidCertificateShareInconsistentTimestamp()
            case ShamirSetupValidateBadOutcome.USER_ID_MUST_BE_SELF:
                return authenticated_cmds.latest.shamir_recovery_setup.RepInvalidCertificateUserIdMustBeSelf()
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.shamir_recovery_setup.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.shamir_recovery_setup.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case ShamirSetupStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case ShamirSetupStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case ShamirSetupStoreBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case ShamirSetupStoreBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_shamir_recovery_delete(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.shamir_recovery_delete.Req,
    ) -> authenticated_cmds.latest.shamir_recovery_delete.Rep:
        match await self.delete(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            shamir_recovery_deletion_certificate=req.shamir_recovery_deletion_certificate,
        ):
            case ShamirRecoveryDeletionCertificate():
                return authenticated_cmds.latest.shamir_recovery_delete.RepOk()
            case ShamirDeleteStoreBadOutcome.SETUP_NOT_FOUND:
                return authenticated_cmds.latest.shamir_recovery_delete.RepShamirRecoveryNotFound()
            case ShamirDeleteStoreBadOutcome.RECIPIENTS_MISMATCH:
                return authenticated_cmds.latest.shamir_recovery_delete.RepRecipientsMismatch()
            case ShamirDeleteSetupAlreadyDeletedBadOutcome() as error:
                return authenticated_cmds.latest.shamir_recovery_delete.RepShamirRecoveryAlreadyDeleted(
                    error.last_shamir_recovery_certificate_timestamp
                )
            case ShamirDeleteValidateBadOutcome.CORRUPTED:
                return authenticated_cmds.latest.shamir_recovery_delete.RepInvalidCertificateCorrupted()
            case ShamirDeleteValidateBadOutcome.USER_ID_MUST_BE_SELF:
                return authenticated_cmds.latest.shamir_recovery_delete.RepInvalidCertificateUserIdMustBeSelf()
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.shamir_recovery_delete.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.shamir_recovery_delete.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case ShamirDeleteStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case ShamirDeleteStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case ShamirDeleteStoreBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case ShamirDeleteStoreBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    async def setup(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        ciphered_data: bytes,
        reveal_token: InvitationToken,
        shamir_recovery_brief_certificate: bytes,
        shamir_recovery_share_certificates: list[bytes],
    ) -> (
        ShamirRecoveryBriefCertificate
        | ShamirSetupStoreBadOutcome
        | ShamirSetupValidateBadOutcome
        | ShamirSetupAlreadyExistsBadOutcome
        | ShamirSetupRevokedRecipientBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def delete(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        shamir_recovery_deletion_certificate: bytes,
    ) -> (
        ShamirRecoveryDeletionCertificate
        | ShamirDeleteStoreBadOutcome
        | ShamirDeleteValidateBadOutcome
        | ShamirDeleteSetupAlreadyDeletedBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError
