# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Dict, List
from uuid import UUID

import pendulum

from parsec.api.protocole import MaintenanceType, RealmRole
from parsec.backend.drivers.postgresql.handler import PGHandler, send_signal
from parsec.backend.drivers.postgresql.message import send_message
from parsec.backend.realm import (
    BaseRealmComponent,
    RealmAccessError,
    RealmAlreadyExistsError,
    RealmEncryptionRevisionError,
    RealmGrantedRole,
    RealmInMaintenanceError,
    RealmMaintenanceError,
    RealmNotFoundError,
    RealmNotInMaintenanceError,
    RealmParticipantsMismatchError,
    RealmStatus,
)
from parsec.types import DeviceID, OrganizationID, UserID

_STR_TO_ROLE = {role.value: role for role in RealmRole}
_STR_TO_MAINTENANCE_TYPE = {type.value: type for type in MaintenanceType}


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
    if users:
        rep = await conn.fetch(
            """
WITH cte_current_realm_roles AS (
    SELECT DISTINCT ON(user_) user_, role
    FROM  realm_user_role
    WHERE realm = get_realm_internal_id($1, $2)
    ORDER BY user_, certified_on DESC
)
SELECT user_.user_id as realm_id, role
FROM user_
LEFT JOIN cte_current_realm_roles
ON user_._id = cte_current_realm_roles.user_
WHERE
    organization = get_organization_internal_id($1)
    AND user_.user_id = ANY($3::VARCHAR[])
""",
            organization_id,
            realm_id,
            users,
        )
        roles = {
            row["realm_id"]: _STR_TO_ROLE[row["role"]] if row["role"] is not None else None
            for row in rep
        }
        for user in users or ():
            if user not in roles:
                raise RealmNotFoundError(f"User `{user}` doesn't exist")
        return roles

    else:
        rep = await conn.fetch(
            """
SELECT DISTINCT ON(user_) get_user_id(user_) as realm_id, role
FROM  realm_user_role
WHERE realm = get_realm_internal_id($1, $2)
ORDER BY user_, certified_on DESC
""",
            organization_id,
            realm_id,
        )

        return {
            row["realm_id"]: _STR_TO_ROLE[row["role"]] for row in rep if row["role"] is not None
        }


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def create(
        self, organization_id: OrganizationID, self_granted_role: RealmGrantedRole
    ) -> None:
        assert self_granted_role.granted_by.user_id == self_granted_role.user_id
        assert self_granted_role.role == RealmRole.OWNER

        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
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
                    self_granted_role.realm_id,
                )
                if not realm_internal_id:
                    raise RealmAlreadyExistsError()

                await conn.execute(
                    """
INSERT INTO realm_user_role(
    realm,
    user_,
    role,
    certificate,
    certified_by,
    certified_on
)
SELECT
    $1,
    get_user_internal_id($2, $3),
    'OWNER',
    $4,
    get_device_internal_id($2, $5),
    $6
""",
                    realm_internal_id,
                    organization_id,
                    self_granted_role.user_id,
                    self_granted_role.certificate,
                    self_granted_role.granted_by,
                    self_granted_role.granted_on,
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

                await send_signal(
                    conn,
                    "realm.roles_updated",
                    organization_id=organization_id,
                    author=self_granted_role.granted_by,
                    realm_id=self_granted_role.realm_id,
                    user=self_granted_role.user_id,
                    role_str=self_granted_role.role.value,
                )

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
SELECT DISTINCT ON(user_) get_user_id(user_), role
FROM  realm_user_role
WHERE realm = get_realm_internal_id($1, $2)
ORDER BY user_, certified_on DESC
""",
                    organization_id,
                    realm_id,
                )

                if not ret:
                    # Existing group must have at least one owner user
                    raise RealmNotFoundError(f"Realm `{realm_id}` doesn't exist")
                roles = {
                    UserID(user_id): _STR_TO_ROLE[role] for user_id, role in ret if role is not None
                }
                if author.user_id not in roles:
                    raise RealmAccessError()

        return roles

    async def get_role_certificates(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        since: pendulum.Pendulum,
    ) -> List[bytes]:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                ret = await conn.fetch(
                    """
SELECT get_user_id(user_), role, certificate, certified_on
FROM  realm_user_role
WHERE realm = get_realm_internal_id($1, $2)
ORDER BY certified_on ASC
""",
                    organization_id,
                    realm_id,
                )

                if not ret:
                    # Existing group must have at least one owner user
                    raise RealmNotFoundError(f"Realm `{realm_id}` doesn't exist")

                out = []
                author_current_role = None
                for user_id, role, certif, certified_on in ret:
                    if not since or certified_on > since:
                        out.append(certif)
                    if user_id == author.user_id:
                        author_current_role = role

                if author_current_role is None:
                    raise RealmAccessError()

                return out

    async def update_roles(
        self, organization_id: OrganizationID, new_role: RealmGrantedRole
    ) -> None:
        if new_role.granted_by.user_id == new_role.user_id:
            raise RealmAccessError("Cannot modify our own role")

        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():

                # Retrieve realm and make sure it is not under maintenance
                rep = await get_realm_status(conn, organization_id, new_role.realm_id)
                if rep["maintenance_type"]:
                    raise RealmInMaintenanceError("Data realm is currently under maintenance")

                # Check access rights
                roles = await get_realm_role_for(
                    conn,
                    organization_id,
                    new_role.realm_id,
                    (new_role.granted_by.user_id, new_role.user_id),
                )
                author_role = roles[new_role.granted_by.user_id]
                existing_user_role = roles[new_role.user_id]

                owner_only = (RealmRole.OWNER,)
                owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)
                if existing_user_role in owner_or_manager or new_role.role in owner_or_manager:
                    needed_roles = owner_only
                else:
                    needed_roles = owner_or_manager

                if author_role not in needed_roles:
                    raise RealmAccessError()

                await conn.execute(
                    """
INSERT INTO realm_user_role(
    realm,
    user_,
    role,
    certificate,
    certified_by,
    certified_on
) SELECT
    get_realm_internal_id($1, $2),
    get_user_internal_id($1, $3),
    $4,
    $5,
    get_device_internal_id($1, $6),
    $7
""",
                    organization_id,
                    new_role.realm_id,
                    new_role.user_id,
                    new_role.role.value if new_role.role else None,
                    new_role.certificate,
                    new_role.granted_by,
                    new_role.granted_on,
                )

                await send_signal(
                    conn,
                    "realm.roles_updated",
                    organization_id=organization_id,
                    author=new_role.granted_by,
                    realm_id=new_role.realm_id,
                    user=new_role.user_id,
                    role_str=new_role.role.value if new_role.role else None,
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
                    raise RealmParticipantsMismatchError(
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

                await send_signal(
                    conn,
                    "realm.maintenance_started",
                    organization_id=organization_id,
                    author=author,
                    realm_id=realm_id,
                    encryption_revision=encryption_revision,
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
                    raise RealmNotInMaintenanceError(f"Realm `{realm_id}` not under maintenance")
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

                await send_signal(
                    conn,
                    "realm.maintenance_finished",
                    organization_id=organization_id,
                    author=author,
                    realm_id=realm_id,
                    encryption_revision=encryption_revision,
                )

    async def get_realms_for_user(
        self, organization_id: OrganizationID, user: UserID
    ) -> Dict[UUID, RealmRole]:
        async with self.dbh.pool.acquire() as conn:
            rep = await conn.fetch(
                """
SELECT DISTINCT ON(realm) get_realm_id(realm) as realm_id, role
FROM  realm_user_role
WHERE user_ = get_user_internal_id($1, $2)
ORDER BY realm, certified_on DESC
""",
                organization_id,
                user,
            )
        return {
            row["realm_id"]: _STR_TO_ROLE.get(row["role"]) for row in rep if row["role"] is not None
        }
