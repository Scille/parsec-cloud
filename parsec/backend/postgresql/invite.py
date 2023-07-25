# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any, List

import triopg

from parsec._parsec import DateTime, InvitationDeletedReason, ShamirRecoveryRecipient
from parsec.api.protocol import (
    HumanHandle,
    InvitationStatus,
    InvitationToken,
    InvitationType,
    OrganizationID,
    UserID,
)
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
from parsec.backend.postgresql.handler import PGHandler, send_signal
from parsec.backend.postgresql.user_queries.find import query_retrieve_active_human_by_email
from parsec.backend.postgresql.utils import (
    Q,
    q_organization_internal_id,
    q_user,
    q_user_internal_id,
)

_q_retrieve_compatible_user_invitation = Q(
    f"""
SELECT
    token
FROM invitation
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND type = $type
    AND greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
    AND claimer_email = $claimer_email
    AND deleted_on IS NULL
LIMIT 1
"""
)


_q_retrieve_compatible_device_invitation = Q(
    f"""
SELECT
    token
FROM invitation
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND type = $type
    AND greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
    AND claimer_email IS NULL
    AND deleted_on IS NULL
LIMIT 1
"""
)


# We use `FOR UPDATE` even though there's no row to lock in practice
# as the insertion will only occur if no invitation token is returned
_q_retrieve_compatible_shamir_recovery_invitation = Q(
    f"""
SELECT
    invitation.token
FROM invitation
JOIN user_
    ON invitation.shamir_recovery = user_.shamir_recovery
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND invitation.type = '{ InvitationType.SHAMIR_RECOVERY.str }'
    AND user_.user_id = $claimer_user_id
    AND invitation.deleted_on IS NULL
LIMIT 1
FOR UPDATE
"""
)


_q_insert_invitation = Q(
    f"""
INSERT INTO invitation(
    organization,
    token,
    type,
    greeter,
    shamir_recovery,
    claimer_email,
    created_on
)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $token,
    $type,
    { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") },
    $shamir_recovery,
    $claimer_email,
    $created_on
)
RETURNING _id
"""
)

_q_insert_shamir_conduit = Q(
    f"""
INSERT INTO shamir_recovery_conduit(
    invitation,
    greeter
)
VALUES (
    $invitation,
    { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
)
"""
)


_q_delete_invitation_info = Q(
    f"""
SELECT
    _id,
    deleted_on
FROM invitation
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND type != '{ InvitationType.SHAMIR_RECOVERY.str }'
    AND greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$deleter")}
    AND token = $token
FOR UPDATE
"""
)

_q_recipient_delete_shamir_recovery_invitation_info = Q(
    f"""
SELECT
    invitation._id,
    deleted_on
FROM invitation
JOIN shamir_recovery_conduit
    ON invitation._id = shamir_recovery_conduit.invitation
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND invitation.token = $token
    AND type = '{ InvitationType.SHAMIR_RECOVERY.str }'
    AND shamir_recovery_conduit.greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$deleter")}
FOR UPDATE
"""
)

_q_claimer_delete_shamir_recovery_invitation_info = Q(
    f"""
SELECT
    invitation._id,
    deleted_on
FROM invitation
JOIN shamir_recovery_setup
    ON invitation.shamir_recovery = shamir_recovery_setup._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND invitation.token = $token
    AND type = '{ InvitationType.SHAMIR_RECOVERY.str }'
    AND shamir_recovery_setup.user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$deleter") }
FOR UPDATE
"""
)

_q_new_setup_delete_shamir_recovery_invitation_info = Q(
    f"""
SELECT
    invitation._id,
    invitation.token
FROM invitation
JOIN shamir_recovery_setup
    ON invitation.shamir_recovery = shamir_recovery_setup._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND type = '{ InvitationType.SHAMIR_RECOVERY.str }'
    AND shamir_recovery_setup.user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter")}
    AND deleted_on IS NULL
FOR UPDATE
"""
)

_q_delete_invitation = Q(
    f"""
UPDATE invitation
SET
    deleted_on = $on,
    deleted_reason = $reason
WHERE
    _id = $row_id
"""
)


_q_get_shamir_recovery_id_for_new_invitation = Q(
    f"""
SELECT
    shamir_recovery
FROM user_
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND user_id = $user_id
FOR UPDATE
"""
)

