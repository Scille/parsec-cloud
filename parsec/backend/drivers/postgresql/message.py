from parsec.backend.message import BaseMessageComponent
from .handler import atomic


class PGMessageComponent(BaseMessageComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    @atomic
    async def perform_message_new(self, conn, sender_device_id, recipient_user_id, body):
        await self.dbh.insert_one(
            conn,
            "INSERT INTO messages (sender_device_id, recipient_user_id, body) VALUES ($1, $2, $3)",
            sender_device_id,
            recipient_user_id,
            body,
        )
        self._signal_message_arrived.send(recipient_user_id)

    @atomic
    async def perform_message_get(self, conn, recipient_user_id, offset):
        return [
            (sender_device_id, body)
            for sender_device_id, body in await self.dbh.fetch_many(
                conn,
                """
                SELECT sender_device_id, body FROM messages WHERE recipient_user_id = $1 OFFSET $2
                """,
                recipient_user_id,
                offset,
            )
        ]
