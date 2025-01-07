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
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemoryShamirRecovery,
    MemoryShamirShare,
)
from parsec.components.shamir import (
    BaseShamirComponent,
    ShamirDeleteSetupAlreadyDeletedBadOutcome,
    ShamirDeleteStoreBadOutcome,
    ShamirDeleteValidateBadOutcome,
    ShamirSetupAlreadyExistsBadOutcome,
    ShamirSetupRevokedRecipientBadOutcome,
    ShamirSetupStoreBadOutcome,
    ShamirSetupValidateBadOutcome,
    shamir_delete_validate,
    shamir_setup_validate,
)


class MemoryShamirComponent(BaseShamirComponent):
    def __init__(self, data: MemoryDatamodel, event_bus: EventBus) -> None:
        super().__init__()
        self._data = data
        self._event_bus = event_bus

    @override
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return ShamirSetupStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return ShamirSetupStoreBadOutcome.ORGANIZATION_EXPIRED

        try:
            device = org.devices[author]
            author_user_id = device.cooked.user_id
            _ = org.users[author_user_id]
        except KeyError:
            return ShamirSetupStoreBadOutcome.AUTHOR_NOT_FOUND

        match shamir_setup_validate(
            now,
            author,
            author_user_id,
            author_verify_key,
            shamir_recovery_brief_certificate,
            shamir_recovery_share_certificates,
        ):
            case (cooked_brief, cooked_shares):
                pass
            case error:
                return error

        async with org.topics_lock(read=["common"], write=["shamir_recovery"]) as (
            common_topic_last_timestamp,
            shamir_topic_last_timestamp,
        ):
            # Ensure all recipients exist and are not revoked
            for share_recipient in cooked_shares.keys():
                try:
                    recipient_user = org.users[share_recipient]
                except KeyError:
                    return ShamirSetupStoreBadOutcome.RECIPIENT_NOT_FOUND
                if recipient_user.is_revoked:
                    return ShamirSetupRevokedRecipientBadOutcome(
                        last_common_certificate_timestamp=common_topic_last_timestamp
                    )

            try:
                previous_shamir_setup = org.shamir_recoveries[author_user_id][-1]
            except IndexError:
                # The user never had a shamir recovery
                previous_shamir_setup = None
            else:
                # The user had already setup a shamir recovery... but it might be deleted
                if not previous_shamir_setup.is_deleted:
                    return ShamirSetupAlreadyExistsBadOutcome(
                        last_shamir_recovery_certificate_timestamp=shamir_topic_last_timestamp
                    )

            # Ensure we are not breaking causality by adding a newer timestamp.

            last_certificate = max(
                common_topic_last_timestamp,
                shamir_topic_last_timestamp,
            )
            if last_certificate >= cooked_brief.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

            # All checks are good, now we do the actual insertion

            org.per_topic_last_timestamp["shamir_recovery"] = cooked_brief.timestamp

            org.shamir_recoveries[author_user_id].append(
                MemoryShamirRecovery(
                    ciphered_data=ciphered_data,
                    reveal_token=reveal_token,
                    cooked_brief=cooked_brief,
                    shamir_recovery_brief_certificate=shamir_recovery_brief_certificate,
                    shares={
                        recipient: MemoryShamirShare(
                            cooked=cooked,
                            shamir_recovery_share_certificates=raw,
                        )
                        for recipient, (raw, cooked) in cooked_shares.items()
                    },
                )
            )

        return cooked_brief

    @override
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return ShamirDeleteStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return ShamirDeleteStoreBadOutcome.ORGANIZATION_EXPIRED

        try:
            device = org.devices[author]
            author_user_id = device.cooked.user_id
            _ = org.users[author_user_id]
        except KeyError:
            return ShamirDeleteStoreBadOutcome.AUTHOR_NOT_FOUND

        match shamir_delete_validate(
            now, author, author_user_id, author_verify_key, shamir_recovery_deletion_certificate
        ):
            case ShamirRecoveryDeletionCertificate() as cooked_deletion:
                pass
            case error:
                return error

        async with org.topics_lock(read=["common"], write=["shamir_recovery"]) as (
            common_topic_last_timestamp,
            last_shamir_certificate_timestamp,
        ):
            # Ensure we are not breaking causality by adding a newer timestamp.

            last_certificate = max(
                common_topic_last_timestamp,
                last_shamir_certificate_timestamp,
            )
            if last_certificate >= cooked_deletion.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

            # Find the shamir recovery setup certificate

            for setup in org.shamir_recoveries[author_user_id]:
                if setup.cooked_brief.timestamp == cooked_deletion.setup_to_delete_timestamp:
                    break
            else:
                return ShamirDeleteStoreBadOutcome.SETUP_NOT_FOUND

            if setup.shares.keys() != cooked_deletion.share_recipients:
                return ShamirDeleteStoreBadOutcome.RECIPIENTS_MISMATCH

            if setup.is_deleted:
                return ShamirDeleteSetupAlreadyDeletedBadOutcome(
                    last_shamir_recovery_certificate_timestamp=last_shamir_certificate_timestamp
                )

            # All checks are good, now we do the actual insertion

            org.per_topic_last_timestamp["shamir_recovery"] = cooked_deletion.timestamp

            setup.cooked_deletion = cooked_deletion
            setup.shamir_recovery_deletion_certificate = shamir_recovery_deletion_certificate

            return cooked_deletion
