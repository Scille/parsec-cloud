# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from libparsec.types import DateTime
from typing import Dict, List, Optional

from parsec.api.protocol import OrganizationID, DeviceID, UserID, RealmID, RealmRole
from parsec.backend.realm import BaseRealmComponent, RealmStatus, RealmGrantedRole, RealmStats
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.realm_queries import (
    query_create,
    query_get_status,
    query_get_stats,
    query_get_current_roles,
    query_get_role_certificates,
    query_get_realms_for_user,
    query_update_roles,
    query_start_reencryption_maintenance,
    query_finish_reencryption_maintenance,
    query_dump_realms_granted_roles,
)


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def create(
        self, organization_id: OrganizationID, self_granted_role: RealmGrantedRole
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_create(conn, organization_id, self_granted_role)

    async def get_status(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID
    ) -> RealmStatus:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_status(conn, organization_id, author, realm_id)

    async def get_stats(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID
    ) -> RealmStats:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_stats(conn, organization_id, author, realm_id)

    async def get_current_roles(
        self, organization_id: OrganizationID, realm_id: RealmID
    ) -> Dict[UserID, RealmRole]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_current_roles(conn, organization_id, realm_id)

    async def get_role_certificates(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID
    ) -> List[bytes]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_role_certificates(conn, organization_id, author, realm_id)

    async def get_realms_for_user(
        self, organization_id: OrganizationID, user: UserID
    ) -> Dict[RealmID, RealmRole]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_realms_for_user(conn, organization_id, user)

    async def update_roles(
        self,
        organization_id: OrganizationID,
        new_role: RealmGrantedRole,
        recipient_message: Optional[bytes] = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_update_roles(conn, organization_id, new_role, recipient_message)

    async def start_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        per_participant_message: Dict[UserID, bytes],
        timestamp: DateTime,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_start_reencryption_maintenance(
                conn,
                organization_id,
                author,
                realm_id,
                encryption_revision,
                per_participant_message,
                timestamp,
            )

    async def finish_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_finish_reencryption_maintenance(
                conn, organization_id, author, realm_id, encryption_revision
            )

    async def dump_realms_granted_roles(
        self, organization_id: OrganizationID
    ) -> List[RealmGrantedRole]:
        async with self.dbh.pool.acquire() as conn:
            return await query_dump_realms_granted_roles(conn, organization_id)
