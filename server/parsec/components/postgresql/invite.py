# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections.abc import Buffer
from dataclasses import dataclass
from enum import Enum, auto
from typing import TypeAlias, override

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
    invited_cmds,
)
from parsec.components.invite import (
    BaseInviteComponent,
    DeviceInvitation,
    GreetingAttemptCancelledBadOutcome,
    Invitation,
    InvitationCreatedBy,
    InvitationCreatedByExternalService,
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
    UserGreetingAdministrator,
    UserInvitation,
    UserOnlineStatus,
)
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.events import send_signal
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.user import PGUserComponent, UserInfo
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_organization_internal_id,
    q_user_internal_id,
    transaction,
)
from parsec.components.user import CheckDeviceBadOutcome, GetProfileForUserUserBadOutcome
from parsec.config import BackendConfig
from parsec.events import (
    EventGreetingAttemptCancelled,
    EventGreetingAttemptJoined,
    EventGreetingAttemptReady,
    EventInvitation,
)

ShamirRecoveryRecipient: TypeAlias = invited_cmds.latest.invite_info.ShamirRecoveryRecipient


@dataclass(frozen=True)
class BaseInvitationInfo:
    internal_id: int
    token: InvitationToken
    type: InvitationType
    created_by: InvitationCreatedBy
    created_on: DateTime
    deleted_on: DateTime | None
    deleted_reason: InvitationStatus | None

    def is_finished(self) -> bool:
        return self.deleted_reason == InvitationStatus.FINISHED

    def is_cancelled(self) -> bool:
        return self.deleted_reason == InvitationStatus.CANCELLED


@dataclass(frozen=True)
class UserInvitationInfo(BaseInvitationInfo):
    claimer_email: str


@dataclass(frozen=True)
class DeviceInvitationInfo(BaseInvitationInfo):
    claimer_user_id: UserID
    claimer_human_handle: HumanHandle


@dataclass(frozen=True)
class ShamirRecoveryInvitationInfo(BaseInvitationInfo):
    claimer_user_id: UserID
    claimer_human_handle: HumanHandle
    shamir_recovery_setup_internal_id: int
    shamir_recovery_threshold: int
    shamir_recovery_created_on: DateTime
    shamir_recovery_deleted_on: DateTime | None


type InvitationInfo = UserInvitationInfo | DeviceInvitationInfo | ShamirRecoveryInvitationInfo


def invitation_info_from_record(record: Record) -> InvitationInfo:
    match record["invitation_internal_id"]:
        case int() as invitation_internal_id:
            pass
        case unknown:
            assert False, repr(unknown)

    match record["token"]:
        case str() as raw_token:
            token = InvitationToken.from_hex(raw_token)
        case unknown:
            assert False, repr(unknown)

    match record["type"]:
        case str() as raw_type:
            type = InvitationType.from_str(raw_type)
        case unknown:
            assert False, repr(unknown)

    match (record["created_by_user_id"], record["created_by_service_label"]):
        case (None, str() as created_by_service_label):
            created_by = InvitationCreatedByExternalService(service_label=created_by_service_label)
        case (str() as created_by_user_id_str, None):
            match (record["created_by_email"], record["created_by_label"]):
                case (str() as created_by_email, str() as created_by_label):
                    created_by = InvitationCreatedByUser(
                        user_id=UserID.from_hex(created_by_user_id_str),
                        human_handle=HumanHandle(email=created_by_email, label=created_by_label),
                    )
                case unknown:
                    assert False, repr(unknown)
        case unknown:
            assert False, repr(unknown)

    match record["created_on"]:
        case DateTime() as created_on:
            pass
        case unknown:
            assert False, repr(unknown)

    match record["deleted_on"]:
        case DateTime() | None as deleted_on:
            pass
        case unknown:
            assert False, repr(unknown)

    match record["deleted_reason"]:
        case None as deleted_reason:
            pass
        case str() as deleted_reason_str:
            deleted_reason = InvitationStatus.from_str(deleted_reason_str)
        case unknown:
            assert False, repr(unknown)

    if type == InvitationType.USER:
        match record["user_invitation_claimer_email"]:
            case str() as claimer_email:
                pass
            case unknown:
                assert False, repr(unknown)

        assert record["device_invitation_claimer_user_id"] is None
        assert record["device_invitation_claimer_human_email"] is None
        assert record["device_invitation_claimer_human_label"] is None
        assert record["shamir_recovery_internal_id"] is None
        assert record["shamir_recovery_user_id"] is None
        assert record["shamir_recovery_human_email"] is None
        assert record["shamir_recovery_human_label"] is None
        assert record["shamir_recovery_created_on"] is None
        assert record["shamir_recovery_deleted_on"] is None
        assert record["shamir_recovery_threshold"] is None

        return UserInvitationInfo(
            internal_id=invitation_internal_id,
            token=token,
            type=type,
            created_by=created_by,
            created_on=created_on,
            deleted_on=deleted_on,
            deleted_reason=deleted_reason,
            claimer_email=claimer_email,
        )

    if type == InvitationType.DEVICE:
        match record["device_invitation_claimer_user_id"]:
            case str() as claimer_user_id_str:
                claimer_user_id = UserID.from_hex(claimer_user_id_str)
            case unknown:
                assert False, repr(unknown)

        match (
            record["device_invitation_claimer_human_email"],
            record["device_invitation_claimer_human_label"],
        ):
            case (str() as claimer_human_email, str() as claimer_human_label):
                claimer_human_handle = HumanHandle(
                    email=claimer_human_email, label=claimer_human_label
                )
            case unknown:
                assert False, repr(unknown)

        assert record["user_invitation_claimer_email"] is None
        assert record["shamir_recovery_internal_id"] is None
        assert record["shamir_recovery_user_id"] is None
        assert record["shamir_recovery_human_email"] is None
        assert record["shamir_recovery_human_label"] is None
        assert record["shamir_recovery_created_on"] is None
        assert record["shamir_recovery_deleted_on"] is None
        assert record["shamir_recovery_threshold"] is None

        return DeviceInvitationInfo(
            internal_id=invitation_internal_id,
            token=token,
            type=type,
            created_by=created_by,
            created_on=created_on,
            deleted_on=deleted_on,
            deleted_reason=deleted_reason,
            claimer_user_id=claimer_user_id,
            claimer_human_handle=claimer_human_handle,
        )

    if type == InvitationType.SHAMIR_RECOVERY:
        match record["shamir_recovery_internal_id"]:
            case int() as shamir_recovery_setup_internal_id:
                pass
            case unknown:
                assert False, repr(unknown)

        match record["shamir_recovery_user_id"]:
            case str() as shamir_recovery_setup_user_id_str:
                claimer_user_id = UserID.from_hex(shamir_recovery_setup_user_id_str)
            case unknown:
                assert False, repr(unknown)

        match (record["shamir_recovery_human_email"], record["shamir_recovery_human_label"]):
            case (
                str() as shamir_recovery_setup_human_email,
                str() as shamir_recovery_setup_human_label,
            ):
                claimer_human_handle = HumanHandle(
                    email=shamir_recovery_setup_human_email, label=shamir_recovery_setup_human_label
                )
            case unknown:
                assert False, repr(unknown)

        match record["shamir_recovery_threshold"]:
            case int() as shamir_recovery_threshold:
                pass
            case unknown:
                assert False, repr(unknown)

        match record["shamir_recovery_created_on"]:
            case DateTime() as shamir_recovery_created_on:
                pass
            case unknown:
                assert False, repr(unknown)

        match record["shamir_recovery_deleted_on"]:
            case DateTime() | None as shamir_recovery_deleted_on:
                pass
            case unknown:
                assert False, repr(unknown)

        assert record["user_invitation_claimer_email"] is None
        assert record["device_invitation_claimer_user_id"] is None
        assert record["device_invitation_claimer_human_email"] is None
        assert record["device_invitation_claimer_human_label"] is None

        # Treat active invitation to a deleted shamir recovery as cancelled
        if shamir_recovery_deleted_on is not None and deleted_on is None:
            deleted_on = shamir_recovery_deleted_on
            deleted_reason = InvitationStatus.CANCELLED

        return ShamirRecoveryInvitationInfo(
            internal_id=invitation_internal_id,
            token=token,
            type=type,
            created_by=created_by,
            created_on=created_on,
            deleted_on=deleted_on,
            deleted_reason=deleted_reason,
            claimer_user_id=claimer_user_id,
            claimer_human_handle=claimer_human_handle,
            shamir_recovery_setup_internal_id=shamir_recovery_setup_internal_id,
            shamir_recovery_threshold=shamir_recovery_threshold,
            shamir_recovery_created_on=shamir_recovery_created_on,
            shamir_recovery_deleted_on=shamir_recovery_deleted_on,
        )

    assert False, f"Unexpected invitation type: {type}"


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


