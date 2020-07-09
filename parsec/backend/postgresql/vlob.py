# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from uuid import UUID
from typing import List, Tuple, Dict, Optional


from parsec.backend.backend_events import BackendEvent
from parsec.api.protocol import DeviceID, OrganizationID
from parsec.backend.realm import RealmRole
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobNotFoundError,
    VlobEncryptionRevisionError,
    VlobInMaintenanceError,
    VlobNotInMaintenanceError,
)
from parsec.backend.postgresql.handler import PGHandler, retry_on_unique_violation
from parsec.backend.postgresql.realm_queries.maintenance import get_realm_status, RealmNotFoundError
from parsec.backend.postgresql.vlob_queries.write import query_update, query_vlob_updated
from parsec.backend.postgresql.vlob_queries.maintenance import (
    query_maintenance_save_reencryption_batch,
    query_maintenance_get_reencryption_batch,
)
from parsec.backend.postgresql.vlob_queries.read import (
    query_read,
    query_poll_changes,
    query_list_versions,
)
from parsec.backend.postgresql.vlob_queries.utils import query_check_realm_access
from parsec.backend.postgresql.vlob_queries.create import query_create


async def _check_realm(
    conn, organization_id, realm_id, encryption_revision, expected_maintenance=False
):
    try:
        rep = await get_realm_status(conn, organization_id, realm_id)

    except RealmNotFoundError as exc:
        raise VlobNotFoundError(*exc.args) from exc

    if expected_maintenance is False:
        if rep["maintenance_type"]:
            raise VlobInMaintenanceError("Data realm is currently under maintenance")
    elif expected_maintenance is True:
        if not rep["maintenance_type"]:
            raise VlobNotInMaintenanceError(f"Realm `{realm_id}` not under maintenance")

    if encryption_revision is not None and rep["encryption_revision"] != encryption_revision:
        raise VlobEncryptionRevisionError()


async def _check_realm_access(conn, organization_id, realm_id, author, allowed_roles):
    await query_check_realm_access(conn, organization_id, realm_id, author, allowed_roles)


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
    await query_vlob_updated(
        conn,
        BackendEvent.REALM_VLOBS_UPDATED,
        organization_id=organization_id,
        author=author,
        realm_id=realm_id,
        src_id=src_id,
        src_version=src_version,
    )


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    @retry_on_unique_violation
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
        vlob_id: UUID,
        version: Optional[int] = None,
        timestamp: Optional[pendulum.Pendulum] = None,
    ) -> Tuple[int, bytes, DeviceID, pendulum.Pendulum]:
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
        vlob_id: UUID,
        version: int,
        timestamp: pendulum.Pendulum,
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
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_poll_changes(conn, organization_id, author, realm_id, checkpoint)

    async def list_versions(
        self, organization_id: OrganizationID, author: DeviceID, vlob_id: UUID
    ) -> Dict[int, Tuple[pendulum.Pendulum, DeviceID]]:
        return await query_list_versions(self.dbh.pool, organization_id, author, vlob_id)

    async def maintenance_get_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        size: int,
    ) -> List[Tuple[UUID, int, bytes]]:
        async with self.dbh.pool.acquire() as conn:
            return await query_maintenance_get_reencryption_batch(
                conn, organization_id, author, realm_id, encryption_revision, size
            )

    async def maintenance_save_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        batch: List[Tuple[UUID, int, bytes]],
    ) -> Tuple[int, int]:
        async with self.dbh.pool.acquire() as conn:
            return await query_maintenance_save_reencryption_batch(
                conn, organization_id, author, realm_id, encryption_revision, batch
            )
