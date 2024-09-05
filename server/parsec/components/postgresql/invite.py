# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections.abc import Buffer
from dataclasses import dataclass
from enum import Enum, auto
from typing import override

from asyncpg import Record

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    DeviceID,
    GreeterOrClaimer,
    GreetingAttemptID,
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
    InviteNewForUserBadOutcome,
    NotReady,
    SendEmailBadOutcome,
    UserInvitation,
)
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.user import PGUserComponent, UserInfo
from parsec.components.postgresql.utils import (
    Q,
    q_device,
    q_device_internal_id,
    q_organization_internal_id,
    q_user,
    q_user_internal_id,
    transaction,
)
from parsec.components.user import CheckDeviceBadOutcome, GetProfileForUserUserBadOutcome
from parsec.config import BackendConfig
from parsec.events import EventInvitation


@dataclass(frozen=True)
class InvitationInfo:
    internal_id: int
    token: InvitationToken
    type: InvitationType
    created_by_user_id: UserID
    created_by_device_id: DeviceID
    created_by_email: str
    created_by_label: str
    claimer_email: str
    created_on: DateTime
    deleted_on: DateTime | None
    deleted_reason: InvitationStatus | None

    def is_finished(self) -> bool:
        return self.deleted_reason == InvitationStatus.FINISHED

    def is_cancelled(self) -> bool:
        return self.deleted_reason == InvitationStatus.CANCELLED

    @classmethod
    def from_record(cls, record: Record) -> InvitationInfo:
        (
            invitation_internal_id,
            token,
            type,
            created_by_user_id_str,
            created_by_device_id_str,
            created_by_email,
            created_by_label,
            claimer_email,
            created_on,
            deleted_on,
            deleted_reason,
        ) = record
        deleted_reason = (
            InvitationStatus.from_str(deleted_reason) if deleted_reason is not None else None
        )
        return cls(
            internal_id=invitation_internal_id,
            token=InvitationToken.from_hex(token),
            type=InvitationType.from_str(type),
            created_by_user_id=UserID.from_hex(created_by_user_id_str),
            created_by_device_id=DeviceID.from_hex(created_by_device_id_str),
            created_by_email=created_by_email,
            created_by_label=created_by_label,
            claimer_email=claimer_email,
            created_on=created_on,
            deleted_on=deleted_on,
            deleted_reason=deleted_reason,
        )


@dataclass(frozen=True)
class GreetingAttemptInfo:
    internal_id: int
    greeting_attempt_id: GreetingAttemptID
    greeter: UserID
    claimer_joined: DateTime | None
    greeter_joined: DateTime | None

    cancelled_by: GreeterOrClaimer | None
    cancelled_reason: CancelledGreetingAttemptReason | None
    cancelled_on: DateTime | None

    def cancelled_info(
        self,
    ) -> tuple[GreeterOrClaimer, CancelledGreetingAttemptReason, DateTime] | None:
        if self.cancelled_by is None or self.cancelled_on is None or self.cancelled_reason is None:
            return None
        return self.cancelled_by, self.cancelled_reason, self.cancelled_on

    @classmethod
    def from_record(cls, record: Record) -> GreetingAttemptInfo:
        (
            greeting_attempt_internal_id,
            greeting_attempt_id,
            greeter,
            greeting_attempt_claimer_joined,
            greeting_attempt_greeter_joined,
            greeting_attempt_cancelled_by,
            greeting_attempt_cancelled_reason,
            greeting_attempt_cancelled_on,
        ) = record
        greeting_attempt_cancelled_reason = (
            CancelledGreetingAttemptReason.from_str(greeting_attempt_cancelled_reason)
            if greeting_attempt_cancelled_reason is not None
            else None
        )
        greeting_attempt_cancelled_by = (
            GreeterOrClaimer.from_str(greeting_attempt_cancelled_by)
            if greeting_attempt_cancelled_by is not None
            else None
        )
        return GreetingAttemptInfo(
            internal_id=greeting_attempt_internal_id,
            greeting_attempt_id=GreetingAttemptID.from_hex(greeting_attempt_id),
            greeter=UserID.from_hex(greeter),
            claimer_joined=greeting_attempt_claimer_joined,
            greeter_joined=greeting_attempt_greeter_joined,
            cancelled_by=greeting_attempt_cancelled_by,
            cancelled_reason=greeting_attempt_cancelled_reason,
            cancelled_on=greeting_attempt_cancelled_on,
        )


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
"""
)

_q_delete_invitation = Q(
    """
