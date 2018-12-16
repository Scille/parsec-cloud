from triopg import UniqueViolationError
from uuid import UUID
from typing import List, Tuple

from parsec.types import DeviceID
from parsec.event_bus import EventBus
from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobError,
    VlobTrustSeedError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler, beacon_component: BaseBeaconComponent):
        self.dbh = dbh
        self.beacon_component = beacon_component

    async def group_check(self, to_check: List[dict]) -> List[dict]:
        changed = []
        to_check_dict = {}
        for x in to_check:
            if x["version"] == 0:
                changed.append({"id": x["id"], "version": 0})
            else:
                to_check_dict[x["id"]] = x

        async with self.dbh.pool.acquire() as conn:
            rows = await conn.fetch(
                """
                SELECT DISTINCT ON (vlob_id) vlob_id, rts, version
                FROM vlobs
                WHERE vlob_id = any($1::uuid[])
                ORDER BY vlob_id, version DESC
            """,
                to_check_dict.keys(),
            )

        for id, rts, version in rows:
            if rts != to_check_dict[id]["rts"]:
                continue
            if version != to_check_dict[id]["version"]:
                changed.append({"id": id, "version": version})

        return changed

    async def create(
        self,
        id: UUID,
        rts: str,
        wts: str,
        blob: bytes,
        author: DeviceID,
        notify_beacon: UUID = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                try:
                    result = await conn.execute(
                        """
                        INSERT INTO vlobs (
                            vlob_id, rts, wts, version, blob, author
                        ) VALUES ($1, $2, $3, 1, $4, $5)
                        """,
                        id,
                        rts,
                        wts,
                        blob,
                        author,
                    )
                except UniqueViolationError:
                    raise VlobAlreadyExistsError()

                if result != "INSERT 0 1":
                    raise VlobError(f"Insertion error: {result}")

                if notify_beacon:
                    await self.beacon_component.ll_update(conn, notify_beacon, id, 1, author)

    async def read(self, id: UUID, rts: str, version: int = None) -> Tuple[int, bytes]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                if version is None:
                    data = await conn.fetchrow(
                        """
                        SELECT rts, version, blob
                        FROM vlobs WHERE vlob_id = $1 ORDER BY version DESC LIMIT 1
                        """,
                        id,
                    )
                    if not data:
                        raise VlobNotFoundError()

                else:
                    data = await conn.fetchrow(
                        """
                        SELECT rts, version, blob
                        FROM vlobs WHERE vlob_id = $1 AND version = $2
                        """,
                        id,
                        version,
                    )
                    if not data:
                        # TODO: not cool to need 2nd request to know the error...
                        exists = await conn.fetchrow(
                            "SELECT true FROM vlobs WHERE vlob_id = $1", id
                        )
                        if exists:
                            raise VlobVersionError()

                        else:
                            raise VlobNotFoundError()

            if data["rts"] != rts:
                raise VlobTrustSeedError()

        return data[1:]

    async def update(
        self,
        id: UUID,
        wts: str,
        version: int,
        blob: bytes,
        author: DeviceID,
        notify_beacon: UUID = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                previous = await conn.fetchrow(
                    """
                    SELECT wts, version, rts
                    FROM vlobs WHERE vlob_id = $1 ORDER BY version DESC LIMIT 1
                    """,
                    id,
                )
                if not previous:
                    raise VlobNotFoundError()
                elif previous[0] != wts:
                    raise VlobTrustSeedError()
                elif previous[1] != version - 1:
                    raise VlobVersionError()

                rts = previous[2]
                try:
                    result = await conn.execute(
                        """
                        INSERT INTO vlobs (
                            vlob_id, rts, wts, version, blob, author
                        ) VALUES ($1, $2, $3, $4, $5, $6)
                        """,
                        id,
                        rts,
                        wts,
                        version,
                        blob,
                        author,
                    )
                except UniqueViolationError:
                    # Should not occurs in theory given we are in a transaction
                    raise VlobVersionError()

                if result != "INSERT 0 1":
                    raise VlobError(f"Insertion error: {result}")

                if notify_beacon:
                    await self.beacon_component.ll_update(conn, notify_beacon, id, version, author)
