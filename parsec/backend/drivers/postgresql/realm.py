# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from uuid import UUID
from typing import Dict, Optional
from triopg import UniqueViolationError

from parsec.api.protocole import RealmRole, MaintenanceType
from parsec.types import DeviceID, UserID, OrganizationID
from parsec.backend.realm import (
    BaseRealmComponent,
    RealmStatus,
    RealmAccessError,
    RealmNotFoundError,
    RealmAlreadyExistsError,
    RealmEncryptionRevisionError,
    RealmMaintenanceError,
    RealmInMaintenanceError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler
from parsec.backend.drivers.postgresql.message import send_message


_STR_TO_ROLE = {role.value: role for role in RealmRole}
_STR_TO_MAINTENANCE_TYPE = {type.value: type for type in MaintenanceType}


async def create_realm(conn, organization_id: OrganizationID, author: DeviceID, realm_id: UUID):
    realm_internal_id = await conn.fetchval(
        """
INSERT INTO realm (
    organization,
    realm_id,
    encryption_revision
) SELECT
    get_organization_internal_id($1),
    $2,
    1
ON CONFLICT (organization, realm_id) DO NOTHING
RETURNING
    _id
""",
        organization_id,
        realm_id,
    )
    if not realm_internal_id:
        raise RealmAlreadyExistsError()

    await conn.execute(
        """
INSERT INTO realm_user_role(
    realm, user_, role
)
SELECT
    $1,
    get_user_internal_id($2, $3),
    'OWNER'
""",
        realm_internal_id,
        organization_id,
        author.user_id,
    )

    await conn.execute(
        """
INSERT INTO vlob_encryption_revision (
    realm,
    encryption_revision
)
SELECT
    $1,
    1
""",
        realm_internal_id,
    )


async def get_realm_status(conn, organization_id, realm_id):
    rep = await conn.fetchrow(
        """
SELECT
encryption_revision,
maintenance_started_by,
maintenance_started_on,
maintenance_type
FROM realm
WHERE _id = get_realm_internal_id($1, $2)
        """,
        organization_id,
        realm_id,
    )
    if not rep:
        raise RealmNotFoundError(f"Realm `{realm_id}` doesn't exist")
    return rep


async def get_realm_role_for(conn, organization_id, realm_id, users=None):
    rep = await conn.fetch(
        """
WITH cte_users AS (
SELECT
    _id,
    user_id
FROM user_
WHERE
    organization = get_organization_internal_id($1)
),
cte_realm_roles AS (
SELECT
    user_,
    role
FROM realm_user_role
WHERE
    realm = get_realm_internal_id($1, $2)
)
SELECT
cte_users.user_id,
cte_realm_roles.role
FROM cte_users
LEFT JOIN cte_realm_roles
ON cte_users._id = cte_realm_roles.user_
        """,
        organization_id,
        realm_id,
    )
    roles = {row["user_id"]: _STR_TO_ROLE.get(row["role"]) for row in rep}
    for user in users or ():
        if user not in roles:
            raise RealmNotFoundError(f"User `{user}` doesn't exist")
    if users:
        return {user_id: role for user_id, role in roles.items() if user_id in users}
    else:
        return {user_id: role for user_id, role in roles.items() if role}


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def get_status(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID
    ) -> RealmStatus:
        async with self.dbh.pool.acquire() as conn:
            ret = await conn.fetchrow(
                """
SELECT
    user_has_realm_read_access(
        get_user_internal_id($1, $3),
        get_realm_internal_id($1, $2)
    ) as has_access,
    encryption_revision,
    get_device_id(maintenance_started_by) as maintenance_started_by,
    maintenance_started_on,
    maintenance_type
FROM realm
WHERE _id = get_realm_internal_id($1, $2)
""",
                organization_id,
                realm_id,
                author.user_id,
            )
        if not ret:
            raise RealmNotFoundError(f"Realm `{realm_id}` doesn't exist")
        if not ret["has_access"]:
            raise RealmAccessError()
        return RealmStatus(
            maintenance_type=_STR_TO_MAINTENANCE_TYPE.get(ret["maintenance_type"]),
            maintenance_started_on=ret["maintenance_started_on"],
            maintenance_started_by=ret["maintenance_started_by"],
            encryption_revision=ret["encryption_revision"],
        )

    async def get_roles(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID
    ) -> Dict[UserID, RealmRole]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetch(
                    """
SELECT
    get_user_id(user_),
    role
FROM realm_user_role
WHERE
    realm = get_realm_internal_id($1, $2)
""",
                    organization_id,
                    realm_id,
                )

                if not ret:
                    # Existing group must have at least one owner user
                    raise RealmNotFoundError(f"Realm `{realm_id}` doesn't exist")
                roles = {UserID(user_id): _STR_TO_ROLE[role] for user_id, role in ret}
                if author.user_id not in roles:
                    raise RealmAccessError()

        return roles

    async def update_roles(
        self,
        organization_id: OrganizationID,
        author: UserID,
        realm_id: UUID,
        user: UserID,
        role: Optional[RealmRole],
    ) -> None:
        if author.user_id == user:
            raise RealmAccessError("Cannot modify our own role")

        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                # Retrieve realm and make sure it is not under maintenance
                rep = await get_realm_status(conn, organization_id, realm_id)
                if rep["maintenance_type"]:
                    raise RealmInMaintenanceError("Data realm is currently under maintenance")

                # Check access rights
                roles = await get_realm_role_for(
                    conn, organization_id, realm_id, (author.user_id, user)
                )
                author_role = roles[author.user_id]
                existing_user_role = roles[user]

                if existing_user_role in (RealmRole.MANAGER, RealmRole.OWNER):
                    needed_roles = (RealmRole.OWNER,)
                else:
                    needed_roles = (RealmRole.MANAGER, RealmRole.OWNER)

                if author_role not in needed_roles:
                    raise RealmAccessError()

                # Do the changes
                if not role:
                    await conn.execute(
                        """
DELETE FROM realm_user_role
WHERE user_ = get_user_internal_id($1, $2)
                        """,
                        organization_id,
                        user,
                    )
                else:
                    await conn.execute(
                        """
INSERT INTO realm_user_role(
    realm,
    user_,
    role
) SELECT
    get_realm_internal_id($1, $2),
    get_user_internal_id($1, $3),
    $4
ON CONFLICT (realm, user_)
DO UPDATE
SET
    role = excluded.role
""",
                        organization_id,
                        realm_id,
                        user,
                        role.value,
                    )

    async def start_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        per_participant_message: Dict[UserID, bytes],
        timestamp: pendulum.Pendulum,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                # Retrieve realm and make sure it is not under maintenance
                rep = await get_realm_status(conn, organization_id, realm_id)
                if rep["maintenance_type"]:
                    raise RealmInMaintenanceError(f"Realm `{realm_id}` alrealy in maintenance")
                if encryption_revision != rep["encryption_revision"] + 1:
                    raise RealmEncryptionRevisionError("Invalid encryption revision")

                roles = await get_realm_role_for(conn, organization_id, realm_id)
                if per_participant_message.keys() ^ roles.keys():
                    raise RealmMaintenanceError(
                        "Realm participants and message recipients mismatch"
                    )

                if roles.get(author.user_id) != RealmRole.OWNER:
                    raise RealmAccessError()

                await conn.execute(
                    """
UPDATE realm
SET
    encryption_revision=$6,
    maintenance_started_by=get_device_internal_id($1, $3),
    maintenance_started_on=$4,
    maintenance_type=$5
WHERE
    _id = get_realm_internal_id($1, $2)
""",
                    organization_id,
                    realm_id,
                    author,
                    timestamp,
                    "REENCRYPTION",
                    encryption_revision,
                )

                await conn.execute(
                    """
INSERT INTO vlob_encryption_revision(
    realm,
    encryption_revision
) SELECT
    get_realm_internal_id($1, $2),
    $3
""",
                    organization_id,
                    realm_id,
                    encryption_revision,
                )

        for recipient, body in per_participant_message.items():
            await send_message(conn, organization_id, author, recipient, timestamp, body)

    async def finish_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                # Retrieve realm and make sure it is not under maintenance
                rep = await get_realm_status(conn, organization_id, realm_id)
                roles = await get_realm_role_for(conn, organization_id, realm_id, [author.user_id])
                if roles.get(author.user_id) != RealmRole.OWNER:
                    raise RealmAccessError()
                if not rep["maintenance_type"]:
                    raise RealmMaintenanceError(f"Realm `{realm_id}` not under maintenance")
                if encryption_revision != rep["encryption_revision"]:
                    raise RealmEncryptionRevisionError("Invalid encryption revision")

                # Test reencryption operations are over

                rep = await conn.fetch(
                    """
WITH cte_encryption_revisions AS (
    SELECT
        _id,
        encryption_revision
    FROM vlob_encryption_revision
    WHERE
        realm = get_realm_internal_id($1, $2)
        AND (encryption_revision = $3 OR encryption_revision = $3 - 1)
)
SELECT encryption_revision, COUNT(*) as count
FROM vlob_atom
INNER JOIN cte_encryption_revisions
ON cte_encryption_revisions._id = vlob_atom.vlob_encryption_revision
GROUP BY encryption_revision
ORDER BY encryption_revision
                    """,
                    organization_id,
                    realm_id,
                    encryption_revision,
                )

                try:
                    previous, current = rep
                except ValueError:
                    raise RealmMaintenanceError("Reencryption operations are not over")
                assert previous["encryption_revision"] == encryption_revision - 1
                assert current["encryption_revision"] == encryption_revision
                assert previous["count"] >= current["count"]
                if previous["count"] != current["count"]:
                    raise RealmMaintenanceError("Reencryption operations are not over")

                await conn.execute(
                    """
UPDATE realm
SET
    maintenance_started_by=NULL,
    maintenance_started_on=NULL,
    maintenance_type=NULL
WHERE
    _id = get_realm_internal_id($1, $2)
""",
                    organization_id,
                    realm_id,
                )
