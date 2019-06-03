# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from triopg import UniqueViolationError
from uuid import UUID
from typing import List, Tuple, Dict, Optional

from parsec.types import DeviceID, OrganizationID
from parsec.backend.realm import RealmRole
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobAccessError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
    VlobEncryptionRevisionError,
    VlobInMaintenanceError,
    VlobMaintenanceError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler, send_signal
from parsec.backend.drivers.postgresql.realm import (
    create_realm,
    get_realm_status,
    RealmNotFoundError,
    RealmAlreadyExistsError,
)


_STR_TO_ROLE = {role.value: role for role in RealmRole}


async def _check_realm(
    conn, organization_id, realm_id, encryption_revision, expected_maintenance=False
):
    try:
        rep = await get_realm_status(conn, organization_id, realm_id)

    except RealmNotFoundError as exc:
        raise VlobNotFoundError(*exc.args) from exc

    if encryption_revision is not None and rep["encryption_revision"] != encryption_revision:
        raise VlobEncryptionRevisionError()

    if expected_maintenance is False:
        if rep["maintenance_type"]:
            raise VlobInMaintenanceError("Data realm is currently under maintenance")
    elif expected_maintenance is True:
        if not rep["maintenance_type"]:
            raise VlobMaintenanceError(f"Realm `{realm_id}` not under maintenance")


async def _check_realm_access(conn, organization_id, realm_id, author, allowed_roles):
    rep = await conn.fetchrow(
        """
WITH cte_realm_roles AS (
SELECT user_, role
FROM realm_user_role
WHERE
    realm = get_realm_internal_id($1, $2)
)
SELECT role
FROM user_
LEFT JOIN cte_realm_roles
ON user_._id = cte_realm_roles.user_
WHERE user_._id = get_user_internal_id($1, $3)
        """,
        organization_id,
        realm_id,
        author.user_id,
    )
    if not rep:
        import pdb

        pdb.set_trace()
        raise VlobNotFoundError(f"User `{author.user_id}` doesn't exist")

    if _STR_TO_ROLE.get(rep[0]) not in allowed_roles:
        raise VlobAccessError()


async def _check_realm_and_maintenance_access(
    conn, organization_id, author, realm_id, encryption_revision
):
    await _check_realm(
        conn, organization_id, realm_id, encryption_revision, expected_maintenance=True
    )
    can_write_roles = (RealmRole.OWNER,)
    await _check_realm_access(conn, organization_id, realm_id, author, can_write_roles)


async def _check_realm_and_write_access(
    conn, organization_id, author, realm_id, encryption_revision
):
    await _check_realm(conn, organization_id, realm_id, encryption_revision)
    can_write_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR)
    await _check_realm_access(conn, organization_id, realm_id, author, can_write_roles)


async def _check_realm_and_read_access(
    conn, organization_id, author, realm_id, encryption_revision
):
    await _check_realm(conn, organization_id, realm_id, encryption_revision)
    can_read_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR, RealmRole.READER)
    await _check_realm_access(conn, organization_id, realm_id, author, can_read_roles)


async def _vlob_updated(
    conn, vlob_atom_internal_id, organization_id, author, realm_id, src_id, src_version=1
):
    index = await conn.fetchval(
        """
INSERT INTO realm_vlob_update (
realm, index, vlob_atom
)
SELECT
get_realm_internal_id($1, $2),
(
    SELECT COALESCE(MAX(index) + 1, 1)
    FROM realm_vlob_update
    WHERE realm = get_realm_internal_id($1, $2)
),
$3
RETURNING index
""",
        organization_id,
        realm_id,
        vlob_atom_internal_id,
    )

    await send_signal(
        conn,
        "realm.vlobs_updated",
        organization_id=organization_id,
        author=author,
        realm_id=realm_id,
        checkpoint=index,
        src_id=src_id,
        src_version=src_version,
    )


