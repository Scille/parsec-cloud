from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.drivers.postgresql.handler import send_signal


class PGBeaconComponent(BaseBeaconComponent):
    def __init__(self, dbh, signal_ns):
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

    async def ll_update_multiple(self, conn, ids, src_id, src_version, author):
        if not ids:
            return

        # TODO: executemany discard return value so there is no way to know if
        # somehting went wrong here, not sure if it's a good idea...
        await conn.executemany(
            """
            INSERT INTO beacons (beacon_id, src_id, src_version) VALUES ($1, $2, $3)
            -- INSERT INTO beacons (id, src_id, src_version) VALUES ($1, $2, $3)
            -- ON CONFLICT (id) DO UPDATE SET src_id = $2, src_version = $3 WHERE beacons.id = $1
            """,
            [(id, src_id, src_version) for id in ids],
        )

        # TODO: index doesn't seem to be used in the core, and is complicated to get here...
        # Maybe we should replace it by a timestamp ?
        index = 42
        for id in ids:
            # TODO: do we really need to provide the author in this signal ?
            await send_signal(
                conn,
                "beacon.updated",
                author=author,
                beacon_id=id,
                index=index,
                src_id=src_id,
                src_version=src_version,
            )
