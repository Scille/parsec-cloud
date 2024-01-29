# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import DateTime, DeviceID, OrganizationID, RealmRole, UserID, VlobID
from parsec.components.postgresql.handler import PGHandler
from parsec.components.postgresql.realm_queries import (
    query_create,
    query_dump_realms_granted_roles,
    query_finish_reencryption_maintenance,
    query_get_current_roles,
    query_get_realms_for_user,
    query_get_role_certificates,
    query_get_stats,
    query_get_status,
    query_start_reencryption_maintenance,
    query_update_roles,
)
from parsec.components.realm import BaseRealmComponent, RealmGrantedRole, RealmStats, RealmStatus


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def create(
        self, organization_id: OrganizationID, self_granted_role: RealmGrantedRole
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_create(conn, organization_id, self_granted_role)

    async def get_status(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID
    ) -> RealmStatus:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_status(conn, organization_id, author, realm_id)

    async def get_stats(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID
    ) -> RealmStats:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_stats(conn, organization_id, author, realm_id)

    async def get_current_roles(
        self, organization_id: OrganizationID, realm_id: VlobID
    ) -> dict[UserID, RealmRole]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_current_roles(conn, organization_id, realm_id)

    async def get_role_certificates(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID
    ) -> list[bytes]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_role_certificates(conn, organization_id, author, realm_id)

    async def get_realms_for_user(
        self, organization_id: OrganizationID, user: UserID
    ) -> dict[VlobID, RealmRole]:
        async with self.dbh.pool.acquire() as conn:
            return await query_get_realms_for_user(conn, organization_id, user)

    async def update_roles(
        self,
        organization_id: OrganizationID,
        new_role: RealmGrantedRole,
        recipient_message: bytes | None = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_update_roles(conn, organization_id, new_role, recipient_message)

    async def start_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        encryption_revision: int,
        per_participant_message: dict[UserID, bytes],
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
        realm_id: VlobID,
        encryption_revision: int,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_finish_reencryption_maintenance(
                conn, organization_id, author, realm_id, encryption_revision
            )

    async def dump_realms_granted_roles(
        self, organization_id: OrganizationID
    ) -> list[RealmGrantedRole]:
        async with self.dbh.pool.acquire() as conn:
            return await query_dump_realms_granted_roles(conn, organization_id)
