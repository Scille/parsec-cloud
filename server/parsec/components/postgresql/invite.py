# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    HumanHandle,
    InvitationStatus,
    InvitationToken,
    InvitationType,
    OrganizationID,
    UserID,
    UserProfile,
)
from parsec.components.events import EventBus
from parsec.components.invite import (
    NEXT_CONDUIT_STATE,
    BaseInviteComponent,
    ConduitListenCtx,
    ConduitState,
    DeviceInvitation,
    Invitation,
    InviteAsInvitedInfoBadOutcome,
    InviteCancelBadOutcome,
    InviteConduitExchangeBadOutcome,
    InviteListBadOutcome,
    InviteNewForDeviceBadOutcome,
    InviteNewForUserBadOutcome,
    SendEmailBadOutcome,
    UserInvitation,
)
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import (
    Q,
    q_device,
    q_device_internal_id,
    q_organization_internal_id,
    q_user,
    q_user_internal_id,
    transaction,
)
from parsec.components.user import CheckDeviceBadOutcome
from parsec.config import BackendConfig
from parsec.events import EventEnrollmentConduit, EventInvitation

_q_retrieve_compatible_user_invitation = Q(
    f"""
SELECT
    token
FROM invitation
LEFT JOIN device ON invitation.created_by = device._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND type = $type
    AND device.user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
    AND claimer_email = $claimer_email
    AND deleted_on IS NULL
LIMIT 1
"""
)

_q_get_human_handle_from_user_id = Q(
    f"""
SELECT
    human.email,
    human.label
FROM human LEFT JOIN user_ ON human._id = user_.human
WHERE
    human.organization = { q_organization_internal_id("$organization_id") }
    AND user_.user_id = $user_id
LIMIT 1
"""
)

_q_retrieve_compatible_device_invitation = Q(
    f"""
SELECT
    token
FROM invitation
INNER JOIN device ON invitation.created_by = device._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND type = $type
    AND device.user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
    AND claimer_email IS NULL
    AND deleted_on IS NULL
LIMIT 1
"""
)


_q_insert_invitation = Q(
    f"""
WITH new_invitations AS (
    INSERT INTO invitation(
        organization,
        token,
        type,
        claimer_email,
        created_by,
        created_on
    )
    VALUES (
        { q_organization_internal_id("$organization_id") },
        $token,
        $type,
        $claimer_email,
        { q_device_internal_id(organization_id="$organization_id", device_id="$created_by") },
        $created_on
    )
    RETURNING _id, created_by
)
INSERT INTO invitation_conduit(
    invitation,
    greeter
)
VALUES (
    (SELECT _id FROM new_invitations),
    (
        SELECT device.user_
        FROM new_invitations
        INNER JOIN device ON new_invitations.created_by = device._id
    )
)
"""
)

_q_delete_invitation = Q(
    """
UPDATE invitation
SET
    deleted_on = $on,
    deleted_reason = $reason
WHERE
    _id = $row_id
"""
)

_q_list_invitations = Q(
    f"""
SELECT
    invitation.token,
    invitation.type,
    { q_user(_id=q_device(_id="invitation.created_by", select="user_"), select="user_id") } as created_by_user_id,
    { q_device(_id="invitation.created_by", select="device_id") } as created_by_device_id,
    human.email,
    human.label,
    invitation.claimer_email,
    invitation.created_on,
    invitation.deleted_on,
    invitation.deleted_reason
FROM invitation
LEFT JOIN device ON invitation.created_by = device._id
LEFT JOIN human ON device.user_ = human._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
ORDER BY created_on
"""
)

_q_list_all_invitations = Q(
    f"""
SELECT
    invitation.token,
    invitation.type,
    { q_user(_id="device.user_", select="user_id") } as created_by_user_id,
    device.device_id as created_by_device_id,
    human.email as created_by_email,
    human.label as created_by_label,
    invitation.claimer_email,
    invitation.created_on,
    invitation.deleted_on,
    invitation.deleted_reason
FROM invitation
LEFT JOIN device ON invitation.created_by = device._id
LEFT JOIN human ON device.user_ = human._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
ORDER BY created_on
"""
)


