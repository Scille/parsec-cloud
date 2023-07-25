# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any

from parsec._parsec import (
    DeviceID,
    OrganizationID,
    ShamirRecoveryBriefCertificate,
    ShamirRecoverySetup,
    ShamirRevealToken,
    UserID,
)
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.invite import (
    delete_shamir_recovery_invitation_if_it_exists,
)
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

    async def remove_recovery_setup(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await conn.execute(
                *_q_set_shamir_recovery(
                    organization_id=organization_id.str,
                    user_id=author.user_id.str,
                    shamir_recovery=None,
                )
            )
            await delete_shamir_recovery_invitation_if_it_exists(
                conn, organization_id, author.user_id
            )

    async def add_recovery_setup(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        setup: ShamirRecoverySetup,
        brief_certificate: ShamirRecoveryBriefCertificate,
        raw_share_certificates: dict[UserID, bytes],
    ) -> None:
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
            for recipient_id, raw_share_certificate in raw_share_certificates.items():
                await conn.execute(
                    *_q_insert_shamir_recovery_share(
                        organization_id=organization_id.str,
                        shamir_recovery=internal_setup_id,
                        recipient=recipient_id.str,
                        share_certificate=raw_share_certificate,
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
            await delete_shamir_recovery_invitation_if_it_exists(
                conn, organization_id, author.user_id
            )

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