_q_list_recipients_for_new_invitation = Q(
    f"""
SELECT
    { q_user(_id="recipient", select="user_id") }
FROM shamir_recovery_share
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND shamir_recovery = $shamir_recovery
FOR UPDATE
"""
)

_q_get_shamir_recovery_info = Q(
    f"""
SELECT
    shamir_recovery,
    threshold
FROM invitation
LEFT JOIN shamir_recovery_setup
    ON invitation.shamir_recovery = shamir_recovery_setup._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND invitation.token = $token
"""
)

_q_get_shamir_recovery_recipients = Q(
    f"""
SELECT
    human.email,
    human.label,
    user_.user_id,
    shares
FROM shamir_recovery_share
JOIN user_
    ON shamir_recovery_share.recipient = user_._id
LEFT JOIN human
    ON user_.human = human._id
WHERE
    shamir_recovery_share.organization = { q_organization_internal_id("$organization_id") }
    AND shamir_recovery_share.shamir_recovery = $shamir_recovery
"""
)


async def _shamir_info(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    token: InvitationToken,
) -> tuple[int, tuple[ShamirRecoveryRecipient, ...]]:
    internal_id, threshold = await conn.fetchrow(
        *_q_get_shamir_recovery_info(
            organization_id=organization_id.str,
            token=token,
        )
    )
    recipients = []
    for email, label, recipient_id, shares in await conn.fetch(
        *_q_get_shamir_recovery_recipients(
            organization_id=organization_id.str, shamir_recovery=internal_id
        )
    ):
        recipients.append(
            ShamirRecoveryRecipient(
                user_id=UserID(recipient_id),
                human_handle=HumanHandle(email, label),
                shares=shares,
            )
        )

    return (threshold, tuple(recipients))


async def delete_shamir_recovery_invitation_if_it_exists(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    user_id: UserID,
) -> None:
    on = DateTime.now()
    reason = InvitationDeletedReason.CANCELLED
    rows = await conn.fetch(
        *_q_new_setup_delete_shamir_recovery_invitation_info(
            organization_id=organization_id.str, greeter=user_id.str
        )
    )
    # In practice, there should be at most one element in rows
    for row_id, raw_token in rows:
        token = InvitationToken.from_hex(raw_token)
        _, recipients = await _shamir_info(conn, organization_id, token)
        await conn.execute(*_q_delete_invitation(row_id=row_id, on=on, reason=reason.str))
        await send_signal(
            conn,
            BackendEvent.INVITE_STATUS_CHANGED,
            organization_id=organization_id,
            greeters=[recipient.user_id for recipient in recipients],
            token=token,
            status=InvitationStatus.DELETED,
        )


async def _do_delete_invitation(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    deleter: UserID,
    token: InvitationToken,
    on: DateTime,
    reason: InvitationDeletedReason,
) -> None:
    # Try user/device invitation first
    row = await conn.fetchrow(
        *_q_delete_invitation_info(
            organization_id=organization_id.str, deleter=deleter.str, token=token
        )
    )

    # Look for shamir recovery invitation otherwise
    is_user_or_device_invitation = bool(row)
    if not row:
        row = await conn.fetchrow(
            *_q_recipient_delete_shamir_recovery_invitation_info(
                organization_id=organization_id.str, deleter=deleter.str, token=token
            )
        )
    # Shamir recovery when the claimer deletes its own invitation
    if not row:
        row = await conn.fetchrow(
            *_q_claimer_delete_shamir_recovery_invitation_info(
                organization_id=organization_id.str, deleter=deleter.str, token=token
            )
        )
    if not row:
        raise InvitationNotFoundError(token)
    row_id, deleted_on = row
    if deleted_on:
        raise InvitationAlreadyDeletedError(token)

    if is_user_or_device_invitation:
        greeters = [deleter]
    else:
        _, recipients = await _shamir_info(conn, organization_id, token)
        greeters = [recipient.user_id for recipient in recipients]

    await conn.execute(*_q_delete_invitation(row_id=row_id, on=on, reason=reason.str))
    await send_signal(
        conn,
        BackendEvent.INVITE_STATUS_CHANGED,
        organization_id=organization_id,
        greeters=greeters,
        token=token,
        status=InvitationStatus.DELETED,
    )


