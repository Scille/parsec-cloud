from typing import List, Tuple
from uuid import UUID

from parsec.types import DeviceID
from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.drivers.postgresql.handler import send_signal, PGHandler


class PGBeaconComponent(BaseBeaconComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def read(self, id: UUID, offset: int) -> List[Tuple[UUID, int]]:
        async with self.dbh.pool.acquire() as conn:
            results = await conn.fetch(
                "SELECT src_id, src_version FROM beacons WHERE beacon_id = $1 ORDER BY _id ASC OFFSET $2",
                id,
                offset,
            )
            return results

    async def update(
        self, id: UUID, src_id: UUID, src_version: int, author: DeviceID = None
    ) -> None:
        if not author:
            return
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self.ll_update(conn, id, src_id, src_version, author)

    async def ll_update(
        self, conn, beacon_id: UUID, src_id: UUID, src_version: int, author: DeviceID = None
    ) -> None:
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
                        (SELECT  beacon_index + 1 FROM beacons WHERE beacon_id=$1 ORDER BY _id DESC LIMIT 1),
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
