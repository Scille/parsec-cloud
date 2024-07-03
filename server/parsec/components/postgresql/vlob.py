# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

import asyncpg

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    SequesterServiceID,
    VlobID,
)
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.components.events import EventBus
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.realm import PGRealmComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import (
    Q,
    q_device,
    q_device_internal_id,
    q_realm_internal_id,
    transaction,
)
from parsec.components.realm import BadKeyIndex, RealmCheckBadOutcome
from parsec.components.user import CheckDeviceBadOutcome
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
from parsec.events import EVENT_VLOB_MAX_BLOB_SIZE, EventVlob

_q_dump_vlobs = Q(
    f"""
SELECT
    vlob_atom.vlob_id,
    realm.realm_id,
    { q_device(_id="vlob_atom.author", select="device_id") } AS author,
    vlob_atom.created_on,
    vlob_atom.blob
FROM vlob_atom
INNER JOIN realm
ON realm._id = vlob_atom.realm
INNER JOIN organization
ON organization._id = realm.organization
WHERE organization_id = $organization_id
"""
)

_q_insert_vlob = Q(
    f"""
WITH new_vlob AS (
    INSERT INTO vlob_atom (
        realm,
        key_index,
        vlob_id,
        version,
        blob,
        size,
        author,
        created_on
    )
    SELECT
        { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
        $key_index,
        $vlob_id,
        $version,
        $blob,
        $blob_len,
        { q_device_internal_id(organization_id="$organization_id", device_id="$author") },
        $timestamp
    RETURNING _id, realm
)
INSERT INTO realm_vlob_update (
    realm,
    index,
    vlob_atom
) (
    SELECT
        realm,
        (
            SELECT COALESCE(MAX(index), 0) + 1 AS checkpoint
            FROM realm_vlob_update
            WHERE realm_vlob_update.realm = new_vlob.realm
        ),
        _id
    FROM new_vlob
)
RETURNING index
"""
)

