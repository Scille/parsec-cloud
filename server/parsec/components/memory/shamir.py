# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from parsec._parsec import DateTime, DeviceID, InvitationToken, OrganizationID, UserID, VerifyKey
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemoryOrganization,
    MemoryShamirSetup,
    MemoryUser,
)
from parsec.components.shamir import (
    BaseShamirComponent,
    ShamirAddOrDeleteRecoverySetupStoreBadOutcome,
    ShamirAddRecoverySetupValidateBadOutcome,
    ShamirInvalidRecipientBadOutcome,
    ShamirSetupAlreadyExistsBadOutcome,
    shamir_add_recovery_setup_validate,
)


class MemoryShamirComponent(BaseShamirComponent):
    def __init__(self, data: MemoryDatamodel, event_bus: EventBus) -> None:
        super().__init__()
        self._data = data
        self._event_bus = event_bus

    def organization_and_user_common_checks(
        self,
        organization_id: OrganizationID,
        author: UserID,
    ) -> tuple[MemoryOrganization, MemoryUser] | ShamirAddOrDeleteRecoverySetupStoreBadOutcome:
        # organization exists and not expired
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return ShamirAddOrDeleteRecoverySetupStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return ShamirAddOrDeleteRecoverySetupStoreBadOutcome.ORGANIZATION_EXPIRED

        # author exists and not revoked and not frozen
        try:
            author_user = org.users[author]
        except KeyError:
            return ShamirAddOrDeleteRecoverySetupStoreBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return ShamirAddOrDeleteRecoverySetupStoreBadOutcome.AUTHOR_REVOKED

        return (org, author_user)

    @override
    async def remove_recovery_setup(
        self,
        organization_id: OrganizationID,
        author: UserID,
    ) -> None | ShamirAddOrDeleteRecoverySetupStoreBadOutcome:
        # TODO update after https://github.com/Scille/parsec-cloud/issues/7364
        match self.organization_and_user_common_checks(organization_id, author):
            case (org, _):
                pass
            case error:
                return error

        async with org.topics_lock(read=["common"], write=["shamir_recovery"]):
            self._data.organizations[organization_id].shamir_setup.pop(author)
            return None

    @override
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
        | TimestampOutOfBallpark
        | ShamirInvalidRecipientBadOutcome
        | RequireGreaterTimestamp
    ):
        match self.organization_and_user_common_checks(organization_id, author):
            case (org, _):
                pass
            case error:
                return error

        async with org.topics_lock(read=["common"], write=["shamir_recovery"]) as (
            common_topic_last_timestamp,
            shamir_topic_last_timestamp,
        ):
            match shamir_add_recovery_setup_validate(
                now, device, author, author_verify_key, setup_brief, setup_shares
            ):
                case (brief, shares):
                    pass
                case error:
                    return error

            # all recipients exists and not revoked
            for share_recipient in shares.keys():
                try:
                    recipient_user = org.users[share_recipient]
                except KeyError:
                    return ShamirInvalidRecipientBadOutcome(share_recipient)
                if recipient_user.is_revoked:
                    return ShamirInvalidRecipientBadOutcome(share_recipient)

            if author in self._data.organizations[organization_id].shamir_setup:
                return ShamirSetupAlreadyExistsBadOutcome(
                    last_shamir_certificate_timestamp=shamir_topic_last_timestamp
                )

            # Ensure we are not breaking causality by adding a newer timestamp.

            last_certificate = max(common_topic_last_timestamp, shamir_topic_last_timestamp)
            if last_certificate >= brief.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

            # All checks are good, now we do the actual insertion

            org.per_topic_last_timestamp["shamir_recovery"] = brief.timestamp

            self._data.organizations[organization_id].shamir_setup[author] = MemoryShamirSetup(
                setup_ciphered_data, setup_reveal_token, brief, shares, setup_brief
            )
