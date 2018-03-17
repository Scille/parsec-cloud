from parsec.backend.message import BaseMessageComponent


class PGMessageComponent(BaseMessageComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def perform_message_new(self, recipient, body):
        await self.dbh.insert_one(
            'INSERT INTO messages (recipient, body) VALUES (%s, %s)',
            (recipient, body)
        )
        self._signal_message_arrived.send(recipient)

    async def perform_message_get(self, id, offset):
        return [
            msg_body.tobytes()
            for msg_body, in await self.dbh.fetch_many(
                'SELECT body FROM messages WHERE recipient = %s OFFSET %s',
                (id, offset)
            )
        ]
