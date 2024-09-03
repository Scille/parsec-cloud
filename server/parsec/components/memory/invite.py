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
    NEXT_CONDUIT_STATE,
    BaseInviteComponent,
    ConduitListenCtx,
    ConduitState,
    DeviceInvitation,
    GreetingAttemptCancelledBadOutcome,
    Invitation,
    InviteAsInvitedInfoBadOutcome,
    InviteCancelBadOutcome,
    InviteClaimerCancelGreetingAttemptBadOutcome,
    InviteClaimerStartGreetingAttemptBadOutcome,
    InviteClaimerStepBadOutcome,
    InviteCompleteBadOutcome,
    InviteConduitExchangeBadOutcome,
    InviteGreeterCancelGreetingAttemptBadOutcome,
    InviteGreeterStartGreetingAttemptBadOutcome,
    InviteGreeterStepBadOutcome,
    InviteListBadOutcome,
    InviteNewForDeviceBadOutcome,
    InviteNewForUserBadOutcome,
    NotReady,
    SendEmailBadOutcome,
    UserInvitation,
)
from parsec.components.memory.datamodel import (
    AdvisoryLock,
    MemoryDatamodel,
    MemoryInvitation,
    MemoryInvitationDeletedReason,
    MemoryUser,
)
from parsec.events import EventEnrollmentConduit, EventInvitation


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

    @override
    async def _claimer_joined(
        self, organization_id: OrganizationID, token: InvitationToken, greeter: UserID
    ) -> None:
        await self._event_bus.send(
            EventInvitation(
                organization_id=organization_id,
                token=token,
                greeter=greeter,
                status=InvitationStatus.READY,
            )
        )

    @override
    async def _claimer_left(
        self, organization_id: OrganizationID, token: InvitationToken, greeter: UserID
    ) -> None:
        await self._event_bus.send(
            EventInvitation(
                organization_id=organization_id,
                token=token,
                greeter=greeter,
                status=InvitationStatus.IDLE,
            )
        )

    @override
    async def _conduit_talk(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        is_greeter: bool,
        state: ConduitState,
        payload: bytes,
        last: bool,
    ) -> ConduitListenCtx | InviteConduitExchangeBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteConduitExchangeBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteConduitExchangeBadOutcome.ORGANIZATION_EXPIRED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteConduitExchangeBadOutcome.INVITATION_NOT_FOUND
        if invitation.is_deleted:
            return InviteConduitExchangeBadOutcome.INVITATION_DELETED

        try:
            greeter_user = org.users[invitation.created_by_user_id]
        except KeyError as exc:
            # Database is corrupted if we end up here !
            assert False, exc
        if is_greeter and greeter_user.is_revoked:
            return InviteConduitExchangeBadOutcome.AUTHOR_REVOKED

        if is_greeter:
            curr_our_payload = invitation.conduit_greeter_payload
            curr_peer_payload = invitation.conduit_claimer_payload
        else:
            assert last is False  # Only greeter can decide when the exchange should finish
            curr_our_payload = invitation.conduit_claimer_payload
            curr_peer_payload = invitation.conduit_greeter_payload

        if invitation.conduit_state != state or curr_our_payload is not None:
            # We are out of sync with the conduit:
            # - the conduit state has changed in our back (maybe reset by the peer)
            # - we want to reset the conduit
            # - we have already provided a payload for the current conduit state (most
            #   likely because a retry of a command that failed due to connection outage)
            if state == ConduitState.STATE_1_WAIT_PEERS:
                # We wait to reset the conduit
                invitation.conduit_state = state
                invitation.conduit_is_last_exchange = False
                invitation.conduit_claimer_payload = None
                invitation.conduit_greeter_payload = None
                curr_peer_payload = None
            else:
                return InviteConduitExchangeBadOutcome.ENROLLMENT_WRONG_STATE

        # Now update the conduit with our payload and send a signal in case
        # the peer is already waiting for us.
        if is_greeter:
            invitation.conduit_is_last_exchange = last
            invitation.conduit_greeter_payload = payload
        else:
            invitation.conduit_claimer_payload = payload

        # Note that in case of conduit resets, this signal will lure the peer into
        # thinking we have answered so he will wakeup and take into account the reset
        await self._event_bus.send(
            EventEnrollmentConduit(
                organization_id=organization_id,
                token=token,
                greeter=greeter_user.cooked.user_id,
            )
        )

        return ConduitListenCtx(
            organization_id=organization_id,
            token=token,
            greeter=greeter_user.cooked.user_id,
            is_greeter=is_greeter,
            state=state,
            payload=payload,
            peer_payload=curr_peer_payload,
            is_last_exchange=invitation.conduit_is_last_exchange,
        )

    @override
    async def _conduit_listen(
        self, now: DateTime, ctx: ConduitListenCtx
    ) -> tuple[bytes, bool] | None | InviteConduitExchangeBadOutcome:
        try:
            org = self._data.organizations[ctx.organization_id]
        except KeyError:
            return InviteConduitExchangeBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteConduitExchangeBadOutcome.ORGANIZATION_EXPIRED

        if ctx.is_greeter:
            assert ctx.greeter is not None
            try:
                greeter_user = org.users[ctx.greeter]
            except KeyError:
                return InviteConduitExchangeBadOutcome.AUTHOR_NOT_FOUND
            if greeter_user.is_revoked:
                return InviteConduitExchangeBadOutcome.AUTHOR_REVOKED

        try:
            invitation = org.invitations[ctx.token]
        except KeyError:
            return InviteConduitExchangeBadOutcome.INVITATION_NOT_FOUND

        # Ignore `invitation.is_deleted` here:
        # - The check have already been done during `_conduit_talk`.
        # - The peer may have already updated the conduite in it final (i.e. deleted)
        #   state, so it's hard to detect that we *are* the last listen allowed.

        if ctx.is_greeter:
            curr_our_payload = invitation.conduit_greeter_payload
            curr_peer_payload = invitation.conduit_claimer_payload
        else:
            curr_our_payload = invitation.conduit_claimer_payload
            curr_peer_payload = invitation.conduit_greeter_payload

        if ctx.peer_payload is None:
            # We are waiting for the peer to provide its payload

            # Only peer payload should be allowed to change
            if invitation.conduit_state != ctx.state or curr_our_payload != ctx.payload:
                return InviteConduitExchangeBadOutcome.ENROLLMENT_WRONG_STATE

            if curr_peer_payload is not None:
                # Our peer has provided it payload (hence it knows
                # about our payload too), we can update the conduit
                # to the next state
                invitation.conduit_state = NEXT_CONDUIT_STATE[ctx.state]
                invitation.conduit_greeter_payload = None
                invitation.conduit_claimer_payload = None
                if (
                    invitation.conduit_state == ConduitState.STATE_4_COMMUNICATE
                    and invitation.conduit_is_last_exchange
                ):
                    invitation.deleted_on = now
                    invitation.deleted_reason = MemoryInvitationDeletedReason.FINISHED
                    await self._event_bus.send(
                        EventInvitation(
                            organization_id=ctx.organization_id,
                            token=ctx.token,
                            greeter=ctx.greeter,
                            status=InvitationStatus.FINISHED,
                        )
                    )

                await self._event_bus.send(
                    EventEnrollmentConduit(
                        organization_id=ctx.organization_id,
                        token=ctx.token,
                        greeter=ctx.greeter,
                    )
                )

                # The peer payload is the one that was in the base right before we updated the state.
                # Similarly, the `is_last_exchange` flag is also the one we captured right before updating
                # the state.
                return curr_peer_payload, invitation.conduit_is_last_exchange

        else:
            # We were waiting for the peer to take into account the
            # payload we provided. This would be done once the conduit
            # has switched to it next state.

            if (
                invitation.conduit_state == NEXT_CONDUIT_STATE[ctx.state]
                and curr_our_payload is None
            ):
                # Careful here: it's possible that the other peer has already sent its payload
                # for the new state by the time we reach this point. This also means that it might
                # already have changed the `last_exchange` flag, so we can't rely on what is currently
                # in the database to determine if this is the last exchange. Instead, we return the
                # flag we captured during the `conduit_talk` along with the peer payload.
                return ctx.peer_payload, ctx.is_last_exchange

            elif (
                invitation.conduit_state != ctx.state
                or curr_our_payload != ctx.payload
                or curr_peer_payload != ctx.peer_payload
            ):
                # Something unexpected has changed in our back...
                return InviteConduitExchangeBadOutcome.ENROLLMENT_WRONG_STATE

        # Peer hasn't answered yet, we should wait and retry later...
        return None

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
            if invitation.created_by_user_id != author_user_id:
                continue

            status = self._get_invitation_status(organization_id, invitation)
            match invitation.type:
                case InvitationType.USER:
                    assert invitation.claimer_email is not None
                    item = UserInvitation(
                        claimer_email=invitation.claimer_email,
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by_device_id=invitation.created_by_device_id,
                        created_by_user_id=invitation.created_by_user_id,
                        created_by_human_handle=author_user.cooked.human_handle,
                        status=status,
                    )
                case InvitationType.DEVICE:
                    item = DeviceInvitation(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by_device_id=invitation.created_by_device_id,
                        created_by_user_id=invitation.created_by_user_id,
                        created_by_human_handle=author_user.cooked.human_handle,
                        status=status,
                    )
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
                case unknown:
                    assert False, unknown

        return per_user_invitations

    # New invite transport API
    # TODO: Remove the old API once the new one is fully functional

    def is_greeter_allowed(self, invitation: MemoryInvitation, greeter: MemoryUser) -> bool:
        if invitation.type == InvitationType.DEVICE:
            return invitation.created_by_user_id == greeter.cooked.user_id
        elif invitation.type == InvitationType.USER:
            return greeter.current_profile == UserProfile.ADMIN
        else:
            raise NotImplementedError

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

        if not self.is_greeter_allowed(invitation, greeter_user):
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

        if not self.is_greeter_allowed(invitation, greeter_user):
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

        if not self.is_greeter_allowed(invitation, greeter_user):
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

        if not self.is_greeter_allowed(invitation, greeter_user):
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

        if not self.is_greeter_allowed(invitation, greeter_user):
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

        if not self.is_greeter_allowed(invitation, greeter_user):
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
        if not self.is_greeter_allowed(invitation, author_user):
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
