# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Tuple, Dict
from uuid import UUID
from triopg.exceptions import NotNullViolationError

from parsec.types import UserID, DeviceID, OrganizationID
from parsec.backend.beacon import (
    BaseBeaconComponent,
    BeaconError,
    BeaconNotFound,
    BeaconAccessError,
)
from parsec.backend.drivers.postgresql.handler import send_signal, PGHandler


class PGBeaconComponent(BaseBeaconComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def get_rights(
        self, organization_id: OrganizationID, author: UserID, id: UUID
    ) -> Dict[DeviceID, Tuple[bool, bool, bool]]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetch(
                    """
SELECT user_can_read_beacon(
        get_user_internal_id($1, $3),
        get_beacon_internal_id($1, $2)
    )
FROM beacon_accesses
WHERE beacon = get_beacon_internal_id($1, $2)
                    """,
                    organization_id,
                    id,
                    author,
                )
                if not ret:
                    raise BeaconNotFound()
                elif not ret[0][0]:
                    raise BeaconAccessError()

                ret = await conn.fetch(
                    """
SELECT
    get_user_id(user_),
    admin_access,
    read_access,
    write_access
FROM beacon_accesses
WHERE
    beacon = get_beacon_internal_id($1, $2)
    and user_can_read_beacon(
        get_user_internal_id($1, $3),
        get_beacon_internal_id($1, $2)
    )
    and (admin_access OR read_access OR write_access)
                    """,
                    organization_id,
                    id,
                    author,
                )

        return {u: (aa, ra, wa) for u, aa, ra, wa in ret}

    async def set_rights(
        self,
        organization_id: OrganizationID,
        author: UserID,
        id: UUID,
        user: UserID,
        admin_access: bool,
        read_access: bool,
        write_access: bool,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetch(
                    """
SELECT user_can_administrate_beacon(
        get_user_internal_id($1, $3),
        get_beacon_internal_id($1, $2)
    )
FROM beacon_accesses
WHERE beacon = get_beacon_internal_id($1, $2)
                    """,
                    organization_id,
                    id,
                    author,
                )
                if not ret:
                    raise BeaconNotFound()
                elif not ret[0][0]:
                    raise BeaconAccessError()

                try:
                    ret = await conn.execute(
                        """
INSERT INTO beacon_accesses (
    beacon,
    user_,
    admin_access,
    read_access,
    write_access
) SELECT
    get_beacon_internal_id($1, $2),
    get_user_internal_id($1, $3),
    $4,
    $5,
    $6
ON CONFLICT (beacon, user_)
DO UPDATE
SET
    admin_access = excluded.admin_access,
    read_access = excluded.read_access,
    write_access = excluded.write_access
                        """,
                        organization_id,
                        id,
                        user,
                        admin_access,
                        read_access,
                        write_access,
                    )
                except NotNullViolationError:
                    raise BeaconError("Unknown user")

    async def poll(
        self, organization_id: OrganizationID, author: UserID, id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetch(
                    """
SELECT user_can_read_beacon(
        get_user_internal_id($1, $3),
        get_beacon_internal_id($1, $2)
    )
FROM beacon_accesses
WHERE beacon = get_beacon_internal_id($1, $2)
                    """,
                    organization_id,
                    id,
                    author,
                )
                if not ret:
                    raise BeaconNotFound()
                elif not ret[0][0]:
                    raise BeaconAccessError()

                ret = await conn.fetch(
                    """
SELECT
    index,
    get_vlob_id(vlob_atoms.vlob),
    vlob_atoms.version
FROM beacon_vlob_atom_updates
LEFT JOIN vlob_atoms ON beacon_vlob_atom_updates.vlob_atom = vlob_atoms._id
WHERE
    beacon = get_beacon_internal_id($1, $2)
    AND index > $3
ORDER BY index ASC
                    """,
                    organization_id,
                    id,
                    checkpoint,
                )

        changes_since_checkpoint = {src_id: src_version for _, src_id, src_version in ret}
        new_checkpoint = ret[-1][0] if ret else checkpoint
        return (new_checkpoint, changes_since_checkpoint)

    async def _vlob_updated(
        self, conn, vlob_atom_internal_id, organization_id, author, id, src_id, src_version=1
    ):
        index = await conn.fetchval(
            """
INSERT INTO beacon_vlob_atom_updates (
    beacon, index, vlob_atom
)
SELECT
    get_beacon_internal_id($1, $2),
    (
        SELECT COALESCE(MAX(index) + 1, 1)
        FROM beacon_vlob_atom_updates
        WHERE beacon = get_beacon_internal_id($1, $2)
    ),
    $3
RETURNING index
""",
            organization_id,
            id,
            vlob_atom_internal_id,
        )

        await send_signal(
            conn,
            "beacon.updated",
            organization_id=organization_id,
            author=author,
            beacon_id=id,
            checkpoint=index,
            src_id=src_id,
            src_version=src_version,
        )
