# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections.abc import Buffer
from typing import override

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    DeviceID,
    GreetingAttemptID,
    InvitationStatus,
    InvitationToken,
    InvitationType,
    OrganizationID,
    UserID,
    UserProfile,
)
from parsec.components.events import EventBus
from parsec.components.invite import (
    BaseInviteComponent,
    DeviceInvitation,
    GreetingAttemptCancelledBadOutcome,
    Invitation,
    InvitationCreatedByUser,
    InviteAsInvitedInfoBadOutcome,
    InviteCancelBadOutcome,
    InviteClaimerCancelGreetingAttemptBadOutcome,
    InviteClaimerStartGreetingAttemptBadOutcome,
    InviteClaimerStepBadOutcome,
    InviteCompleteBadOutcome,
    InviteGreeterCancelGreetingAttemptBadOutcome,
    InviteGreeterStartGreetingAttemptBadOutcome,
    InviteGreeterStepBadOutcome,
    InviteListBadOutcome,
    InviteNewForDeviceBadOutcome,
    InviteNewForShamirRecoveryBadOutcome,
    InviteNewForUserBadOutcome,
    InviteShamirRecoveryRevealBadOutcome,
    NotReady,
    SendEmailBadOutcome,
    ShamirRecoveryInvitation,
    ShamirRecoveryRecipient,
    UserGreetingAdministrator,
    UserInvitation,
    UserOnlineStatus,
)
from parsec.components.memory.datamodel import (
    AdvisoryLock,
    MemoryDatamodel,
    MemoryInvitation,
    MemoryInvitationDeletedReason,
    MemoryOrganization,
    MemoryUser,
)
from parsec.config import BackendConfig
from parsec.events import (
    EventGreetingAttemptCancelled,
    EventGreetingAttemptJoined,
    EventGreetingAttemptReady,
    EventInvitation,
)


def _is_invitation_cancelled(org: MemoryOrganization, invitation: MemoryInvitation) -> bool:
    if invitation.is_cancelled:
        return True
    if invitation.type == InvitationType.SHAMIR_RECOVERY:
        assert invitation.claimer_user_id is not None
        assert invitation.shamir_recovery_index is not None
        shamir_recoveries = org.shamir_recoveries[invitation.claimer_user_id]
        shamir_recovery = shamir_recoveries[invitation.shamir_recovery_index]
        return shamir_recovery.is_deleted
    return False