_q_info_invitation = Q(
    f"""
SELECT
    invitation._id,
    invitation.type,
    { q_user(_id=q_device(_id="invitation.created_by", select="user_"), select="user_id") } as created_by_user_id,
    { q_device(_id="invitation.created_by", select="device_id") } as created_by_device_id,
    human.email,
    human.label,
    invitation.claimer_email,
    invitation.created_on,
    invitation.deleted_on,
    invitation.deleted_reason
FROM invitation
LEFT JOIN device ON invitation.created_by = device._id
LEFT JOIN human ON device.user_ = human._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND token = $token
LIMIT 1
"""
)

_q_get_invitation_created_by = Q(
    f"""
SELECT
    { q_user(_id=q_device(_id="invitation.created_by", select="user_"), select="user_id") } as created_by
FROM invitation
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND token = $token
"""
)

_q_conduit_greeter_info = Q(
    f"""
SELECT
    invitation_conduit._id,
    conduit_state,
    conduit_greeter_payload,
    conduit_claimer_payload,
    deleted_on,
    last_exchange
FROM invitation
INNER JOIN invitation_conduit ON
    invitation._id = invitation_conduit.invitation
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND invitation.token = $token
    AND invitation_conduit.greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
FOR UPDATE
"""
)


_q_conduit_claimer_info = Q(
    f"""
SELECT
    invitation_conduit._id,
    conduit_state,
    conduit_greeter_payload,
    conduit_claimer_payload,
    deleted_on,
    last_exchange
FROM invitation
INNER JOIN invitation_conduit ON
    invitation._id = invitation_conduit.invitation
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND token = $token
    AND invitation_conduit.greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
FOR UPDATE
"""
)


_q_conduit_update = Q(
    """
UPDATE invitation_conduit
SET
    conduit_state = $conduit_state,
    conduit_greeter_payload = $conduit_greeter_payload,
    conduit_claimer_payload = $conduit_claimer_payload
WHERE
    _id = $row_id
"""
)

_q_conduit_update_with_last_exchange = Q(
    """
UPDATE invitation_conduit
SET
    conduit_state = $conduit_state,
    conduit_greeter_payload = $conduit_greeter_payload,
    conduit_claimer_payload = $conduit_claimer_payload,
    last_exchange = $last
WHERE
    _id = $row_id
"""
)

_q_retrieve_active_human_by_email = Q(
    f"""
SELECT
    user_.user_id
FROM user_ LEFT JOIN human ON user_.human=human._id
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    AND human.email = $email
    AND (user_.revoked_on IS NULL OR user_.revoked_on > $now)
LIMIT 1
"""
)


async def query_retrieve_active_human_by_email(
    conn: AsyncpgConnection, organization_id: OrganizationID, email: str
) -> UserID | None:
    result = await conn.fetchrow(
        *_q_retrieve_active_human_by_email(
            organization_id=organization_id.str,
            now=DateTime.now(),
            email=email,
        )
    )
    if result:
        return UserID.from_hex(result["user_id"])
    return None


async def _do_new_user_or_device_invitation(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author_user_id: UserID,
    author_device_id: DeviceID,
    claimer_email: str | None,
    created_on: DateTime,
    invitation_type: InvitationType,
    suggested_token: InvitationToken,
) -> InvitationToken:
    match invitation_type:
        case InvitationType.USER:
            assert claimer_email is not None
            q = _q_retrieve_compatible_user_invitation(
                organization_id=organization_id.str,
                type=invitation_type.str,
                user_id=author_user_id,
                claimer_email=claimer_email,
            )
        case InvitationType.DEVICE:
            q = _q_retrieve_compatible_device_invitation(
                organization_id=organization_id.str,
                type=invitation_type.str,
                user_id=author_user_id,
            )
        case _:
            assert False, "No other invitation type for the moment"

    # Check if no compatible invitations already exists
    row = await conn.fetchrow(*q)
    if row:
        token = InvitationToken.from_hex(row["token"])
    else:
        token = suggested_token
        await conn.execute(
            *_q_insert_invitation(
                organization_id=organization_id.str,
                type=invitation_type.str,
                token=token.hex,
                claimer_email=claimer_email,
                created_by=author_device_id,
                created_on=created_on,
            )
        )
    await send_signal(
        conn,
        EventInvitation(
            organization_id=organization_id,
            token=token,
            greeter=author_user_id,
            status=InvitationStatus.IDLE,
        ),
    )
    return token


async def _human_handle_from_user_id(
    conn: AsyncpgConnection, organization_id: OrganizationID, user_id: UserID
) -> HumanHandle | None:
    result = await conn.fetchrow(
        *_q_get_human_handle_from_user_id(
            organization_id=organization_id.str,
            user_id=user_id,
        )
    )
    if result:
        return HumanHandle(email=result["email"], label=result["label"])
    return None


