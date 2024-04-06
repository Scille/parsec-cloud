# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import Any, assert_never, override

from parsec._parsec import (
    DateTime,
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
    Invitation,
    InviteAsInvitedInfoBadOutcome,
    InviteCancelBadOutcome,
    InviteConduitClaimerExchangeBadOutcome,
    InviteConduitExchangeResetReason,
    InviteConduitGreeterExchangeBadOutcome,
    InviteListBadOutcome,
    InviteNewForDeviceBadOutcome,
    InviteNewForUserBadOutcome,
    SendEmailBadOutcome,
    UserInvitation,
)
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemoryInvitation,
    MemoryInvitationDeletedReason,
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
                case unknown:
                    assert_never(unknown)
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

    async def conduit_greeter_exchange(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        step: int,
        greeter_payload: bytes,
        last: bool = False,
        reset_reason: InviteConduitExchangeResetReason = InviteConduitExchangeResetReason.NORMAL,
    ) -> bytes | InviteConduitExchangeResetReason | InviteConduitGreeterExchangeBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteConduitGreeterExchangeBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteConduitGreeterExchangeBadOutcome.ORGANIZATION_EXPIRED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteConduitGreeterExchangeBadOutcome.INVITATION_NOT_FOUND
        if invitation.is_deleted:
            return InviteConduitGreeterExchangeBadOutcome.INVITATION_DELETED

        try:
            greeter_user = org.users[invitation.greeter]
        except KeyError as exc:
            # Database is corrupted if we end up here !
            assert False, exc
        if greeter_user.is_revoked:
            return InviteConduitGreeterExchangeBadOutcome.AUTHOR_REVOKED

        # We are out of sync with the conduit:
        # - we want to reset the conduit
        # - the conduit state has changed in our back (maybe reset by the peer)
        # - we have already provided a different payload for the current conduit state
        #   (providing the same payload is okay given we play idempotent in this case)
        # - we want to set a step that is not the next one
        current_greeter_step = len(invitation.conduit_greeter_payloads)
        current_claimer_step = len(invitation.conduit_claimer_payloads)
        if step == 0 and current_greeter_step != 0:
            # We reset the conduit
            invitation.conduit_last_exchange_step = None
            invitation.conduit_claimer_payloads.clear()
            invitation.conduit_greeter_payloads.clear()
            invitation.conduit_reset_reason = reset_reason
        elif (
            invitation.conduit_last_exchange_step is not None
            and step > invitation.conduit_last_exchange_step
        ):
            return invitation.conduit_reset_reason
        elif step > current_greeter_step + 1 or step >= current_claimer_step + 1:
            # Conduit has changed in our back
            # It is okay to try to set a previous step as long as the payload stays the same !
            return invitation.conduit_reset_reason

        try:
            already_provided_step_payload = invitation.conduit_greeter_payloads[step]
        except IndexError:
            assert len(invitation.conduit_greeter_payloads) == step
            # First time we provide a payload for this step
            invitation.conduit_greeter_payloads.append(greeter_payload)
            if last:
                invitation.conduit_last_exchange_step = step
        else:
            if already_provided_step_payload != greeter_payload:
                # We have already provided a different payload
                return invitation.conduit_reset_reason

        # Finally get the claimer payload if it's available
        try:
            claimer_payload = invitation.conduit_claimer_payloads[step]
        except IndexError:
            return InviteConduitGreeterExchangeBadOutcome.RETRY_NEEDED
        return claimer_payload

    async def conduit_claimer_exchange(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        step: int,
        claimer_payload: bytes,
        reset_reason: InviteConduitExchangeResetReason = InviteConduitExchangeResetReason.NORMAL,
    ) -> (
        tuple[bytes, bool]
        | InviteConduitExchangeResetReason
        | InviteConduitClaimerExchangeBadOutcome
    ):
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteConduitClaimerExchangeBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteConduitClaimerExchangeBadOutcome.ORGANIZATION_EXPIRED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return InviteConduitClaimerExchangeBadOutcome.INVITATION_NOT_FOUND
        if invitation.is_deleted:
            return InviteConduitClaimerExchangeBadOutcome.INVITATION_DELETED

        # We are out of sync with the conduit:
        # - we want to reset the conduit
        # - the conduit state has changed in our back (maybe reset by the peer)
        # - we have already provided a different payload for the current conduit state
        #   (providing the same payload is okay given we play idempotent in this case)
        current_claimer_step = len(invitation.conduit_claimer_payloads)
        current_greeter_step = len(invitation.conduit_greeter_payloads)
        if step == 0 and current_claimer_step != 0:
            # We reset the conduit
            invitation.conduit_last_exchange_step = None
            invitation.conduit_claimer_payloads.clear()
            invitation.conduit_greeter_payloads.clear()
            invitation.conduit_reset_reason = reset_reason
        elif (
            invitation.conduit_last_exchange_step is not None
            and step > invitation.conduit_last_exchange_step
        ):
            return invitation.conduit_reset_reason
        elif step > current_claimer_step + 1 or step >= current_greeter_step + 1:
            # Conduit has changed in our back
            # It is okay to try to set a previous step as long as the payload stays the same !
            return invitation.conduit_reset_reason

        try:
            already_provided_step_payload = invitation.conduit_claimer_payloads[step]
        except IndexError:
            assert len(invitation.conduit_claimer_payloads) == step
            # First time we provide a payload for this step
            invitation.conduit_claimer_payloads.append(claimer_payload)
        else:
            if already_provided_step_payload != claimer_payload:
                # We have already provided a different payload
                return invitation.conduit_reset_reason

        # Finally get the greeter payload if it's available
        try:
            greeter_payload = invitation.conduit_greeter_payloads[step]
        except IndexError:
            return InviteConduitClaimerExchangeBadOutcome.RETRY_NEEDED
        return greeter_payload, invitation.conduit_last_exchange_step == step

    @override
    async def new_for_user(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: UserID,
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

        try:
            author_user = org.users[author]
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
                and invitation.greeter == author
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
                greeter=author,
                claimer_email=claimer_email,
                created_on=now,
            )

            await self._event_bus.send(
                EventInvitation(
                    organization_id=organization_id,
                    token=token,
                    greeter=author,
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
        author: UserID,
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

        try:
            author_user = org.users[author]
        except KeyError:
            return InviteNewForDeviceBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return InviteNewForDeviceBadOutcome.AUTHOR_REVOKED

        for invitation in org.invitations.values():
            if (
                force_token is None
                and not invitation.is_deleted
                and invitation.type == InvitationType.DEVICE
                and invitation.greeter == author
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
                greeter=author,
                claimer_email=None,
                created_on=now,
            )

            await self._event_bus.send(
                EventInvitation(
                    organization_id=organization_id,
                    token=token,
                    greeter=author,
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
        author: UserID,
        token: InvitationToken,
    ) -> None | InviteCancelBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteCancelBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteCancelBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_user = org.users[author]
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
                greeter=author,
                status=InvitationStatus.CANCELLED,
            )
        )

    @override
    async def list_as_user(
        self, organization_id: OrganizationID, author: UserID
    ) -> list[Invitation] | InviteListBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return InviteListBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteListBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_user = org.users[author]
        except KeyError:
            return InviteListBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return InviteListBadOutcome.AUTHOR_REVOKED

        items = []
        for invitation in org.invitations.values():
            if invitation.greeter != author:
                continue

            status = self._get_invitation_status(organization_id, invitation)
            match invitation.type:
                case InvitationType.USER:
                    assert invitation.claimer_email is not None
                    item = UserInvitation(
                        claimer_email=invitation.claimer_email,
                        token=invitation.token,
                        created_on=invitation.created_on,
                        greeter_user_id=author_user.cooked.user_id,
                        greeter_human_handle=author_user.cooked.human_handle,
                        status=status,
                    )
                case InvitationType.DEVICE:
                    item = DeviceInvitation(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        greeter_user_id=author_user.cooked.user_id,
                        greeter_human_handle=author_user.cooked.human_handle,
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
        greeter = org.users[invitation.greeter].cooked

        match invitation.type:
            case InvitationType.USER:
                assert invitation.claimer_email is not None
                return UserInvitation(
                    claimer_email=invitation.claimer_email,
                    created_on=invitation.created_on,
                    status=self._get_invitation_status(organization_id, invitation),
                    greeter_user_id=greeter.user_id,
                    greeter_human_handle=greeter.human_handle,
                    token=invitation.token,
                )
            case InvitationType.DEVICE:
                return DeviceInvitation(
                    created_on=invitation.created_on,
                    status=self._get_invitation_status(organization_id, invitation),
                    greeter_user_id=greeter.user_id,
                    greeter_human_handle=greeter.human_handle,
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
                current_user_invitations = per_user_invitations[invitation.greeter]
            except KeyError:
                current_user_invitations = []
                per_user_invitations[invitation.greeter] = current_user_invitations
            greeter = org.users[invitation.greeter].cooked
            match invitation.type:
                case InvitationType.USER:
                    assert invitation.claimer_email is not None
                    current_user_invitations.append(
                        UserInvitation(
                            claimer_email=invitation.claimer_email,
                            created_on=invitation.created_on,
                            status=self._get_invitation_status(organization_id, invitation),
                            greeter_user_id=greeter.user_id,
                            greeter_human_handle=greeter.human_handle,
                            token=invitation.token,
                        )
                    )
                case InvitationType.DEVICE:
                    current_user_invitations.append(
                        DeviceInvitation(
                            created_on=invitation.created_on,
                            status=self._get_invitation_status(organization_id, invitation),
                            greeter_user_id=greeter.user_id,
                            greeter_human_handle=greeter.human_handle,
                            token=invitation.token,
                        )
                    )
                case unknown:
                    assert False, unknown

        return per_user_invitations
