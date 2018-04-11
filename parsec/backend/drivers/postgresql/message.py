from parsec.backend.message import BaseMessageComponent


class PGMessageComponent(BaseMessageComponent):

    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def perform_message_new(self, sender_device_id, recipient_user_id, body):
        await self.dbh.insert_one(
            "INSERT INTO messages (sender_device_id, recipient_user_id, body) VALUES (%s, %s, %s)",
            (sender_device_id, recipient_user_id, body),
        )
        self._signal_message_arrived.send(recipient_user_id)

    async def perform_message_get(self, recipient_user_id, offset):
        return [
            (sender_device_id, body.tobytes())
            for sender_device_id, body in await self.dbh.fetch_many(
                "SELECT sender_device_id, body FROM messages WHERE recipient_user_id = %s OFFSET %s",
                (recipient_user_id, offset),
            )
        ]
