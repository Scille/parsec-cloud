# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import List, Tuple

from parsec.types import UserID, DeviceID, OrganizationID
from parsec.backend.message import BaseMessageComponent, MessageError
from parsec.backend.drivers.postgresql.handler import send_signal, PGHandler


class PGMessageComponent(BaseMessageComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def send(
        self, organization_id: OrganizationID, sender: DeviceID, recipient: UserID, body: bytes
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                result = await conn.execute(
                    """
INSERT INTO messages (
    organization,
    sender,
    recipient,
    body
)
SELECT
    get_organization_internal_id($1),
    get_device_internal_id($1, $2),
    get_user_internal_id($1, $3),
    $4
""",
                    organization_id,
                    sender,
                    recipient,
                    body,
                )
                if result != "INSERT 0 1":
                    raise MessageError(f"Insertion error: {result}")

                # TODO: index doesn't seem to be used in the core, and is complicated to get here...
                # Maybe we should replace it by a timestamp ?
                index, = await conn.fetchrow(
                    """
SELECT COUNT(*) FROM messages
WHERE recipient = get_user_internal_id($1, $2)
""",
                    organization_id,
                    recipient,
                )
                await send_signal(
                    conn,
                    "message.received",
                    organization_id=organization_id,
                    author=sender,
                    recipient=recipient,
                    index=index,
                )

    async def get(
        self, organization_id: OrganizationID, recipient: UserID, offset: int
    ) -> List[Tuple[DeviceID, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            data = await conn.fetch(
                """
SELECT get_device_id(sender), body
FROM messages
WHERE recipient = get_user_internal_id($1, $2)
ORDER BY _id ASC OFFSET $3
""",
                organization_id,
                recipient,
                offset,
            )
        # TODO: we should configure a DeviceID custom serializer in dbh
        return [(DeviceID(d[0]), d[1]) for d in data]
