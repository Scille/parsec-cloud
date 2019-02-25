# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import List, Tuple
from uuid import UUID

from parsec.types import DeviceID, OrganizationID
from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.drivers.postgresql.handler import send_signal, PGHandler


class PGBeaconComponent(BaseBeaconComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def read(
        self, organization_id: OrganizationID, id: UUID, offset: int
    ) -> List[Tuple[UUID, int]]:
        async with self.dbh.pool.acquire() as conn:
            results = await conn.fetch(
                """
SELECT src_id, src_version
FROM beacons
WHERE
    organization = (
        SELECT _id from organizations WHERE organization_id = $1
    )
    AND beacon_id = $2
ORDER BY _id ASC OFFSET $3
""",
                organization_id,
                id,
                offset,
            )
            return results

    async def update(
        self,
        organization_id: OrganizationID,
        id: UUID,
        src_id: UUID,
        src_version: int,
        author: DeviceID = None,
    ) -> None:
        if not author:
            return
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                await self.ll_update(conn, organization_id, id, src_id, src_version, author)

    async def ll_update(
        self,
        conn,
        organization_id: OrganizationID,
        beacon_id: UUID,
        src_id: UUID,
        src_version: int,
        author: DeviceID = None,
    ) -> None:
        beacon_index = await conn.fetchval(
            """
INSERT INTO beacons (
    organization,
    beacon_id,
    beacon_index,
    src_id,
    src_version
)
SELECT
    _id,
    $2,
    (
        -- Retrieve last index of this beacon, or default to 1
        SELECT COALESCE(
            (
                SELECT  beacon_index + 1
                FROM beacons
                WHERE organization = organizations._id AND beacon_id=$2
                ORDER BY _id DESC LIMIT 1
            ),
            1
        )
    ),
    $3, $4
FROM organizations
WHERE organization_id = $1
RETURNING beacon_index
""",
            organization_id,
            beacon_id,
            src_id,
            src_version,
        )

        # TODO: do we really need to provide the author in this signal ?
        await send_signal(
            conn,
            "beacon.updated",
            organization_id=organization_id,
            author=author,
            beacon_id=beacon_id,
            index=beacon_index,
            src_id=src_id,
            src_version=src_version,
        )
