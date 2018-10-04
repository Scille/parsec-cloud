from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.drivers.postgresql.handler import send_signal


class PGBeaconComponent(BaseBeaconComponent):
    def __init__(self, dbh, event_bus):
        self.dbh = dbh

    async def read(self, id, offset):
        async with self.dbh.pool.acquire() as conn:
            results = await conn.fetch(
                "SELECT src_id, src_version FROM beacons WHERE beacon_id = $1 ORDER BY _id ASC OFFSET $2",
                id,
                offset,
            )
            return [
                {"src_id": src_id, "src_version": src_version} for src_id, src_version in results
            ]

    async def update(self, id, src_id, src_version, author="anonymous"):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self.ll_update_multiple(conn, (id,), src_id, src_version, author)

    async def ll_update_multiple(self, conn, beacon_ids, src_id, src_version, author):
        if not beacon_ids:
            return

        for beacon_id in beacon_ids:
            beacon_index = await conn.fetchval(
                """
                INSERT INTO beacons (
                    beacon_id,
                    beacon_index,
                    src_id,
                    src_version
                ) VALUES (
                    $1,
                    (
                        -- Retrieve last index of this beacon, or default to 1
                        SELECT COALESCE(
                            (SELECT  beacon_id + 1 FROM beacons WHERE beacon_id=$1 ORDER BY _id DESC LIMIT 1),
                            1
                        )
                    ),
                    $2,
                    $3
                ) RETURNING beacon_index
                """,
                beacon_id,
                src_id,
                src_version,
            )

            # TODO: do we really need to provide the author in this signal ?
            await send_signal(
                conn,
                "beacon.updated",
                author=author,
                beacon_id=beacon_id,
                index=beacon_index,
                src_id=src_id,
                src_version=src_version,
            )
