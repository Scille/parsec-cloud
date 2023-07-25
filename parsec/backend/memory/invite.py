# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from collections import defaultdict
from copy import deepcopy
from typing import TYPE_CHECKING, Any, Callable, Coroutine, List, Tuple

import attr

from parsec._parsec import DateTime, InvitationDeletedReason, ShamirRecoveryRecipient
from parsec.api.protocol import InvitationStatus, InvitationToken, OrganizationID, UserID
from parsec.backend.backend_events import BackendEvent
from parsec.backend.invite import (
    NEXT_CONDUIT_STATE,
    BaseInviteComponent,
    ConduitListenCtx,
    ConduitState,
    DeviceInvitation,
    Invitation,
    InvitationAlreadyDeletedError,
    InvitationAlreadyMemberError,
    InvitationError,
    InvitationInvalidStateError,
    InvitationNotFoundError,
    InvitationShamirRecoveryGreeterNotInRecipients,
    InvitationShamirRecoveryNotSetup,
    ShamirRecoveryInvitation,
    UserInvitation,
)

if TYPE_CHECKING:
    from parsec.backend.memory.shamir import MemoryShamirComponent
    from parsec.backend.memory.user import MemoryUserComponent


@attr.s(slots=True, auto_attribs=True)
class Conduit:
    state: ConduitState = ConduitState.STATE_1_WAIT_PEERS
    claimer_payload: bytes | None = None
    greeter_payload: bytes | None = None


class OrganizationStore:
    def __init__(self) -> None:
        self.invitations: dict[InvitationToken, Invitation] = {}
        self.deleted_invitations: dict[
            InvitationToken, Tuple[DateTime, InvitationDeletedReason]
        ] = {}
        self.conduits: dict[InvitationToken, Conduit] = defaultdict(Conduit)
        self.shamir_conduits: dict[tuple[InvitationToken, UserID], Conduit] = defaultdict(Conduit)


