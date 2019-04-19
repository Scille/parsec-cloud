# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from triopg import UniqueViolationError
from uuid import UUID
from typing import List, Tuple, Dict

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobAccessError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler, send_signal


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def _vlob_updated(
        self,
        conn,
        vlob_atom_internal_id,
        organization_id,
        author,
        vlob_group,
        src_id,
        src_version=1,
    ):
        index = await conn.fetchval(
            """
INSERT INTO vlob_group_update (
    vlob_group, index, vlob_atom
)
SELECT
    get_vlob_group_internal_id($1, $2),
    (
        SELECT COALESCE(MAX(index) + 1, 1)
        FROM vlob_group_update
        WHERE vlob_group = get_vlob_group_internal_id($1, $2)
    ),
    $3
RETURNING index
""",
            organization_id,
            vlob_group,
            vlob_atom_internal_id,
        )

        await send_signal(
            conn,
            "vlob_group.updated",
            organization_id=organization_id,
            author=author,
            id=vlob_group,
            checkpoint=index,
            src_id=src_id,
            src_version=src_version,
        )

    async def get_group_rights(
        self, organization_id: OrganizationID, author: UserID, id: UUID
    ) -> Dict[DeviceID, Tuple[bool, bool, bool]]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetch(
                    """
SELECT user_has_vlob_group_any_right(
        get_user_internal_id($1, $3),
        get_vlob_group_internal_id($1, $2)
    )
FROM vlob_group_right
WHERE vlob_group = get_vlob_group_internal_id($1, $2)
""",
                    organization_id,
                    id,
                    author,
                )
                if not ret:
                    raise VlobNotFoundError(f"Group `{id}` doesn't exist")
                elif not ret[0][0]:
                    raise VlobAccessError()

                ret = await conn.fetch(
                    """
SELECT
    get_user_id(user_),
    admin_right,
    read_right,
    write_right
FROM vlob_group_right
WHERE
    vlob_group = get_vlob_group_internal_id($1, $2)
    and (admin_right OR read_right OR write_right)
""",
                    organization_id,
                    id,
                )

        return {u: (aa, ra, wa) for u, aa, ra, wa in ret}

    async def update_group_rights(
        self,
        organization_id: OrganizationID,
        author: UserID,
        id: UUID,
        user: UserID,
        admin_right: bool,
        read_right: bool,
        write_right: bool,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetch(
                    """
SELECT
    vlob_group,
    get_user_internal_id($1, $4),
    user_has_vlob_group_admin_right(
        get_user_internal_id($1, $3),
        get_vlob_group_internal_id($1, $2)
    )
FROM vlob_group_right
WHERE vlob_group = get_vlob_group_internal_id($1, $2)
""",
                    organization_id,
                    id,
                    author,
                    user,
                )
                if not ret:
                    raise VlobNotFoundError(f"Group `{id}` doesn't exist")

                internal_vlob_group_id, internal_user_id, has_right = ret[0]
                if not has_right:
                    raise VlobAccessError()

                if not internal_user_id:
                    raise VlobNotFoundError(f"User `{user}` doesn't exist")

                if not internal_vlob_group_id:
                    raise VlobNotFoundError(f"Group `{id}` doesn't exist")

                await conn.execute(
                    """
INSERT INTO vlob_group_right (
    vlob_group,
    user_,
    admin_right,
    read_right,
    write_right
) SELECT
    $1,
    $2,
    $3,
    $4,
    $5
ON CONFLICT (vlob_group, user_)
DO UPDATE
SET
    admin_right = excluded.admin_right,
    read_right = excluded.read_right,
    write_right = excluded.write_right
""",
                    internal_vlob_group_id,
                    internal_user_id,
                    admin_right,
                    read_right,
                    write_right,
                )

    async def poll_group(
        self, organization_id: OrganizationID, author: UserID, id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetch(
                    """
SELECT user_has_vlob_group_read_right(
        get_user_internal_id($1, $3),
        get_vlob_group_internal_id($1, $2)
    )
FROM vlob_group_right
WHERE vlob_group = get_vlob_group_internal_id($1, $2)
""",
                    organization_id,
                    id,
                    author,
                )
                if not ret:
                    raise VlobNotFoundError(f"Group `{id}` doesn't exist")

                elif not ret[0][0]:
                    raise VlobAccessError()

                ret = await conn.fetch(
                    """
SELECT
    index,
    get_vlob_id(vlob_atom.vlob),
    vlob_atom.version
FROM vlob_group_update
LEFT JOIN vlob_atom ON vlob_group_update.vlob_atom = vlob_atom._id
WHERE
    vlob_group = get_vlob_group_internal_id($1, $2)
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

    async def group_check(
        self, organization_id: OrganizationID, user: UserID, to_check: List[dict]
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
FROM vlob_atom
LEFT JOIN vlob ON vlob._id = vlob_atom.vlob
WHERE
    organization = get_organization_internal_id($1)
    AND vlob_id = any($3::uuid[])
    AND user_has_vlob_group_read_right(
        get_user_internal_id($1, $2),
        vlob_group
    )
ORDER BY vlob_id, version DESC
""",
                organization_id,
                user,
                to_check_dict.keys(),
            )

        for id, version in rows:
            if version != to_check_dict[id]["version"]:
                changed.append({"id": id, "version": version})

        return changed

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        id: UUID,
        group: UUID,
        timestamp: pendulum.Pendulum,
        blob: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                # Create vlob group if doesn't exist yet
                ret = await conn.execute(
                    """
INSERT INTO vlob_group (
    organization,
    vlob_group_id
) SELECT
    get_organization_internal_id($1),
    $2
ON CONFLICT DO NOTHING
""",
                    organization_id,
                    group,
                )
                if ret == "INSERT 0 1":
                    await conn.execute(
                        """
INSERT INTO vlob_group_right (
    vlob_group, user_, admin_right, read_right, write_right
) SELECT
    get_vlob_group_internal_id($1, $2),
    get_user_internal_id($1, $3),
    TRUE, TRUE, TRUE
""",
                        organization_id,
                        group,
                        author.user_id,
                    )

                # Actually create the vlob
                try:
                    vlob_internal_id = await conn.fetchval(
                        """
INSERT INTO vlob (
    organization, vlob_group, vlob_id
)
SELECT
    get_organization_internal_id($1),
    get_vlob_group_internal_id($1, $3),
    $4
WHERE
    user_has_vlob_group_write_right(
        get_user_internal_id($1, $2),
        get_vlob_group_internal_id($1, $3)
    )
RETURNING _id
""",
                        organization_id,
                        author.user_id,
                        group,
                        id,
                    )

                except UniqueViolationError:
                    raise VlobAlreadyExistsError()

                if not vlob_internal_id:
                    raise VlobAccessError()

                # Finally insert the first vlob atom
                vlob_atom_internal_id = await conn.fetchval(
                    """
INSERT INTO vlob_atom (
    vlob, version, blob, author, created_on
)
SELECT
    $2,
    1,
    $3,
    get_device_internal_id($1, $4),
    $5
RETURNING _id
""",
                    organization_id,
                    vlob_internal_id,
                    blob,
                    author,
                    timestamp,
                )

                await self._vlob_updated(
                    conn, vlob_atom_internal_id, organization_id, author, group, id
                )

    async def read(
        self, organization_id: OrganizationID, user: UserID, id: UUID, version: int = None
    ) -> Tuple[int, bytes, DeviceID, pendulum.Pendulum]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                if version is None:
                    data = await conn.fetchrow(
                        """
SELECT
    user_has_vlob_group_read_right(
        get_user_internal_id($1, $2),
        vlob.vlob_group
    ),
    version,
    blob,
    get_device_id(author),
    created_on
FROM vlob_atom
LEFT JOIN vlob ON vlob._id = vlob_atom.vlob
WHERE
    vlob = get_vlob_internal_id($1, $3)
ORDER BY version DESC
""",
                        organization_id,
                        user,
                        id,
                    )
                    if not data:
                        raise VlobNotFoundError(f"Vlob `{id}` doesn't exist")

                else:
                    data = await conn.fetchrow(
                        """
SELECT
    user_has_vlob_group_read_right(
        get_user_internal_id($1, $2),
        vlob.vlob_group
    ),
    version,
    blob,
    get_device_id(author),
    created_on
FROM vlob_atom
LEFT JOIN vlob ON vlob._id = vlob_atom.vlob
WHERE
    vlob = get_vlob_internal_id($1, $3)
    AND version = $4
""",
                        organization_id,
                        user,
                        id,
                        version,
                    )
                    if not data:
                        exists = await conn.fetchval(
                            """SELECT get_vlob_internal_id($1, $2)""", organization_id, id
                        )
                        if exists:
                            raise VlobVersionError()

                        else:
                            raise VlobNotFoundError(f"Vlob `{id}` doesn't exist")

            if not data[0]:
                raise VlobAccessError()

        return data[1:]

    async def update(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        id: UUID,
        version: int,
        timestamp: pendulum.Pendulum,
        blob: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                previous = await conn.fetchrow(
                    """
SELECT
    user_has_vlob_group_write_right(
        get_user_internal_id($1, $2),
        (SELECT vlob_group FROM vlob where _id = vlob)
    ),
    version
FROM vlob_atom
WHERE
    vlob = get_vlob_internal_id($1, $3)
ORDER BY version DESC LIMIT 1
""",
                    organization_id,
                    author.user_id,
                    id,
                )
                if not previous:
                    raise VlobNotFoundError(f"Vlob `{id}` doesn't exist")

                elif not previous[0]:
                    raise VlobAccessError()

                elif previous[1] != version - 1:
                    raise VlobVersionError()

                try:
                    vlob_group_id, vlob_atom_internal_id = await conn.fetchrow(
                        """
INSERT INTO vlob_atom (
    vlob, version, blob, author, created_on
)
SELECT
    get_vlob_internal_id($1, $2),
    $3,
    $4,
    get_device_internal_id($1, $5),
    $6
RETURNING (
    SELECT get_vlob_group_id(vlob_group)
    FROM vlob
    WHERE _id = vlob
), _id
""",
                        organization_id,
                        id,
                        version,
                        blob,
                        author,
                        timestamp,
                    )

                except UniqueViolationError:
                    # Should not occurs in theory given we are in a transaction
                    raise VlobVersionError()

                await self._vlob_updated(
                    conn, vlob_atom_internal_id, organization_id, author, vlob_group_id, id, version
                )
