from parsec.utils import ParsecError
from parsec.backend.message import BaseMessageComponent
from parsec.backend.drivers.postgresql.handler import send_signal


class PGMessageComponent(BaseMessageComponent):
    def __init__(self, dbh, event_bus):
        self.dbh = dbh

    async def perform_message_new(self, sender_device_id, recipient_user_id, body):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                result = await conn.execute(
                    """
                    INSERT INTO messages (sender, recipient, body) VALUES ($1, $2, $3)
                    """,
                    sender_device_id,
                    recipient_user_id,
                    body,
                )
                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error")

                # TODO: index doesn't seem to be used in the core, and is complicated to get here...
                # Maybe we should replace it by a timestamp ?
                index, = await conn.fetchrow(
                    """
                    SELECT COUNT(*) FROM messages WHERE recipient = $1
                    """,
                    recipient_user_id,
                )
                await send_signal(
                    conn,
                    "message.received",
                    author=sender_device_id,
                    recipient=recipient_user_id,
                    index=index,
                )

    async def perform_message_get(self, recipient_user_id, offset):
        async with self.dbh.pool.acquire() as conn:
            return await conn.fetch(
                "SELECT sender, body FROM messages WHERE recipient = $1 ORDER BY _id ASC OFFSET $2",
                recipient_user_id,
                offset,
            )
