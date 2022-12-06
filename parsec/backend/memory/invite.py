# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from collections import defaultdict
from typing import TYPE_CHECKING, Any, Callable, Coroutine, List, Tuple

import attr

from parsec._parsec import DateTime, InvitationDeletedReason
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
    InvitationInvalidStateError,
    InvitationNotFoundError,
    UserInvitation,
)

if TYPE_CHECKING:
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

    def register_components(self, user: MemoryUserComponent, **other_components: Any) -> None:
        self._user_component = user

    def _get_invitation_and_conduit(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        expected_greeter: UserID | None = None,
    ) -> Tuple[Invitation, Conduit]:
        org = self._organizations[organization_id]
        invitation = org.invitations.get(token)
        if not invitation or (expected_greeter and invitation.greeter_user_id != expected_greeter):
            raise InvitationNotFoundError(token)
        if token in org.deleted_invitations:
            raise InvitationAlreadyDeletedError(token)
        return invitation, org.conduits[token]

    async def _conduit_talk(
        self,
        organization_id: OrganizationID,
        greeter: UserID | None,
        token: InvitationToken,
        state: ConduitState,
        payload: bytes,
    ) -> ConduitListenCtx:
        is_greeter = greeter is not None
        _, conduit = self._get_invitation_and_conduit(
            organization_id, token, expected_greeter=greeter
        )
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
            token=token,
            state=state,
            payload=payload,
            peer_payload=curr_peer_payload,
        )

    async def _conduit_listen(self, ctx: ConduitListenCtx) -> bytes | None:
        _, conduit = self._get_invitation_and_conduit(
            ctx.organization_id, ctx.token, expected_greeter=ctx.greeter
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

    async def _new(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        created_on: DateTime | None,
        claimer_email: str | None = None,
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
            if claimer_email:
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
            greeter=invitation.greeter_user_id,
            token=invitation.token,
            status=invitation.status,
        )
        return invitation

    async def delete(
        self,
        organization_id: OrganizationID,
        greeter: UserID,
        token: InvitationToken,
        on: DateTime,
        reason: InvitationDeletedReason,
    ) -> None:
        self._get_invitation_and_conduit(organization_id, token, expected_greeter=greeter)
        org = self._organizations[organization_id]
        org.deleted_invitations[token] = (on, reason)
        await self._send_event(
            BackendEvent.INVITE_STATUS_CHANGED,
            organization_id=organization_id,
            greeter=greeter,
            token=token,
            status=InvitationStatus.DELETED,
        )

    async def list(self, organization_id: OrganizationID, greeter: UserID) -> List[Invitation]:
        org = self._organizations[organization_id]
        invitations_with_claimer_online = self._claimers_ready[organization_id]
        invitations = []
        for invitation in org.invitations.values():
            if invitation.greeter_user_id != greeter or invitation.token in org.deleted_invitations:
                continue
            if invitation.token in invitations_with_claimer_online:
                invitations.append(invitation.evolve(status=InvitationStatus.READY))
            else:
                invitations.append(invitation)
        return sorted(invitations, key=lambda x: x.created_on)

    async def info(self, organization_id: OrganizationID, token: InvitationToken) -> Invitation:
        invitation, _ = self._get_invitation_and_conduit(organization_id, token)
        return invitation

    async def claimer_joined(
        self, organization_id: OrganizationID, greeter: UserID, token: InvitationToken
    ) -> None:
        await self._send_event(
            BackendEvent.INVITE_STATUS_CHANGED,
            organization_id=organization_id,
            greeter=greeter,
            token=token,
            status=InvitationStatus.READY,
        )

    async def claimer_left(
        self, organization_id: OrganizationID, greeter: UserID, token: InvitationToken
    ) -> None:
        await self._send_event(
            BackendEvent.INVITE_STATUS_CHANGED,
            organization_id=organization_id,
            greeter=greeter,
            token=token,
            status=InvitationStatus.IDLE,
        )
