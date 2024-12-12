# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections.abc import Buffer
from typing import Any, override

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
from parsec.components.invite import (
    BaseInviteComponent,
    DeviceInvitation,
    GreetingAttemptCancelledBadOutcome,
    Invitation,
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
    InviteNewForShamirBadOutcome,
    InviteNewForUserBadOutcome,
    NotReady,
    SendEmailBadOutcome,
    ShamirRecoveryInvitation,
    ShamirRecoveryRecipient,
    UserInvitation,
)
from parsec.components.memory.datamodel import (
    AdvisoryLock,
    MemoryDatamodel,
    MemoryInvitation,
    MemoryInvitationDeletedReason,
    MemoryOrganization,
    MemoryUser,
)
from parsec.events import EventInvitation


class MemoryInviteComponent(BaseInviteComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        *args: Any,
        **kwargs: Any,
    ) -> None:
        super().__init__(*args, **kwargs)
        self._data = data

    def _get_invitation_status(
        self, organization_id: OrganizationID, invitation: MemoryInvitation
    ) -> InvitationStatus:
        if invitation.deleted_reason:
            match invitation.deleted_reason:
                case MemoryInvitationDeletedReason.CANCELLED:
                    return InvitationStatus.CANCELLED
                case MemoryInvitationDeletedReason.FINISHED:
                    return InvitationStatus.FINISHED

        elif invitation.token in self._claimers_ready[organization_id]:
            return InvitationStatus.READY
        else:
            return InvitationStatus.IDLE

    def _get_shamir_recovery_invitation(
        self, org: MemoryOrganization, invitation: MemoryInvitation
    ) -> ShamirRecoveryInvitation | None:
        assert invitation.claimer_user_id is not None

        claimer_shamir_recoveries = org.shamir_recoveries[invitation.claimer_user_id]
        try:
            # All but the last shamir recovery are deleted
            last_shamir_recovery = claimer_shamir_recoveries[-1]
        except IndexError:
            last_shamir_recovery = None
        if last_shamir_recovery is None or last_shamir_recovery.is_deleted:
            return None

        threshold = last_shamir_recovery.cooked_brief.threshold
        par_recipient_shares = last_shamir_recovery.cooked_brief.per_recipient_shares
        recipients = [
            ShamirRecoveryRecipient(
                user_id=user_id,
                human_handle=org.users[user_id].cooked.human_handle,
                shares=shares,
            )
            for user_id, shares in par_recipient_shares.items()
        ]
        recipients.sort(key=lambda x: x.human_handle.label)
        status = self._get_invitation_status(org.organization_id, invitation)
        created_by_human_handle = org.users[invitation.created_by_user_id].cooked.human_handle
        claimer_human_handle = org.users[invitation.claimer_user_id].cooked.human_handle
        return ShamirRecoveryInvitation(
            token=invitation.token,
            created_on=invitation.created_on,
            created_by_device_id=invitation.created_by_device_id,
            created_by_user_id=invitation.created_by_user_id,
            created_by_human_handle=created_by_human_handle,
            status=status,
            claimer_user_id=invitation.claimer_user_id,
            claimer_human_handle=claimer_human_handle,
            threshold=threshold,
            recipients=recipients,
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
                    and invitation.created_by_user_id == author_user_id
                    and invitation.claimer_email == claimer_email
                ):
                    # An invitation already exists for what the user has asked for
                    token = invitation.token
                    break

            else:
                # Must create a new invitation

                token = force_token or InvitationToken.new()
                org.invitations[token] = MemoryInvitation(
                    token=token,
                    type=InvitationType.USER,
                    created_by_user_id=author_user_id,
                    created_by_device_id=author,
                    claimer_email=claimer_email,
                    claimer_user_id=None,
                    created_on=now,
                )

                await self._event_bus.send(
                    EventInvitation(
                        organization_id=organization_id,
                        token=token,
                        greeter=author_user_id,
                        status=InvitationStatus.IDLE,
                    )
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
                    and invitation.created_by_user_id == author_user_id
                ):
                    # An invitation already exists for what the user has asked for
                    token = invitation.token
                    break

            else:
                # Must create a new invitation

                token = force_token or InvitationToken.new()
                org.invitations[token] = MemoryInvitation(
                    token=token,
                    type=InvitationType.DEVICE,
                    created_by_user_id=author_user_id,
                    created_by_device_id=author,
                    claimer_user_id=author_user_id,
                    claimer_email=None,
                    created_on=now,
                )

                await self._event_bus.send(
                    EventInvitation(
                        organization_id=organization_id,
                        token=token,
                        greeter=author_user_id,
                        status=InvitationStatus.IDLE,
                    )
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
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForShamirBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteNewForShamirBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteNewForShamirBadOutcome.ORGANIZATION_EXPIRED

        async with (
            org.topics_lock(read=["common"]),
            org.advisory_lock_exclusive(AdvisoryLock.InvitationCreation),
        ):
            try:
                author_device = org.devices[author]
            except KeyError:
                return InviteNewForShamirBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            try:
                author_user = org.users[author_user_id]
            except KeyError:
                return InviteNewForShamirBadOutcome.AUTHOR_NOT_FOUND
            if author_user.is_revoked:
                return InviteNewForShamirBadOutcome.AUTHOR_REVOKED

            # Check that the claimer exists
            claimer = org.users.get(claimer_user_id)
            if claimer is None:
                return InviteNewForShamirBadOutcome.USER_NOT_FOUND
            claimer_human_handle = claimer.cooked.human_handle

            # Check that a non-deleted shamir setup exists
            claimer_shamir_recoveries = org.shamir_recoveries[claimer_user_id]
            try:
                # All but the last shamir recovery are deleted
                last_shamir_recovery = claimer_shamir_recoveries[-1]
            except IndexError:
                last_shamir_recovery = None
            if last_shamir_recovery is None or last_shamir_recovery.is_deleted:
                # Since the author only knows about a shamir recovery if they are part of it,
                # we don't have a specific error for the case where the shamir setup doesn't exist
                return InviteNewForShamirBadOutcome.AUTHOR_NOT_ALLOWED

            # Author is not part of the recipients
            if author_user_id not in last_shamir_recovery.shares:
                return InviteNewForShamirBadOutcome.AUTHOR_NOT_ALLOWED

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
                org.invitations[token] = MemoryInvitation(
                    token=token,
                    type=InvitationType.SHAMIR_RECOVERY,
                    created_by_user_id=author_user_id,
                    created_by_device_id=author,
                    claimer_email=claimer_human_handle.email,
                    created_on=now,
                    claimer_user_id=claimer_user_id,
                )

                await self._event_bus.send(
                    EventInvitation(
                        organization_id=organization_id,
                        token=token,
                        greeter=author_user_id,
                        status=InvitationStatus.IDLE,
                    )
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

        async with org.topics_lock(read=["common"]):
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
            if invitation.is_deleted:
                return InviteCancelBadOutcome.INVITATION_ALREADY_DELETED

            invitation.deleted_on = now
            invitation.deleted_reason = MemoryInvitationDeletedReason.CANCELLED

            await self._event_bus.send(
                EventInvitation(
                    organization_id=organization_id,
                    token=token,
                    greeter=author_user_id,
                    status=InvitationStatus.CANCELLED,
                )
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
                    # In the future, this might change to:
                    #     if author_user.current_profile == UserProfile.ADMIN
                    # so that any admin can greet a user
                    if invitation.created_by_user_id != author_user_id:
                        continue
                    assert invitation.claimer_email is not None
                    status = self._get_invitation_status(organization_id, invitation)
                    item = UserInvitation(
                        claimer_email=invitation.claimer_email,
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by_device_id=invitation.created_by_device_id,
                        created_by_user_id=invitation.created_by_user_id,
                        # This should also change once any admin can greet a user
                        created_by_human_handle=author_user.cooked.human_handle,
                        status=status,
                    )
                case InvitationType.DEVICE:
                    if invitation.created_by_user_id != author_user_id:
                        continue
                    status = self._get_invitation_status(organization_id, invitation)
                    item = DeviceInvitation(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by_device_id=invitation.created_by_device_id,
                        created_by_user_id=invitation.created_by_user_id,
                        created_by_human_handle=author_user.cooked.human_handle,
                        status=status,
                    )
                case InvitationType.SHAMIR_RECOVERY:
                    shamir_recovery_invitation = self._get_shamir_recovery_invitation(
                        org, invitation
                    )
                    # There is no corresponding setup for this invitation, ignore it
                    if shamir_recovery_invitation is None:
                        continue
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
        created_by_human_handle = org.users[invitation.created_by_user_id].cooked.human_handle

        match invitation.type:
            case InvitationType.USER:
                assert invitation.claimer_email is not None
                return UserInvitation(
                    claimer_email=invitation.claimer_email,
                    created_on=invitation.created_on,
                    status=self._get_invitation_status(organization_id, invitation),
                    created_by_user_id=invitation.created_by_user_id,
                    created_by_device_id=invitation.created_by_device_id,
                    created_by_human_handle=created_by_human_handle,
                    token=invitation.token,
                )
            case InvitationType.DEVICE:
                return DeviceInvitation(
                    created_on=invitation.created_on,
                    status=self._get_invitation_status(organization_id, invitation),
                    created_by_user_id=invitation.created_by_user_id,
                    created_by_device_id=invitation.created_by_device_id,
                    created_by_human_handle=created_by_human_handle,
                    token=invitation.token,
                )
            case InvitationType.SHAMIR_RECOVERY:
                shamir_recovery_invitation = self._get_shamir_recovery_invitation(org, invitation)
                if shamir_recovery_invitation is None:
                    # TODO: The invitation is not actually deleted, but the corresponding setup has
                    # been deleted. This is a bit misleading, we should find a way to differentiate
                    # between the two cases.
                    return InviteAsInvitedInfoBadOutcome.INVITATION_DELETED
                return shamir_recovery_invitation
            case unknown:
                assert False, unknown

    @override
    async def test_dump_all_invitations(
        self, organization_id: OrganizationID
    ) -> dict[UserID, list[Invitation]]:
        org = self._data.organizations[organization_id]
        per_user_invitations = {}
        for invitation in org.invitations.values():
            try:
                current_user_invitations = per_user_invitations[invitation.created_by_user_id]
            except KeyError:
                current_user_invitations = []
                per_user_invitations[invitation.created_by_user_id] = current_user_invitations
            created_by_human_handle = org.users[invitation.created_by_user_id].cooked.human_handle
            match invitation.type:
                case InvitationType.USER:
                    assert invitation.claimer_email is not None
                    current_user_invitations.append(
                        UserInvitation(
                            claimer_email=invitation.claimer_email,
                            created_on=invitation.created_on,
                            status=self._get_invitation_status(organization_id, invitation),
                            created_by_user_id=invitation.created_by_user_id,
                            created_by_device_id=invitation.created_by_device_id,
                            created_by_human_handle=created_by_human_handle,
                            token=invitation.token,
                        )
                    )
                case InvitationType.DEVICE:
                    current_user_invitations.append(
                        DeviceInvitation(
                            created_on=invitation.created_on,
                            status=self._get_invitation_status(organization_id, invitation),
                            created_by_user_id=invitation.created_by_user_id,
                            created_by_device_id=invitation.created_by_device_id,
                            created_by_human_handle=created_by_human_handle,
                            token=invitation.token,
                        )
                    )
                case InvitationType.SHAMIR_RECOVERY:
                    shamir_recovery_invitation = self._get_shamir_recovery_invitation(
                        org, invitation
                    )
                    if shamir_recovery_invitation is not None:
                        current_user_invitations.append(shamir_recovery_invitation)
                case unknown:
                    assert False, unknown

        return per_user_invitations

    # New invite transport API

    def is_greeter_allowed(
        self, org: MemoryOrganization, invitation: MemoryInvitation, greeter: MemoryUser
    ) -> bool:
        if invitation.type == InvitationType.DEVICE:
            return invitation.created_by_user_id == greeter.cooked.user_id
        elif invitation.type == InvitationType.USER:
            return greeter.current_profile == UserProfile.ADMIN
        elif invitation.type == InvitationType.SHAMIR_RECOVERY:
            assert invitation.claimer_user_id is not None
            shamir_recoveries = org.shamir_recoveries.get(invitation.claimer_user_id)
            if not shamir_recoveries:
                return False
            shamir_recovery, *_ = shamir_recoveries
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
        if invitation.is_cancelled:
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
        if invitation.is_cancelled:
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
        if invitation.is_cancelled:
            return InviteGreeterCancelGreetingAttemptBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(org, invitation, greeter_user):
            return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_ALLOWED

        if attempt.cancelled_reason is not None:
            return GreetingAttemptCancelledBadOutcome(*attempt.cancelled_reason)
        if attempt.greeter_joined is None:
            return InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        attempt.greeter_cancel(now, reason)

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
        if invitation.is_cancelled:
            return InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(org, invitation, greeter_user):
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETER_NOT_ALLOWED

        if attempt.cancelled_reason is not None:
            return GreetingAttemptCancelledBadOutcome(*attempt.cancelled_reason)
        if attempt.claimer_joined is None:
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        attempt.claimer_cancel(now, reason)

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
        if invitation.is_cancelled:
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
        if invitation.is_cancelled:
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
        if invitation.is_cancelled:
            return InviteCompleteBadOutcome.INVITATION_CANCELLED
        if invitation.is_completed:
            return InviteCompleteBadOutcome.INVITATION_ALREADY_COMPLETED

        # Only the greeter or the claimer can complete the invitation
        if not self.is_greeter_allowed(org, invitation, author_user):
            if not invitation.claimer_email == author_user.cooked.human_handle.email:
                return InviteCompleteBadOutcome.AUTHOR_NOT_ALLOWED

        invitation.deleted_on = now
        invitation.deleted_reason = MemoryInvitationDeletedReason.FINISHED

        await self._event_bus.send(
            EventInvitation(
                organization_id=organization_id,
                token=token,
                greeter=author_user_id,
                status=InvitationStatus.FINISHED,
            )
        )