_q_retrieve_shamir_recovery_setup = Q(
    """
-- We do not lock the shamir topic in read here.
-- This is because topic locking is usually reserved to operations that needs to accept or reject
-- consistent timestamped data from the users, such as certificates or vlobs. Here, only the invitations
-- gets updated, which do not have to remain consistent with other timestamped data.
-- Less lock means less ways to mess up the ordering and create a hard-to-debug deadlock,
-- so we simply accept that our checks might be slightly outdated when the invitation is written.
SELECT shamir_recovery_setup._id AS shamir_recovery_setup_internal_id
FROM shamir_recovery_setup
INNER JOIN organization ON shamir_recovery_setup.organization = organization._id
INNER JOIN user_ ON shamir_recovery_setup.user_ = user_._id
WHERE organization.organization_id = $organization_id
AND user_.user_id = $user_id
AND deleted_on IS NULL
"""
)

_q_retrieve_shamir_recovery_ciphered_data = Q(
    """
SELECT
    ciphered_data,
    reveal_token,
    deleted_on
FROM shamir_recovery_setup
INNER JOIN organization ON shamir_recovery_setup.organization = organization._id
WHERE organization.organization_id = $organization_id
AND shamir_recovery_setup._id = $shamir_recovery_setup_internal_id
"""
)


_q_is_greeter_in_recipients = Q(
    """
SELECT shamir_recovery_share._id
FROM shamir_recovery_share
INNER JOIN user_ ON shamir_recovery_share.recipient = user_._id
WHERE shamir_recovery_share.shamir_recovery = $shamir_recovery_setup_internal_id
AND user_.user_id = $greeter_id
"""
)

_q_get_human_handle_from_user_id = Q(
    f"""
SELECT
    human.email,
    human.label
FROM human LEFT JOIN user_ ON human._id = user_.human
WHERE
    human.organization = {q_organization_internal_id("$organization_id")}
    AND user_.user_id = $user_id
LIMIT 1
"""
)

_q_retrieve_compatible_user_invitation = Q(
    """
SELECT
    token
FROM invitation
INNER JOIN organization ON invitation.organization = organization._id
WHERE
    organization.organization_id = $organization_id
    AND type = 'USER'
    AND user_invitation_claimer_email = $user_invitation_claimer_email
    AND deleted_on IS NULL
LIMIT 1
"""
)

_q_retrieve_compatible_device_invitation = Q(
    """
SELECT
    token
FROM invitation
LEFT JOIN organization ON invitation.organization = organization._id
INNER JOIN user_ ON invitation.device_invitation_claimer = user_._id
WHERE
    organization.organization_id = $organization_id
    AND type = 'DEVICE'
    AND user_.user_id = $device_invitation_claimer_user_id
    AND deleted_on IS NULL
LIMIT 1
"""
)

_q_retrieve_compatible_shamir_recovery_invitation = Q(
    f"""
SELECT
    token
FROM invitation
WHERE
    invitation.organization = {q_organization_internal_id("$organization_id")}
    AND type = 'SHAMIR_RECOVERY'
    AND shamir_recovery = $shamir_recovery_setup
    AND deleted_on IS NULL
LIMIT 1
"""
)

