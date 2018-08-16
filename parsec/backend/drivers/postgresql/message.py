from parsec.utils import ParsecError
from parsec.backend.message import BaseMessageComponent


class PGMessageComponent(BaseMessageComponent):
    def __init__(self, dbh, signal_ns):
        self._signal_message_received = signal_ns.signal("message.received")
        self.dbh = dbh

    async def perform_message_new(self, sender_device_id, recipient_user_id, body):
        async with self.dbh.pool.acquire() as conn:
            result = await conn.execute(
                "INSERT INTO messages (sender_device_id, recipient_user_id, body) VALUES ($1, $2, $3)",
                sender_device_id,
                recipient_user_id,
                body,
            )
            if result != "INSERT 0 1":
                raise ParsecError("Insertion error")
        self._signal_message_arrived.send(recipient_user_id)

    async def perform_message_get(self, recipient_user_id, offset):
        async with self.dbh.pool.acquire() as conn:
            return await conn.fetch(
                "SELECT sender_device_id, body FROM messages WHERE recipient_user_id = $1 OFFSET $2",
                recipient_user_id,
                offset,
            )
