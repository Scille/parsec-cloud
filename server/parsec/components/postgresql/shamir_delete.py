# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    ShamirRecoveryDeletionCertificate,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.shamir import (
    ShamirDeleteSetupAlreadyDeletedBadOutcome,
    ShamirDeleteStoreBadOutcome,
    ShamirDeleteValidateBadOutcome,
)


async def delete(
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
    raise NotImplementedError