_q_retrieve_shamir_recovery_recipients = Q(
    """
SELECT
    user_.user_id,
    user_.revoked_on,
    human.email,
    human.label,
    shamir_recovery_share.shares
FROM shamir_recovery_share
INNER JOIN user_ ON shamir_recovery_share.recipient = user_._id
INNER JOIN human ON human._id = user_.human
WHERE
    shamir_recovery_share.shamir_recovery = $internal_shamir_recovery_setup_id
"""
)


_q_insert_invitation = Q(
    f"""
INSERT INTO invitation(
    organization,
    token,
    type,
    user_invitation_claimer_email,
    device_invitation_claimer,
    shamir_recovery,
    created_by_device,
    created_on
)
VALUES (
    {q_organization_internal_id("$organization_id")},
    $token,
    $type,
    $user_invitation_claimer_email,
    {q_user_internal_id(organization_id="$organization_id", user_id="$device_invitation_claimer_user_id")},
    $shamir_recovery_setup,
    {q_device_internal_id(organization_id="$organization_id", device_id="$created_by")},
    $created_on
)
RETURNING _id, created_by_device
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
    -- Use DISTINCT to avoid duplicates due to multiple shamir_recovery_share rows
    DISTINCT invitation._id AS invitation_internal_id,
    invitation.token,
    invitation.type,
    created_by_user.user_id AS created_by_user_id,
    created_by_human.email AS created_by_email,
    created_by_human.label AS created_by_label,
    invitation.created_by_service_label,
    invitation.user_invitation_claimer_email,
    device_invitation_claimer.user_id AS device_invitation_claimer_user_id,
    device_invitation_claimer_human.email AS device_invitation_claimer_human_email,
    device_invitation_claimer_human.label AS device_invitation_claimer_human_label,
    invitation.shamir_recovery AS shamir_recovery_internal_id,
    shamir_recovery_user.user_id AS shamir_recovery_user_id,
    shamir_recovery_human.email AS shamir_recovery_human_email,
    shamir_recovery_human.label AS shamir_recovery_human_label,
    shamir_recovery_setup.threshold AS shamir_recovery_threshold,
    shamir_recovery_setup.created_on AS shamir_recovery_created_on,
    shamir_recovery_setup.deleted_on AS shamir_recovery_deleted_on,
    invitation.created_on,
    invitation.deleted_on,
    invitation.deleted_reason
FROM invitation
LEFT JOIN device AS created_by_device ON invitation.created_by_device = created_by_device._id
LEFT JOIN user_ AS created_by_user ON created_by_device.user_ = created_by_user._id
LEFT JOIN human AS created_by_human ON created_by_human._id = created_by_user.human
LEFT JOIN user_ AS device_invitation_claimer ON invitation.device_invitation_claimer = device_invitation_claimer._id
LEFT JOIN human AS device_invitation_claimer_human ON device_invitation_claimer.human = device_invitation_claimer_human._id
LEFT JOIN shamir_recovery_setup ON invitation.shamir_recovery = shamir_recovery_setup._id
LEFT JOIN user_ AS shamir_recovery_user ON shamir_recovery_setup.user_ = shamir_recovery_user._id
LEFT JOIN human AS shamir_recovery_human ON shamir_recovery_user.human = shamir_recovery_human._id
LEFT JOIN shamir_recovery_share ON shamir_recovery_share.shamir_recovery = shamir_recovery_setup._id
LEFT JOIN user_ AS recipient_user_ ON shamir_recovery_share.recipient = recipient_user_._id
WHERE
    invitation.organization = {q_organization_internal_id("$organization_id")}
    -- Different invitation types have different filtering rules
    AND (
        (invitation.type = 'USER' AND $is_admin)
        OR (invitation.type = 'DEVICE' AND device_invitation_claimer.user_id = $user_id)
        OR (invitation.type = 'SHAMIR_RECOVERY' AND recipient_user_.user_id = $user_id)
    )
ORDER BY created_on
"""
)

