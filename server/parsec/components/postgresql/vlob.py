# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncpg

from parsec._parsec import DateTime, DeviceID, OrganizationID, SequesterServiceID, VlobID
from parsec.components.organization import SequesterAuthority
from parsec.components.postgresql.handler import PGHandler, retry_on_unique_violation
from parsec.components.postgresql.sequester import get_sequester_authority, get_sequester_services
from parsec.components.postgresql.vlob_queries import (
    query_create,
    query_list_versions,
    query_maintenance_get_reencryption_batch,
    query_maintenance_save_reencryption_batch,
    query_poll_changes,
    query_update,
)
from parsec.components.sequester import BaseSequesterService, SequesterDisabledError
from parsec.components.vlob import (
    BaseVlobComponent,
    VlobSequesterDisabledError,
    VlobSequesterServiceInconsistencyError,
    extract_sequestered_data_and_proceed_webhook,
)


async def _check_sequestered_organization(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    sequester_authority: SequesterAuthority | None,
    sequester_blob: dict[SequesterServiceID, bytes] | None,
) -> dict[SequesterServiceID, BaseSequesterService] | None:
    if sequester_blob is None and sequester_authority is None:
        # Sequester is disable, fetching sequester services is pointless
        return None

    if sequester_authority is None:
        raise VlobSequesterDisabledError()

    configured_services = {
        s.service_id: s
        for s in await get_sequester_services(
            conn=conn, organization_id=organization_id, with_disabled=False
        )
    }
    requested_sequester_services = sequester_blob.keys() if sequester_blob is not None else set()

    if configured_services.keys() != requested_sequester_services:
        raise VlobSequesterServiceInconsistencyError(
            sequester_authority_certificate=sequester_authority.certificate,
            sequester_services_certificates=[
                s.service_certificate for s in configured_services.values()
            ],
        )

    return configured_services


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh
        self._sequester_organization_authority_cache: dict[
            OrganizationID, SequesterAuthority | None
        ] = {}

    async def _fetch_organization_sequester_authority(
        self, conn: asyncpg.Connection, organization_id: OrganizationID
    ) -> None:
        sequester_authority: SequesterAuthority | None
        try:
            sequester_authority = await get_sequester_authority(conn, organization_id)
        except SequesterDisabledError:
            sequester_authority = None
        self._sequester_organization_authority_cache[organization_id] = sequester_authority

    async def _get_sequester_organization_authority(
        self, conn: asyncpg.Connection, organization_id: OrganizationID
    ) -> SequesterAuthority | None:
        if organization_id not in self._sequester_organization_authority_cache:
            await self._fetch_organization_sequester_authority(conn, organization_id)
        return self._sequester_organization_authority_cache[organization_id]

    async def _extract_sequestered_data_and_proceed_webhook(
        self,
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
        sequester_blob: dict[SequesterServiceID, bytes] | None,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: VlobID,
        timestamp: DateTime,
    ) -> dict[SequesterServiceID, bytes] | None:
        sequester_authority = await self._get_sequester_organization_authority(
            conn, organization_id
        )
        services = await _check_sequestered_organization(
            conn,
            organization_id=organization_id,
            sequester_authority=sequester_authority,
            sequester_blob=sequester_blob,
        )
        if not sequester_blob or not services:
            return None

        sequestered_data = await extract_sequestered_data_and_proceed_webhook(
            services,
            organization_id,
            author,
            encryption_revision,
            vlob_id,
            timestamp,
            sequester_blob,
        )

        return sequestered_data

    @retry_on_unique_violation
    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        encryption_revision: int,
        vlob_id: VlobID,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            sequester_blob = await self._extract_sequestered_data_and_proceed_webhook(
                conn,
                organization_id=organization_id,
                sequester_blob=sequester_blob,
                author=author,
                encryption_revision=encryption_revision,
                vlob_id=vlob_id,
                timestamp=timestamp,
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
        version: int | None = None,
        timestamp: DateTime | None = None,
    ) -> tuple[int, bytes, DeviceID, DateTime, DateTime, int]:
        # TODO: fix me !
        raise NotImplementedError
        # async with self.dbh.pool.acquire() as conn:
        # return await query_read(
        #     conn, organization_id, author, encryption_revision, vlob_id, version, timestamp
        # )

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
        sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            sequester_blob = await self._extract_sequestered_data_and_proceed_webhook(
                conn,
                organization_id=organization_id,
                sequester_blob=sequester_blob,
                author=author,
                encryption_revision=encryption_revision,
                vlob_id=vlob_id,
                timestamp=timestamp,
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
        self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID, checkpoint: int
    ) -> tuple[int, dict[VlobID, int]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_poll_changes(conn, organization_id, author, realm_id, checkpoint)

    async def list_versions(
        self, organization_id: OrganizationID, author: DeviceID, vlob_id: VlobID
    ) -> dict[int, tuple[DateTime, DeviceID]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_list_versions(conn, organization_id, author, vlob_id)

    async def maintenance_get_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        encryption_revision: int,
        size: int,
    ) -> list[tuple[VlobID, int, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_maintenance_get_reencryption_batch(
                conn, organization_id, author, realm_id, encryption_revision, size
            )

    async def maintenance_save_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        encryption_revision: int,
        batch: list[tuple[VlobID, int, bytes]],
    ) -> tuple[int, int]:
        async with self.dbh.pool.acquire() as conn:
            return await query_maintenance_save_reencryption_batch(
                conn, organization_id, author, realm_id, encryption_revision, batch
            )
