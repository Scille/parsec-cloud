from collections import defaultdict

from parsec.backend.beacon import BaseBeaconComponent


class PGBeaconComponent(BaseBeaconComponent):
    def __init__(self, dbh, signal_ns):
        self._signal_beacon_updated = signal_ns.signal("beacon.updated")
        self.beacons = defaultdict(list)

    async def read(self, id, offset):
        return self.beacons[id][offset:]
        async with self.dbh.pool.acquire() as conn:
            results = await conn.fetch(
                "SELECT src_id, src_version FROM beacons WHERE id = $1 ORDER BY _id ASC OFFSET $2",
                id,
                offset,
            )
            return [
                {"src_id": src_id, "src_version": src_version} for src_id, src_version in results
            ]

    async def update(self, id, src_id, src_version, author="anonymous"):
        async with self.dbh.pool.acquire() as conn:
            result = await conn.execute(
                """
                INSERT INTO beacons (id, src_id, src_version) VALUES
                (
                    $1,
                    (SELECT _id FROM users WHERE user_id=$2),
                    $3
                )
                """,
                sender_device_id,
                recipient_user_id,
                body,
            )
            if result != "INSERT 0 1":
                raise ParsecError("Insertion error")
        self._signal_message_arrived.send(recipient_user_id)

        self.beacons[id].append({"src_id": src_id, "src_version": src_version})
        index = len(self.beacons[id])
        self._signal_beacon_updated.send(
            None, author=author, beacon_id=id, index=index, src_id=src_id, src_version=src_version
        )


# class PGMessageComponent(BaseMessageComponent):
#     def __init__(self, dbh, signal_ns):
#         self._signal_message_received = signal_ns.signal("message.received")
#         self.dbh = dbh

#     async def perform_message_new(self, sender_device_id, recipient_user_id, body):
#         async with self.dbh.pool.acquire() as conn:
#             result = await conn.execute(
#                 """
#                 INSERT INTO messages (sender_device_id, recipient_user_id, body) VALUES
#                 (
#                     $1,
#                     (SELECT _id FROM users WHERE user_id=$2),
#                     $3
#                 )
#                 """,
#                 sender_device_id,
#                 recipient_user_id,
#                 body,
#             )
#             if result != "INSERT 0 1":
#                 raise ParsecError("Insertion error")
#         self._signal_message_arrived.send(recipient_user_id)

#     async def perform_message_get(self, recipient_user_id, offset):
#         async with self.dbh.pool.acquire() as conn:
#             return await conn.fetch(
#                 "SELECT sender_device_id, body FROM messages WHERE recipient_user_id = $1 OFFSET $2",
#                 recipient_user_id,
#                 offset,
#             )