_q_list_all_invitations = Q(
    f"""
SELECT
    invitation._id AS invitation_internal_id,
    invitation.token,
    invitation.type,
    created_by_user.user_id AS created_by_user_id,
    created_by_human.email AS created_by_email,
    created_by_human.label AS created_by_label,
    invitation.created_by_service_label,
    invitation.user_invitation_claimer_email,
    device_invitation_claimer.user_id AS device_invitation_claimer_user_id,
    device_invitation_claimer_human.email AS device_invitation_claimer_human_email,
    device_invitation_claimer_human.label AS device_invitation_claimer_human_label,
    invitation.shamir_recovery AS shamir_recovery_internal_id,
    shamir_recovery_user.user_id AS shamir_recovery_user_id,
    shamir_recovery_human.email AS shamir_recovery_human_email,
    shamir_recovery_human.label AS shamir_recovery_human_label,
    shamir_recovery_setup.threshold AS shamir_recovery_threshold,
    shamir_recovery_setup.created_on AS shamir_recovery_created_on,
    shamir_recovery_setup.deleted_on AS shamir_recovery_deleted_on,
    invitation.created_on,
    invitation.deleted_on,
    invitation.deleted_reason
FROM invitation
LEFT JOIN device AS created_by_device ON invitation.created_by_device = created_by_device._id
LEFT JOIN user_ AS created_by_user ON created_by_device.user_ = created_by_user._id
LEFT JOIN human AS created_by_human ON created_by_human._id = created_by_user.human
LEFT JOIN user_ AS device_invitation_claimer ON invitation.device_invitation_claimer = device_invitation_claimer._id
LEFT JOIN human AS device_invitation_claimer_human ON device_invitation_claimer.human = device_invitation_claimer_human._id
LEFT JOIN shamir_recovery_setup ON invitation.shamir_recovery = shamir_recovery_setup._id
LEFT JOIN user_ AS shamir_recovery_user ON shamir_recovery_setup.user_ = shamir_recovery_user._id
LEFT JOIN human AS shamir_recovery_human ON shamir_recovery_user.human = shamir_recovery_human._id
WHERE
    invitation.organization = {q_organization_internal_id("$organization_id")}
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
            created_by_user.user_id AS created_by_user_id,
            created_by_human.email AS created_by_email,
            created_by_human.label AS created_by_label,
            invitation.created_by_service_label,
            invitation.user_invitation_claimer_email,
            device_invitation_claimer.user_id AS device_invitation_claimer_user_id,
            device_invitation_claimer_human.email AS device_invitation_claimer_human_email,
            device_invitation_claimer_human.label AS device_invitation_claimer_human_label,
            invitation.shamir_recovery AS shamir_recovery_internal_id,
            shamir_recovery_user.user_id AS shamir_recovery_user_id,
            shamir_recovery_human.email AS shamir_recovery_human_email,
            shamir_recovery_human.label AS shamir_recovery_human_label,
            shamir_recovery_setup.threshold AS shamir_recovery_threshold,
            shamir_recovery_setup.created_on AS shamir_recovery_created_on,
            shamir_recovery_setup.deleted_on AS shamir_recovery_deleted_on,
            invitation.created_on,
            invitation.deleted_on,
            invitation.deleted_reason
        FROM invitation
        INNER JOIN selected_invitation ON invitation._id = selected_invitation.invitation_internal_id
        LEFT JOIN device AS created_by_device ON invitation.created_by_device = created_by_device._id
        LEFT JOIN user_ AS created_by_user ON created_by_device.user_ = created_by_user._id
        LEFT JOIN human AS created_by_human ON created_by_human._id = created_by_user.human
        LEFT JOIN user_ AS device_invitation_claimer ON invitation.device_invitation_claimer = device_invitation_claimer._id
        LEFT JOIN human AS device_invitation_claimer_human ON device_invitation_claimer.human = device_invitation_claimer_human._id
        LEFT JOIN shamir_recovery_setup ON invitation.shamir_recovery = shamir_recovery_setup._id
        LEFT JOIN user_ AS shamir_recovery_user ON shamir_recovery_setup.user_ = shamir_recovery_user._id
        LEFT JOIN human AS shamir_recovery_human ON shamir_recovery_user.human = shamir_recovery_human._id
        """)


_q_info_invitation_for_share = make_q_info_invitation(for_share=True)
_q_info_invitation_for_update = make_q_info_invitation(for_update=True)
_q_info_invitation_for_update_from_greeting_attempt_id = make_q_info_invitation(
    for_update=True, from_greeting_attempt_id=True
)


