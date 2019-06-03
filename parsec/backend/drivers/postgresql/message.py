# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pendulum import Pendulum
from typing import List, Tuple

from parsec.types import UserID, DeviceID, OrganizationID
from parsec.backend.message import BaseMessageComponent
from parsec.backend.drivers.postgresql.handler import send_signal, PGHandler


async def send_message(conn, organization_id, sender, recipient, timestamp, body):
    index = await conn.fetchval(
        """
INSERT INTO message (
    organization,
    recipient,
    timestamp,
    index,
    sender,
    body
)
SELECT
    get_organization_internal_id($1),
    get_user_internal_id($1, $2),
    $5,
    (
        SELECT COUNT(*) + 1
        FROM message
        WHERE recipient = get_user_internal_id($1, $2)
    ),
    get_device_internal_id($1, $3),
    $4
RETURNING index
""",
        organization_id,
        recipient,
        sender,
        body,
        timestamp,
    )

    await send_signal(
        conn,
        "message.received",
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
        timestamp: Pendulum,
        body: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await send_message(conn, organization_id, sender, recipient, timestamp, body)

    async def get(
        self, organization_id: OrganizationID, recipient: UserID, offset: int
    ) -> List[Tuple[DeviceID, Pendulum, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            data = await conn.fetch(
                """
SELECT get_device_id(sender), timestamp, body
FROM message
WHERE recipient = get_user_internal_id($1, $2)
ORDER BY _id ASC OFFSET $3
""",
                organization_id,
                recipient,
                offset,
            )
        return [(DeviceID(d[0]), d[1], d[2]) for d in data]