_q_poll_changes = Q(
    f"""
SELECT
    index,
    vlob_id,
    vlob_atom.version
FROM realm_vlob_update
LEFT JOIN vlob_atom ON realm_vlob_update.vlob_atom = vlob_atom._id
WHERE
    vlob_atom.realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    AND index > $checkpoint
ORDER BY index ASC
"""
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

_q_get_latest_vlob = Q(
    f"""
SELECT
    vlob_atom.key_index,
    { q_device(select="device_id", _id="author") } AS author,
    vlob_atom.version,
    vlob_atom.created_on,
    vlob_atom.blob
FROM vlob_atom
INNER JOIN realm ON vlob_atom.realm = realm._id
INNER JOIN organization ON realm.organization = organization._id
WHERE organization_id = $organization_id
AND vlob_id = $vlob_id
ORDER BY version DESC
LIMIT 1
"""
)

_q_get_vlob_at_timestamp = Q(
    f"""
SELECT
    vlob_atom.key_index,
    { q_device(select="device_id", _id="author") } AS author,
    vlob_atom.version,
    vlob_atom.created_on,
    vlob_atom.blob
FROM vlob_atom
INNER JOIN realm ON vlob_atom.realm = realm._id
INNER JOIN organization ON realm.organization = organization._id
WHERE organization_id = $organization_id
AND vlob_atom.vlob_id = $vlob_id
AND vlob_atom.created_on <= $timestamp
ORDER BY version DESC
LIMIT 1
"""
)

_q_get_vlob_at_version = Q(
    f"""
SELECT
    vlob_atom.key_index,
    { q_device(select="device_id", _id="author") } AS author,
    vlob_atom.version,
    vlob_atom.created_on,
    vlob_atom.blob
FROM vlob_atom
INNER JOIN realm ON vlob_atom.realm = realm._id
INNER JOIN organization ON realm.organization = organization._id
WHERE organization_id = $organization_id
AND vlob_atom.vlob_id = $vlob_id
AND vlob_atom.version = $version
"""
)


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus):
        self.pool = pool
        self.event_bus = event_bus
        self.organization: PGOrganizationComponent
        self.user: PGUserComponent
        self.realm: PGRealmComponent
        # TODO: Should this remain?
        # self._sequester_organization_authority_cache: dict[
        #     OrganizationID, SequesterAuthority | None
        # ] = {}

    def register_components(
        self,
        organization: PGOrganizationComponent,
        user: PGUserComponent,
        realm: PGRealmComponent,
        **kwargs,
    ) -> None:
        self.organization = organization
        self.user = user
        self.realm = realm

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
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return VlobCreateBadOutcome.ORGANIZATION_NOT_FOUND

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, last_common_certificate_timestamp):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return VlobCreateBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return VlobCreateBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return VlobCreateBadOutcome.AUTHOR_REVOKED

        match await self.realm._check_realm_topic(conn, organization_id, realm_id, author_user_id):
            case (role, realm_key_index, last_realm_certificate_timestamp):
                if role not in (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR):
                    return VlobCreateBadOutcome.AUTHOR_NOT_ALLOWED
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return VlobCreateBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return VlobCreateBadOutcome.AUTHOR_NOT_ALLOWED

        # We only accept the last key
        if realm_key_index != key_index:
            return BadKeyIndex(
                last_realm_certificate_timestamp=last_realm_certificate_timestamp,
            )

        match timestamps_in_the_ballpark(timestamp, now):
            case TimestampOutOfBallpark() as error:
                return error
            case _:
                pass

        if timestamp <= last_common_certificate_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=last_common_certificate_timestamp)

        if timestamp <= last_realm_certificate_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=last_realm_certificate_timestamp)

        if org.is_sequestered:
            # TODO: Implement sequester
            raise NotImplementedError

        else:
            if sequester_blob is not None:
                return VlobCreateBadOutcome.ORGANIZATION_NOT_SEQUESTERED

        # All checks are good, now we do the actual insertion

        try:
            new_checkpoint = await conn.fetchval(
                *_q_insert_vlob(
                    organization_id=organization_id.str,
                    realm_id=realm_id,
                    author=author,
                    key_index=key_index,
                    vlob_id=vlob_id,
                    blob=blob,
                    blob_len=len(blob),
                    timestamp=timestamp,
                    version=1,
                )
            )

        except asyncpg.UniqueViolationError:
            return VlobCreateBadOutcome.VLOB_ALREADY_EXISTS

        assert new_checkpoint >= 1

        event = EventVlob(
            organization_id=organization_id,
            author=author,
            realm_id=realm_id,
            timestamp=timestamp,
            vlob_id=vlob_id,
            version=1,
            blob=blob if len(blob) < EVENT_VLOB_MAX_BLOB_SIZE else None,
            last_common_certificate_timestamp=last_common_certificate_timestamp,
            last_realm_certificate_timestamp=last_realm_certificate_timestamp,
        )
        await self.event_bus.send(event)

    @override
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
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return VlobUpdateBadOutcome.ORGANIZATION_NOT_FOUND

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, last_common_certificate_timestamp):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return VlobUpdateBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return VlobUpdateBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return VlobUpdateBadOutcome.AUTHOR_REVOKED

        match await self._get_vlob_info(conn, organization_id, vlob_id):
            case (realm_id, current_version):
                pass
            case None:
                return VlobUpdateBadOutcome.VLOB_NOT_FOUND

        match await self.realm._check_realm_topic(conn, organization_id, realm_id, author_user_id):
            case (role, realm_key_index, last_realm_certificate_timestamp):
                if role not in (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR):
                    return VlobUpdateBadOutcome.AUTHOR_NOT_ALLOWED
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                assert False, "Should not happen"
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return VlobUpdateBadOutcome.AUTHOR_NOT_ALLOWED

        # We only accept the last key
        if realm_key_index != key_index:
            return BadKeyIndex(last_realm_certificate_timestamp=last_realm_certificate_timestamp)

        maybe_error = timestamps_in_the_ballpark(timestamp, now)
        if maybe_error is not None:
            return maybe_error

        if timestamp <= last_common_certificate_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=last_common_certificate_timestamp)

        if timestamp <= last_realm_certificate_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=last_realm_certificate_timestamp)

        if org.is_sequestered:
            # TODO: Implement sequester
            raise NotImplementedError
        else:
            if sequester_blob is not None:
                return VlobUpdateBadOutcome.ORGANIZATION_NOT_SEQUESTERED

        if version != current_version + 1:
            return VlobUpdateBadOutcome.BAD_VLOB_VERSION

        # All checks are good, now we do the actual insertion

        new_checkpoint = await conn.fetchval(
            *_q_insert_vlob(
                organization_id=organization_id.str,
                realm_id=realm_id,
                author=author,
                key_index=key_index,
                vlob_id=vlob_id,
                blob=blob,
                blob_len=len(blob),
                timestamp=timestamp,
                version=version,
            )
        )

        assert new_checkpoint >= 1

        await self.event_bus.send(
            EventVlob(
                organization_id=organization_id,
                author=author,
                realm_id=realm_id,
                timestamp=timestamp,
                vlob_id=vlob_id,
                version=version,
                blob=blob if len(blob) < EVENT_VLOB_MAX_BLOB_SIZE else None,
                last_common_certificate_timestamp=last_common_certificate_timestamp,
                last_realm_certificate_timestamp=last_realm_certificate_timestamp,
            )
        )

    @override
    @transaction
    async def poll_changes(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        checkpoint: int,
    ) -> tuple[int, list[tuple[VlobID, int]]] | VlobPollChangesAsUserBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return VlobPollChangesAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return VlobPollChangesAsUserBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return VlobPollChangesAsUserBadOutcome.AUTHOR_REVOKED

        match await self.realm._check_realm_topic(conn, organization_id, realm_id, author_user_id):
            case (_, _, _):
                pass
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return VlobPollChangesAsUserBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_ALLOWED

        rows = await conn.fetch(
            *_q_poll_changes(
                organization_id=organization_id.str,
                realm_id=realm_id,
                checkpoint=checkpoint,
            )
        )

        items = {}
        current_checkpoint = checkpoint
        for row in rows:
            current_checkpoint = row["index"]
            vlob_id = VlobID.from_hex(row["vlob_id"])
            version = row["version"]
            items[vlob_id] = version

        return current_checkpoint, list(items.items())

    @override
    @transaction
    async def read_batch(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        vlobs: list[VlobID],
        at: DateTime | None,
    ) -> VlobReadResult | VlobReadAsUserBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return VlobReadAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return VlobReadAsUserBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, last_common_certificate_timestamp):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return VlobReadAsUserBadOutcome.AUTHOR_REVOKED

        match await self.realm._check_realm_topic(conn, organization_id, realm_id, author_user_id):
            case (_, _, last_realm_certificate_timestamp):
                pass
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return VlobReadAsUserBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return VlobReadAsUserBadOutcome.AUTHOR_NOT_ALLOWED

        output = []
        for vlob_id in vlobs:
            if at is None:
                row = await conn.fetchrow(
                    *_q_get_latest_vlob(
                        organization_id=organization_id.str,
                        vlob_id=vlob_id,
                    )
                )
            else:
                row = await conn.fetchrow(
                    *_q_get_vlob_at_timestamp(
                        organization_id=organization_id.str,
                        vlob_id=vlob_id,
                        timestamp=at,
                    )
                )
            if row is None:
                continue
            key_index = row["key_index"]
            vlob_author = DeviceID.from_hex(row["author"])
            version = row["version"]
            created_on = row["created_on"]
            blob = row["blob"]
            output.append(
                (
                    vlob_id,
                    key_index,
                    vlob_author,
                    version,
                    created_on,
                    blob,
                )
            )

        return VlobReadResult(
            items=output,
            needed_common_certificate_timestamp=last_common_certificate_timestamp,
            needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
        )

    @override
    @transaction
    async def read_versions(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        items: list[tuple[VlobID, int]],
    ) -> VlobReadResult | VlobReadAsUserBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return VlobReadAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return VlobReadAsUserBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, last_common_certificate_timestamp):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return VlobReadAsUserBadOutcome.AUTHOR_REVOKED

        match await self.realm._check_realm_topic(conn, organization_id, realm_id, author_user_id):
            case (_, _, last_realm_certificate_timestamp):
                pass
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return VlobReadAsUserBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return VlobReadAsUserBadOutcome.AUTHOR_NOT_ALLOWED

        output = []
        for vlob_id, vlob_version in items:
            if vlob_version < 1:
                continue
            row = await conn.fetchrow(
                *_q_get_vlob_at_version(
                    organization_id=organization_id.str,
                    vlob_id=vlob_id,
                    version=vlob_version,
                )
            )
            if row is None:
                continue
            key_index = row["key_index"]
            vlob_author = DeviceID.from_hex(row["author"])
            version = row["version"]
            created_on = row["created_on"]
            blob = row["blob"]
            output.append(
                (
                    vlob_id,
                    key_index,
                    vlob_author,
                    version,
                    created_on,
                    blob,
                )
            )

        return VlobReadResult(
            items=output,
            needed_common_certificate_timestamp=last_common_certificate_timestamp,
            needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
        )

    @override
    @transaction
    async def test_dump_vlobs(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> dict[VlobID, list[tuple[DeviceID, DateTime, VlobID, bytes]]]:
        rows = await conn.fetch(*_q_dump_vlobs(organization_id=organization_id.str))
        result: dict[VlobID, list[tuple[DeviceID, DateTime, VlobID, bytes]]] = {}
        for row in rows:
            vlob_id = VlobID.from_hex(row["vlob_id"])
            realm_id = VlobID.from_hex(row["realm_id"])
            author = DeviceID.from_hex(row["author"])
            if vlob_id not in result:
                result[vlob_id] = []
            result[vlob_id].append(
                (
                    author,
                    row["created_on"],
                    realm_id,
                    row["blob"],
                )
            )
        return result