UPDATE invitation
SET
    deleted_on = $on,
    deleted_reason = $reason
WHERE
    _id = $invitation_internal_id
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
LEFT JOIN human ON human._id = (SELECT user_.human FROM user_ WHERE user_._id = device.user_)
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
LEFT JOIN human ON human._id = (SELECT user_.human FROM user_ WHERE user_._id = device.user_)
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
ORDER BY created_on
"""
)


def make_q_info_invitation(
    from_greeting_attempt_id: bool = False, for_share: bool = False, for_update: bool = False
) -> Q:
    assert for_update ^ for_share
    share_or_update = "SHARE" if for_share else "UPDATE"

    if from_greeting_attempt_id:
        select_invitation = f"""
            SELECT invitation._id AS invitation_internal_id
            FROM greeting_attempt
            INNER JOIN greeting_session ON greeting_attempt.greeting_session = greeting_session._id
            INNER JOIN invitation ON greeting_session.invitation = invitation._id
            INNER JOIN organization ON invitation.organization = organization._id
            WHERE organization.organization_id = $organization_id
            AND greeting_attempt.greeting_attempt_id = $greeting_attempt_id
            FOR {share_or_update} OF invitation
            """
    else:
        select_invitation = f"""
            SELECT invitation._id AS invitation_internal_id
            FROM invitation
            INNER JOIN organization ON invitation.organization = organization._id
            WHERE organization.organization_id = $organization_id
            AND token = $token
            FOR {share_or_update} OF invitation
            """
    return Q(f"""
        WITH selected_invitation AS ({select_invitation})
        SELECT
            invitation._id AS invitation_internal_id,
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
        INNER JOIN selected_invitation ON invitation._id = selected_invitation.invitation_internal_id
        INNER JOIN device ON invitation.created_by = device._id
        INNER JOIN human ON human._id = (SELECT user_.human FROM user_ WHERE user_._id = device.user_)
        """)


_q_info_invitation_for_share = make_q_info_invitation(for_share=True)
_q_info_invitation_for_update = make_q_info_invitation(for_update=True)
_q_info_invitation_for_update_from_greeting_attempt_id = make_q_info_invitation(
    for_update=True, from_greeting_attempt_id=True
)


_q_greeting_attempt_info = Q(
    f"""
SELECT
    greeting_attempt._id,
    greeting_attempt.greeting_attempt_id,
    { q_user(_id="greeting_session.greeter", select="user_id") } as greeter,
    greeting_attempt.claimer_joined,
    greeting_attempt.greeter_joined,
    greeting_attempt.cancelled_by,
    greeting_attempt.cancelled_reason,
    greeting_attempt.cancelled_on
FROM greeting_attempt
INNER JOIN greeting_session ON greeting_attempt.greeting_session = greeting_session._id
INNER JOIN invitation ON greeting_session.invitation = invitation._id
INNER JOIN device ON invitation.created_by = device._id
INNER JOIN human ON human._id = (SELECT user_.human FROM user_ WHERE user_._id = device.user_)
INNER JOIN organization ON invitation.organization = organization._id
WHERE organization.organization_id = $organization_id
AND greeting_attempt.greeting_attempt_id = $greeting_attempt_id
AND invitation.token = $token
"""
)

_q_retrieve_active_human_by_email = Q(
    f"""
SELECT
    user_.user_id
FROM user_ LEFT JOIN human ON user_.human = human._id
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    AND human.email = $email
    AND (user_.revoked_on IS NULL OR user_.revoked_on > $now)
LIMIT 1
"""
)

_q_lock = Q(
    # Use 66 as magic number to represent invitation creation lock
    # (note this is not strictly needed right now given there is no other
    # advisory lock in the application, but may avoid weird error if we
    # introduce a new advisory lock while forgetting about this one)
    "SELECT pg_advisory_xact_lock(66, _id) FROM organization WHERE organization_id = $organization_id"
)


async def q_take_invitation_create_write_lock(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> None:
    """
    Only a single active invitation for a given email is allowed.

    However we cannot enforce this purely in PostgreSQL (e.g. with a unique index)
    since removing an invitation is not done by deleting its row but by setting
    its `deleted_on` column.

    So the easy way to solve this is to get rid of the concurrency altogether
    (considering invitation creation is far from being performance intensive !)
    by requesting a per-organization PostgreSQL Advisory Lock to be held before
    the invitation creation procedure starts any checks involving the invitations.
    """
    await conn.execute(*_q_lock(organization_id=organization_id.str))


_q_get_or_create_greeting_session = Q(
    f"""
