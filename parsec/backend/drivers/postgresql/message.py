from typing import List, Tuple

from parsec.types import UserID, DeviceID
from parsec.backend.message import BaseMessageComponent, MessageError
from parsec.backend.drivers.postgresql.handler import send_signal, PGHandler


class PGMessageComponent(BaseMessageComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def send(self, sender: DeviceID, recipient: UserID, body: bytes) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                result = await conn.execute(
                    """
                    INSERT INTO messages (
                        sender, recipient, body
                    ) VALUES ($1, $2, $3)
                    """,
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
                    SELECT COUNT(*) FROM messages WHERE recipient = $1
                    """,
                    recipient,
                )
                await send_signal(
                    conn, "message.received", author=sender, recipient=recipient, index=index
                )

    async def get(self, recipient: UserID, offset: int) -> List[Tuple[DeviceID, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            data = await conn.fetch(
                "SELECT sender, body FROM messages WHERE recipient = $1 ORDER BY _id ASC OFFSET $2",
                recipient,
                offset,
            )
        # TODO: we should configure a DeviceID custom serializer in dbh
        return [(DeviceID(d[0]), d[1]) for d in data]
