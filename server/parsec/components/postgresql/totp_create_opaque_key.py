# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import DeviceID, OrganizationID, SecretKey, TOTPOpaqueKeyID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import Q
from parsec.components.totp import TOTPCreateOpaqueKeyBadOutcome

_q_insert_opaque_key = Q("""
INSERT INTO totp_opaque_key (
    user_,
    opaque_key_id,
    opaque_key
)
VALUES (
    $user_internal_id,
    $opaque_key_id,
    $opaque_key
)
""")


async def totp_create_opaque_key(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
) -> tuple[TOTPOpaqueKeyID, SecretKey] | TOTPCreateOpaqueKeyBadOutcome:
    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as auth:
            pass
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return TOTPCreateOpaqueKeyBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return TOTPCreateOpaqueKeyBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return TOTPCreateOpaqueKeyBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return TOTPCreateOpaqueKeyBadOutcome.AUTHOR_REVOKED

    opaque_key_id = TOTPOpaqueKeyID.new()
    opaque_key = SecretKey.generate()

    await conn.execute(
        *_q_insert_opaque_key(
            user_internal_id=auth.user_internal_id,
            opaque_key_id=opaque_key_id,
            opaque_key=opaque_key.secret,
        )
    )

    return (opaque_key_id, opaque_key)
