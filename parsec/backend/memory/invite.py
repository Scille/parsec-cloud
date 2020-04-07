# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from uuid import UUID
from typing import List, Optional
from collections import defaultdict
from pendulum import Pendulum

from parsec.api.protocol import OrganizationID, UserID, InvitationStatus, InvitationDeletedReason
from parsec.backend.invite import (
    ConduitState,
    NEXT_CONDUIT_STATE,
    ConduitListenCtx,
    BaseInviteComponent,
    Invitation,
    InvitationNotFoundError,
    InvitationAlreadyExistsError,
    InvitationAlreadyDeletedError,
    InvitationInvalidStateError,
)


@attr.s(slots=True, auto_attribs=True)
class Conduit:
    state: ConduitState = ConduitState.STATE_1_WAIT_PEERS
    claimer_payload: Optional[bytes] = None
    greeter_payload: Optional[bytes] = None


class OrganizationStore:
    def __init__(self):
        self.invitations = {}
        self.conduits = defaultdict(Conduit)


class MemoryInviteComponent(BaseInviteComponent):
    def __init__(self, send_event, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._send_event = send_event
        self._organizations = defaultdict(OrganizationStore)

    def register_components(self, **other_components):
        pass

    async def _conduit_talk(
        self,
        organization_id: OrganizationID,
        greeter: Optional[UserID],
        token: UUID,
        state: ConduitState,
        payload: bytes,
    ) -> ConduitListenCtx:
        is_greeter = greeter is not None
        org = self._organizations[organization_id]
        invitation = org.invitations.get(token)

        if not invitation or (is_greeter and invitation.greeter_user_id != greeter):
            raise InvitationNotFoundError(token)

        if invitation.status == InvitationStatus.DELETED:
            raise InvitationAlreadyDeletedError(token)

        conduit = org.conduits[token]
        if is_greeter:
            curr_our_payload = conduit.greeter_payload
            curr_peer_payload = conduit.claimer_payload
        else:
            curr_our_payload = conduit.claimer_payload
            curr_peer_payload = conduit.greeter_payload

        if conduit.state != state or curr_our_payload is not None:
            # We are out of sync with the conduit:
            # - the conduit state has changed in our back (maybe reseted by the peer)
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
            "invite.conduit_updated", organization_id=organization_id, token=token
        )

        return ConduitListenCtx(
            organization_id=organization_id,
            greeter=greeter,
            token=token,
            state=state,
            payload=payload,
            peer_payload=curr_peer_payload,
        )

    async def _conduit_listen(self, ctx: ConduitListenCtx) -> Optional[bytes]:
        if ctx.payload == b"retry nonce":
            import pdb

            pdb.set_trace()
        org = self._organizations[ctx.organization_id]
        invitation = org.invitations.get(ctx.token)

        if not invitation or (ctx.is_greeter and invitation.greeter_user_id != ctx.greeter):
            raise InvitationNotFoundError(ctx.token)

        if invitation.status == InvitationStatus.DELETED:
            raise InvitationAlreadyDeletedError(ctx.token)

        conduit = org.conduits[ctx.token]
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
                    "invite.conduit_updated", organization_id=ctx.organization_id, token=ctx.token
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

    async def new(self, organization_id: OrganizationID, invitation: Invitation) -> None:
        assert invitation.status == InvitationStatus.IDLE  # Sanity check
        org = self._organizations[organization_id]
        if invitation.token in org.invitations:
            raise InvitationAlreadyExistsError(invitation.token)
        org.invitations[invitation.token] = invitation

    async def delete(
        self,
        organization_id: OrganizationID,
        greeter: UserID,
        token: UUID,
        on: Pendulum,
        reason: InvitationDeletedReason,
    ) -> None:
        org = self._organizations[organization_id]
        invitation = org.invitations.get(token)
        if not invitation or invitation.greeter_user_id != greeter:
            raise InvitationNotFoundError(token)
        if invitation.status == InvitationStatus.DELETED:
            raise InvitationAlreadyDeletedError(token)
        org.invitations[token] = invitation.evolve(
            status=InvitationStatus.DELETED, deleted_on=on, deleted_reason=reason
        )
        await self._send_event(
            "invite.status_changed",
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
            if invitation.greeter_user_id != greeter:
                continue
            if (
                invitation.status != InvitationStatus.DELETED
                and invitation.token in invitations_with_claimer_online
            ):
                invitations.append(invitation.evolve(status=InvitationStatus.READY))
            else:
                invitations.append(invitation)
        return invitations

    async def info(self, organization_id: OrganizationID, token: UUID) -> Invitation:
        org = self._organizations[organization_id]
        invitation = org.invitations.get(token)
        if not invitation:
            raise InvitationNotFoundError(token)
        if invitation.status == InvitationStatus.DELETED:
            raise InvitationAlreadyDeletedError(token)
        return invitation

    async def claimer_joined(
        self, organization_id: OrganizationID, greeter: UserID, token: UUID
    ) -> None:
        await self._send_event(
            "invite.status_changed",
            organization_id=organization_id,
            greeter=greeter,
            token=token,
            status=InvitationStatus.READY,
        )

    async def claimer_left(
        self, organization_id: OrganizationID, greeter: UserID, token: UUID
    ) -> None:
        await self._send_event(
            "invite.status_changed",
            organization_id=organization_id,
            greeter=greeter,
            token=token,
            status=InvitationStatus.IDLE,
        )
