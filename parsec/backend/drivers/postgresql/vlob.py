# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from triopg import UniqueViolationError
from uuid import UUID
from typing import List, Tuple

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobAccessError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler, beacon_component: BaseBeaconComponent):
        self.dbh = dbh
        self.beacon_component = beacon_component

    async def group_check(
        self, organization_id: OrganizationID, user_id: UserID, to_check: List[dict]
    ) -> List[dict]:

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
SELECT DISTINCT ON (vlob_id) vlob_id, version
FROM vlob_atoms
LEFT JOIN vlobs ON vlobs._id = vlob_atoms.vlob
WHERE
    organization = get_organization_internal_id($1)
    AND vlob_id = any($3::uuid[])
    AND user_can_read_beacon(get_user_internal_id($1, $2), beacon)
ORDER BY vlob_id, version DESC
""",
                organization_id,
                user_id,
                to_check_dict.keys(),
            )

        for id, version in rows:
            if version != to_check_dict[id]["version"]:
                changed.append({"id": id, "version": version})

        return changed

    async def create(
        self, organization_id: OrganizationID, author: DeviceID, beacon: UUID, id: UUID, blob: bytes
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                # Create beacon if doesn't exist yet
                ret = await conn.execute(
                    """
INSERT INTO beacons (
    organization,
    beacon_id
) SELECT
    get_organization_internal_id($1),
    $2
ON CONFLICT DO NOTHING
                    """,
                    organization_id,
                    beacon,
                )
                if ret == "INSERT 0 1":
                    await conn.execute(
                        """
INSERT INTO beacon_accesses (
    beacon, user_, admin_access, read_access, write_access
) SELECT
    get_beacon_internal_id($1, $2),
    get_user_internal_id($1, $3),
    TRUE, TRUE, TRUE
                        """,
                        organization_id,
                        beacon,
                        author.user_id,
                    )

                # Actually create the vlob
                try:
                    vlob_internal_id = await conn.fetchval(
                        """
INSERT INTO vlobs (
    organization, beacon, vlob_id
)
SELECT
    get_organization_internal_id($1),
    get_beacon_internal_id($1, $3),
    $4
WHERE
    user_can_write_beacon(
        get_user_internal_id($1, $2),
        get_beacon_internal_id($1, $3)
    )
RETURNING _id
""",
                        organization_id,
                        author.user_id,
                        beacon,
                        id,
                    )
                except UniqueViolationError:
                    raise VlobAlreadyExistsError()

                if not vlob_internal_id:
                    raise VlobAccessError()

                # Finally insert the first vlob atom
                vlob_atom_internal_id = await conn.fetchval(
                    """
INSERT INTO vlob_atoms (
    vlob, version, blob, author
)
SELECT
    $2,
    1,
    $3,
    get_device_internal_id($1, $4)
RETURNING _id
""",
                    organization_id,
                    vlob_internal_id,
                    blob,
                    author,
                )

                await self.beacon_component._vlob_updated(
                    conn, vlob_atom_internal_id, organization_id, author, beacon, id
                )

    async def read(
        self, organization_id: OrganizationID, user_id: UserID, id: UUID, version: int = None
    ) -> Tuple[int, bytes]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                if version is None:
                    data = await conn.fetchrow(
                        """
SELECT
    user_can_read_beacon(
        get_user_internal_id($1, $2),
        vlobs.beacon
    ),
    version,
    blob
-- TODO retrieve also `author` and `created_on`
FROM vlob_atoms
LEFT JOIN vlobs ON vlobs._id = vlob_atoms.vlob
WHERE
    vlob = get_vlob_internal_id($1, $3)
ORDER BY version DESC
""",
                        organization_id,
                        user_id,
                        id,
                    )
                    if not data:
                        raise VlobNotFoundError()

                else:
                    data = await conn.fetchrow(
                        """
SELECT
    user_can_read_beacon(
        get_user_internal_id($1, $2),
        vlobs.beacon
    ),
    version,
    blob
-- TODO retrieve also `author` and `created_on`
FROM vlob_atoms
LEFT JOIN vlobs ON vlobs._id = vlob_atoms.vlob
WHERE
    vlob = get_vlob_internal_id($1, $3)
    AND version = $4
""",
                        organization_id,
                        user_id,
                        id,
                        version,
                    )
                    if not data:
                        # TODO: not cool to need 2nd request to know the error...
                        exists = await conn.fetchval(
                            """
SELECT get_vlob_internal_id($1, $2)
""",
                            organization_id,
                            id,
                        )
                        if exists:
                            raise VlobVersionError()

                        else:
                            raise VlobNotFoundError()

            if not data[0]:
                raise VlobAccessError()

        return data[1:]

    async def update(
        self, organization_id: OrganizationID, author: DeviceID, id: UUID, version: int, blob: bytes
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                previous = await conn.fetchrow(
                    """
SELECT
    user_can_write_beacon(
        get_user_internal_id($1, $2),
        (SELECT beacon FROM vlobs where _id = vlob)
    ),
    version
FROM vlob_atoms
WHERE
    vlob = get_vlob_internal_id($1, $3)
ORDER BY version DESC LIMIT 1
""",
                    organization_id,
                    author.user_id,
                    id,
                )
                if not previous:
                    raise VlobNotFoundError()
                elif not previous[0]:
                    raise VlobAccessError()
                elif previous[1] != version - 1:
                    raise VlobVersionError()

                try:
                    beacon_id, vlob_atom_internal_id = await conn.fetchrow(
                        """
INSERT INTO vlob_atoms (
    vlob, version, blob, author
)
SELECT
    get_vlob_internal_id($1, $2),
    $3,
    $4,
    get_device_internal_id($1, $5)
RETURNING (
    SELECT (
        SELECT beacon_id
        FROM beacons
        WHERE _id = beacon
    ) FROM vlobs
    WHERE _id = vlob
), _id
""",
                        organization_id,
                        id,
                        version,
                        blob,
                        author,
                    )

                except UniqueViolationError:
                    # Should not occurs in theory given we are in a transaction
                    raise VlobVersionError()

                await self.beacon_component._vlob_updated(
                    conn, vlob_atom_internal_id, organization_id, author, beacon_id, id, version
                )
