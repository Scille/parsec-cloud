# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    SequesterServiceID,
    VlobID,
)
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
)
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.utils import (
    Q,
    no_transaction,
    retryable,
    transaction,
)
from parsec.components.postgresql.vlob_create import vlob_create
from parsec.components.postgresql.vlob_poll_changes import vlob_poll_changes
from parsec.components.postgresql.vlob_read_batch import vlob_read_batch
from parsec.components.postgresql.vlob_read_versions import vlob_read_versions
from parsec.components.postgresql.vlob_test_dump_vlobs import vlob_test_dump_vlobs
from parsec.components.postgresql.vlob_update import vlob_update
from parsec.components.realm import BadKeyIndex
from parsec.components.vlob import (
    BaseVlobComponent,
    RejectedBySequesterService,
    SequesterInconsistency,
    SequesterServiceNotAvailable,
    VlobCreateBadOutcome,
    VlobPollChangesAsUserBadOutcome,
    VlobReadAsUserBadOutcome,
    VlobReadResult,
    VlobUpdateBadOutcome,
)

_q_get_vlob_info = Q(
    """
SELECT realm_id, version
FROM vlob_atom
INNER JOIN realm ON vlob_atom.realm = realm._id
INNER JOIN organization ON realm.organization = organization._id
WHERE organization_id = $organization_id
AND vlob_id = $vlob_id
ORDER BY version DESC
LIMIT 1
"""
)


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus):
        self.pool = pool
        self.event_bus = event_bus

    async def _get_vlob_info(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, vlob_id: VlobID
    ) -> tuple[VlobID, int] | None:
        row = await conn.fetchrow(
            *_q_get_vlob_info(
                organization_id=organization_id.str,
                vlob_id=vlob_id,
            )
        )
        if row is None:
            return None
        return (VlobID.from_hex(row["realm_id"]), row["version"])

    @override
    @retryable
    @transaction
    async def create(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        vlob_id: VlobID,
        key_index: int,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    ) -> (
        None
        | BadKeyIndex
        | VlobCreateBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
        | RejectedBySequesterService
        | SequesterServiceNotAvailable
        | SequesterInconsistency
    ):
        return await vlob_create(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            realm_id,
            vlob_id,
            key_index,
            timestamp,
            blob,
            sequester_blob,
        )

    @override
    @retryable
    @transaction
    async def update(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        vlob_id: VlobID,
        key_index: int,
        version: int,
        timestamp: DateTime,
        blob: bytes,
        # Sequester is a special case, so gives it a default version to simplify tests
        sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    ) -> (
        None
        | BadKeyIndex
        | VlobUpdateBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
        | RejectedBySequesterService
        | SequesterServiceNotAvailable
        | SequesterInconsistency
    ):
        return await vlob_update(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            vlob_id,
            key_index,
            version,
            timestamp,
            blob,
            sequester_blob,
        )

    @override
    @no_transaction
    async def poll_changes(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        checkpoint: int,
    ) -> tuple[int, list[tuple[VlobID, int]]] | VlobPollChangesAsUserBadOutcome:
        return await vlob_poll_changes(conn, organization_id, author, realm_id, checkpoint)

    @override
    @no_transaction
    async def read_batch(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        vlobs: list[VlobID],
        at: DateTime | None,
    ) -> VlobReadResult | VlobReadAsUserBadOutcome:
        return await vlob_read_batch(conn, organization_id, author, realm_id, vlobs, at)

    @override
    @no_transaction
    async def read_versions(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        items: list[tuple[VlobID, int]],
    ) -> VlobReadResult | VlobReadAsUserBadOutcome:
        return await vlob_read_versions(conn, organization_id, author, realm_id, items)

    @override
    @no_transaction
    async def test_dump_vlobs(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> dict[VlobID, list[tuple[DeviceID, DateTime, VlobID, bytes]]]:
        return await vlob_test_dump_vlobs(conn, organization_id)