_q_human_handle_per_user = f"""
SELECT user_._id AS user_, email, label
FROM human LEFT JOIN user_
    ON human._id = user_.human
WHERE
    human.organization = { q_organization_internal_id("$organization_id") }
    AND user_.revoked_on IS NULL
"""


_q_get_email = Q(
    f"""
SELECT
    human.email
FROM human LEFT JOIN user_
    ON human._id = user_.human
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    AND user_.user_id = $user_id
    AND user_.revoked_on IS NULL
"""
)

_q_get_invitation_type = Q(
    f"""
SELECT
    type
FROM invitation
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND token = $token
FOR UPDATE
"""
)

_q_list_device_and_user_invitations = Q(
    f"""
WITH human_handle_per_user AS ({_q_human_handle_per_user})
SELECT
    token,
    type,
    { q_user(_id="greeter", select="user_id") },
    human_handle_per_user.email,
    human_handle_per_user.label,
    claimer_email,
    created_on,
    deleted_on,
    deleted_reason
FROM invitation LEFT JOIN human_handle_per_user
    ON invitation.greeter = human_handle_per_user.user_
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
    AND type != '{ InvitationType.SHAMIR_RECOVERY.str }'
    AND deleted_on IS NULL
ORDER BY created_on
"""
)


_q_info_invitation = Q(
    f"""
WITH human_handle_per_user AS ({_q_human_handle_per_user})
SELECT
    type,
    { q_user(_id="greeter", select="user_id") },
    human_handle_per_user.email,
    human_handle_per_user.label,
    claimer_email,
    { q_user(_id="shamir_recovery_setup.user_", select="user_id") },
    created_on,
    deleted_on,
    deleted_reason
FROM invitation
LEFT JOIN human_handle_per_user
    ON invitation.greeter = human_handle_per_user.user_
LEFT JOIN shamir_recovery_setup
    ON invitation.shamir_recovery = shamir_recovery_setup._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND token = $token
LIMIT 1
"""
)


_q_conduit_device_and_user_greeter_info = Q(
    f"""
SELECT
    _id,
    conduit_state,
    conduit_greeter_payload,
    conduit_claimer_payload,
    deleted_on
FROM invitation
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND type != '{ InvitationType.SHAMIR_RECOVERY.str }'
    AND token = $token
    AND greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
FOR UPDATE
"""
)

_q_conduit_shamir_recovery_greeter_info = Q(
    f"""
SELECT
    shamir_recovery_conduit._id,
    shamir_recovery_conduit.conduit_state,
    shamir_recovery_conduit.conduit_greeter_payload,
    shamir_recovery_conduit.conduit_claimer_payload,
    deleted_on
FROM invitation
JOIN shamir_recovery_conduit
    ON invitation._id = shamir_recovery_conduit.invitation
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND invitation.token = $token
    AND type = '{ InvitationType.SHAMIR_RECOVERY.str }'
    AND shamir_recovery_conduit.greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
FOR UPDATE
"""
)


_q_conduit_device_and_user_claimer_info = Q(
    f"""
SELECT
    _id,
    conduit_state,
    conduit_greeter_payload,
    conduit_claimer_payload,
    deleted_on
FROM invitation
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND token = $token
    AND type != '{ InvitationType.SHAMIR_RECOVERY.str }'
FOR UPDATE
"""
)


_q_conduit_shamir_recovery_claimer_info = Q(
    f"""
SELECT
    shamir_recovery_conduit._id,
    shamir_recovery_conduit.conduit_state,
    shamir_recovery_conduit.conduit_greeter_payload,
    shamir_recovery_conduit.conduit_claimer_payload,
    deleted_on
FROM invitation
JOIN shamir_recovery_conduit
    ON invitation._id = shamir_recovery_conduit.invitation
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND invitation.token = $token
    AND shamir_recovery_conduit.greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
    AND type = '{ InvitationType.SHAMIR_RECOVERY.str }'
FOR UPDATE
"""
)


_q_conduit_update = Q(
    f"""
UPDATE invitation
SET
    conduit_state = $conduit_state,
    conduit_greeter_payload = $conduit_greeter_payload,
    conduit_claimer_payload = $conduit_claimer_payload
WHERE
    _id = $row_id
"""
)