class MemoryInviteComponent(BaseInviteComponent):
    def __init__(
        self, send_event: Callable[..., Coroutine[Any, Any, None]], *args: Any, **kwargs: Any
    ) -> None:
        super().__init__(*args, **kwargs)
        self._send_event = send_event
        self._organizations: dict[OrganizationID, OrganizationStore] = defaultdict(
            OrganizationStore
        )
        self._user_component: MemoryUserComponent | None = None
        self._shamir_component: MemoryShamirComponent | None = None

    def register_components(
        self, user: MemoryUserComponent, shamir: MemoryShamirComponent, **other_components: Any
    ) -> None:
        self._user_component = user
        self._shamir_component = shamir

    def _get_invitation(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
    ) -> Invitation:
        org = self._organizations[organization_id]
        invitation = org.invitations.get(token)
        if not invitation:
            raise InvitationNotFoundError(token)
        if token in org.deleted_invitations:
            raise InvitationAlreadyDeletedError(token)
        return invitation

    def _get_invitation_and_conduit(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeter: UserID | None,
    ) -> tuple[Invitation, Conduit]:
        org = self._organizations[organization_id]
        invitation = self._get_invitation(organization_id, token)
        if isinstance(invitation, ShamirRecoveryInvitation):
            if greeter is None:
                raise InvitationError("`greeter_user_id` is not provided")
            _, recipients = self._shamir_info(organization_id, invitation.claimer_user_id)
            recipients = {recipient.user_id for recipient in recipients}
            if greeter not in recipients:
                raise InvitationNotFoundError(token)
            return invitation, org.shamir_conduits[(token, greeter)]
        if greeter is not None and invitation.greeter_user_id != greeter:
            raise InvitationNotFoundError(token)
        return invitation, org.conduits[token]

    async def _conduit_talk(
        self,
        organization_id: OrganizationID,
        greeter: UserID | None,
        is_greeter: bool,
        token: InvitationToken,
        state: ConduitState,
        payload: bytes,
    ) -> ConduitListenCtx:
        _, conduit = self._get_invitation_and_conduit(organization_id, token, greeter=greeter)
        if is_greeter:
            curr_our_payload = conduit.greeter_payload
            curr_peer_payload = conduit.claimer_payload
        else:
            curr_our_payload = conduit.claimer_payload
            curr_peer_payload = conduit.greeter_payload

        if conduit.state != state or curr_our_payload is not None:
            # We are out of sync with the conduit:
            # - the conduit state has changed in our back (maybe reset by the peer)
            # - we want to reset the conduit
            # - we have already provided a payload for the current conduit state (most
            #   likely because a retry of a command that failed due to connection outage)
            if state == ConduitState.STATE_1_WAIT_PEERS:
                # We wait to reset the conduit
                conduit.state = state
                conduit.claimer_payload = None
                conduit.greeter_payload = None
                curr_peer_payload = None
            else:
                raise InvitationInvalidStateError()

        # Now update the conduit with our payload and send a signal if
        # the peer is already waiting for us.
        if is_greeter:
            conduit.greeter_payload = payload
        else:
            conduit.claimer_payload = payload

        # Note that in case of conduit reset, this signal will lure the peer into
        # thinking we have answered so he will wakeup and take into account the reset
        await self._send_event(
            BackendEvent.INVITE_CONDUIT_UPDATED,
            organization_id=organization_id,
            token=token,
        )

        return ConduitListenCtx(
            organization_id=organization_id,
            greeter=greeter,
            is_greeter=is_greeter,
            token=token,
            state=state,
            payload=payload,
            peer_payload=curr_peer_payload,
        )

    async def _conduit_listen(self, ctx: ConduitListenCtx) -> bytes | None:
        _, conduit = self._get_invitation_and_conduit(
            ctx.organization_id, ctx.token, greeter=ctx.greeter
        )
        if ctx.is_greeter:
            curr_our_payload = conduit.greeter_payload
            curr_peer_payload = conduit.claimer_payload
        else:
            curr_our_payload = conduit.claimer_payload
            curr_peer_payload = conduit.greeter_payload

        if ctx.peer_payload is None:
            # We are waiting for the peer to provite it payload

            # Only peer payload should be allowed to change
            if conduit.state != ctx.state or curr_our_payload != ctx.payload:
                raise InvitationInvalidStateError()

            if curr_peer_payload is not None:
                # Our peer has provided it payload (hence it knows
                # about our payload too), we can update the conduit
                # to the next state
                conduit.state = NEXT_CONDUIT_STATE[ctx.state]
                conduit.greeter_payload = None
                conduit.claimer_payload = None
                await self._send_event(
                    BackendEvent.INVITE_CONDUIT_UPDATED,
                    organization_id=ctx.organization_id,
                    token=ctx.token,
                )
                return curr_peer_payload

        else:
            # We were waiting for the peer to take into account the
            # payload we provided. This would be done once the conduit
            # has switched to it next state.

            if conduit.state == NEXT_CONDUIT_STATE[ctx.state] and curr_our_payload is None:
                return ctx.peer_payload
            elif (
                conduit.state != ctx.state
                or curr_our_payload != ctx.payload
                or curr_peer_payload != ctx.peer_payload
            ):
                # Something unexpected has changed in our back...
                raise InvitationInvalidStateError()

        # Peer hasn't answered yet, we should wait and retry later...
        return None

    async def new_for_user(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        claimer_email: str,
        created_on: DateTime | None = None,
    ) -> UserInvitation:
        """
        Raise: InvitationAlreadyMemberError
        """
        assert self._user_component is not None

        org = self._user_component._organizations[organization_id]

        for _, user in org.users.items():
            if (
                user.human_handle
                and user.human_handle.email == claimer_email
                and not user.is_revoked()
            ):
                raise InvitationAlreadyMemberError()
        result = await self._new(
            organization_id=organization_id,
            greeter_user_id=greeter_user_id,
            claimer_email=claimer_email,
            created_on=created_on,
        )
        assert isinstance(result, UserInvitation)
        return result

    async def new_for_device(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        created_on: DateTime | None = None,
    ) -> DeviceInvitation:
        result = await self._new(
            organization_id=organization_id,
            greeter_user_id=greeter_user_id,
            created_on=created_on,
        )
        assert isinstance(result, DeviceInvitation)
        return result

    async def new_for_shamir_recovery(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        claimer_user_id: UserID,
        created_on: DateTime | None = None,
    ) -> ShamirRecoveryInvitation:
        assert self._user_component is not None

        # Check for existing invitation
        org = self._organizations[organization_id]
        for invitation in org.invitations.values():
            if (
                isinstance(invitation, ShamirRecoveryInvitation)
                and invitation.claimer_user_id == claimer_user_id
                and invitation.token not in org.deleted_invitations
            ):
                # A shamir invitation alread exists for that user
                return invitation

        # Gather information for new invitation
        created_on = created_on or DateTime.now()
        greeter_human_handle = self._user_component._get_user(
            organization_id, greeter_user_id
        ).human_handle
        claimer_user = self._user_component._get_user(organization_id, claimer_user_id)
        claimer_email = claimer_user.human_handle.email if claimer_user.human_handle else None
        _, recipients = self._shamir_info(organization_id, claimer_user_id)
        allowed_greeters = {recipient.user_id for recipient in recipients}

        # Check greeter
        if greeter_user_id not in allowed_greeters:
            raise InvitationShamirRecoveryGreeterNotInRecipients()

        # Create invitation
        invitation = ShamirRecoveryInvitation(
            greeter_user_id=greeter_user_id,
            greeter_human_handle=greeter_human_handle,
            claimer_email=claimer_email,
            claimer_user_id=claimer_user_id,
            created_on=created_on,
        )
        org.invitations[invitation.token] = invitation
        await self._send_event(
            BackendEvent.INVITE_STATUS_CHANGED,
            organization_id=organization_id,
            greeters=[recipient.user_id for recipient in recipients],
            token=invitation.token,
            status=invitation.status,
        )
        return invitation

    async def _new(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        created_on: DateTime | None,
        claimer_email: str | None = None,
        claimer_user_id: UserID | None = None,
    ) -> Invitation:
        assert self._user_component is not None

        org = self._organizations[organization_id]
        for invitation in org.invitations.values():
            if (
                invitation.greeter_user_id == greeter_user_id
                and getattr(invitation, "claimer_email", None) == claimer_email
                and invitation.token not in org.deleted_invitations
            ):
                # An invitation already exists for what the user has asked for
                return invitation
        else:
            # Must create a new invitation
            created_on = created_on or DateTime.now()
            greeter_human_handle = self._user_component._get_user(
                organization_id, greeter_user_id
            ).human_handle
            if claimer_email is not None:
                invitation = UserInvitation(
                    greeter_user_id=greeter_user_id,
                    greeter_human_handle=greeter_human_handle,
                    claimer_email=claimer_email,
                    created_on=created_on,
                )
            else:  # Device
                invitation = DeviceInvitation(
                    greeter_user_id=greeter_user_id,
                    greeter_human_handle=greeter_human_handle,
                    created_on=created_on,
                )
            org.invitations[invitation.token] = invitation

        await self._send_event(
            BackendEvent.INVITE_STATUS_CHANGED,
            organization_id=organization_id,
            greeters=[invitation.greeter_user_id],
            token=invitation.token,
            status=invitation.status,
        )
        return invitation

    async def delete(
        self,
        organization_id: OrganizationID,
        deleter: UserID,
        token: InvitationToken,
        on: DateTime,
        reason: InvitationDeletedReason,
    ) -> None:
        invitation = self._get_invitation(organization_id, token)
        if isinstance(invitation, ShamirRecoveryInvitation):
            _, recipients = self._shamir_info(organization_id, invitation.claimer_user_id)
            recipients = {recipient.user_id for recipient in recipients}
            # The claimer is authorized to delete the invitation
            if deleter not in recipients and deleter != invitation.claimer_user_id:
                raise InvitationNotFoundError(token)
        elif invitation.greeter_user_id != deleter:
            raise InvitationNotFoundError(token)
        org = self._organizations[organization_id]
        org.deleted_invitations[token] = (on, reason)
        await self._send_invite_status_changed(
            organization_id, invitation, InvitationStatus.DELETED
        )

    async def delete_shamir_invitation_if_it_exists(
        self,
        organization_id: OrganizationID,
        claimer: UserID,
        on: DateTime | None = None,
    ) -> bool:
        org = self._organizations[organization_id]
        # Find the corresponding invitation
        for token, invitation in org.invitations.items():
            if not isinstance(invitation, ShamirRecoveryInvitation):
                continue
            if invitation.claimer_user_id == claimer:
                break
        # None found, return
        else:
            return False
        # Check that it's not already deleted
        try:
            invitation = self._get_invitation(organization_id, token)
        except InvitationAlreadyDeletedError:
            return False
        assert isinstance(invitation, ShamirRecoveryInvitation)
        _, recipients = self._shamir_info(organization_id, invitation.claimer_user_id)
        # Delete the invitation and send the event
        on = on or DateTime.now()
        reason = InvitationDeletedReason.CANCELLED
        org.deleted_invitations[token] = (on, reason)
        await self._send_event(
            BackendEvent.INVITE_STATUS_CHANGED,
            organization_id=organization_id,
            greeters=[recipient.user_id for recipient in recipients],
            token=token,
            status=InvitationStatus.DELETED,
        )
        # Indicate the an invitation has actually been deleted
        return True

    async def list(self, organization_id: OrganizationID, greeter: UserID) -> List[Invitation]:
        org = self._organizations[organization_id]
        invitations_with_claimer_online = self._claimers_ready[organization_id]
        invitations = []
        for invitation in org.invitations.values():
            if isinstance(invitation, ShamirRecoveryInvitation):
                _, recipients = self._shamir_info(organization_id, invitation.claimer_user_id)
                if greeter not in {recipient.user_id for recipient in recipients}:
                    continue
            elif invitation.greeter_user_id != greeter:
                continue
            if invitation.token in org.deleted_invitations:
                continue
            if invitation.token in invitations_with_claimer_online:
                invitations.append(invitation.evolve(status=InvitationStatus.READY))
            else:
                invitations.append(invitation)
        return sorted(invitations, key=lambda x: x.created_on)

    async def info(self, organization_id: OrganizationID, token: InvitationToken) -> Invitation:
        invitation = self._get_invitation(organization_id, token)
        return invitation

    async def _send_invite_status_changed(
        self,
        organization_id: OrganizationID,
        invitation: Invitation,
        invitation_status: InvitationStatus,
    ) -> None:
        if isinstance(invitation, ShamirRecoveryInvitation):
            _, recipients = self._shamir_info(organization_id, invitation.claimer_user_id)
            await self._send_event(
                BackendEvent.INVITE_STATUS_CHANGED,
                organization_id=organization_id,
                greeters=[recipient.user_id for recipient in recipients],
                token=invitation.token,
                status=invitation_status,
            )
        else:
            await self._send_event(
                BackendEvent.INVITE_STATUS_CHANGED,
                organization_id=organization_id,
                greeters=[invitation.greeter_user_id],
                token=invitation.token,
                status=invitation_status,
            )

    async def claimer_joined(self, organization_id: OrganizationID, invitation: Invitation) -> None:
        await self._send_invite_status_changed(organization_id, invitation, InvitationStatus.READY)

    async def claimer_left(self, organization_id: OrganizationID, invitation: Invitation) -> None:
        await self._send_invite_status_changed(organization_id, invitation, InvitationStatus.IDLE)

    def test_duplicate_organization(self, id: OrganizationID, new_id: OrganizationID) -> None:
        self._organizations[new_id] = deepcopy(self._organizations[id])

    def test_drop_organization(self, id: OrganizationID) -> None:
        self._organizations.pop(id, None)

    def _shamir_info(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> tuple[int, tuple[ShamirRecoveryRecipient, ...]]:
        assert self._shamir_component is not None

        try:
            item = self._shamir_component._shamir_recovery_items[(organization_id, user_id)]
        except KeyError as exc:
            raise InvitationShamirRecoveryNotSetup(exc)

        return (item.threshold, item.recipients)

    async def shamir_info(
        self, organization_id: OrganizationID, token: InvitationToken
    ) -> tuple[int, tuple[ShamirRecoveryRecipient, ...]]:
        invitation = self._get_invitation(organization_id, token)
        assert isinstance(invitation, ShamirRecoveryInvitation)
        return self._shamir_info(organization_id, invitation.claimer_user_id)
