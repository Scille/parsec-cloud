# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any

import triopg

from parsec._parsec import (
    DateTime,
    DeviceID,
    InvitationDeletedReason,
    InvitationStatus,
    InvitationToken,
    InvitationType,
    OrganizationID,
    ShamirRecoverySetup,
    ShamirRevealToken,
    UserID,
    VerifyKey,
)
from parsec.backend.backend_events import BackendEvent
from parsec.backend.postgresql.handler import PGHandler, send_signal
from parsec.backend.postgresql.utils import (
    Q,
    q_organization_internal_id,
    q_shamir_recovery_internal_id,
    q_user_internal_id,
)
from parsec.backend.shamir import BaseShamirComponent

_q_shamir_recovery_get_brief_certificate = Q(
    f"""
SELECT
    brief_certificate
FROM shamir_recovery_setup
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND _id = { q_shamir_recovery_internal_id("$organization_id", "$user_id") }
LIMIT 1
"""
)

_q_insert_shamir_recovery_setup = Q(
    f"""
INSERT INTO shamir_recovery_setup(
    organization,
    user_,
    brief_certificate,
    reveal_token,
    threshold,
    shares,
    ciphered_data
)
VALUES (
    { q_organization_internal_id("$organization_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $brief_certificate,
    $reveal_token,
    $threshold,
    $shares,
    $ciphered_data
)
RETURNING _id
"""
)

_q_insert_shamir_recovery_share = Q(
    f"""
INSERT INTO shamir_recovery_share(
    organization,
    shamir_recovery,
    recipient,
    share_certificate,
    shares
)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $shamir_recovery,
    { q_user_internal_id(organization_id="$organization_id", user_id="$recipient") },
    $share_certificate,
    $shares
)
RETURNING _id
"""
)

_q_shamir_recovery_list_share_certificates = Q(
    f"""
SELECT
    brief_certificate,
    share_certificate
FROM shamir_recovery_share
JOIN shamir_recovery_setup
    ON shamir_recovery_share.shamir_recovery = shamir_recovery_setup._id
JOIN user_
    ON shamir_recovery_setup._id = user_.shamir_recovery
WHERE
    shamir_recovery_share.organization = { q_organization_internal_id("$organization_id") }
    AND recipient = { q_user_internal_id(organization_id="$organization_id", user_id="$recipient") }
    AND user_._id IS NOT NULL
"""
)


_q_set_shamir_recovery = Q(
    f"""
UPDATE user_
SET
    shamir_recovery = $shamir_recovery
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND user_id = $user_id
"""
)

_q_delete_shamir_recovery_invitation_info = Q(
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
    organization = { q_organization_internal_id("$organization_id") }
    AND _id = $row_id
"""
)

_q_get_ciphered_data = Q(
    f"""
SELECT
    ciphered_data
FROM shamir_recovery_setup
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND reveal_token = $reveal_token
"""
)


async def _do_delete_invitation_if_it_exists(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    user_id: UserID,
) -> None:
    on = DateTime.now()
    reason = InvitationDeletedReason.CANCELLED
    rows = await conn.fetch(
        *_q_delete_shamir_recovery_invitation_info(
            organization_id=organization_id.str, greeter=user_id.str
        )
    )
    # In practice, there should be at most one element in rows
    for row_id, raw_token in rows:
        await conn.execute(
            *_q_delete_invitation(
                organization_id=organization_id.str, row_id=row_id, on=on, reason=reason.str
            )
        )
        await send_signal(
            conn,
            BackendEvent.INVITE_STATUS_CHANGED,
            organization_id=organization_id,
            greeter=user_id,
            token=InvitationToken.from_hex(raw_token),
            status=InvitationStatus.DELETED,
        )


class PGShamirComponent(BaseShamirComponent):
    def __init__(self, dbh: PGHandler, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.dbh = dbh

    async def recovery_others_list(
        self,
        organization_id: OrganizationID,
        author_id: DeviceID,
    ) -> list[tuple[bytes, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            rows = await conn.fetch(
                *_q_shamir_recovery_list_share_certificates(
                    organization_id=organization_id.str,
                    recipient=author_id.user_id.str,
                )
            )
            return [(row["brief_certificate"], row["share_certificate"]) for row in rows]

    async def recovery_self_info(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> bytes | None:
        async with self.dbh.pool.acquire() as conn:
            row = await conn.fetchrow(
                *_q_shamir_recovery_get_brief_certificate(
                    organization_id=organization_id.str,
                    user_id=author.user_id.str,
                )
            )
            if row is None:
                return None
            return row["brief_certificate"]

    async def recovery_setup(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        setup: ShamirRecoverySetup | None,
    ) -> None:
        if setup is None:
            # Unset as active shamir recovery
            async with self.dbh.pool.acquire() as conn, conn.transaction():
                await conn.execute(
                    *_q_set_shamir_recovery(
                        organization_id=organization_id.str,
                        user_id=author.user_id.str,
                        shamir_recovery=None,
                    )
                )
                await _do_delete_invitation_if_it_exists(conn, organization_id, author.user_id)
            return
        # Verify the certificates
        brief_certificate, share_certificates = self._verify_certificates(
            setup,
            author,
            author_verify_key,
        )
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            # Insert setup
            internal_setup_id = await conn.fetchval(
                *_q_insert_shamir_recovery_setup(
                    organization_id=organization_id.str,
                    user_id=author.user_id.str,
                    brief_certificate=setup.brief,
                    reveal_token=setup.reveal_token,
                    threshold=brief_certificate.threshold,
                    shares=sum(brief_certificate.per_recipient_shares.values()),
                    ciphered_data=setup.ciphered_data,
                )
            )
            # Insert shares
            for recipient_id, share_certificate in share_certificates.items():
                await conn.execute(
                    *_q_insert_shamir_recovery_share(
                        organization_id=organization_id.str,
                        shamir_recovery=internal_setup_id,
                        recipient=recipient_id.str,
                        share_certificate=share_certificate,
                        shares=brief_certificate.per_recipient_shares[recipient_id],
                    )
                )
            # Set as active shamir recovery
            await conn.execute(
                *_q_set_shamir_recovery(
                    organization_id=organization_id.str,
                    user_id=author.user_id.str,
                    shamir_recovery=internal_setup_id,
                )
            )
            # Remove pending invitation if it exists
            await _do_delete_invitation_if_it_exists(conn, organization_id, author.user_id)

    async def recovery_reveal(
        self,
        organization_id: OrganizationID,
        reveal_token: ShamirRevealToken,
    ) -> bytes | None:
        async with self.dbh.pool.acquire() as conn:
            (data,) = await conn.fetchrow(
                *_q_get_ciphered_data(
                    organization_id=organization_id.str, reveal_token=reveal_token
                )
            )
            return data
