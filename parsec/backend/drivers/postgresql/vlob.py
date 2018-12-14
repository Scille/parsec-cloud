from triopg import UniqueViolationError

# from parsec.utils import ParsecError
from parsec.backend.vlob import BaseVlobComponent

# from parsec.backend.exceptions import (
#     TrustSeedError,
#     VersionError,
#     NotFoundError,
#     AlreadyExistsError,
# )

# TODO: fix this mock
class VlobAtom:
    pass


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh, event_bus, beacon_component):
        self.dbh = dbh
        self.beacon_component = beacon_component

    async def group_check(self, to_check):
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
                raise TrustSeedError()
            if version != to_check_dict[id]["version"]:
                changed.append({"id": id, "version": version})

        return changed

    async def create(self, id, rts, wts, blob, notify_beacon=None, author="anonymous"):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                try:
                    result = await conn.execute(
                        "INSERT INTO vlobs (vlob_id, rts, wts, version, blob) VALUES ($1, $2, $3, 1, $4)",
                        id,
                        rts,
                        wts,
                        blob,
                    )
                except UniqueViolationError:
                    raise AlreadyExistsError("Vlob already exists.")

                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")

                if notify_beacon:
                    await self.beacon_component.ll_update(conn, notify_beacon, id, 1, author)

        return VlobAtom(id=id, read_trust_seed=rts, write_trust_seed=wts, blob=blob)

    async def read(self, id, rts, version=None):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                if version is None:
                    data = await conn.fetchrow(
                        "SELECT rts, wts, version, blob FROM vlobs WHERE vlob_id = $1 ORDER BY version DESC LIMIT 1",
                        id,
                    )
                    if not data:
                        raise NotFoundError("Vlob not found.")

                else:
                    data = await conn.fetchrow(
                        "SELECT rts, wts, version, blob FROM vlobs WHERE vlob_id = $1 AND version = $2",
                        id,
                        version,
                    )
                    if not data:
                        # TODO: not cool to need 2nd request to know the error...
                        exists = await conn.fetchrow(
                            "SELECT true FROM vlobs WHERE vlob_id = $1", id
                        )
                        if exists:
                            raise VersionError("Wrong blob version.")

                        else:
                            raise NotFoundError("Vlob not found.")

            if data["rts"] != rts:
                raise TrustSeedError()

        return VlobAtom(
            id=id,
            read_trust_seed=rts,
            write_trust_seed=data["wts"],
            blob=data["blob"],
            version=data["version"],
        )

    async def update(self, id, wts, version, blob, notify_beacon=None, author="anonymous"):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                previous = await conn.fetchrow(
                    "SELECT wts, version, rts FROM vlobs WHERE vlob_id = $1 ORDER BY version DESC LIMIT 1",
                    id,
                )
                if not previous:
                    raise NotFoundError("Vlob not found.")
                elif previous[0] != wts:
                    raise TrustSeedError("Invalid write trust seed.")
                elif previous[1] != version - 1:
                    raise VersionError("Wrong blob version.")

                rts = previous[2]
                try:
                    result = await conn.execute(
                        "INSERT INTO vlobs (vlob_id, rts, wts, version, blob) VALUES ($1, $2, $3, $4, $5)",
                        id,
                        rts,
                        wts,
                        version,
                        blob,
                    )
                except UniqueViolationError:
                    raise VersionError("Wrong blob version.")

                if result != "INSERT 0 1":
                    raise ParsecError("Insertion error.")
                if notify_beacon:
                    await self.beacon_component.ll_update(conn, notify_beacon, id, version, author)