WITH result AS (
    INSERT INTO greeting_session(invitation, greeter)
    VALUES (
        $invitation_internal_id,
        { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
    )
    ON CONFLICT DO NOTHING
    RETURNING greeting_session._id
)
SELECT _id FROM result
UNION ALL SELECT _id
FROM greeting_session
WHERE invitation = $invitation_internal_id
AND greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
LIMIT 1
"""
)

_q_get_or_create_active_attempt = Q(
    f"""
WITH result AS (
    INSERT INTO greeting_attempt(
        organization,
        greeting_attempt_id,
        greeting_session
    )
    VALUES (
        { q_organization_internal_id("$organization_id") },
        $new_greeting_attempt_id,
        $greeting_session_id
    )
    -- The `unique_active_attempt` index ensures that there is only one active attempt at a time
    ON CONFLICT DO NOTHING
    RETURNING greeting_attempt._id, greeting_attempt_id
)
SELECT _id, greeting_attempt_id FROM result
UNION ALL SELECT _id, greeting_attempt_id
FROM greeting_attempt
WHERE greeting_session = $greeting_session_id
AND cancelled_on IS NULL
LIMIT 1
"""
)

_q_greeter_join_or_cancel = Q(
    """
UPDATE greeting_attempt
SET
    greeter_joined = CASE WHEN greeter_joined IS NULL
        THEN $now
        ELSE greeter_joined
        END,
    cancelled_reason = CASE WHEN greeter_joined IS NULL
        THEN cancelled_reason
        ELSE 'AUTOMATICALLY_CANCELLED'
        END,
    cancelled_on = CASE WHEN greeter_joined IS NULL
        THEN cancelled_on
        ELSE $now
        END,
    cancelled_by = CASE WHEN greeter_joined IS NULL
        THEN cancelled_by
        ELSE 'GREETER'
        END
WHERE
    _id = $greeting_attempt_internal_id
RETURNING cancelled_on
"""
)

_q_claimer_join_or_cancel = Q(
    """
UPDATE greeting_attempt
SET
    claimer_joined = CASE WHEN claimer_joined IS NULL
        THEN $now
        ELSE claimer_joined
        END,
    cancelled_reason = CASE WHEN claimer_joined IS NULL
        THEN cancelled_reason
        ELSE 'AUTOMATICALLY_CANCELLED'
        END,
    cancelled_on = CASE WHEN claimer_joined IS NULL
        THEN cancelled_on
        ELSE $now
        END,
    cancelled_by = CASE WHEN claimer_joined IS NULL
        THEN cancelled_by
        ELSE 'CLAIMER'
        END
WHERE
    _id = $greeting_attempt_internal_id
RETURNING cancelled_on
"""
)

_q_cancel_greeting_attempt = Q(
    """
UPDATE greeting_attempt
SET
    cancelled_reason = $cancelled_reason,
    cancelled_on = $cancelled_on,
    cancelled_by = $cancelled_by
WHERE
    _id = $greeting_attempt_internal_id
"""
)

_q_step_check_too_advanced = Q(
    """
SELECT coalesce(
    (
        SELECT
            claimer_data IS NULL or greeter_data IS NULL
        FROM greeting_step
        WHERE
            greeting_attempt = $greeting_attempt_internal_id
            AND step + 1 = $step
    ),
    TRUE
)
"""
)

_q_greeter_step_check_mismatch = Q(
    """
SELECT coalesce(
    (
        SELECT
            greeter_data IS NOT NULL AND greeter_data != $greeter_data
        FROM greeting_step
        WHERE
            greeting_attempt = $greeting_attempt_internal_id
            AND step = $step
    ),
    FALSE
)
"""
)

_q_claimer_step_check_mismatch = Q(
    """
SELECT coalesce(
    (
        SELECT
            claimer_data IS NOT NULL AND claimer_data != $claimer_data
        FROM greeting_step
        WHERE
            greeting_attempt = $greeting_attempt_internal_id
            AND step = $step
    ),
    FALSE
)
"""
)

_q_greeter_step = Q(
    """
INSERT INTO greeting_step(
    greeting_attempt,
    step,
    greeter_data
)
VALUES (
    $greeting_attempt_internal_id,
    $step,
    $greeter_data
)
ON CONFLICT (greeting_attempt, step) DO UPDATE
SET greeter_data = $greeter_data
RETURNING claimer_data
"""
)


_q_claimer_step = Q(
    """
INSERT INTO greeting_step(
    greeting_attempt,
    step,
    claimer_data
)
VALUES (
    $greeting_attempt_internal_id,
    $step,
    $claimer_data
)
ON CONFLICT (greeting_attempt, step) DO UPDATE
SET claimer_data = $claimer_data
RETURNING greeter_data
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

    # Take lock to prevent any concurrent invitation creation
    await q_take_invitation_create_write_lock(conn, organization_id)

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

        invitation_info = await self.lock_invitation(conn, organization_id, token)
        if invitation_info is None:
            return InviteCancelBadOutcome.INVITATION_NOT_FOUND
        if invitation_info.deleted_on is not None:
            return InviteCancelBadOutcome.INVITATION_ALREADY_DELETED

        await conn.execute(
            *_q_delete_invitation(
                invitation_internal_id=invitation_info.internal_id,
                on=now,
                reason=InvitationStatus.CANCELLED.str,
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

        invitation_info = await self.get_invitation(conn, organization_id, token)
        if not invitation_info:
            return InviteAsInvitedInfoBadOutcome.INVITATION_NOT_FOUND

        if invitation_info.deleted_on:
            return InviteAsInvitedInfoBadOutcome.INVITATION_DELETED

        greeter_human_handle = HumanHandle(
            email=invitation_info.created_by_email, label=invitation_info.created_by_label
        )
        if invitation_info.type == InvitationType.USER:
            return UserInvitation(
                created_by_user_id=invitation_info.created_by_user_id,
                created_by_device_id=invitation_info.created_by_device_id,
                created_by_human_handle=greeter_human_handle,
                claimer_email=invitation_info.claimer_email,
                token=token,
                created_on=invitation_info.created_on,
                status=InvitationStatus.READY,
            )
        else:  # Device
            return DeviceInvitation(
                created_by_user_id=invitation_info.created_by_user_id,
                created_by_device_id=invitation_info.created_by_device_id,
                created_by_human_handle=greeter_human_handle,
                token=token,
                created_on=invitation_info.created_on,
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

    # New invite transport API
    # TODO: Remove the old API once the new one is fully functional

    # Helpers

    async def get_greeting_session(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        greeter: UserID,
        invitation_internal_id: int,
    ) -> int:
        greeting_session_id = await conn.fetchval(
            *_q_get_or_create_greeting_session(
                invitation_internal_id=invitation_internal_id,
                organization_id=organization_id.str,
                greeter_user_id=greeter,
            )
        )
        assert greeting_session_id is not None
        return greeting_session_id

    async def get_active_greeting_attempt(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        greeting_session_id: int,
    ) -> tuple[int, GreetingAttemptID]:
        new_greeting_attempt_id = GreetingAttemptID.new()
        row = await conn.fetchrow(
            *_q_get_or_create_active_attempt(
                organization_id=organization_id.str,
                greeting_session_id=greeting_session_id,
                new_greeting_attempt_id=new_greeting_attempt_id,
            )
        )
        assert row is not None
        greeting_attempt_internal_id, greeting_attempt_id = row
        greeting_attempt_id = GreetingAttemptID.from_hex(greeting_attempt_id)
        return greeting_attempt_internal_id, greeting_attempt_id

    async def join_or_cancel(
        self,
        peer: GreeterOrClaimer,
        conn: AsyncpgConnection,
        greeting_attempt_internal_id: int,
        now: DateTime,
    ) -> bool:
        request = (
            _q_greeter_join_or_cancel
            if peer == GreeterOrClaimer.GREETER
            else _q_claimer_join_or_cancel
        )
        cancelled_on = await conn.fetchval(
            *request(
                greeting_attempt_internal_id=greeting_attempt_internal_id,
                now=DateTime.now(),
            )
        )
        return cancelled_on is None

    async def new_attempt(
        self,
        peer: GreeterOrClaimer,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        greeter: UserID,
        invitation_internal_id: int,
        now: DateTime,
    ) -> GreetingAttemptID:
        # Get the greeting session
        greeting_session_id = await self.get_greeting_session(
            conn,
            organization_id=organization_id,
            greeter=greeter,
            invitation_internal_id=invitation_internal_id,
        )
        # Get the active attempt
        (
            greeting_attempt_internal_id,
            greeting_attempt_id,
        ) = await self.get_active_greeting_attempt(
            conn,
            organization_id=organization_id,
            greeting_session_id=greeting_session_id,
        )
        # Try to join the attempt
        is_active = await self.join_or_cancel(peer, conn, greeting_attempt_internal_id, now)
        # The attempt has been joined
        if is_active:
            return greeting_attempt_id
        # The attempt was already joined and has been cancelled
        else:
            # Get a fresh attempt
            (
                greeting_attempt_internal_id,
                greeting_attempt_id,
            ) = await self.get_active_greeting_attempt(
                conn,
                organization_id=organization_id,
                greeting_session_id=greeting_session_id,
            )
            # Join the attempt
            is_active = await self.join_or_cancel(peer, conn, greeting_attempt_internal_id, now)
            # It has to be active since we just created it
            assert is_active
            return greeting_attempt_id

    def is_greeter_allowed(
        self,
        invitation_info: InvitationInfo,
        greeter_id: UserID,
        greeter_profile: UserProfile,
    ) -> bool:
        if invitation_info.type == InvitationType.DEVICE:
            return invitation_info.created_by_user_id == greeter_id
        elif invitation_info.type == InvitationType.USER:
            return greeter_profile == UserProfile.ADMIN
        else:
            raise NotImplementedError

    async def get_invitation(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, token: InvitationToken
    ) -> InvitationInfo | None:
        row = await conn.fetchrow(
            *_q_info_invitation_for_share(organization_id=organization_id.str, token=token.hex)
        )
        return None if row is None else InvitationInfo.from_record(row)

    async def lock_invitation(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        identifier: InvitationToken,
    ) -> InvitationInfo | None:
        row = await conn.fetchrow(
            *_q_info_invitation_for_update(
                organization_id=organization_id.str, token=identifier.hex
            )
        )
        return None if row is None else InvitationInfo.from_record(row)

    async def lock_invitation_from_greeting_attempt_id(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        greeting_attempt_id: GreetingAttemptID,
    ) -> InvitationInfo | None:
        row = await conn.fetchrow(
            *_q_info_invitation_for_update_from_greeting_attempt_id(
                organization_id=organization_id.str, greeting_attempt_id=greeting_attempt_id
            )
        )
        return None if row is None else InvitationInfo.from_record(row)

    async def get_greeting_attempt_info(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        greeting_attempt: GreetingAttemptID,
        invitation_token: InvitationToken,
    ) -> GreetingAttemptInfo | None:
        row = await conn.fetchrow(
            *_q_greeting_attempt_info(
                organization_id=organization_id.str,
                greeting_attempt_id=greeting_attempt,
                token=invitation_token.hex,
            )
        )
        return None if row is None else GreetingAttemptInfo.from_record(row)

    async def cancel_greeting_attempt(
        self,
        conn: AsyncpgConnection,
        greeting_attempt_internal_id: int,
        origin: GreeterOrClaimer,
        reason: CancelledGreetingAttemptReason,
        now: DateTime,
    ) -> None:
        await conn.execute(
            *_q_cancel_greeting_attempt(
                greeting_attempt_internal_id=greeting_attempt_internal_id,
                cancelled_by=origin.str,
                cancelled_reason=reason.str,
                cancelled_on=now,
            )
        )

    class StepOutcome(Enum):
        MISMATCH = auto()
        NOT_READY = auto()
        TOO_ADVANCED = auto()

    async def _greeter_step(
        self,
        conn: AsyncpgConnection,
        greeting_attempt_internal_id: int,
        step: int,
        greeter_data: bytes,
    ) -> bytes | StepOutcome:
        # Check if the step is too advanced
        if step > 0:
            too_advanced = await conn.fetchval(
                *_q_step_check_too_advanced(
                    greeting_attempt_internal_id=greeting_attempt_internal_id,
                    step=step,
                )
            )
            assert too_advanced is not None
            if too_advanced:
                return self.StepOutcome.TOO_ADVANCED
        # Check if the greeter data is a mismatch
        mismatch = await conn.fetchval(
            *_q_greeter_step_check_mismatch(
                greeting_attempt_internal_id=greeting_attempt_internal_id,
                step=step,
                greeter_data=greeter_data,
            )
        )
        if mismatch:
            return self.StepOutcome.MISMATCH
        # Update the step
        claimer_data = await conn.fetchval(
            *_q_greeter_step(
                greeting_attempt_internal_id=greeting_attempt_internal_id,
                step=step,
                greeter_data=greeter_data,
            )
        )
        if claimer_data is None:
            return self.StepOutcome.NOT_READY
        return claimer_data

    async def _claimer_step(
        self,
        conn: AsyncpgConnection,
        greeting_attempt_internal_id: int,
        step: int,
        claimer_data: bytes,
    ) -> bytes | StepOutcome:
        # Check if the step is too advanced
        if step > 0:
            too_advanced = await conn.fetchval(
                *_q_step_check_too_advanced(
                    greeting_attempt_internal_id=greeting_attempt_internal_id,
                    step=step,
                )
            )
            assert too_advanced is not None
            if too_advanced:
                return self.StepOutcome.TOO_ADVANCED
        # Check if the claimer data is a mismatch
        mismatch = await conn.fetchval(
            *_q_claimer_step_check_mismatch(
                greeting_attempt_internal_id=greeting_attempt_internal_id,
                step=step,
                claimer_data=claimer_data,
            )
        )
        assert mismatch is not None
        if mismatch:
            return self.StepOutcome.MISMATCH
        # Update the step
        greeter_data = await conn.fetchval(
            *_q_claimer_step(
                greeting_attempt_internal_id=greeting_attempt_internal_id,
                step=step,
                claimer_data=claimer_data,
            )
        )
        if greeter_data is None:
            return self.StepOutcome.NOT_READY
        return greeter_data

    # Transactions

    @override
    @transaction
    async def greeter_start_greeting_attempt(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        token: InvitationToken,
    ) -> GreetingAttemptID | InviteGreeterStartGreetingAttemptBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteGreeterStartGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteGreeterStartGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (greeter_user_id, greeter_profile, _):
                assert greeter == greeter_user_id
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_REVOKED

        invitation_info = await self.lock_invitation(conn, organization_id, token)
        if invitation_info is None:
            return InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_NOT_FOUND
        if invitation_info.is_finished():
            return InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_COMPLETED
        if invitation_info.is_cancelled():
            return InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(invitation_info, greeter, greeter_profile):
            return InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_NOT_ALLOWED

        greeter_attempt_id = await self.new_attempt(
            GreeterOrClaimer.GREETER,
            conn,
            organization_id=organization_id,
            greeter=greeter,
            invitation_internal_id=invitation_info.internal_id,
            now=now,
        )
        return greeter_attempt_id

    @override
    @transaction
    async def claimer_start_greeting_attempt(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeter: UserID,
    ) -> GreetingAttemptID | InviteClaimerStartGreetingAttemptBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteClaimerStartGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteClaimerStartGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._get_profile_for_user(conn, organization_id, greeter):
            case UserProfile() as greeter_profile:
                pass
            case GetProfileForUserUserBadOutcome.USER_NOT_FOUND:
                return InviteClaimerStartGreetingAttemptBadOutcome.GREETER_NOT_FOUND
            case GetProfileForUserUserBadOutcome.USER_REVOKED:
                return InviteClaimerStartGreetingAttemptBadOutcome.GREETER_REVOKED

        invitation_info = await self.lock_invitation(conn, organization_id, token)
        if invitation_info is None:
            return InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_NOT_FOUND
        if invitation_info.is_finished():
            return InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_COMPLETED
        if invitation_info.is_cancelled():
            return InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_CANCELLED

        if not self.is_greeter_allowed(invitation_info, greeter, greeter_profile):
            return InviteClaimerStartGreetingAttemptBadOutcome.GREETER_NOT_ALLOWED

        greeter_attempt_id = await self.new_attempt(
            GreeterOrClaimer.CLAIMER,
            conn,
            organization_id=organization_id,
            greeter=greeter,
            invitation_internal_id=invitation_info.internal_id,
            now=now,
        )
        return greeter_attempt_id

    @override
    @transaction
    async def greeter_cancel_greeting_attempt(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        greeting_attempt: GreetingAttemptID,
        reason: CancelledGreetingAttemptReason,
    ) -> None | InviteGreeterCancelGreetingAttemptBadOutcome | GreetingAttemptCancelledBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteGreeterCancelGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteGreeterCancelGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (greeter_user_id, greeter_profile, _):
                assert greeter == greeter_user_id
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_REVOKED

        invitation_info = await self.lock_invitation_from_greeting_attempt_id(
            conn, organization_id, greeting_attempt
        )
        if invitation_info is None:
            return InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND
        if invitation_info.is_finished():
            return InviteGreeterCancelGreetingAttemptBadOutcome.INVITATION_COMPLETED
        if invitation_info.is_cancelled():
            return InviteGreeterCancelGreetingAttemptBadOutcome.INVITATION_CANCELLED

        greeting_attempt_info = await self.get_greeting_attempt_info(
            conn, organization_id, greeting_attempt, invitation_info.token
        )
        if greeting_attempt_info is None or greeting_attempt_info.greeter != greeter:
            return InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND
        if not self.is_greeter_allowed(invitation_info, greeter, greeter_profile):
            return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_ALLOWED

        if (cancelled_info := greeting_attempt_info.cancelled_info()) is not None:
            return GreetingAttemptCancelledBadOutcome(*cancelled_info)
        if greeting_attempt_info.greeter_joined is None:
            return InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        await self.cancel_greeting_attempt(
            conn, greeting_attempt_info.internal_id, GreeterOrClaimer.GREETER, reason, now
        )

    @override
    @transaction
    async def claimer_cancel_greeting_attempt(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeting_attempt: GreetingAttemptID,
        reason: CancelledGreetingAttemptReason,
    ) -> None | InviteClaimerCancelGreetingAttemptBadOutcome | GreetingAttemptCancelledBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteClaimerCancelGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteClaimerCancelGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED

        # Lock common topic because we need it later to check if the greeter is allowed
        # Still, we want to lock it before the invitation to have a consistent lock order
        await self.user._check_common_topic(conn, organization_id)

        invitation_info = await self.lock_invitation(conn, organization_id, token)
        if invitation_info is None:
            return InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_NOT_FOUND
        if invitation_info.is_finished():
            return InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_COMPLETED
        if invitation_info.is_cancelled():
            return InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_CANCELLED

        greeting_attempt_info = await self.get_greeting_attempt_info(
            conn, organization_id, greeting_attempt, token
        )
        if greeting_attempt_info is None:
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND

        match await self.user._get_profile_for_user(
            conn, organization_id, greeting_attempt_info.greeter, check_common_topic=False
        ):
            case UserProfile() as greeter_profile:
                pass
            case GetProfileForUserUserBadOutcome.USER_NOT_FOUND:
                assert False
            case GetProfileForUserUserBadOutcome.USER_REVOKED:
                return InviteClaimerCancelGreetingAttemptBadOutcome.GREETER_REVOKED

        if not self.is_greeter_allowed(
            invitation_info, greeting_attempt_info.greeter, greeter_profile
        ):
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETER_NOT_ALLOWED

        if (cancelled_info := greeting_attempt_info.cancelled_info()) is not None:
            return GreetingAttemptCancelledBadOutcome(*cancelled_info)
        if greeting_attempt_info.claimer_joined is None:
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        await self.cancel_greeting_attempt(
            conn, greeting_attempt_info.internal_id, GreeterOrClaimer.CLAIMER, reason, now
        )

    @override
    @transaction
    async def greeter_step(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        greeting_attempt: GreetingAttemptID,
        step_index: int,
        greeter_data: bytes,
    ) -> bytes | NotReady | InviteGreeterStepBadOutcome | GreetingAttemptCancelledBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteGreeterStepBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteGreeterStepBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (greeter_user_id, greeter_profile, _):
                assert greeter == greeter_user_id
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteGreeterStepBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteGreeterStepBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteGreeterStepBadOutcome.AUTHOR_REVOKED

        invitation_info = await self.lock_invitation_from_greeting_attempt_id(
            conn, organization_id, greeting_attempt
        )
        if invitation_info is None:
            return InviteGreeterStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND
        if invitation_info.is_finished():
            return InviteGreeterStepBadOutcome.INVITATION_COMPLETED
        if invitation_info.is_cancelled():
            return InviteGreeterStepBadOutcome.INVITATION_CANCELLED

        greeting_attempt_info = await self.get_greeting_attempt_info(
            conn, organization_id, greeting_attempt, invitation_info.token
        )
        if greeting_attempt_info is None or greeting_attempt_info.greeter != greeter:
            return InviteGreeterStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND
        if not self.is_greeter_allowed(invitation_info, greeter, greeter_profile):
            return InviteGreeterStepBadOutcome.AUTHOR_NOT_ALLOWED

        if (cancelled_info := greeting_attempt_info.cancelled_info()) is not None:
            return GreetingAttemptCancelledBadOutcome(*cancelled_info)
        if greeting_attempt_info.greeter_joined is None:
            return InviteGreeterStepBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        match await self._greeter_step(
            conn, greeting_attempt_info.internal_id, step_index, greeter_data
        ):
            case self.StepOutcome.MISMATCH:
                return InviteGreeterStepBadOutcome.STEP_MISMATCH
            case self.StepOutcome.TOO_ADVANCED:
                return InviteGreeterStepBadOutcome.STEP_TOO_ADVANCED
            case self.StepOutcome.NOT_READY:
                return NotReady()
            case Buffer() as data:
                return data

    @override
    @transaction
    async def claimer_step(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeting_attempt: GreetingAttemptID,
        step_index: int,
        claimer_data: bytes,
    ) -> bytes | NotReady | InviteClaimerStepBadOutcome | GreetingAttemptCancelledBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteClaimerStepBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteClaimerStepBadOutcome.ORGANIZATION_EXPIRED

        # Lock common topic because we need it later to check if the greeter is allowed
        # Still, we want to lock it before the invitation to have a consistent lock order
        await self.user._check_common_topic(conn, organization_id)

        invitation_info = await self.lock_invitation(conn, organization_id, token)
        if invitation_info is None:
            return InviteClaimerStepBadOutcome.INVITATION_NOT_FOUND
        if invitation_info.is_finished():
            return InviteClaimerStepBadOutcome.INVITATION_COMPLETED
        if invitation_info.is_cancelled():
            return InviteClaimerStepBadOutcome.INVITATION_CANCELLED

        greeting_attempt_info = await self.get_greeting_attempt_info(
            conn, organization_id, greeting_attempt, token
        )
        if greeting_attempt_info is None:
            return InviteClaimerStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND

        match await self.user._get_profile_for_user(
            conn, organization_id, greeting_attempt_info.greeter, check_common_topic=False
        ):
            case UserProfile() as greeter_profile:
                pass
            case GetProfileForUserUserBadOutcome.USER_NOT_FOUND:
                assert False
            case GetProfileForUserUserBadOutcome.USER_REVOKED:
                return InviteClaimerStepBadOutcome.GREETER_REVOKED

        if not self.is_greeter_allowed(
            invitation_info, greeting_attempt_info.greeter, greeter_profile
        ):
            return InviteClaimerStepBadOutcome.GREETER_NOT_ALLOWED

        if (cancelled_info := greeting_attempt_info.cancelled_info()) is not None:
            return GreetingAttemptCancelledBadOutcome(*cancelled_info)
        if greeting_attempt_info.claimer_joined is None:
            return InviteClaimerStepBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        match await self._claimer_step(
            conn, greeting_attempt_info.internal_id, step_index, claimer_data
        ):
            case self.StepOutcome.MISMATCH:
                return InviteClaimerStepBadOutcome.STEP_MISMATCH
            case self.StepOutcome.TOO_ADVANCED:
                return InviteClaimerStepBadOutcome.STEP_TOO_ADVANCED
            case self.StepOutcome.NOT_READY:
                return NotReady()
            case Buffer() as data:
                return data

    @override
    @transaction
    async def complete(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        token: InvitationToken,
    ) -> None | InviteCompleteBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteCompleteBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteCompleteBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, current_profile, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteCompleteBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteCompleteBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteCompleteBadOutcome.AUTHOR_REVOKED
        if current_profile != UserProfile.ADMIN:
            return InviteCompleteBadOutcome.AUTHOR_NOT_ALLOWED

        invitation_info = await self.lock_invitation(conn, organization_id, token)
        if invitation_info is None:
            return InviteCompleteBadOutcome.INVITATION_NOT_FOUND
        if invitation_info.is_finished():
            return InviteCompleteBadOutcome.INVITATION_ALREADY_COMPLETED
        if invitation_info.is_cancelled():
            return InviteCompleteBadOutcome.INVITATION_CANCELLED

        match await self.user.get_user_info(conn, organization_id, author_user_id):
            case UserInfo() as author_info:
                pass
            case None:
                return InviteCompleteBadOutcome.AUTHOR_NOT_FOUND

        # Only the greeter or the claimer can complete the invitation
        if not self.is_greeter_allowed(invitation_info, author_user_id, current_profile):
            if not invitation_info.claimer_email == author_info.human_handle.email:
                return InviteCompleteBadOutcome.AUTHOR_NOT_ALLOWED

        await conn.execute(
            *_q_delete_invitation(
                invitation_internal_id=invitation_info.internal_id,
                on=now,
                reason="FINISHED",  # TODO: use an enum see #8224
            )
        )

        await self._event_bus.send(
            EventInvitation(
                organization_id=organization_id,
                token=token,
                greeter=author_user_id,
                status=InvitationStatus.FINISHED,
            )
        )