_q_shamir_conduit_update = Q(
    f"""
UPDATE shamir_recovery_conduit
SET
    conduit_state = $conduit_state,
    conduit_greeter_payload = $conduit_greeter_payload,
    conduit_claimer_payload = $conduit_claimer_payload
WHERE
    _id = $row_id
"""
)


_q_list_shamir_recovery_invitations = Q(
    f"""
WITH human_handle_per_user AS ({_q_human_handle_per_user})
SELECT
    invitation.token,
    type,
    { q_user(_id="invitation.greeter", select="user_id") },
    human_handle_per_user.email,
    human_handle_per_user.label,
    { q_user(_id="shamir_recovery_setup.user_", select="user_id") },
    invitation.shamir_recovery,
    shamir_recovery_setup.threshold,
    created_on,
    deleted_on,
    deleted_reason
FROM invitation
LEFT JOIN human_handle_per_user
    ON invitation.greeter = human_handle_per_user.user_
JOIN shamir_recovery_conduit
    ON invitation._id = shamir_recovery_conduit.invitation
JOIN shamir_recovery_setup
    ON invitation.shamir_recovery = shamir_recovery_setup._id
WHERE
    invitation.organization = { q_organization_internal_id("$organization_id") }
    AND shamir_recovery_conduit.greeter = { q_user_internal_id(organization_id="$organization_id", user_id="$greeter_user_id") }
    AND invitation.deleted_on IS NULL
ORDER BY created_on
"""
)


