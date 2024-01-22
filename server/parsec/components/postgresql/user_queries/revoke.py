# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncpg

from parsec._parsec import (
    BackendEventUserUpdatedOrRevoked,
    DateTime,
    DeviceID,
    OrganizationID,
    UserID,
)
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.user_queries.create import q_take_user_device_write_lock
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_organization_internal_id,
    q_user,
    query,
)
from parsec.components.user import UserAlreadyRevokedError, UserError, UserNotFoundError

_q_revoke_user = Q(
    f"""
UPDATE user_ SET
    revoked_user_certificate = $revoked_user_certificate,
    revoked_user_certifier = { q_device_internal_id(organization_id="$organization_id", device_id="$revoked_user_certifier") },
    revoked_on = $revoked_on
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND user_id = $user_id
    AND revoked_on IS NULL
"""
)


_q_revoke_user_error = Q(
    q_user(organization_id="$organization_id", user_id="$user_id", select="revoked_on")
)


@query(in_transaction=True)
async def query_revoke_user(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    user_id: UserID,
    revoked_user_certificate: bytes,
    revoked_user_certifier: DeviceID,
    revoked_on: DateTime | None = None,
) -> None:
    await q_take_user_device_write_lock(conn, organization_id)
    result = await conn.execute(
        *_q_revoke_user(
            organization_id=organization_id.str,
            user_id=user_id.str,
            revoked_user_certificate=revoked_user_certificate,
            revoked_user_certifier=revoked_user_certifier.str,
            revoked_on=revoked_on or DateTime.now(),
        )
    )

    if result != "UPDATE 1":
        # TODO: avoid having to do another query to find the error
        err_result = await conn.fetchrow(
            *_q_revoke_user_error(organization_id=organization_id.str, user_id=user_id.str)
        )
        if not err_result:
            raise UserNotFoundError(user_id)

        elif err_result[0]:
            raise UserAlreadyRevokedError()

        else:
            raise UserError(f"Update error: {result}")
    else:
        await send_signal(
            conn,
            BackendEventUserUpdatedOrRevoked(
                organization_id=organization_id, user_id=user_id, profile=None
            ),
        )