class MemoryInviteComponent(BaseInviteComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        event_bus: EventBus,
        config: BackendConfig,
    ) -> None:
        super().__init__(config)
        self._event_bus = event_bus
        self._data = data

    def _get_shamir_recovery_invitation(
        self, org: MemoryOrganization, invitation: MemoryInvitation
    ) -> ShamirRecoveryInvitation:
        assert invitation.claimer_user_id is not None

        assert invitation.shamir_recovery_index is not None
        claimer_shamir_recoveries = org.shamir_recoveries[invitation.claimer_user_id]
        shamir_recovery = claimer_shamir_recoveries[invitation.shamir_recovery_index]

        threshold = shamir_recovery.cooked_brief.threshold
        par_recipient_shares = shamir_recovery.cooked_brief.per_recipient_shares
        recipients = [
            ShamirRecoveryRecipient(
                user_id=user_id,
                human_handle=org.users[user_id].cooked.human_handle,
                shares=shares,
                revoked_on=org.users[user_id].revoked_on,
                online_status=UserOnlineStatus.UNKNOWN,
            )
            for user_id, shares in par_recipient_shares.items()
        ]
        recipients.sort(key=lambda x: x.human_handle.label)
        claimer_human_handle = org.users[invitation.claimer_user_id].cooked.human_handle

        # Consider an active invitation as CANCELLED if the corresponding shamir recovery is deleted
        status = invitation.invitation_status
        if status == InvitationStatus.PENDING and shamir_recovery.is_deleted:
            status = InvitationStatus.CANCELLED

        return ShamirRecoveryInvitation(
            token=invitation.token,
            created_on=invitation.created_on,
            created_by=invitation.created_by,
            status=status,
            claimer_user_id=invitation.claimer_user_id,
            claimer_human_handle=claimer_human_handle,
            threshold=threshold,
            recipients=recipients,
            shamir_recovery_created_on=shamir_recovery.cooked_brief.timestamp,
            shamir_recovery_deleted_on=shamir_recovery.deleted_on,
        )

    def _get_user_greeting_administrators(
        self, org: MemoryOrganization, invitation: MemoryInvitation
    ) -> list[UserGreetingAdministrator]:
        user_id_to_last_greeter_joined = {}
        for user_id, session in invitation.greeting_sessions.items():
            for greeting_attempt_id in reversed(session.greeting_attempts):
                greeting_attempt = org.greeting_attempts[greeting_attempt_id]
                if greeting_attempt.greeter_joined is not None:
                    user_id_to_last_greeter_joined[user_id] = greeting_attempt.greeter_joined
                    break

        return sorted(
            (
                UserGreetingAdministrator(
                    user_id=user_id,
                    human_handle=user.cooked.human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=user_id_to_last_greeter_joined.get(user_id),
                )
                for user_id, user in org.users.items()
                if user.current_profile == UserProfile.ADMIN and not user.is_revoked
            ),
            key=lambda x: x.human_handle.label,
        )

    async def send_invitation_event(
        self, org: MemoryOrganization, invitation: MemoryInvitation
    ) -> None:
        if invitation.type == InvitationType.USER:
            # All non-revoked admins can greet a user invitation
            possible_greeters = {
                user_id
                for user_id, user in org.users.items()
                if user.current_profile == UserProfile.ADMIN and not user.is_revoked
            }
        elif invitation.type == InvitationType.DEVICE:
            assert invitation.claimer_user_id is not None
            # Only the corresponding user can greet a device invitation
            possible_greeters = {invitation.claimer_user_id}
        elif invitation.type == InvitationType.SHAMIR_RECOVERY:
            assert invitation.claimer_user_id is not None
            assert invitation.shamir_recovery_index is not None
            shamir_recoveries = org.shamir_recoveries[invitation.claimer_user_id]
            shamir_recovery = shamir_recoveries[invitation.shamir_recovery_index]
            # Only the non-revoked recipients can greet a shamir recovery invitation
            possible_greeters = {
                user_id for user_id in shamir_recovery.shares if not org.users[user_id].is_revoked
            }
        else:
            assert False, invitation.type
        await self._event_bus.send(
            EventInvitation(
                organization_id=org.organization_id,
                token=invitation.token,
                possible_greeters=possible_greeters,
                status=invitation.invitation_status,
            )
        )

    @override
    async def new_for_user(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        claimer_email: str,
        send_email: bool,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForUserBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteNewForUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteNewForUserBadOutcome.ORGANIZATION_EXPIRED

        async with (
            org.topics_lock(read=["common"]),
            org.advisory_lock_exclusive(AdvisoryLock.InvitationCreation),
        ):
            try:
                author_device = org.devices[author]
            except KeyError:
                return InviteNewForUserBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            try:
                author_user = org.users[author_user_id]
            except KeyError:
                return InviteNewForUserBadOutcome.AUTHOR_NOT_FOUND
            if author_user.is_revoked:
                return InviteNewForUserBadOutcome.AUTHOR_REVOKED

            if author_user.current_profile != UserProfile.ADMIN:
                return InviteNewForUserBadOutcome.AUTHOR_NOT_ALLOWED

            for user in org.users.values():
                if not user.is_revoked and user.cooked.human_handle.email == claimer_email:
                    return InviteNewForUserBadOutcome.CLAIMER_EMAIL_ALREADY_ENROLLED

            for invitation in org.invitations.values():
                if (
                    force_token is None
                    and not invitation.is_deleted
                    and invitation.type == InvitationType.USER
                    and invitation.claimer_email == claimer_email
                ):
                    # An invitation already exists for what the user has asked for
                    token = invitation.token
                    break

            else:
                # Must create a new invitation
                token = force_token or InvitationToken.new()
                created_by = InvitationCreatedByUser(
                    user_id=author_user_id,
                    human_handle=author_user.cooked.human_handle,
                )
                org.invitations[token] = invitation = MemoryInvitation(
                    token=token,
                    type=InvitationType.USER,
                    created_by=created_by,
                    claimer_email=claimer_email,
                    claimer_user_id=None,
                    shamir_recovery_index=None,
                    created_on=now,
                )

            await self.send_invitation_event(
                org=org,
                invitation=invitation,
            )

            if send_email:
                send_email_outcome = await self._send_user_invitation_email(
                    organization_id=organization_id,
                    claimer_email=claimer_email,
                    greeter_human_handle=author_user.cooked.human_handle,
                    token=token,
                )
            else:
                send_email_outcome = None

            return token, send_email_outcome

    @override
    async def new_for_device(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        send_email: bool,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForDeviceBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteNewForDeviceBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteNewForDeviceBadOutcome.ORGANIZATION_EXPIRED

        async with (
            org.topics_lock(read=["common"]),
            org.advisory_lock_exclusive(AdvisoryLock.InvitationCreation),
        ):
            try:
                author_device = org.devices[author]
            except KeyError:
                return InviteNewForDeviceBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            try:
                author_user = org.users[author_user_id]
            except KeyError:
                return InviteNewForDeviceBadOutcome.AUTHOR_NOT_FOUND
            if author_user.is_revoked:
                return InviteNewForDeviceBadOutcome.AUTHOR_REVOKED

            for invitation in org.invitations.values():
                if (
                    force_token is None
                    and not invitation.is_deleted
                    and invitation.type == InvitationType.DEVICE
                    and invitation.claimer_user_id == author_user_id
                ):
                    # An invitation already exists for what the user has asked for
                    token = invitation.token
                    break

            else:
                # Must create a new invitation
                token = force_token or InvitationToken.new()
                created_by = InvitationCreatedByUser(
                    user_id=author_user_id,
                    human_handle=author_user.cooked.human_handle,
                )
                org.invitations[token] = invitation = MemoryInvitation(
                    token=token,
                    type=InvitationType.DEVICE,
                    created_by=created_by,
                    claimer_user_id=author_user_id,
                    claimer_email=None,
                    shamir_recovery_index=None,
                    created_on=now,
                )

            await self.send_invitation_event(
                org=org,
                invitation=invitation,
            )

            if send_email:
                send_email_outcome = await self._send_device_invitation_email(
                    organization_id=organization_id,
                    email=author_user.cooked.human_handle.email,
                    token=token,
                )
            else:
                send_email_outcome = None

            return token, send_email_outcome

    @override
    async def new_for_shamir_recovery(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        send_email: bool,
        claimer_user_id: UserID,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForShamirRecoveryBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteNewForShamirRecoveryBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteNewForShamirRecoveryBadOutcome.ORGANIZATION_EXPIRED

        async with (
            org.topics_lock(read=["common"]),
            org.advisory_lock_exclusive(AdvisoryLock.InvitationCreation),
        ):
            try:
                author_device = org.devices[author]
            except KeyError:
                return InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            try:
                author_user = org.users[author_user_id]
            except KeyError:
                return InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_FOUND
            if author_user.is_revoked:
                return InviteNewForShamirRecoveryBadOutcome.AUTHOR_REVOKED

            # Check that the claimer exists
            claimer = org.users.get(claimer_user_id)
            if claimer is None:
                return InviteNewForShamirRecoveryBadOutcome.USER_NOT_FOUND
            claimer_human_handle = claimer.cooked.human_handle

            # Check that a non-deleted shamir setup exists
            claimer_shamir_recoveries = org.shamir_recoveries[claimer_user_id]
            last_shamir_recovery_index = len(claimer_shamir_recoveries) - 1
            try:
                # All but the last shamir recovery are deleted
                last_shamir_recovery = claimer_shamir_recoveries[last_shamir_recovery_index]
            except IndexError:
                last_shamir_recovery = None
            if last_shamir_recovery is None or last_shamir_recovery.is_deleted:
                # Since the author only knows about a shamir recovery if they are part of it,
                # we don't have a specific error for the case where the shamir setup doesn't exist
                return InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_ALLOWED

            # Author is not part of the recipients
            if author_user_id not in last_shamir_recovery.shares:
                return InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_ALLOWED

            for invitation in org.invitations.values():
                if (
                    force_token is None
                    and not invitation.is_deleted
                    and invitation.type == InvitationType.SHAMIR_RECOVERY
                    and invitation.claimer_user_id == claimer_user_id
                ):
                    # An invitation already exists for what the user has asked for
                    token = invitation.token
                    break

            else:
                # Must create a new invitation

                token = force_token or InvitationToken.new()
                created_by = InvitationCreatedByUser(
                    user_id=author_user_id,
                    human_handle=author_user.cooked.human_handle,
                )
                org.invitations[token] = invitation = MemoryInvitation(
                    token=token,
                    type=InvitationType.SHAMIR_RECOVERY,
                    created_by=created_by,
                    claimer_email=claimer_human_handle.email,
                    created_on=now,
                    claimer_user_id=claimer_user_id,
                    shamir_recovery_index=last_shamir_recovery_index,
                )

            await self.send_invitation_event(
                org=org,
                invitation=invitation,
            )

            if send_email:
                send_email_outcome = await self._send_shamir_recovery_invitation_email(
                    organization_id=organization_id,
                    email=claimer_human_handle.email,
                    token=token,
                    greeter_human_handle=author_user.cooked.human_handle,
                )
            else:
                send_email_outcome = None

            return token, send_email_outcome

    @override
    async def cancel(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        token: InvitationToken,
    ) -> None | InviteCancelBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteCancelBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteCancelBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return InviteCancelBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        try:
            author_user = org.users[author_user_id]
        except KeyError:
            return InviteCancelBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return InviteCancelBadOutcome.AUTHOR_REVOKED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteCancelBadOutcome.INVITATION_NOT_FOUND

        if _is_invitation_cancelled(org, invitation):
            return InviteCancelBadOutcome.INVITATION_ALREADY_CANCELLED
        if invitation.is_completed:
            return InviteCancelBadOutcome.INVITATION_COMPLETED

        # Only the greeter or the claimer can cancel the invitation
        if not self.is_greeter_allowed(org, invitation, author_user):
            if invitation.type == InvitationType.USER:
                assert invitation.claimer_email is not None
                if invitation.claimer_email != author_user.cooked.human_handle.email:
                    return InviteCancelBadOutcome.AUTHOR_NOT_ALLOWED
            elif invitation.type in (InvitationType.DEVICE, InvitationType.SHAMIR_RECOVERY):
                assert invitation.claimer_user_id is not None
                if invitation.claimer_user_id != author_user.cooked.user_id:
                    return InviteCancelBadOutcome.AUTHOR_NOT_ALLOWED
            else:
                assert False, invitation.type

        invitation.deleted_on = now
        invitation.deleted_reason = MemoryInvitationDeletedReason.CANCELLED

        await self.send_invitation_event(
            org=org,
            invitation=invitation,
        )

    @override
    async def list(
        self, organization_id: OrganizationID, author: DeviceID
    ) -> list[Invitation] | InviteListBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteListBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteListBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return InviteListBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        try:
            author_user = org.users[author_user_id]
        except KeyError:
            return InviteListBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return InviteListBadOutcome.AUTHOR_REVOKED

        items = []
        for invitation in org.invitations.values():
            match invitation.type:
                case InvitationType.USER:
                    if author_user.current_profile != UserProfile.ADMIN:
                        continue
                    assert invitation.claimer_email is not None
                    item = UserInvitation(
                        claimer_email=invitation.claimer_email,
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by=invitation.created_by,
                        administrators=self._get_user_greeting_administrators(org, invitation),
                        status=invitation.invitation_status,
                    )
                case InvitationType.DEVICE:
                    if invitation.claimer_user_id != author_user_id:
                        continue
                    assert invitation.claimer_user_id is not None
                    item = DeviceInvitation(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by=invitation.created_by,
                        claimer_user_id=invitation.claimer_user_id,
                        claimer_human_handle=org.users[
                            invitation.claimer_user_id
                        ].cooked.human_handle,
                        status=invitation.invitation_status,
                    )
                case InvitationType.SHAMIR_RECOVERY:
                    shamir_recovery_invitation = self._get_shamir_recovery_invitation(
                        org, invitation
                    )
                    # The author is not part of the recipients
                    if not any(
                        recipient.user_id == author_user_id
                        for recipient in shamir_recovery_invitation.recipients
                    ):
                        continue
                    item = shamir_recovery_invitation
                case unknown:
                    # TODO: find a way to type `InvitationType` as a proper enum
                    # so that we can use `assert_never` here
                    assert False, unknown
            items.append(item)

        return sorted(items, key=lambda x: x.created_on)

    @override
    async def info_as_invited(
        self, organization_id: OrganizationID, token: InvitationToken
    ) -> Invitation | InviteAsInvitedInfoBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteAsInvitedInfoBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteAsInvitedInfoBadOutcome.ORGANIZATION_EXPIRED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteAsInvitedInfoBadOutcome.INVITATION_NOT_FOUND
        if invitation.is_deleted:
            return InviteAsInvitedInfoBadOutcome.INVITATION_DELETED

        match invitation.type:
            case InvitationType.USER:
                assert invitation.claimer_email is not None
                return UserInvitation(
                    claimer_email=invitation.claimer_email,
                    created_on=invitation.created_on,
                    status=invitation.invitation_status,
                    created_by=invitation.created_by,
                    token=invitation.token,
                    administrators=self._get_user_greeting_administrators(org, invitation),
                )
            case InvitationType.DEVICE:
                assert invitation.claimer_user_id is not None
                return DeviceInvitation(
                    created_on=invitation.created_on,
                    status=invitation.invitation_status,
                    created_by=invitation.created_by,
                    claimer_user_id=invitation.claimer_user_id,
                    claimer_human_handle=org.users[invitation.claimer_user_id].cooked.human_handle,
                    token=invitation.token,
                )
            case InvitationType.SHAMIR_RECOVERY:
                shamir_recovery_invitation = self._get_shamir_recovery_invitation(org, invitation)
                # Treat deleted shamir recovery as deleted invitation
                if shamir_recovery_invitation.shamir_recovery_is_deleted:
                    return InviteAsInvitedInfoBadOutcome.INVITATION_DELETED
                return shamir_recovery_invitation
            case unknown:
                assert False, unknown

    @override
    async def shamir_recovery_reveal(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        reveal_token: InvitationToken,
    ) -> bytes | InviteShamirRecoveryRevealBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteShamirRecoveryRevealBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteShamirRecoveryRevealBadOutcome.ORGANIZATION_EXPIRED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteShamirRecoveryRevealBadOutcome.INVITATION_NOT_FOUND
        if invitation.is_deleted:
            return InviteShamirRecoveryRevealBadOutcome.INVITATION_DELETED

        if invitation.type != InvitationType.SHAMIR_RECOVERY:
            return InviteShamirRecoveryRevealBadOutcome.BAD_INVITATION_TYPE
        assert invitation.claimer_user_id is not None

        # Failing this assert means that some data has been corrupted,
        # since there needs to be a valid shamir setup in order to create
        # a shamir recovery invitation
        assert invitation.claimer_user_id in org.shamir_recoveries
        shamir_recoveries = org.shamir_recoveries[invitation.claimer_user_id]
        if not shamir_recoveries:
            return InviteShamirRecoveryRevealBadOutcome.BAD_REVEAL_TOKEN

        *_, shamir_recovery = shamir_recoveries
        if shamir_recovery.reveal_token != reveal_token:
            return InviteShamirRecoveryRevealBadOutcome.BAD_REVEAL_TOKEN

        return shamir_recovery.ciphered_data

    @override
    async def test_dump_all_invitations(
        self, organization_id: OrganizationID
    ) -> dict[UserID, list[Invitation]]:
        org = self._data.organizations[organization_id]
        per_user_invitations = {}
        for invitation in org.invitations.values():
            # TODO: Update method to also return invitation created by external services
            if not isinstance(invitation.created_by, InvitationCreatedByUser):
                continue
            try:
                current_user_invitations = per_user_invitations[invitation.created_by.user_id]
            except KeyError:
                current_user_invitations = []
                per_user_invitations[invitation.created_by.user_id] = current_user_invitations

            match invitation.type:
                case InvitationType.USER:
                    assert invitation.claimer_email is not None
                    current_user_invitations.append(
                        UserInvitation(
                            claimer_email=invitation.claimer_email,
                            created_on=invitation.created_on,
                            status=invitation.invitation_status,
                            created_by=invitation.created_by,
                            token=invitation.token,
                            administrators=self._get_user_greeting_administrators(org, invitation),
                        )
                    )
                case InvitationType.DEVICE:
                    assert invitation.claimer_user_id is not None
                    current_user_invitations.append(
                        DeviceInvitation(
                            created_on=invitation.created_on,
                            status=invitation.invitation_status,
                            created_by=invitation.created_by,
                            token=invitation.token,
                            claimer_user_id=invitation.claimer_user_id,
                            claimer_human_handle=org.users[
                                invitation.claimer_user_id
                            ].cooked.human_handle,
                        )
                    )
                case InvitationType.SHAMIR_RECOVERY:
                    shamir_recovery_invitation = self._get_shamir_recovery_invitation(
                        org, invitation
                    )
                    current_user_invitations.append(shamir_recovery_invitation)
                case unknown:
                    assert False, unknown

        return per_user_invitations

    # New invite transport API

    def is_greeter_allowed(
        self, org: MemoryOrganization, invitation: MemoryInvitation, greeter: MemoryUser
    ) -> bool:
        if invitation.type == InvitationType.DEVICE:
            assert invitation.claimer_user_id is not None
            return invitation.claimer_user_id == greeter.cooked.user_id
        elif invitation.type == InvitationType.USER:
            return greeter.current_profile == UserProfile.ADMIN
        elif invitation.type == InvitationType.SHAMIR_RECOVERY:
            assert invitation.claimer_user_id is not None
            assert invitation.shamir_recovery_index is not None
            shamir_recoveries = org.shamir_recoveries[invitation.claimer_user_id]
            shamir_recovery = shamir_recoveries[invitation.shamir_recovery_index]
            return shamir_recovery.shares.get(greeter.cooked.user_id) is not None
        else:
            assert False, invitation.type

    @override
    async def greeter_start_greeting_attempt(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        token: InvitationToken,
    ) -> GreetingAttemptID | InviteGreeterStartGreetingAttemptBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteGreeterStartGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteGreeterStartGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED

        try:
            greeter_device = org.devices[author]
            assert greeter_device.cooked.user_id == greeter
            greeter_user = org.users[greeter]
        except KeyError:
            return InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_NOT_FOUND

        if greeter_user.is_revoked:
            return InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_REVOKED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_NOT_FOUND
        if invitation.is_completed:
            return InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_COMPLETED
        if _is_invitation_cancelled(org, invitation):
            return InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(org, invitation, greeter_user):
            return InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_NOT_ALLOWED

        greeting_session = invitation.get_greeting_session(greeter)
        greeting_attempt = greeting_session.new_attempt_for_greeter(org, now)
        return greeting_attempt.greeting_attempt

    @override
    async def claimer_start_greeting_attempt(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeter: UserID,
    ) -> GreetingAttemptID | InviteClaimerStartGreetingAttemptBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteClaimerStartGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteClaimerStartGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED

        try:
            greeter_user = org.users[greeter]
        except KeyError:
            return InviteClaimerStartGreetingAttemptBadOutcome.GREETER_NOT_FOUND

        if greeter_user.is_revoked:
            return InviteClaimerStartGreetingAttemptBadOutcome.GREETER_REVOKED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_NOT_FOUND
        if invitation.is_completed:
            return InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_COMPLETED
        if _is_invitation_cancelled(org, invitation):
            return InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(org, invitation, greeter_user):
            return InviteClaimerStartGreetingAttemptBadOutcome.GREETER_NOT_ALLOWED

        greeting_session = invitation.get_greeting_session(greeter)
        greeting_attempt = greeting_session.new_attempt_for_claimer(org, now)
        return greeting_attempt.greeting_attempt

    @override
    async def greeter_cancel_greeting_attempt(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        greeting_attempt: GreetingAttemptID,
        reason: CancelledGreetingAttemptReason,
    ) -> None | InviteGreeterCancelGreetingAttemptBadOutcome | GreetingAttemptCancelledBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteGreeterCancelGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteGreeterCancelGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED

        try:
            greeter_device = org.devices[author]
            assert greeter_device.cooked.user_id == greeter
            greeter_user = org.users[greeter]
        except KeyError:
            return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_FOUND

        if greeter_user.is_revoked:
            return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_REVOKED

        try:
            attempt = org.greeting_attempts[greeting_attempt]
            invitation = org.invitations[attempt.token]
        except KeyError:
            return InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND
        if attempt.greeter_id != greeter:
            return InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND

        if invitation.is_completed:
            return InviteGreeterCancelGreetingAttemptBadOutcome.INVITATION_COMPLETED
        if _is_invitation_cancelled(org, invitation):
            return InviteGreeterCancelGreetingAttemptBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(org, invitation, greeter_user):
            return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_ALLOWED

        if attempt.cancelled_reason is not None:
            return GreetingAttemptCancelledBadOutcome(*attempt.cancelled_reason)
        if attempt.greeter_joined is None:
            return InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        attempt.greeter_cancel(now, reason)

        await self._event_bus.send(
            EventGreetingAttemptCancelled(
                organization_id=organization_id,
                token=invitation.token,
                greeter=greeter,
                greeting_attempt=greeting_attempt,
            )
        )

    @override
    async def claimer_cancel_greeting_attempt(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeting_attempt: GreetingAttemptID,
        reason: CancelledGreetingAttemptReason,
    ) -> None | InviteClaimerCancelGreetingAttemptBadOutcome | GreetingAttemptCancelledBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteClaimerCancelGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteClaimerCancelGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED

        try:
            attempt = org.greeting_attempts[greeting_attempt]
            invitation = org.invitations[attempt.token]
        except KeyError:
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND
        if attempt.token != token:
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND

        greeter_user = org.users[attempt.greeter_id]
        if greeter_user.is_revoked:
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETER_REVOKED

        if invitation.is_completed:
            return InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_COMPLETED
        if _is_invitation_cancelled(org, invitation):
            return InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(org, invitation, greeter_user):
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETER_NOT_ALLOWED

        if attempt.cancelled_reason is not None:
            return GreetingAttemptCancelledBadOutcome(*attempt.cancelled_reason)
        if attempt.claimer_joined is None:
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        attempt.claimer_cancel(now, reason)

        await self._event_bus.send(
            EventGreetingAttemptCancelled(
                organization_id=organization_id,
                token=invitation.token,
                greeter=attempt.greeter_id,
                greeting_attempt=greeting_attempt,
            )
        )

    @override
    async def greeter_step(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        greeting_attempt: GreetingAttemptID,
        step_index: int,
        greeter_data: bytes,
    ) -> bytes | NotReady | InviteGreeterStepBadOutcome | GreetingAttemptCancelledBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteGreeterStepBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteGreeterStepBadOutcome.ORGANIZATION_EXPIRED

        try:
            greeter_device = org.devices[author]
            assert greeter_device.cooked.user_id == greeter
            greeter_user = org.users[greeter]
        except KeyError:
            return InviteGreeterStepBadOutcome.AUTHOR_NOT_FOUND

        if greeter_user.is_revoked:
            return InviteGreeterStepBadOutcome.AUTHOR_REVOKED

        try:
            attempt = org.greeting_attempts[greeting_attempt]
            invitation = org.invitations[attempt.token]
        except KeyError:
            return InviteGreeterStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND
        if attempt.greeter_id != greeter:
            return InviteGreeterStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND

        if invitation.is_completed:
            return InviteGreeterStepBadOutcome.INVITATION_COMPLETED
        if _is_invitation_cancelled(org, invitation):
            return InviteGreeterStepBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(org, invitation, greeter_user):
            return InviteGreeterStepBadOutcome.AUTHOR_NOT_ALLOWED

        if attempt.cancelled_reason is not None:
            return GreetingAttemptCancelledBadOutcome(*attempt.cancelled_reason)
        if attempt.greeter_joined is None:
            return InviteGreeterStepBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        match attempt.greeter_step(step_index, greeter_data):
            case attempt.StepOutcome.MISMATCH:
                return InviteGreeterStepBadOutcome.STEP_MISMATCH
            case attempt.StepOutcome.TOO_ADVANCED:
                return InviteGreeterStepBadOutcome.STEP_TOO_ADVANCED
            case attempt.StepOutcome.NOT_READY:
                return NotReady()
            case Buffer() as data:
                # When completing the `WAIT_PEER` step, send a `GreetingAttemptJoined` event
                if step_index == 0:
                    await self._event_bus.send(
                        EventGreetingAttemptJoined(
                            organization_id=org.organization_id,
                            token=invitation.token,
                            greeter=attempt.greeter_id,
                            greeting_attempt=greeting_attempt,
                        )
                    )
                return data

    @override
    async def claimer_step(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeting_attempt: GreetingAttemptID,
        step_index: int,
        claimer_data: bytes,
    ) -> bytes | NotReady | InviteClaimerStepBadOutcome | GreetingAttemptCancelledBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteClaimerStepBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteClaimerStepBadOutcome.ORGANIZATION_EXPIRED

        try:
            attempt = org.greeting_attempts[greeting_attempt]
            invitation = org.invitations[attempt.token]
        except KeyError:
            return InviteClaimerStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND
        if attempt.token != token:
            return InviteClaimerStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND

        greeter_user = org.users[attempt.greeter_id]
        if greeter_user.is_revoked:
            return InviteClaimerStepBadOutcome.GREETER_REVOKED

        if invitation.is_completed:
            return InviteClaimerStepBadOutcome.INVITATION_COMPLETED
        if _is_invitation_cancelled(org, invitation):
            return InviteClaimerStepBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(org, invitation, greeter_user):
            return InviteClaimerStepBadOutcome.GREETER_NOT_ALLOWED

        if attempt.cancelled_reason is not None:
            return GreetingAttemptCancelledBadOutcome(*attempt.cancelled_reason)
        if attempt.claimer_joined is None:
            return InviteClaimerStepBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        match attempt.claimer_step(step_index, claimer_data):
            case attempt.StepOutcome.MISMATCH:
                return InviteClaimerStepBadOutcome.STEP_MISMATCH
            case attempt.StepOutcome.TOO_ADVANCED:
                return InviteClaimerStepBadOutcome.STEP_TOO_ADVANCED
            case attempt.StepOutcome.NOT_READY:
                # During the `WAIT_PEER` step, send a `GreetingAttemptReady` event to the greeter
                if step_index == 0:
                    await self._event_bus.send(
                        EventGreetingAttemptReady(
                            organization_id=org.organization_id,
                            token=invitation.token,
                            greeter=attempt.greeter_id,
                            greeting_attempt=greeting_attempt,
                        )
                    )
                return NotReady()
            case Buffer() as data:
                return data

    @override
    async def complete(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        token: InvitationToken,
    ) -> None | InviteCompleteBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteCompleteBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteCompleteBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return InviteCompleteBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        try:
            author_user = org.users[author_user_id]
        except KeyError:
            return InviteCompleteBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return InviteCompleteBadOutcome.AUTHOR_REVOKED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteCompleteBadOutcome.INVITATION_NOT_FOUND
        if _is_invitation_cancelled(org, invitation):
            return InviteCompleteBadOutcome.INVITATION_CANCELLED
        if invitation.is_completed:
            return InviteCompleteBadOutcome.INVITATION_ALREADY_COMPLETED

        # Only the greeter or the claimer can complete the invitation
        if not self.is_greeter_allowed(org, invitation, author_user):
            if invitation.type == InvitationType.USER:
                assert invitation.claimer_email is not None
                if invitation.claimer_email != author_user.cooked.human_handle.email:
                    return InviteCompleteBadOutcome.AUTHOR_NOT_ALLOWED
            elif invitation.type in (InvitationType.DEVICE, InvitationType.SHAMIR_RECOVERY):
                assert invitation.claimer_user_id is not None
                if invitation.claimer_user_id != author_user.cooked.user_id:
                    return InviteCompleteBadOutcome.AUTHOR_NOT_ALLOWED
            else:
                assert False, invitation.type

        invitation.deleted_on = now
        invitation.deleted_reason = MemoryInvitationDeletedReason.FINISHED

        await self.send_invitation_event(
            org=org,
            invitation=invitation,
        )
