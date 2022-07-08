# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from pendulum import DateTime
from typing import List, Tuple, Dict, Optional

from parsec.api.protocol import OrganizationID, DeviceID, RealmID, VlobID
from parsec.api.protocol.sequester import SequesterServiceID
from parsec.backend.postgresql.utils import Q, q_organization_internal_id
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobSequesterDisabledError,
    VlobSequesterServiceInconsistencyError,
)
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
from parsec.backend.postgresql.sequester import q_get_organisation_sequester_authority

_q_get_sequester_service = Q(
    f"""
    SELECT service_id, service_certificate
    FROM sequester_service
    WHERE organization={ q_organization_internal_id("$organization_id") }
    AND deleted_on IS NULL
    ORDER BY _id
"""
)


async def _check_sequestered_organization(
    conn,
    organization_id: OrganizationID,
    sequester_authority: Optional[bytes],
    sequester_blob: Optional[Dict[SequesterServiceID, bytes]],
):
    if sequester_blob is None and sequester_authority is None:
        # Sequester is disable, fetching sequester services is pointless
        return

    if sequester_authority is None:
        raise VlobSequesterDisabledError()

    row = await conn.fetch(*_q_get_sequester_service(organization_id=organization_id.str))
    configured_services = {SequesterServiceID(data["service_id"]) for data in row}
    requested_sequester_services = sequester_blob.keys() if sequester_blob is not None else set()

    if configured_services != requested_sequester_services:
        raise VlobSequesterServiceInconsistencyError(
            sequester_authority_certificate=sequester_authority,
            sequester_services_certificates=[data["service_certificate"] for data in row],
        )


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh
        self._sequester_organization_authority_cache: Dict[OrganizationID, Optional[bytes]] = {}

    async def _fetch_organization_sequester_authority(self, conn, organization_id: OrganizationID):
        row = await conn.fetchrow(
            *q_get_organisation_sequester_authority(organization_id=organization_id.str)
        )

        self._sequester_organization_authority_cache[organization_id] = row[0] if row else None

    async def _get_sequester_organization_authority(
        self, conn, organization_id: OrganizationID
    ) -> Optional[bytes]:
        if organization_id not in self._sequester_organization_authority_cache:
            await self._fetch_organization_sequester_authority(conn, organization_id)
        return self._sequester_organization_authority_cache[organization_id]

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
            sequester_authority = await self._get_sequester_organization_authority(
                conn, organization_id
            )
            await _check_sequestered_organization(
                conn,
                organization_id=organization_id,
                sequester_authority=sequester_authority,
                sequester_blob=sequester_blob,
            )
            await query_create(
                conn,
                organization_id,
                author,
                realm_id,
                encryption_revision,
                vlob_id,
                timestamp,
                blob,
                sequester_blob,
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
        sequester_blob: Optional[Dict[SequesterServiceID, bytes]] = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            sequester_authority = await self._get_sequester_organization_authority(
                conn, organization_id
            )
            await _check_sequestered_organization(
                conn,
                organization_id=organization_id,
                sequester_authority=sequester_authority,
                sequester_blob=sequester_blob,
            )
            return await query_update(
                conn,
                organization_id,
                author,
                encryption_revision,
                vlob_id,
                version,
                timestamp,
                blob,
                sequester_blob,
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
