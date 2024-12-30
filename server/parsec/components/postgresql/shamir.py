# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    InvitationToken,
    OrganizationID,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryDeletionCertificate,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.shamir_delete import shamir_delete
from parsec.components.postgresql.shamir_setup import shamir_setup
from parsec.components.postgresql.utils import transaction
from parsec.components.shamir import (
    BaseShamirComponent,
    ShamirDeleteSetupAlreadyDeletedBadOutcome,
    ShamirDeleteStoreBadOutcome,
    ShamirDeleteValidateBadOutcome,
    ShamirSetupAlreadyExistsBadOutcome,
    ShamirSetupRevokedRecipientBadOutcome,
    ShamirSetupStoreBadOutcome,
    ShamirSetupValidateBadOutcome,
)


class PGShamirComponent(BaseShamirComponent):
    def __init__(self, pool: AsyncpgPool) -> None:
        super().__init__()
        self.pool = pool

    @override
    @transaction
    async def setup(
        self,
        conn: AsyncpgConnection,
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
        return await shamir_setup(
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            ciphered_data,
            reveal_token,
            shamir_recovery_brief_certificate,
            shamir_recovery_share_certificates,
        )

    @override
    @transaction
    async def delete(
        self,
        conn: AsyncpgConnection,
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
        return await shamir_delete(
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            shamir_recovery_deletion_certificate,
        )