async def _get_realm_id_from_vlob_id(conn, organization_id, vlob_id):
    realm_id = await conn.fetchval(
        """
SELECT
    realm.realm_id
FROM vlob_atom
INNER JOIN vlob_encryption_revision
ON  vlob_atom.vlob_encryption_revision = vlob_encryption_revision._id
INNER JOIN realm
ON vlob_encryption_revision.realm = realm._id
WHERE vlob_atom._id = get_vlob_atom_internal_id($1, $2)
LIMIT 1
            """,
        organization_id,
        vlob_id,
    )
    if not realm_id:
        raise VlobNotFoundError(f"Vlob `{vlob_id}` doesn't exist")
    return realm_id


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        vlob_id: UUID,
        timestamp: pendulum.Pendulum,
        blob: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                try:
                    await create_realm(conn, organization_id, author, realm_id)
                except RealmAlreadyExistsError:
                    pass

                await _check_realm_and_write_access(
                    conn, organization_id, author, realm_id, encryption_revision
                )

                # Actually create the vlob
                try:
                    vlob_atom_internal_id = await conn.fetchval(
                        """
INSERT INTO vlob_atom (
    organization,
    vlob_encryption_revision,
    vlob_id,
    version,
    blob,
    author,
    created_on
)
SELECT
    get_organization_internal_id($1),
    get_vlob_encryption_revision_internal_id($1, $3, $4),
    $5,
    1,
    $6,
    get_device_internal_id($1, $2),
    $7
RETURNING _id
""",
                        organization_id,
                        author,
                        realm_id,
                        encryption_revision,
                        vlob_id,
                        blob,
                        timestamp,
                    )

                except UniqueViolationError:
                    raise VlobAlreadyExistsError()

                await _vlob_updated(
                    conn, vlob_atom_internal_id, organization_id, author, realm_id, vlob_id
                )

    async def read(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: UUID,
        version: Optional[int] = None,
    ) -> Tuple[int, bytes, DeviceID, pendulum.Pendulum]:

        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                realm_id = await _get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
                await _check_realm_and_read_access(
                    conn, organization_id, author, realm_id, encryption_revision
                )

                if version is None:
                    data = await conn.fetchrow(
                        """
SELECT
    version,
    blob,
    get_device_id(author) as author,
    created_on
FROM vlob_atom
WHERE
    vlob_encryption_revision = get_vlob_encryption_revision_internal_id($1, $2, $3)
    AND vlob_id = $4
ORDER BY version DESC
LIMIT 1
""",
                        organization_id,
                        realm_id,
                        encryption_revision,
                        vlob_id,
                    )
                    assert data  # _get_realm_id_from_vlob_id checks vlob presence

                else:
                    data = await conn.fetchrow(
                        """
SELECT
    version,
    blob,
    get_device_id(author) as author,
    created_on
FROM vlob_atom
WHERE
    vlob_encryption_revision = get_vlob_encryption_revision_internal_id($1, $2, $3)
    AND vlob_id = $4
    AND version = $5
""",
                        organization_id,
                        realm_id,
                        encryption_revision,
                        vlob_id,
                        version,
                    )
                    if not data:
                        raise VlobVersionError()

        return list(data)

    async def update(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: UUID,
        version: int,
        timestamp: pendulum.Pendulum,
        blob: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                realm_id = await _get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
                await _check_realm_and_write_access(
                    conn, organization_id, author, realm_id, encryption_revision
                )

                previous = await conn.fetchrow(
                    """
SELECT
    version
FROM vlob_atom
WHERE
    organization = get_organization_internal_id($1)
    AND vlob_id = $2
ORDER BY version DESC LIMIT 1
""",
                    organization_id,
                    vlob_id,
                )
                if not previous:
                    raise VlobNotFoundError(f"Vlob `{vlob_id}` doesn't exist")

                elif previous["version"] != version - 1:
                    raise VlobVersionError()

                try:
                    vlob_atom_internal_id = await conn.fetchval(
                        """
INSERT INTO vlob_atom (
    organization,
    vlob_encryption_revision,
    vlob_id,
    version,
    blob,
    author,
    created_on
)
SELECT
    get_organization_internal_id($1),
    get_vlob_encryption_revision_internal_id($1, $3, $4),
    $5,
    $8,
    $6,
    get_device_internal_id($1, $2),
    $7
RETURNING _id
""",
                        organization_id,
                        author,
                        realm_id,
                        encryption_revision,
                        vlob_id,
                        blob,
                        timestamp,
                        version,
                    )

                except UniqueViolationError:
                    # Should not occurs in theory given we are in a transaction
                    raise VlobVersionError()

                await _vlob_updated(
                    conn, vlob_atom_internal_id, organization_id, author, realm_id, vlob_id, version
                )

    async def group_check(
        self, organization_id: OrganizationID, author: DeviceID, to_check: List[dict]
    ) -> List[dict]:
        changed = []
        to_check_dict = {}
        for x in to_check:
            if x["version"] == 0:
                changed.append({"vlob_id": x["vlob_id"], "version": 0})
            else:
                to_check_dict[x["vlob_id"]] = x

        async with self.dbh.pool.acquire() as conn:
            rows = await conn.fetch(
                """
SELECT DISTINCT ON (vlob_id) vlob_id, version
FROM vlob_atom
WHERE
    organization = get_organization_internal_id($1)
    AND vlob_id = any($3::uuid[])
    AND user_has_realm_read_access(
        get_user_internal_id($1, $2),
        (
            SELECT realm
            FROM vlob_encryption_revision
            WHERE _id = vlob_encryption_revision
        )
    )
    AND NOT is_realm_in_maintenance(
        (
            SELECT realm
            FROM vlob_encryption_revision
            WHERE _id = vlob_encryption_revision
        )
    )
ORDER BY vlob_id, version DESC
""",
                organization_id,
                author.user_id,
                to_check_dict.keys(),
            )

        for vlob_id, version in rows:
            if version != to_check_dict[vlob_id]["version"]:
                changed.append({"vlob_id": vlob_id, "version": version})

        return changed

    async def poll_changes(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                await _check_realm_and_read_access(conn, organization_id, author, realm_id, None)

                ret = await conn.fetch(
                    """
SELECT
    index,
    vlob_id,
    vlob_atom.version
FROM realm_vlob_update
LEFT JOIN vlob_atom ON realm_vlob_update.vlob_atom = vlob_atom._id
WHERE
    realm = get_realm_internal_id($1, $2)
    AND index > $3
ORDER BY index ASC
""",
                    organization_id,
                    realm_id,
                    checkpoint,
                )

        changes_since_checkpoint = {src_id: src_version for _, src_id, src_version in ret}
        new_checkpoint = ret[-1][0] if ret else checkpoint
        return (new_checkpoint, changes_since_checkpoint)

    async def maintenance_get_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        size: int,
    ) -> List[Tuple[UUID, int, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                await _check_realm_and_maintenance_access(
                    conn, organization_id, author, realm_id, encryption_revision
                )
                rep = await conn.fetch(
                    """
WITH cte_to_encrypt AS (
    SELECT vlob_id, version, blob
    FROM vlob_atom
    WHERE vlob_encryption_revision = get_vlob_encryption_revision_internal_id($1, $2, $3 - 1)
),
cte_encrypted AS (
    SELECT vlob_id, version
    FROM vlob_atom
    WHERE vlob_encryption_revision = get_vlob_encryption_revision_internal_id($1, $2, $3)
)
SELECT
    cte_to_encrypt.vlob_id,
    cte_to_encrypt.version,
    blob
FROM cte_to_encrypt
LEFT JOIN cte_encrypted
ON cte_to_encrypt.vlob_id = cte_encrypted.vlob_id AND cte_to_encrypt.version = cte_encrypted.version
WHERE cte_encrypted.vlob_id IS NULL
LIMIT $4
                    """,
                    organization_id,
                    realm_id,
                    encryption_revision,
                    size,
                )
                return [(row["vlob_id"], row["version"], row["blob"]) for row in rep]

    async def maintenance_save_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        batch: List[Tuple[UUID, int, bytes]],
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                await _check_realm_and_maintenance_access(
                    conn, organization_id, author, realm_id, encryption_revision
                )
                for vlob_id, version, blob in batch:
                    await conn.execute(
                        """
INSERT INTO vlob_atom(
    organization,
    vlob_encryption_revision,
    vlob_id,
    version,
    blob,
    author,
    created_on,
    deleted_on
)
SELECT
    organization,
    get_vlob_encryption_revision_internal_id(
        $1,
        $2,
        $5
    ),
    $3,
    $4,
    $6,
    author,
    created_on,
    deleted_on
FROM vlob_atom
WHERE
    organization = get_organization_internal_id($1)
    AND vlob_id = $3
    AND version = $4
ON CONFLICT DO NOTHING
""",
                        organization_id,
                        realm_id,
                        vlob_id,
                        version,
                        encryption_revision,
                        blob,
                    )
