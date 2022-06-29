# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from pendulum import DateTime
from typing import List, Tuple, Dict, Optional

from parsec.api.protocol import OrganizationID, DeviceID, RealmID, VlobID
from parsec.api.protocol.sequester import SequesterServiceID
from parsec.backend.vlob import BaseVlobComponent
from parsec.backend.postgresql.handler import PGHandler, retry_on_unique_violation
from parsec.backend.postgresql.vlob_queries import (
    query_update,
    query_maintenance_save_reencryption_batch,
    query_maintenance_get_reencryption_batch,
    query_read,
    query_poll_changes,
    query_list_versions,
    query_create,
)


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    @retry_on_unique_violation
    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        vlob_id: VlobID,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: Optional[Dict[SequesterServiceID, bytes]] = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            await query_create(
                conn,
                organization_id,
                author,
                realm_id,
                encryption_revision,
                vlob_id,
                timestamp,
                blob,
            )

    async def read(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: VlobID,
        version: Optional[int] = None,
        timestamp: Optional[DateTime] = None,
    ) -> Tuple[int, bytes, DeviceID, DateTime, DateTime]:
        async with self.dbh.pool.acquire() as conn:
            return await query_read(
                conn, organization_id, author, encryption_revision, vlob_id, version, timestamp
            )

    @retry_on_unique_violation
    async def update(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: VlobID,
        version: int,
        timestamp: DateTime,
        blob: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            return await query_update(
                conn,
                organization_id,
                author,
                encryption_revision,
                vlob_id,
                version,
                timestamp,
                blob,
            )

    async def poll_changes(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID, checkpoint: int
    ) -> Tuple[int, Dict[VlobID, int]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_poll_changes(conn, organization_id, author, realm_id, checkpoint)

    async def list_versions(
        self, organization_id: OrganizationID, author: DeviceID, vlob_id: VlobID
    ) -> Dict[int, Tuple[DateTime, DeviceID]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_list_versions(conn, organization_id, author, vlob_id)

    async def maintenance_get_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        size: int,
    ) -> List[Tuple[VlobID, int, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_maintenance_get_reencryption_batch(
                conn, organization_id, author, realm_id, encryption_revision, size
            )

    async def maintenance_save_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        batch: List[Tuple[VlobID, int, bytes]],
    ) -> Tuple[int, int]:
        async with self.dbh.pool.acquire() as conn:
            return await query_maintenance_save_reencryption_batch(
                conn, organization_id, author, realm_id, encryption_revision, batch
            )
