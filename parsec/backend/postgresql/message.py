# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import List, Tuple

import triopg

from parsec._parsec import DateTime
from parsec.api.protocol import DeviceID, OrganizationID, UserID
from parsec.backend.backend_events import BackendEvent
from parsec.backend.message import BaseMessageComponent
from parsec.backend.postgresql.handler import PGHandler, send_signal
from parsec.backend.postgresql.utils import (
    Q,
    q_device,
    q_device_internal_id,
    q_organization_internal_id,
    q_user_internal_id,
)

_q_insert_message = Q(
    f"""
    INSERT INTO message (organization, recipient, timestamp, index, sender, body)
    VALUES (
        { q_organization_internal_id("$organization_id") },
        { q_user_internal_id(organization_id="$organization_id", user_id="$recipient") },
        $timestamp,
        (
            SELECT COUNT(*) + 1
            FROM message
            WHERE
                recipient = { q_user_internal_id(organization_id="$organization_id", user_id="$recipient") }
        ),
        { q_device_internal_id(organization_id="$organization_id", device_id="$sender") },
        $body
    )
    RETURNING index
"""
)


_q_get_messages = Q(
    f"""
SELECT
    { q_device(_id="message.sender", select="device_id") },
    timestamp,
    body
FROM message
WHERE
    recipient = { q_user_internal_id(organization_id="$organization_id", user_id="$recipient") }
ORDER BY _id ASC
OFFSET $offset
"""
)


async def send_message(
    conn: triopg._triopg.TrioConnectionProxy,
    organization_id: OrganizationID,
    sender: DeviceID,
    recipient: UserID,
    timestamp: DateTime,
    body: bytes,
) -> None:
    index = await conn.fetchval(
        *_q_insert_message(
            organization_id=organization_id.str,
            recipient=recipient.str,
            timestamp=timestamp,
            sender=sender.str,
            body=body,
        )
    )

    await send_signal(
        conn,
        BackendEvent.MESSAGE_RECEIVED,
        organization_id=organization_id,
        author=sender,
        recipient=recipient,
        index=index,
    )


class PGMessageComponent(BaseMessageComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def send(
        self,
        organization_id: OrganizationID,
        sender: DeviceID,
        recipient: UserID,
        timestamp: DateTime,
        body: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await send_message(conn, organization_id, sender, recipient, timestamp, body)

    async def get(
        self, organization_id: OrganizationID, recipient: UserID, offset: int
    ) -> List[Tuple[DeviceID, DateTime, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            data = await conn.fetch(
                *_q_get_messages(
                    organization_id=organization_id.str, recipient=recipient.str, offset=offset
                )
            )
        return [(DeviceID(d[0]), d[1], d[2]) for d in data]