class PGInviteComponent(BaseInviteComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus, config: BackendConfig) -> None:
        super().__init__(event_bus, config)
        self.pool = pool
        self.organization: PGOrganizationComponent

    def register_components(
        self, organization: PGOrganizationComponent, user: PGUserComponent, **kwargs
    ) -> None:
        self.organization = organization
        self.user = user

    @override
    @transaction
    async def new_for_user(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        claimer_email: str,
        send_email: bool,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForUserBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteNewForUserBadOutcome.ORGANIZATION_NOT_FOUND
        if organization.is_expired:
            return InviteNewForUserBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, current_profile, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteNewForUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteNewForUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteNewForUserBadOutcome.AUTHOR_REVOKED
        if current_profile != UserProfile.ADMIN:
            return InviteNewForUserBadOutcome.AUTHOR_NOT_ALLOWED

        user_id = await query_retrieve_active_human_by_email(conn, organization_id, claimer_email)
        if user_id:
            return InviteNewForUserBadOutcome.CLAIMER_EMAIL_ALREADY_ENROLLED

        suggested_token = force_token or InvitationToken.new()
        token = await _do_new_user_or_device_invitation(
            conn,
            organization_id=organization_id,
            author_user_id=author_user_id,
            author_device_id=author,
            claimer_email=claimer_email,
            created_on=now,
            invitation_type=InvitationType.USER,
            suggested_token=suggested_token,
        )

        if send_email:
            greeter_human_handle = await _human_handle_from_user_id(
                conn, organization_id=organization_id, user_id=author_user_id
            )
            if not greeter_human_handle:
                assert (
                    False
                )  # TODO: Need a specific SendEmailBadOutcome or InviteNewForUserBadOutcome
            send_email_outcome = await self._send_user_invitation_email(
                organization_id=organization_id,
                claimer_email=claimer_email,
                greeter_human_handle=greeter_human_handle,
                token=token,
            )
        else:
            send_email_outcome = None
        return token, send_email_outcome

    @override
    @transaction
    async def new_for_device(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        send_email: bool,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForDeviceBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteNewForDeviceBadOutcome.ORGANIZATION_NOT_FOUND
        if organization.is_expired:
            return InviteNewForDeviceBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteNewForDeviceBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteNewForDeviceBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteNewForDeviceBadOutcome.AUTHOR_REVOKED

        suggested_token = force_token or InvitationToken.new()
        token = await _do_new_user_or_device_invitation(
            conn,
            organization_id=organization_id,
            author_user_id=author_user_id,
            author_device_id=author,
            claimer_email=None,
            created_on=now,
            invitation_type=InvitationType.DEVICE,
            suggested_token=suggested_token,
        )

        if send_email:
            human_handle = await _human_handle_from_user_id(
                conn, organization_id=organization_id, user_id=author_user_id
            )
            if not human_handle:
                assert (
                    False
                )  # TODO: Need a specific SendEmailBadOutcome or InviteNewForUserBadOutcome
            send_email_outcome = await self._send_device_invitation_email(
                organization_id=organization_id,
                email=human_handle.email,
                token=token,
            )
        else:
            send_email_outcome = None
        return token, send_email_outcome

    @override
    @transaction
    async def cancel(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        token: InvitationToken,
    ) -> None | InviteCancelBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteCancelBadOutcome.ORGANIZATION_NOT_FOUND
        if organization.is_expired:
            return InviteCancelBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteCancelBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteCancelBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteCancelBadOutcome.AUTHOR_REVOKED

        row = await conn.fetchrow(
            *_q_info_invitation(organization_id=organization_id.str, token=token.hex)
        )
        if row is None:
            return InviteCancelBadOutcome.INVITATION_NOT_FOUND
        if row["deleted_on"] is not None:
            return InviteCancelBadOutcome.INVITATION_ALREADY_DELETED

        await conn.execute(
            *_q_delete_invitation(
                row_id=row["_id"],
                on=now,
                reason="CANCELLED",  # TODO: use an enum
            )
        )

        await self._event_bus.send(
            EventInvitation(
                organization_id=organization_id,
                token=token,
                greeter=author_user_id,
                status=InvitationStatus.CANCELLED,
            )
        )

    @override
    @transaction
    async def list(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, author: DeviceID
    ) -> list[Invitation] | InviteListBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteListBadOutcome.ORGANIZATION_NOT_FOUND
        if organization.is_expired:
            return InviteListBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteListBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteListBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteListBadOutcome.AUTHOR_REVOKED

        rows = await conn.fetch(
            *_q_list_invitations(organization_id=organization_id.str, user_id=author_user_id)
        )

        invitations_with_claimer_online = self._claimers_ready[organization_id]
        invitations = []
        for (
            token_str,
            type,
            created_by_user_id_str,
            created_by_device_id_str,
            created_by_email,
            created_by_label,
            claimer_email,
            created_on,
            deleted_on,
            deleted_reason,
        ) in rows:
            created_by_user_id = UserID.from_hex(created_by_user_id_str)
            created_by_device_id = DeviceID.from_hex(created_by_device_id_str)
            token = InvitationToken.from_hex(token_str)
            assert created_by_email is not None
            greeter_human_handle = HumanHandle(email=created_by_email, label=created_by_label)

            if deleted_on:
                status = InvitationStatus.from_str(deleted_reason)
            elif token in invitations_with_claimer_online:
                status = InvitationStatus.READY
            else:
                status = InvitationStatus.IDLE

            invitation: Invitation
            if type == InvitationType.USER.str:
                invitation = UserInvitation(
                    created_by_user_id=created_by_user_id,
                    created_by_device_id=created_by_device_id,
                    created_by_human_handle=greeter_human_handle,
                    claimer_email=claimer_email,
                    token=token,
                    created_on=created_on,
                    status=status,
                )
            else:  # Device
                invitation = DeviceInvitation(
                    created_by_user_id=created_by_user_id,
                    created_by_device_id=created_by_device_id,
                    created_by_human_handle=greeter_human_handle,
                    token=token,
                    created_on=created_on,
                    status=status,
                )
            invitations.append(invitation)
        return invitations

    async def _info_as_invited(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, token: InvitationToken
    ) -> Invitation | InviteAsInvitedInfoBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteAsInvitedInfoBadOutcome.ORGANIZATION_NOT_FOUND

        if organization.is_expired:
            return InviteAsInvitedInfoBadOutcome.ORGANIZATION_EXPIRED
        row = await conn.fetchrow(
            *_q_info_invitation(organization_id=organization_id.str, token=token.hex)
        )
        if not row:
            return InviteAsInvitedInfoBadOutcome.INVITATION_NOT_FOUND
        (
            _id,
            type,
            created_by_user_id_str,
            created_by_device_id_str,
            created_by_email,
            created_by_label,
            claimer_email,
            created_on,
            deleted_on,
            _deleted_reason,
        ) = row
        created_by_user_id = UserID.from_hex(created_by_user_id_str)
        created_by_device_id = DeviceID.from_hex(created_by_device_id_str)
        if deleted_on:
            return InviteAsInvitedInfoBadOutcome.INVITATION_DELETED
        assert created_by_email is not None
        greeter_human_handle = HumanHandle(email=created_by_email, label=created_by_label)
        if type == InvitationType.USER.str:
            return UserInvitation(
                created_by_user_id=created_by_user_id,
                created_by_device_id=created_by_device_id,
                created_by_human_handle=greeter_human_handle,
                claimer_email=claimer_email,
                token=token,
                created_on=created_on,
                status=InvitationStatus.READY,
            )
        else:  # Device
            return DeviceInvitation(
                created_by_user_id=created_by_user_id,
                created_by_device_id=created_by_device_id,
                created_by_human_handle=greeter_human_handle,
                token=token,
                created_on=created_on,
                status=InvitationStatus.READY,
            )

    @override
    @transaction
    async def info_as_invited(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, token: InvitationToken
    ) -> Invitation | InviteAsInvitedInfoBadOutcome:
        return await self._info_as_invited(conn, organization_id, token)

    @override
    @transaction
    async def _conduit_talk(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        token: InvitationToken,
        is_greeter: bool,
        state: ConduitState,
        payload: bytes,
        last: bool,  # Only for greeter
    ) -> ConduitListenCtx | InviteConduitExchangeBadOutcome:
        # On top of retrieving the invitation row, this query lock the row
        # in the database for the duration of the transaction.
        # Hence concurrent request will be on hold until the end of the transaction.
        row = await conn.fetchrow(
            *_q_get_invitation_created_by(organization_id=organization_id.str, token=token.hex)
        )
        if not row:
            return InviteConduitExchangeBadOutcome.INVITATION_NOT_FOUND
        # The greeter is the author of the invitation for the moment
        # TODO: make it more flexible later
        greeter = UserID.from_hex(row["created_by"])

        if is_greeter:
            row = await conn.fetchrow(
                *_q_conduit_greeter_info(
                    organization_id=organization_id.str,
                    greeter_user_id=greeter,
                    token=token.hex,
                )
            )
        else:
            row = await conn.fetchrow(
                *_q_conduit_claimer_info(
                    organization_id=organization_id.str,
                    token=token.hex,
                    greeter_user_id=greeter,
                )
            )

        if not row:
            return InviteConduitExchangeBadOutcome.INVITATION_NOT_FOUND

        row_id = row["_id"]
        curr_conduit_state = ConduitState(row["conduit_state"])
        curr_greeter_payload = row["conduit_greeter_payload"]
        curr_claimer_payload = row["conduit_claimer_payload"]
        curr_last_exchange = row["last_exchange"]

        if is_greeter:
            curr_our_payload = curr_greeter_payload
            curr_peer_payload = curr_claimer_payload
        else:
            curr_our_payload = curr_claimer_payload
            curr_peer_payload = curr_greeter_payload

        if row["deleted_on"]:
            return InviteConduitExchangeBadOutcome.INVITATION_DELETED

        if curr_conduit_state != state or curr_our_payload is not None:
            # We are out of sync with the conduit:
            # - the conduit state has changed in our back (maybe reset by the peer)
            # - we want to reset the conduit
            # - we have already provided a payload for the current conduit state (most
            #   likely because a retry of a command that failed due to connection outage)
            if state == ConduitState.STATE_1_WAIT_PEERS:
                # We wait to reset the conduit
                curr_conduit_state = state
                curr_claimer_payload = None
                curr_greeter_payload = None
                curr_our_payload = None
                curr_peer_payload = None
                curr_last_exchange = False
            else:
                return InviteConduitExchangeBadOutcome.ENROLLMENT_WRONG_STATE

        # Now update the conduit with our payload and send a signal if
        # the peer is already waiting for us.
        if is_greeter:
            await conn.execute(
                *_q_conduit_update_with_last_exchange(
                    row_id=row_id,
                    conduit_state=curr_conduit_state.value,
                    conduit_greeter_payload=payload,
                    conduit_claimer_payload=curr_claimer_payload,
                    last=last,
                )
            )
            curr_last_exchange = last
        else:
            await conn.execute(
                *_q_conduit_update(
                    row_id=row_id,
                    conduit_state=curr_conduit_state.value,
                    conduit_greeter_payload=curr_greeter_payload,
                    conduit_claimer_payload=payload,
                )
            )
        # Note that in case of conduit reset, this signal will lure the peer into
        # thinking we have answered so he will wakeup and take into account the reset
        await self._event_bus.send(
            EventEnrollmentConduit(
                organization_id=organization_id,
                token=token,
                greeter=greeter,
            )
        )

        return ConduitListenCtx(
            organization_id=organization_id,
            greeter=greeter,
            token=token,
            state=state,
            is_greeter=is_greeter,
            payload=payload,
            peer_payload=curr_peer_payload,
            is_last_exchange=curr_last_exchange,
        )

    @override
    @transaction
    async def _conduit_listen(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        ctx: ConduitListenCtx,
    ) -> tuple[bytes, bool] | None | InviteConduitExchangeBadOutcome:
        if ctx.is_greeter:
            row = await conn.fetchrow(
                *_q_conduit_greeter_info(
                    organization_id=ctx.organization_id.str,
                    greeter_user_id=ctx.greeter,
                    token=ctx.token.hex,
                )
            )
        else:
            row = await conn.fetchrow(
                *_q_conduit_claimer_info(
                    organization_id=ctx.organization_id.str,
                    greeter_user_id=ctx.greeter,
                    token=ctx.token.hex,
                )
            )

        if not row:
            return InviteConduitExchangeBadOutcome.INVITATION_NOT_FOUND

        row_id = row["_id"]
        curr_conduit_state = ConduitState(row["conduit_state"])
        is_last_exchange = row["last_exchange"]

        # Ignore `invitation.is_deleted` here:
        # - The check has already been done during `_conduit_talk`.
        # - The peer may have already updated the conduit in its final (i.e. deleted)
        #   state, so it's hard to detect that this *is* the last listen allowed.

        if ctx.is_greeter:
            curr_our_payload = row["conduit_greeter_payload"]
            curr_peer_payload = row["conduit_claimer_payload"]
        else:
            curr_our_payload = row["conduit_claimer_payload"]
            curr_peer_payload = row["conduit_greeter_payload"]

        if ctx.peer_payload is None:
            # We are waiting for the peer to provide its payload

            # Only peer payload should be allowed to change
            if curr_conduit_state != ctx.state or curr_our_payload != ctx.payload:
                return InviteConduitExchangeBadOutcome.ENROLLMENT_WRONG_STATE

            if curr_peer_payload is not None:
                # Our peer has provided it payload (hence it knows
                # about our payload too), we can update the conduit
                # to the next state
                await conn.execute(
                    *_q_conduit_update(
                        row_id=row_id,
                        conduit_state=NEXT_CONDUIT_STATE[ctx.state].value,
                        conduit_greeter_payload=None,
                        conduit_claimer_payload=None,
                    )
                )

                # If this was the last exchange, the invitation can be marked as finished
                if ctx.state == ConduitState.STATE_4_COMMUNICATE and is_last_exchange:
                    await conn.execute(
                        *_q_delete_invitation(
                            row_id=row_id,
                            on=now,
                            reason="FINISHED",  # TODO: use an enum
                        )
                    )

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
                return curr_peer_payload, is_last_exchange

        else:
            # We were waiting for the peer to take into account the
            # payload we provided. This would be done once the conduit
            # has switched to it next state.

            if curr_conduit_state == NEXT_CONDUIT_STATE[ctx.state] and curr_our_payload is None:
                # Careful here: it's possible that the other peer has already sent its payload
                # for the new state by the time we reach this point. This also means that it might
                # already have changed the `last_exchange` flag, so we can't rely on what is currently
                # in the database to determine if this is the last exchange. Instead, we return the
                # flag we captured during the `conduit_talk` along with the peer payload.
                return ctx.peer_payload, ctx.is_last_exchange
            elif (
                curr_conduit_state != ctx.state
                or curr_our_payload != ctx.payload
                or curr_peer_payload != ctx.peer_payload
            ):
                # Something unexpected has changed in our back...
                return InviteConduitExchangeBadOutcome.ENROLLMENT_WRONG_STATE

        # Peer hasn't answered yet, we should wait and retry later...
        return None

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
    @transaction
    async def test_dump_all_invitations(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> dict[UserID, list[Invitation]]:
        per_user_invitations = {}
        invitations_with_claimer_online = self._claimers_ready[organization_id]

        # Loop over rows
        rows = await conn.fetch(*_q_list_all_invitations(organization_id=organization_id.str))
        for (
            token_str,
            type,
            created_by_user_id_raw,
            created_by_device_id_raw,
            created_by_email,
            created_by_label,
            claimer_email,
            created_on,
            deleted_on,
            deleted_reason,
        ) in rows:
            # Parse row
            token = InvitationToken.from_hex(token_str)
            created_by_user_id = UserID.from_hex(created_by_user_id_raw)
            created_by_device_id = DeviceID.from_hex(created_by_device_id_raw)
            current_user_invitations = per_user_invitations.setdefault(created_by_user_id, [])
            type = InvitationType.from_str(type)
            created_by_human_handle = HumanHandle(email=created_by_email, label=created_by_label)
            if deleted_on:
                status = InvitationStatus.from_str(deleted_reason)
            elif token in invitations_with_claimer_online:
                status = InvitationStatus.READY
            else:
                status = InvitationStatus.IDLE

            # Append the invite
            match type:
                case InvitationType.USER:
                    current_user_invitations.append(
                        UserInvitation(
                            claimer_email=claimer_email,
                            created_on=created_on,
                            status=status,
                            created_by_user_id=created_by_user_id,
                            created_by_device_id=created_by_device_id,
                            created_by_human_handle=created_by_human_handle,
                            token=token,
                        )
                    )
                case InvitationType.DEVICE:
                    current_user_invitations.append(
                        DeviceInvitation(
                            created_on=created_on,
                            status=status,
                            created_by_user_id=created_by_user_id,
                            created_by_device_id=created_by_device_id,
                            created_by_human_handle=created_by_human_handle,
                            token=token,
                        )
                    )
                case unknown:
                    assert False, unknown

        return per_user_invitations
