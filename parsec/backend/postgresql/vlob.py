# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from pendulum import DateTime
from typing import List, Tuple, Dict, Optional

from parsec.api.protocol import OrganizationID, DeviceID, RealmID, VlobID
from parsec.api.protocol.sequester import SequesterServiceID
from parsec.backend.organization import SequesterAuthority
from parsec.backend.sequester import SequesterDisabledError
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobSequesterDisabledError,
    VlobSequesterServiceInconsistencyError,
)
from parsec.backend.postgresql.utils import Q, q_organization_internal_id
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
from parsec.backend.postgresql.sequester import get_sequester_authority


_q_get_sequester_enabled_services = Q(
    f"""
    SELECT service_id, service_certificate
    FROM sequester_service
    WHERE organization={ q_organization_internal_id("$organization_id") }
    AND disabled_on IS NULL
    ORDER BY _id
"""
)


async def _check_sequestered_organization(
    conn,
    organization_id: OrganizationID,
    sequester_authority: Optional[SequesterAuthority],
    sequester_blob: Optional[Dict[SequesterServiceID, bytes]],
):
    if sequester_blob is None and sequester_authority is None:
        # Sequester is disable, fetching sequester services is pointless
        return

    if sequester_authority is None:
        raise VlobSequesterDisabledError()

    rows = await conn.fetch(*_q_get_sequester_enabled_services(organization_id=organization_id.str))
    configured_services = {SequesterServiceID(row["service_id"]) for row in rows}
    requested_sequester_services = sequester_blob.keys() if sequester_blob is not None else set()

    if configured_services != requested_sequester_services:
        raise VlobSequesterServiceInconsistencyError(
            sequester_authority_certificate=sequester_authority.certificate,
            sequester_services_certificates=[row["service_certificate"] for row in rows],
        )


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh
        self._sequester_organization_authority_cache: Dict[
            OrganizationID, Optional[SequesterAuthority]
        ] = {}

    async def _fetch_organization_sequester_authority(self, conn, organization_id: OrganizationID):
        sequester_authority: Optional[SequesterAuthority]
        try:
            sequester_authority = await get_sequester_authority(conn, organization_id)
        except SequesterDisabledError:
            sequester_authority = None
        self._sequester_organization_authority_cache[organization_id] = sequester_authority

    async def _get_sequester_organization_authority(
        self, conn, organization_id: OrganizationID
    ) -> Optional[SequesterAuthority]:
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