_q_greeting_attempt_info = Q(
    """
SELECT
    greeting_attempt._id,
    greeting_attempt.greeting_attempt_id,
    user_.user_id AS greeter,
    greeting_attempt.claimer_joined,
    greeting_attempt.greeter_joined,
    greeting_attempt.cancelled_by,
    greeting_attempt.cancelled_reason,
    greeting_attempt.cancelled_on
FROM greeting_attempt
INNER JOIN greeting_session ON greeting_attempt.greeting_session = greeting_session._id
INNER JOIN invitation ON greeting_session.invitation = invitation._id
INNER JOIN user_ ON greeting_session.greeter = user_._id
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
    user_.organization = {q_organization_internal_id("$organization_id")}
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
        {q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id")}
    )
    ON CONFLICT DO NOTHING
    RETURNING greeting_session._id
)
SELECT _id FROM result
UNION ALL SELECT _id
FROM greeting_session
WHERE invitation = $invitation_internal_id
AND greeter = {q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id")}
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
        {q_organization_internal_id("$organization_id")},
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

_q_list_administrators = Q(
    """
SELECT
    user_id
FROM user_
INNER JOIN organization ON user_.organization = organization._id
WHERE
    organization.organization_id = $organization_id
    AND user_.current_profile = 'ADMIN'
    AND user_.revoked_on IS NULL
"""
)

_q_list_non_revoked_recipient = Q(
    """
SELECT
    user_.user_id
FROM shamir_recovery_share
INNER JOIN user_ ON shamir_recovery_share.recipient = user_._id
INNER JOIN organization ON user_.organization = organization._id
WHERE
    organization.organization_id = $organization_id
    AND shamir_recovery_share.shamir_recovery = $shamir_recovery
    AND user_.revoked_on IS NULL
"""
)

_q_list_user_greeting_administrators = Q(
    """
SELECT
    user_.user_id,
    human.email,
    human.label,
    MAX(greeting_attempt.greeter_joined) AS last_greeter_joined
FROM user_
INNER JOIN organization ON user_.organization = organization._id
INNER JOIN human ON user_.human = human._id
-- Use left joins to list all greeting attempts corresponding to a given invitation token
-- for this specific administrator, while ensuring at least one row with NULL values so
-- that the MAX() function returns NULL if no greeting attempt is found.
LEFT JOIN greeting_session ON user_._id = greeting_session.greeter
LEFT JOIN invitation ON greeting_session.invitation = invitation._id
LEFT JOIN greeting_attempt ON greeting_session._id = greeting_attempt.greeting_session AND invitation.token = $token
WHERE
    organization.organization_id = $organization_id
    AND user_.current_profile = 'ADMIN'
    AND user_.revoked_on IS NULL
GROUP BY user_.user_id, human.email, human.label
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


async def _send_invitation_event(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    invitation_info: InvitationInfo,
    status: InvitationStatus,
) -> None:
    match invitation_info:
        case UserInvitationInfo():
            return await _send_invitation_event_for_user(
                conn,
                organization_id,
                invitation_info.token,
                status,
            )
        case DeviceInvitationInfo():
            return await _send_invitation_event_for_device(
                conn,
                organization_id,
                invitation_info.token,
                invitation_info.claimer_user_id,
                status,
            )
        case ShamirRecoveryInvitationInfo():
            return await _send_invitation_event_for_shamir_recovery(
                conn,
                organization_id,
                invitation_info.token,
                invitation_info.shamir_recovery_setup_internal_id,
                status,
            )


async def _send_invitation_event_for_user(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    token: InvitationToken,
    status: InvitationStatus,
) -> None:
    rows = await conn.fetch(
        *_q_list_administrators(
            organization_id=organization_id.str,
        )
    )
    possible_greeters = set()
    for row in rows:
        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
                possible_greeters.add(user_id)
            case unknown:
                assert False, repr(unknown)

    await send_signal(
        conn,
        EventInvitation(
            organization_id=organization_id,
            token=token,
            possible_greeters=possible_greeters,
            status=status,
        ),
    )


async def _send_invitation_event_for_device(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    token: InvitationToken,
    claimer_user_id: UserID,
    status: InvitationStatus,
) -> None:
    # Only the corresponding user can greet a device invitation
    possible_greeters = {claimer_user_id}
    await send_signal(
        conn,
        EventInvitation(
            organization_id=organization_id,
            token=token,
            possible_greeters=possible_greeters,
            status=status,
        ),
    )


async def _send_invitation_event_for_shamir_recovery(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    token: InvitationToken,
    shamir_recovery_setup_internal_id: int,
    status: InvitationStatus,
) -> None:
    # Only the non-revoked recipients can greet a shamir recovery invitation
    rows = await conn.fetch(
        *_q_list_non_revoked_recipient(
            organization_id=organization_id.str,
            shamir_recovery=shamir_recovery_setup_internal_id,
        )
    )
    possible_greeters = set()
    for row in rows:
        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
                possible_greeters.add(user_id)
            case unknown:
                assert False, repr(unknown)

    await send_signal(
        conn,
        EventInvitation(
            organization_id=organization_id,
            token=token,
            possible_greeters=possible_greeters,
            status=status,
        ),
    )


async def _do_new_invitation(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author_user_id: UserID,
    author_device_id: DeviceID,
    user_invitation_claimer_email: str | None,
    device_invitation_claimer_user_id: UserID | None,
    shamir_recovery_setup: int | None,
    created_on: DateTime,
    invitation_type: InvitationType,
    suggested_token: InvitationToken,
) -> InvitationToken:
    match invitation_type:
        case InvitationType.USER:
            assert user_invitation_claimer_email is not None
            q = _q_retrieve_compatible_user_invitation(
                organization_id=organization_id.str,
                user_invitation_claimer_email=user_invitation_claimer_email,
            )
        case InvitationType.DEVICE:
            q = _q_retrieve_compatible_device_invitation(
                organization_id=organization_id.str,
                device_invitation_claimer_user_id=device_invitation_claimer_user_id,
            )
        case InvitationType.SHAMIR_RECOVERY:
            assert shamir_recovery_setup is not None
            q = _q_retrieve_compatible_shamir_recovery_invitation(
                organization_id=organization_id.str,
                shamir_recovery_setup=shamir_recovery_setup,
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
                user_invitation_claimer_email=user_invitation_claimer_email,
                device_invitation_claimer_user_id=device_invitation_claimer_user_id,
                shamir_recovery_setup=shamir_recovery_setup,
                created_by=author_device_id,
                created_on=created_on,
            )
        )

    # Send event to notify the possible greeters
    match invitation_type:
        case InvitationType.USER:
            await _send_invitation_event_for_user(
                conn,
                organization_id=organization_id,
                token=token,
                status=InvitationStatus.PENDING,
            )
        case InvitationType.DEVICE:
            await _send_invitation_event_for_device(
                conn,
                organization_id=organization_id,
                token=token,
                claimer_user_id=author_user_id,
                status=InvitationStatus.PENDING,
            )
        case InvitationType.SHAMIR_RECOVERY:
            assert shamir_recovery_setup is not None
            await _send_invitation_event_for_shamir_recovery(
                conn,
                organization_id=organization_id,
                token=token,
                shamir_recovery_setup_internal_id=shamir_recovery_setup,
                status=InvitationStatus.PENDING,
            )
        case _:
            assert False, "No other invitation type for the moment"

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
    def __init__(self, pool: AsyncpgPool, config: BackendConfig) -> None:
        super().__init__(config)
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
        token = await _do_new_invitation(
            conn,
            organization_id=organization_id,
            author_user_id=author_user_id,
            author_device_id=author,
            user_invitation_claimer_email=claimer_email,
            device_invitation_claimer_user_id=None,
            shamir_recovery_setup=None,
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
        token = await _do_new_invitation(
            conn,
            organization_id=organization_id,
            author_user_id=author_user_id,
            author_device_id=author,
            user_invitation_claimer_email=None,
            device_invitation_claimer_user_id=author_user_id,
            shamir_recovery_setup=None,
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
    async def new_for_shamir_recovery(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        send_email: bool,
        claimer_user_id: UserID,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForShamirRecoveryBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteNewForShamirRecoveryBadOutcome.ORGANIZATION_NOT_FOUND
        if organization.is_expired:
            return InviteNewForShamirRecoveryBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteNewForShamirRecoveryBadOutcome.AUTHOR_REVOKED

        # Check that the claimer exists
        claimer_human_handle = await _human_handle_from_user_id(
            conn, organization_id=organization_id, user_id=claimer_user_id
        )
        if not claimer_human_handle:
            return InviteNewForShamirRecoveryBadOutcome.USER_NOT_FOUND

        row = await conn.fetchrow(
            *_q_retrieve_shamir_recovery_setup(
                organization_id=organization_id.str,
                user_id=claimer_user_id,
            )
        )
        if row is None:
            return InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_ALLOWED
        match row["shamir_recovery_setup_internal_id"]:
            case int() as shamir_recovery_setup:
                pass
            case unknown:
                assert False, repr(unknown)

        shamir_recovery_recipients = await self._get_shamir_recovery_recipients(
            conn, internal_shamir_recovery_setup_id=shamir_recovery_setup
        )
        if not any(author_user_id == recipient.user_id for recipient in shamir_recovery_recipients):
            return InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_ALLOWED

        suggested_token = force_token or InvitationToken.new()
        token = await _do_new_invitation(
            conn,
            organization_id=organization_id,
            author_user_id=author_user_id,
            author_device_id=author,
            user_invitation_claimer_email=None,
            device_invitation_claimer_user_id=None,
            shamir_recovery_setup=shamir_recovery_setup,
            created_on=now,
            invitation_type=InvitationType.SHAMIR_RECOVERY,
            suggested_token=suggested_token,
        )

        if send_email:
            author_human_handle = await _human_handle_from_user_id(
                conn, organization_id=organization_id, user_id=author_user_id
            )
            if not author_human_handle:
                assert (
                    False
                )  # TODO: Need a specific SendEmailBadOutcome or InviteNewForUserBadOutcome

            send_email_outcome = await self._send_shamir_recovery_invitation_email(
                organization_id=organization_id,
                email=claimer_human_handle.email,
                token=token,
                greeter_human_handle=author_human_handle,
            )
        else:
            send_email_outcome = None

        return token, send_email_outcome

    async def _get_shamir_recovery_recipients(
        self, conn: AsyncpgConnection, internal_shamir_recovery_setup_id: int
    ) -> list[ShamirRecoveryRecipient]:
        rows = await conn.fetch(
            *_q_retrieve_shamir_recovery_recipients(
                internal_shamir_recovery_setup_id=internal_shamir_recovery_setup_id
            )
        )
        recipients = [
            ShamirRecoveryRecipient(
                user_id=UserID.from_hex(row["user_id"]),
                human_handle=HumanHandle(email=row["email"], label=row["label"]),
                shares=row["shares"],
                revoked_on=row["revoked_on"],
                online_status=UserOnlineStatus.UNKNOWN,
            )
            for row in rows
        ]
        recipients.sort(key=lambda x: x.human_handle.label)

        return recipients

    async def _get_user_greeting_administrators(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, token: InvitationToken
    ) -> list[UserGreetingAdministrator]:
        administrators = []
        rows = await conn.fetch(
            *_q_list_user_greeting_administrators(
                organization_id=organization_id.str, token=token.hex
            )
        )
        for row in rows:
            match row["user_id"]:
                case str() as raw_user_id:
                    user_id = UserID.from_hex(raw_user_id)
                case unknown:
                    assert False, repr(unknown)

            match (row["email"], row["label"]):
                case (str() as raw_email, str() as raw_label):
                    human_handle = HumanHandle(email=raw_email, label=raw_label)
                case unknown:
                    assert False, repr(unknown)

            match row["last_greeter_joined"]:
                case DateTime() | None as last_greeting_attempt_joined_on:
                    pass
                case unknown:
                    assert False, repr(unknown)

            administrators.append(
                UserGreetingAdministrator(
                    user_id=user_id,
                    human_handle=human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=last_greeting_attempt_joined_on,
                )
            )

        administrators.sort(key=lambda x: x.human_handle.label)
        return administrators

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
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteCancelBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return InviteCancelBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, current_profile, _):
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
        if invitation_info.is_finished():
            return InviteCancelBadOutcome.INVITATION_COMPLETED
        if invitation_info.is_cancelled():
            return InviteCancelBadOutcome.INVITATION_ALREADY_CANCELLED

        match await self.user.get_user_info(conn, organization_id, author_user_id):
            case UserInfo() as author_info:
                pass
            case None:
                return InviteCancelBadOutcome.AUTHOR_NOT_FOUND

        # Only the greeter or the claimer can complete the invitation
        if not await self.is_greeter_allowed(
            conn, invitation_info, author_user_id, current_profile
        ):
            match invitation_info:
                case UserInvitationInfo():
                    if author_info.human_handle.email != invitation_info.claimer_email:
                        return InviteCancelBadOutcome.AUTHOR_NOT_ALLOWED
                case DeviceInvitationInfo() | ShamirRecoveryInvitationInfo():
                    if author_user_id != invitation_info.claimer_user_id:
                        return InviteCancelBadOutcome.AUTHOR_NOT_ALLOWED

        await conn.execute(
            *_q_delete_invitation(
                invitation_internal_id=invitation_info.internal_id,
                on=now,
                reason=InvitationStatus.CANCELLED.str,
            )
        )

        await _send_invitation_event(
            conn,
            organization_id=organization_id,
            invitation_info=invitation_info,
            status=InvitationStatus.PENDING,
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
            case (author_user_id, author_profile, _):
                author_is_admin = author_profile == UserProfile.ADMIN
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return InviteListBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return InviteListBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return InviteListBadOutcome.AUTHOR_REVOKED

        rows = await conn.fetch(
            *_q_list_invitations(
                organization_id=organization_id.str,
                user_id=author_user_id,
                is_admin=author_is_admin,
            )
        )

        invitations = []
        for record in rows:
            invitation_info = invitation_info_from_record(record)

            if invitation_info.deleted_on:
                assert invitation_info.deleted_reason is not None
                status = invitation_info.deleted_reason
            else:
                status = InvitationStatus.PENDING

            invitation: Invitation
            match invitation_info:
                case UserInvitationInfo():
                    # Note that the `administrators` field is actually not used in the context of the `invite_list` command.
                    # Still, we compute it here for consistency. In order to save this unnecessary query to the database,
                    # we could update the invite API to take this difference into account.
                    administrators = await self._get_user_greeting_administrators(
                        conn, organization_id, invitation_info.token
                    )
                    invitation = UserInvitation(
                        created_by=invitation_info.created_by,
                        claimer_email=invitation_info.claimer_email,
                        token=invitation_info.token,
                        created_on=invitation_info.created_on,
                        administrators=administrators,
                        status=status,
                    )
                case DeviceInvitationInfo():
                    invitation = DeviceInvitation(
                        claimer_user_id=invitation_info.claimer_user_id,
                        claimer_human_handle=invitation_info.claimer_human_handle,
                        created_by=invitation_info.created_by,
                        token=invitation_info.token,
                        created_on=invitation_info.created_on,
                        status=status,
                    )
                case ShamirRecoveryInvitationInfo():
                    shamir_recovery_recipients = await self._get_shamir_recovery_recipients(
                        conn, invitation_info.shamir_recovery_setup_internal_id
                    )
                    invitation = ShamirRecoveryInvitation(
                        created_by=invitation_info.created_by,
                        token=invitation_info.token,
                        created_on=invitation_info.created_on,
                        status=status,
                        claimer_user_id=invitation_info.claimer_user_id,
                        claimer_human_handle=invitation_info.claimer_human_handle,
                        threshold=invitation_info.shamir_recovery_threshold,
                        recipients=shamir_recovery_recipients,
                        shamir_recovery_created_on=invitation_info.shamir_recovery_created_on,
                        shamir_recovery_deleted_on=invitation_info.shamir_recovery_deleted_on,
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

        match invitation_info:
            case UserInvitationInfo():
                administrators = await self._get_user_greeting_administrators(
                    conn, organization_id, token
                )
                return UserInvitation(
                    created_by=invitation_info.created_by,
                    claimer_email=invitation_info.claimer_email,
                    token=token,
                    created_on=invitation_info.created_on,
                    administrators=administrators,
                    status=InvitationStatus.PENDING,
                )
            case DeviceInvitationInfo():
                return DeviceInvitation(
                    claimer_user_id=invitation_info.claimer_user_id,
                    claimer_human_handle=invitation_info.claimer_human_handle,
                    created_by=invitation_info.created_by,
                    token=token,
                    created_on=invitation_info.created_on,
                    status=InvitationStatus.PENDING,
                )
            case ShamirRecoveryInvitationInfo():
                shamir_recovery_recipients = await self._get_shamir_recovery_recipients(
                    conn, invitation_info.shamir_recovery_setup_internal_id
                )
                return ShamirRecoveryInvitation(
                    created_by=invitation_info.created_by,
                    token=token,
                    created_on=invitation_info.created_on,
                    status=InvitationStatus.PENDING,
                    claimer_user_id=invitation_info.claimer_user_id,
                    claimer_human_handle=invitation_info.claimer_human_handle,
                    threshold=invitation_info.shamir_recovery_threshold,
                    recipients=shamir_recovery_recipients,
                    shamir_recovery_created_on=invitation_info.shamir_recovery_created_on,
                    shamir_recovery_deleted_on=invitation_info.shamir_recovery_deleted_on,
                )

    @override
    @transaction
    async def info_as_invited(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, token: InvitationToken
    ) -> Invitation | InviteAsInvitedInfoBadOutcome:
        return await self._info_as_invited(conn, organization_id, token)

    @override
    @transaction
    async def shamir_recovery_reveal(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        token: InvitationToken,
        reveal_token: InvitationToken,
    ) -> bytes | InviteShamirRecoveryRevealBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return InviteShamirRecoveryRevealBadOutcome.ORGANIZATION_NOT_FOUND

        if organization.is_expired:
            return InviteShamirRecoveryRevealBadOutcome.ORGANIZATION_EXPIRED

        invitation_info = await self.get_invitation(conn, organization_id, token)
        if not invitation_info:
            return InviteShamirRecoveryRevealBadOutcome.INVITATION_NOT_FOUND

        if invitation_info.deleted_on:
            return InviteShamirRecoveryRevealBadOutcome.INVITATION_DELETED

        if not isinstance(invitation_info, ShamirRecoveryInvitationInfo):
            return InviteShamirRecoveryRevealBadOutcome.BAD_INVITATION_TYPE

        row = await conn.fetchrow(
            *_q_retrieve_shamir_recovery_ciphered_data(
                organization_id=organization_id.str,
                shamir_recovery_setup_internal_id=invitation_info.shamir_recovery_setup_internal_id,
            )
        )
        assert row is not None

        match row["reveal_token"]:
            case str() as reveal_token_str:
                expected_reveal_token = InvitationToken.from_hex(reveal_token_str)
            case unknown:
                assert False, repr(unknown)

        match row["deleted_on"]:
            case DateTime() | None as deleted_on:
                pass
            case unknown:
                assert False, repr(unknown)

        match row["ciphered_data"]:
            case Buffer() as ciphered_data:
                pass
            case unknown:
                assert False, repr(unknown)

        if deleted_on is not None or reveal_token != expected_reveal_token:
            return InviteShamirRecoveryRevealBadOutcome.BAD_REVEAL_TOKEN

        return bytes(ciphered_data)

    @override
    @transaction
    async def test_dump_all_invitations(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> dict[UserID, list[Invitation]]:
        per_user_invitations: dict[UserID, list[Invitation]] = {}

        # Loop over rows
        rows = await conn.fetch(*_q_list_all_invitations(organization_id=organization_id.str))
        for record in rows:
            invitation_info = invitation_info_from_record(record)

            # TODO: Update method to also return invitation created by external services
            if not isinstance(invitation_info.created_by, InvitationCreatedByUser):
                continue

            current_user_invitations = per_user_invitations.setdefault(
                invitation_info.created_by.user_id, []
            )

            if invitation_info.deleted_on:
                assert invitation_info.deleted_reason is not None
                status = invitation_info.deleted_reason
            else:
                status = InvitationStatus.PENDING

            # Append the invite
            match invitation_info:
                case UserInvitationInfo():
                    administrators = await self._get_user_greeting_administrators(
                        conn, organization_id, invitation_info.token
                    )
                    current_user_invitations.append(
                        UserInvitation(
                            claimer_email=invitation_info.claimer_email,
                            created_on=invitation_info.created_on,
                            status=status,
                            created_by=invitation_info.created_by,
                            administrators=administrators,
                            token=invitation_info.token,
                        )
                    )
                case DeviceInvitationInfo():
                    current_user_invitations.append(
                        DeviceInvitation(
                            created_on=invitation_info.created_on,
                            created_by=invitation_info.created_by,
                            status=status,
                            claimer_human_handle=invitation_info.claimer_human_handle,
                            claimer_user_id=invitation_info.claimer_user_id,
                            token=invitation_info.token,
                        )
                    )
                case ShamirRecoveryInvitationInfo():
                    shamir_recovery_recipients = await self._get_shamir_recovery_recipients(
                        conn, invitation_info.shamir_recovery_setup_internal_id
                    )
                    current_user_invitations.append(
                        ShamirRecoveryInvitation(
                            created_on=invitation_info.created_on,
                            status=status,
                            created_by=invitation_info.created_by,
                            token=invitation_info.token,
                            claimer_human_handle=invitation_info.claimer_human_handle,
                            claimer_user_id=invitation_info.claimer_user_id,
                            threshold=invitation_info.shamir_recovery_threshold,
                            recipients=shamir_recovery_recipients,
                            shamir_recovery_created_on=invitation_info.shamir_recovery_created_on,
                            shamir_recovery_deleted_on=invitation_info.shamir_recovery_deleted_on,
                        )
                    )

        return per_user_invitations

    # New invite transport API

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
                now=now,
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

    async def is_greeter_allowed(
        self,
        conn: AsyncpgConnection,
        invitation_info: InvitationInfo,
        greeter_id: UserID,
        greeter_profile: UserProfile,
    ) -> bool:
        match invitation_info:
            case UserInvitationInfo():
                return greeter_profile == UserProfile.ADMIN
            case DeviceInvitationInfo():
                return invitation_info.claimer_user_id == greeter_id
            case ShamirRecoveryInvitationInfo():
                row = await conn.fetchrow(
                    *_q_is_greeter_in_recipients(
                        shamir_recovery_setup_internal_id=invitation_info.shamir_recovery_setup_internal_id,
                        greeter_id=greeter_id,
                    )
                )
                return row is not None

    async def get_invitation(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, token: InvitationToken
    ) -> InvitationInfo | None:
        row = await conn.fetchrow(
            *_q_info_invitation_for_share(organization_id=organization_id.str, token=token.hex)
        )
        return None if row is None else invitation_info_from_record(row)

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
        return None if row is None else invitation_info_from_record(row)

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
        return None if row is None else invitation_info_from_record(row)

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

        if not await self.is_greeter_allowed(conn, invitation_info, greeter, greeter_profile):
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

        if not await self.is_greeter_allowed(conn, invitation_info, greeter, greeter_profile):
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
        if not await self.is_greeter_allowed(conn, invitation_info, greeter, greeter_profile):
            return InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_ALLOWED

        if (cancelled_info := greeting_attempt_info.cancelled_info()) is not None:
            return GreetingAttemptCancelledBadOutcome(*cancelled_info)
        if greeting_attempt_info.greeter_joined is None:
            return InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        await self.cancel_greeting_attempt(
            conn, greeting_attempt_info.internal_id, GreeterOrClaimer.GREETER, reason, now
        )

        await send_signal(
            conn,
            EventGreetingAttemptCancelled(
                organization_id=organization_id,
                token=invitation_info.token,
                greeter=greeting_attempt_info.greeter,
                greeting_attempt=greeting_attempt,
            ),
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

        if not await self.is_greeter_allowed(
            conn, invitation_info, greeting_attempt_info.greeter, greeter_profile
        ):
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETER_NOT_ALLOWED

        if (cancelled_info := greeting_attempt_info.cancelled_info()) is not None:
            return GreetingAttemptCancelledBadOutcome(*cancelled_info)
        if greeting_attempt_info.claimer_joined is None:
            return InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED

        await self.cancel_greeting_attempt(
            conn, greeting_attempt_info.internal_id, GreeterOrClaimer.CLAIMER, reason, now
        )

        await send_signal(
            conn,
            EventGreetingAttemptCancelled(
                organization_id=organization_id,
                token=invitation_info.token,
                greeter=greeting_attempt_info.greeter,
                greeting_attempt=greeting_attempt,
            ),
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
        if not await self.is_greeter_allowed(conn, invitation_info, greeter, greeter_profile):
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
                # When completing the `WAIT_PEER` step, send a `GreetingAttemptJoined` event
                if step_index == 0:
                    await send_signal(
                        conn,
                        EventGreetingAttemptJoined(
                            organization_id=org.organization_id,
                            token=invitation_info.token,
                            greeter=greeting_attempt_info.greeter,
                            greeting_attempt=greeting_attempt,
                        ),
                    )
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

        if not await self.is_greeter_allowed(
            conn, invitation_info, greeting_attempt_info.greeter, greeter_profile
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
                # During the `WAIT_PEER` step, send a `GreetingAttemptReady` event to the greeter
                if step_index == 0:
                    await send_signal(
                        conn,
                        EventGreetingAttemptReady(
                            organization_id=org.organization_id,
                            token=token,
                            greeter=greeting_attempt_info.greeter,
                            greeting_attempt=greeting_attempt,
                        ),
                    )
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
        if not await self.is_greeter_allowed(
            conn, invitation_info, author_user_id, current_profile
        ):
            match invitation_info:
                case UserInvitationInfo():
                    if author_info.human_handle.email != invitation_info.claimer_email:
                        return InviteCompleteBadOutcome.AUTHOR_NOT_ALLOWED
                case DeviceInvitationInfo() | ShamirRecoveryInvitationInfo():
                    if author_user_id != invitation_info.claimer_user_id:
                        return InviteCompleteBadOutcome.AUTHOR_NOT_ALLOWED

        await conn.execute(
            *_q_delete_invitation(
                invitation_internal_id=invitation_info.internal_id,
                on=now,
                reason=InvitationStatus.FINISHED.str,
            )
        )

        await _send_invitation_event(
            conn,
            organization_id=organization_id,
            invitation_info=invitation_info,
            status=InvitationStatus.FINISHED,
        )