async def _conduit_talk(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    greeter: UserID | None,
    is_greeter: bool,
    token: InvitationToken,
    state: ConduitState,
    payload: bytes,
) -> ConduitListenCtx:
    async with conn.transaction():
        # On top of retrieving the invitation row, this query lock the row
        # in the database for the duration of the transaction.
        # Hence concurrent request will be on hold until the end of the transaction.
        row = await conn.fetchrow(
            *_q_get_invitation_type(organization_id=organization_id.str, token=token)
        )
        if not row:
            raise InvitationNotFoundError(token)
        (type,) = row
        is_shamir_recovery = type == InvitationType.SHAMIR_RECOVERY.str
        conduit_update = _q_shamir_conduit_update if is_shamir_recovery else _q_conduit_update

        if is_greeter and not is_shamir_recovery:
            assert greeter is not None
            row = await conn.fetchrow(
                *_q_conduit_device_and_user_greeter_info(
                    organization_id=organization_id.str,
                    greeter_user_id=greeter.str,
                    token=token,
                )
            )
        elif is_greeter and is_shamir_recovery:
            assert greeter is not None
            row = await conn.fetchrow(
                *_q_conduit_shamir_recovery_greeter_info(
                    organization_id=organization_id.str,
                    greeter_user_id=greeter.str,
                    token=token,
                )
            )
        elif not is_greeter and not is_shamir_recovery:
            row = await conn.fetchrow(
                *_q_conduit_device_and_user_claimer_info(
                    organization_id=organization_id.str,
                    token=token,
                )
            )
        elif not is_greeter and is_shamir_recovery:
            assert greeter is not None
            row = await conn.fetchrow(
                *_q_conduit_shamir_recovery_claimer_info(
                    organization_id=organization_id.str,
                    token=token,
                    greeter_user_id=greeter.str,
                )
            )
        else:
            assert False

        if not row:
            raise InvitationNotFoundError(token)

        row_id = row["_id"]
        curr_conduit_state = ConduitState(row["conduit_state"])
        curr_greeter_payload = row["conduit_greeter_payload"]
        curr_claimer_payload = row["conduit_claimer_payload"]

        if is_greeter:
            curr_our_payload = curr_greeter_payload
            curr_peer_payload = curr_claimer_payload
        else:
            curr_our_payload = curr_claimer_payload
            curr_peer_payload = curr_greeter_payload

        if row["deleted_on"]:
            raise InvitationAlreadyDeletedError(token)

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
            else:
                raise InvitationInvalidStateError()

        # Now update the conduit with our payload and send a signal if
        # the peer is already waiting for us.
        if is_greeter:
            curr_greeter_payload = payload
        else:
            curr_claimer_payload = payload
        await conn.execute(
            *conduit_update(
                row_id=row_id,
                conduit_state=curr_conduit_state.value,
                conduit_greeter_payload=curr_greeter_payload,
                conduit_claimer_payload=curr_claimer_payload,
            )
        )
        # Note that in case of conduit reset, this signal will lure the peer into
        # thinking we have answered so he will wakeup and take into account the reset
        await send_signal(
            conn, BackendEvent.INVITE_CONDUIT_UPDATED, organization_id=organization_id, token=token
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


async def _conduit_listen(
    conn: triopg._triopg.TrioConnectionProxy, ctx: ConduitListenCtx
) -> bytes | None:
    async with conn.transaction():
        row = await conn.fetchrow(
            *_q_get_invitation_type(organization_id=ctx.organization_id.str, token=ctx.token)
        )
        if not row:
            raise InvitationNotFoundError(ctx.token)
        (type,) = row
        is_shamir_recovery = type == InvitationType.SHAMIR_RECOVERY.str
        conduit_update = _q_shamir_conduit_update if is_shamir_recovery else _q_conduit_update

        if ctx.is_greeter and not is_shamir_recovery:
            assert ctx.greeter is not None
            row = await conn.fetchrow(
                *_q_conduit_device_and_user_greeter_info(
                    organization_id=ctx.organization_id.str,
                    greeter_user_id=ctx.greeter.str,
                    token=ctx.token,
                )
            )
        elif ctx.is_greeter and is_shamir_recovery:
            assert ctx.greeter is not None
            row = await conn.fetchrow(
                *_q_conduit_shamir_recovery_greeter_info(
                    organization_id=ctx.organization_id.str,
                    greeter_user_id=ctx.greeter.str,
                    token=ctx.token,
                )
            )
        elif not ctx.is_greeter and not is_shamir_recovery:
            row = await conn.fetchrow(
                *_q_conduit_device_and_user_claimer_info(
                    organization_id=ctx.organization_id.str, token=ctx.token
                )
            )
        elif not ctx.is_greeter and is_shamir_recovery:
            assert ctx.greeter is not None
            row = await conn.fetchrow(
                *_q_conduit_shamir_recovery_claimer_info(
                    organization_id=ctx.organization_id.str,
                    token=ctx.token,
                    greeter_user_id=ctx.greeter.str,
                )
            )
        else:
            assert False

        if not row:
            raise InvitationNotFoundError()

        row_id = row["_id"]
        curr_conduit_state = ConduitState(row["conduit_state"])

        if row["deleted_on"]:
            raise InvitationAlreadyDeletedError()

        if ctx.is_greeter:
            curr_our_payload = row["conduit_greeter_payload"]
            curr_peer_payload = row["conduit_claimer_payload"]
        else:
            curr_our_payload = row["conduit_claimer_payload"]
            curr_peer_payload = row["conduit_greeter_payload"]

        if ctx.peer_payload is None:
            # We are waiting for the peer to provite it payload

            # Only peer payload should be allowed to change
            if curr_conduit_state != ctx.state or curr_our_payload != ctx.payload:
                raise InvitationInvalidStateError()

            if curr_peer_payload is not None:
                # Our peer has provided it payload (hence it knows
                # about our payload too), we can update the conduit
                # to the next state
                await conn.execute(
                    *conduit_update(
                        row_id=row_id,
                        conduit_state=NEXT_CONDUIT_STATE[ctx.state].value,
                        conduit_greeter_payload=None,
                        conduit_claimer_payload=None,
                    )
                )
                await send_signal(
                    conn,
                    BackendEvent.INVITE_CONDUIT_UPDATED,
                    organization_id=ctx.organization_id,
                    token=ctx.token,
                )
                return curr_peer_payload

        else:
            # We were waiting for the peer to take into account the
            # payload we provided. This would be done once the conduit
            # has switched to it next state.

            if curr_conduit_state == NEXT_CONDUIT_STATE[ctx.state] and curr_our_payload is None:
                return ctx.peer_payload
            elif (
                curr_conduit_state != ctx.state
                or curr_our_payload != ctx.payload
                or curr_peer_payload != ctx.peer_payload
            ):
                # Something unexpected has changed in our back...
                raise InvitationInvalidStateError()

    # Peer hasn't answered yet, we should wait and retry later...
    return None


async def _do_new_user_or_device_invitation(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    greeter_user_id: UserID,
    claimer_email: str | None,
    created_on: DateTime,
) -> InvitationToken:
    if claimer_email:
        invitation_type = InvitationType.USER
        q = _q_retrieve_compatible_user_invitation(
            organization_id=organization_id.str,
            type=invitation_type.str,
            greeter_user_id=greeter_user_id.str,
            claimer_email=claimer_email,
        )
    else:
        invitation_type = InvitationType.DEVICE
        q = _q_retrieve_compatible_device_invitation(
            organization_id=organization_id.str,
            type=invitation_type.str,
            greeter_user_id=greeter_user_id.str,
        )

    # Check if no compatible invitations already exists
    row = await conn.fetchrow(*q)
    if row:
        return InvitationToken.from_hex(row["token"])
    # No risk of UniqueViolationError given token is a uuid4
    token = InvitationToken.new()
    await conn.execute(
        *_q_insert_invitation(
            organization_id=organization_id.str,
            type=invitation_type.str,
            token=token,
            greeter_user_id=greeter_user_id.str,
            shamir_recovery=None,
            claimer_email=claimer_email,
            created_on=created_on,
        )
    )
    await send_signal(
        conn,
        BackendEvent.INVITE_STATUS_CHANGED,
        organization_id=organization_id,
        greeters=[greeter_user_id],
        token=token,
        status=InvitationStatus.IDLE,
    )
    return token


async def _do_new_shamir_recovery_invitation(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    greeter_user_id: UserID,
    claimer_user_id: UserID,
    created_on: DateTime,
) -> InvitationToken:
    row = await conn.fetchrow(
        *_q_retrieve_compatible_shamir_recovery_invitation(
            organization_id=organization_id.str, claimer_user_id=claimer_user_id.str
        )
    )
    if row:
        return InvitationToken.from_hex(row["token"])
    row = await conn.fetchrow(
        *_q_get_shamir_recovery_id_for_new_invitation(
            organization_id=organization_id.str,
            user_id=claimer_user_id.str,
        )
    )
    if not row:
        raise InvitationShamirRecoveryNotSetup()
    internal_id = row["shamir_recovery"]
    if internal_id is None:
        raise InvitationShamirRecoveryNotSetup()
    recipient_user_ids = [
        UserID(recipient)
        for recipient, in await conn.fetch(
            *_q_list_recipients_for_new_invitation(
                organization_id=organization_id.str,
                shamir_recovery=internal_id,
            )
        )
    ]
    if greeter_user_id not in recipient_user_ids:
        raise InvitationShamirRecoveryGreeterNotInRecipients()
    token = InvitationToken.new()
    invitation_internal_id = await conn.fetchval(
        *_q_insert_invitation(
            organization_id=organization_id.str,
            type=InvitationType.SHAMIR_RECOVERY.str,
            token=token,
            greeter_user_id=greeter_user_id.str,
            shamir_recovery=internal_id,
            claimer_email=None,
            created_on=created_on,
        )
    )
    for recipient_user_id in recipient_user_ids:
        await conn.execute(
            *_q_insert_shamir_conduit(
                invitation=invitation_internal_id,
                organization_id=organization_id.str,
                greeter_user_id=recipient_user_id.str,
            )
        )

    await send_signal(
        conn,
        BackendEvent.INVITE_STATUS_CHANGED,
        organization_id=organization_id,
        greeters=recipient_user_ids,
        token=token,
        status=InvitationStatus.IDLE,
    )
    return token


class PGInviteComponent(BaseInviteComponent):
    def __init__(self, dbh: PGHandler, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.dbh = dbh

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
        created_on = created_on or DateTime.now()
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            user_id = await query_retrieve_active_human_by_email(
                conn, organization_id, claimer_email
            )
            if user_id:
                raise InvitationAlreadyMemberError()

            token = await _do_new_user_or_device_invitation(
                conn,
                organization_id=organization_id,
                greeter_user_id=greeter_user_id,
                claimer_email=claimer_email,
                created_on=created_on,
            )
        return UserInvitation(
            greeter_user_id=greeter_user_id,
            greeter_human_handle=None,
            claimer_email=claimer_email,
            token=token,
            created_on=created_on,
        )

    async def new_for_device(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        created_on: DateTime | None = None,
    ) -> DeviceInvitation:
        """
        Raise: Nothing
        """
        created_on = created_on or DateTime.now()
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            token = await _do_new_user_or_device_invitation(
                conn,
                organization_id=organization_id,
                greeter_user_id=greeter_user_id,
                claimer_email=None,
                created_on=created_on,
            )
        return DeviceInvitation(
            greeter_user_id=greeter_user_id,
            greeter_human_handle=None,
            token=token,
            created_on=created_on,
        )

    async def new_for_shamir_recovery(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        claimer_user_id: UserID,
        created_on: DateTime | None = None,
    ) -> ShamirRecoveryInvitation:
        """
        Raise: InvitationShamirRecoveryNotSetup
        """
        created_on = created_on or DateTime.now()
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            row = await conn.fetchrow(
                *_q_get_email(
                    organization_id=organization_id.str,
                    user_id=claimer_user_id.str,
                )
            )
            if not row:
                raise InvitationError("The user cannot be found")
            claimer_email = row["email"]
            token = await _do_new_shamir_recovery_invitation(
                conn,
                organization_id=organization_id,
                greeter_user_id=greeter_user_id,
                claimer_user_id=claimer_user_id,
                created_on=created_on,
            )
        return ShamirRecoveryInvitation(
            greeter_user_id=greeter_user_id,
            greeter_human_handle=None,
            claimer_email=claimer_email,
            claimer_user_id=claimer_user_id,
            token=token,
            created_on=created_on,
        )

    async def delete(
        self,
        organization_id: OrganizationID,
        deleter: UserID,
        token: InvitationToken,
        on: DateTime,
        reason: InvitationDeletedReason,
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await _do_delete_invitation(conn, organization_id, deleter, token, on, reason)

    async def list(self, organization_id: OrganizationID, greeter: UserID) -> List[Invitation]:
        async with self.dbh.pool.acquire() as conn:
            device_and_user_rows = await conn.fetch(
                *_q_list_device_and_user_invitations(
                    organization_id=organization_id.str, greeter_user_id=greeter.str
                )
            )
            shamir_recovery_rows = await conn.fetch(
                *_q_list_shamir_recovery_invitations(
                    organization_id=organization_id.str, greeter_user_id=greeter.str
                )
            )

        invitations_with_claimer_online = self._claimers_ready[organization_id]
        invitations: list[UserInvitation | DeviceInvitation | ShamirRecoveryInvitation] = []
        for (
            token_uuid,
            type,
            greeter,
            greeter_human_handle_email,
            greeter_human_handle_label,
            claimer_email,
            created_on,
            deleted_on,
            deleted_reason,
        ) in device_and_user_rows:
            token = InvitationToken.from_hex(token_uuid)
            greeter_human_handle = None
            if greeter_human_handle_email:
                greeter_human_handle = HumanHandle(
                    email=greeter_human_handle_email, label=greeter_human_handle_label
                )

            if deleted_on:
                status = InvitationStatus.DELETED
            elif token in invitations_with_claimer_online:
                status = InvitationStatus.READY
            else:
                status = InvitationStatus.IDLE

            invitation: Invitation
            if type == InvitationType.USER.str:
                invitation = UserInvitation(
                    greeter_user_id=UserID(greeter),
                    greeter_human_handle=greeter_human_handle,
                    claimer_email=claimer_email,
                    token=token,
                    created_on=created_on,
                    status=status,
                )
            elif type == InvitationType.DEVICE.str:
                invitation = DeviceInvitation(
                    greeter_user_id=UserID(greeter),
                    greeter_human_handle=greeter_human_handle,
                    token=token,
                    created_on=created_on,
                    status=status,
                )
            else:
                assert False
            invitations.append(invitation)

        for (
            token_uuid,
            type,
            greeter,
            greeter_human_handle_email,
            greeter_human_handle_label,
            claimer_user_id,
            shamir_recovery_internal_id,
            threshold,
            created_on,
            deleted_on,
            deleted_reason,
        ) in shamir_recovery_rows:
            assert type == InvitationType.SHAMIR_RECOVERY.str
            token = InvitationToken.from_hex(token_uuid)
            greeter_human_handle = None
            if greeter_human_handle_email:
                greeter_human_handle = HumanHandle(
                    email=greeter_human_handle_email, label=greeter_human_handle_label
                )

            if deleted_on:
                status = InvitationStatus.DELETED
            elif token in invitations_with_claimer_online:
                status = InvitationStatus.READY
            else:
                status = InvitationStatus.IDLE

            invitation = ShamirRecoveryInvitation(
                greeter_user_id=UserID(greeter),
                greeter_human_handle=greeter_human_handle,
                token=token,
                claimer_email=None,
                claimer_user_id=UserID(claimer_user_id),
                created_on=created_on,
                status=status,
            )
            invitations.append(invitation)
        return invitations

    async def info(self, organization_id: OrganizationID, token: InvitationToken) -> Invitation:
        async with self.dbh.pool.acquire() as conn:
            row = await conn.fetchrow(
                *_q_info_invitation(organization_id=organization_id.str, token=token)
            )
        if not row:
            raise InvitationNotFoundError(token)

        (
            type,
            greeter,
            greeter_human_handle_email,
            greeter_human_handle_label,
            claimer_email,
            claimer_user_id,
            created_on,
            deleted_on,
            deleted_reason,
        ) = row

        if deleted_on:
            raise InvitationAlreadyDeletedError(token)

        greeter_human_handle = None
        if greeter_human_handle_email:
            greeter_human_handle = HumanHandle(
                email=greeter_human_handle_email, label=greeter_human_handle_label
            )

        if type == InvitationType.USER.str:
            return UserInvitation(
                greeter_user_id=UserID(greeter),
                greeter_human_handle=greeter_human_handle,
                claimer_email=claimer_email,
                token=token,
                created_on=created_on,
                status=InvitationStatus.READY,
            )
        elif type == InvitationType.DEVICE.str:
            return DeviceInvitation(
                greeter_user_id=UserID(greeter),
                greeter_human_handle=greeter_human_handle,
                token=token,
                created_on=created_on,
                status=InvitationStatus.READY,
            )
        elif type == InvitationType.SHAMIR_RECOVERY.str:
            return ShamirRecoveryInvitation(
                greeter_user_id=UserID(greeter),
                greeter_human_handle=greeter_human_handle,
                claimer_email=None,
                claimer_user_id=UserID(claimer_user_id),
                token=token,
                created_on=created_on,
                status=InvitationStatus.READY,
            )
        else:
            assert False

    async def _conduit_talk(
        self,
        organization_id: OrganizationID,
        greeter: UserID | None,
        is_greeter: bool,
        token: InvitationToken,
        state: ConduitState,
        payload: bytes,
    ) -> ConduitListenCtx:
        async with self.dbh.pool.acquire() as conn:
            return await _conduit_talk(
                conn, organization_id, greeter, is_greeter, token, state, payload
            )

    async def _conduit_listen(self, ctx: ConduitListenCtx) -> bytes | None:
        async with self.dbh.pool.acquire() as conn:
            return await _conduit_listen(conn, ctx)

    async def claimer_joined(self, organization_id: OrganizationID, invitation: Invitation) -> None:
        async with self.dbh.pool.acquire() as conn:
            if isinstance(invitation, ShamirRecoveryInvitation):
                _, recipients = await _shamir_info(conn, organization_id, invitation.token)
                greeters = [recipient.user_id for recipient in recipients]
            else:
                greeters = [invitation.greeter_user_id]
            print("sending READY", greeters)
            await send_signal(
                conn,
                BackendEvent.INVITE_STATUS_CHANGED,
                organization_id=organization_id,
                greeters=greeters,
                token=invitation.token,
                status=InvitationStatus.READY,
            )

    async def claimer_left(self, organization_id: OrganizationID, invitation: Invitation) -> None:
        async with self.dbh.pool.acquire() as conn:
            if isinstance(invitation, ShamirRecoveryInvitation):
                _, recipients = await _shamir_info(conn, organization_id, invitation.token)
                greeters = [recipient.user_id for recipient in recipients]
            else:
                greeters = [invitation.greeter_user_id]
            await send_signal(
                conn,
                BackendEvent.INVITE_STATUS_CHANGED,
                organization_id=organization_id,
                greeters=greeters,
                token=invitation.token,
                status=InvitationStatus.IDLE,
            )

    async def shamir_info(
        self, organization_id: OrganizationID, token: InvitationToken
    ) -> tuple[int, tuple[ShamirRecoveryRecipient, ...]]:
        async with self.dbh.pool.acquire() as conn:
            return await _shamir_info(conn, organization_id, token)
