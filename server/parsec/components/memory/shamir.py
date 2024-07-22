# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from parsec._parsec import DeviceID, OrganizationID, UserID, VerifyKey, authenticated_cmds
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

    async def organization_and_user_common_checks(
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
        match await self.organization_and_user_common_checks(organization_id, author):
            case (_, _):
                pass
            case error:
                return error

        self._data.organizations[organization_id].shamir_setup.pop(author)
        return None

    @override
    async def add_recovery_setup(
        self,
        organization_id: OrganizationID,
        author: UserID,
        device: DeviceID,
        author_verify_key: VerifyKey,
        setup: authenticated_cmds.latest.shamir_recovery_setup.ShamirRecoverySetup,
    ) -> (
        None
        | ShamirAddOrDeleteRecoverySetupStoreBadOutcome
        | ShamirAddRecoverySetupValidateBadOutcome
        | ShamirSetupAlreadyExistsBadOutcome
        | TimestampOutOfBallpark
        | ShamirInvalidRecipientBadOutcome
        | RequireGreaterTimestamp
    ):
        match await self.organization_and_user_common_checks(organization_id, author):
            case (org, _):
                pass
            case error:
                return error

        match shamir_add_recovery_setup_validate(setup, device, author, author_verify_key):
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

        # check that certificate timestamps are strictly increasing in the shamir topic
        match self._data.organizations[organization_id].last_shamir_certificate_timestamp:
            case None:
                # no previous shamir setup
                pass
            case last_shamir_timestamp if last_shamir_timestamp < brief.timestamp:
                # previous shamir happened strictly before
                pass
            case last_shamir_timestamp:
                return RequireGreaterTimestamp(last_shamir_timestamp)

        match self._data.organizations[organization_id].shamir_setup.get(author):
            case None:
                self._data.organizations[organization_id].shamir_setup[author] = MemoryShamirSetup(
                    setup.ciphered_data, setup.reveal_token, brief, shares, setup.brief
                )
            case MemoryShamirSetup() as s:
                return ShamirSetupAlreadyExistsBadOutcome(s.brief.timestamp)
