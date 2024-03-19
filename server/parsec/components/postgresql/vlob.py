# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import assert_never, override

import asyncpg

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    SequesterServiceID,
    UserProfile,
    VlobID,
)
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.components.events import EventBus
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.realm import PGRealmComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import transaction
from parsec.components.postgresql.vlob_queries.write import _q_create
from parsec.components.realm import BadKeyIndex, KeyIndex, RealmCheckBadOutcome
from parsec.components.user import CheckUserWithDeviceBadOutcome
from parsec.components.vlob import (
    BaseVlobComponent,
    RejectedBySequesterService,
    SequesterInconsistency,
    SequesterServiceNotAvailable,
    VlobCreateBadOutcome,
)
from parsec.events import EventVlob

# async def _check_sequestered_organization(
#     conn: asyncpg.Connection,
#     organization_id: OrganizationID,
#     sequester_authority: SequesterAuthority | None,
#     sequester_blob: dict[SequesterServiceID, bytes] | None,
# ) -> dict[SequesterServiceID, BaseSequesterService] | None:
#     if sequester_blob is None and sequester_authority is None:
#         # Sequester is disable, fetching sequester services is pointless
#         return None

#     if sequester_authority is None:
#         raise VlobSequesterDisabledError()

#     configured_services = {
#         s.service_id: s
#         for s in await get_sequester_services(
#             conn=conn, organization_id=organization_id, with_disabled=False
#         )
#     }
#     requested_sequester_services = sequester_blob.keys() if sequester_blob is not None else set()

#     if configured_services.keys() != requested_sequester_services:
#         raise VlobSequesterServiceInconsistencyError(
#             sequester_authority_certificate=sequester_authority.certificate,
#             sequester_services_certificates=[
#                 s.service_certificate for s in configured_services.values()
#             ],
#         )

#     return configured_services


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, pool: asyncpg.Pool, event_bus: EventBus):
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

    # async def _fetch_organization_sequester_authority(
    #     self, conn: asyncpg.Connection, organization_id: OrganizationID
    # ) -> None:
    #     sequester_authority: SequesterAuthority | None
    #     try:
    #         sequester_authority = await get_sequester_authority(conn, organization_id)
    #     except SequesterDisabledError:
    #         sequester_authority = None
    #     self._sequester_organization_authority_cache[organization_id] = sequester_authority

    # async def _get_sequester_organization_authority(
    #     self, conn: asyncpg.Connection, organization_id: OrganizationID
    # ) -> SequesterAuthority | None:
    #     if organization_id not in self._sequester_organization_authority_cache:
    #         await self._fetch_organization_sequester_authority(conn, organization_id)
    #     return self._sequester_organization_authority_cache[organization_id]

    # async def _extract_sequestered_data_and_proceed_webhook(
    #     self,
    #     conn: asyncpg.Connection,
    #     organization_id: OrganizationID,
    #     sequester_blob: dict[SequesterServiceID, bytes] | None,
    #     author: DeviceID,
    #     encryption_revision: int,
    #     vlob_id: VlobID,
    #     timestamp: DateTime,
    # ) -> dict[SequesterServiceID, bytes] | None:
    #     sequester_authority = await self._get_sequester_organization_authority(
    #         conn, organization_id
    #     )
    #     services = await _check_sequestered_organization(
    #         conn,
    #         organization_id=organization_id,
    #         sequester_authority=sequester_authority,
    #         sequester_blob=sequester_blob,
    #     )
    #     if not sequester_blob or not services:
    #         return None

    #     sequestered_data = await extract_sequestered_data_and_proceed_webhook(
    #         services,
    #         organization_id,
    #         author,
    #         encryption_revision,
    #         vlob_id,
    #         timestamp,
    #         sequester_blob,
    #     )

    #     return sequestered_data

    @override
    @transaction
    async def create(
        self,
        conn: asyncpg.Connection,
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
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return VlobCreateBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as org:
                pass
            case unknown:
                assert_never(unknown)

        match await self.user._check_user(conn, organization_id, author):
            case CheckUserWithDeviceBadOutcome.DEVICE_NOT_FOUND:
                return VlobCreateBadOutcome.AUTHOR_NOT_FOUND
            case CheckUserWithDeviceBadOutcome.USER_NOT_FOUND:
                return VlobCreateBadOutcome.AUTHOR_NOT_FOUND
            case CheckUserWithDeviceBadOutcome.USER_REVOKED:
                return VlobCreateBadOutcome.AUTHOR_REVOKED
            case UserProfile():
                pass
            case unknown:
                assert_never(unknown)

        match await self.realm._check_realm(conn, organization_id, realm_id, author):
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return VlobCreateBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return VlobCreateBadOutcome.AUTHOR_NOT_ALLOWED
            case (RealmRole() as role, KeyIndex() as realm_key_index):
                if role not in (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR):
                    return VlobCreateBadOutcome.AUTHOR_NOT_ALLOWED
            case unknown:
                assert_never(unknown)

        # We only accept the last key
        if realm_key_index != key_index:
            return BadKeyIndex(
                last_realm_certificate_timestamp=timestamp  # TODO: this is not the right timestamp
            )

        match timestamps_in_the_ballpark(timestamp, now):
            case TimestampOutOfBallpark() as error:
                return error

        # TODO: Fix causality
        # if timestamp < org.last_certificate_timestamp:
        #     return RequireGreaterTimestamp(strictly_greater_than=org.last_certificate_timestamp)

        if org.is_sequestered:
            # TODO: Implement sequester
            raise NotImplementedError
            # assert org.sequester_services is not None
            # if sequester_blob is None or sequester_blob.keys() != org.sequester_services.keys():
            #     return SequesterInconsistency(
            #         last_common_certificate_timestamp=org.last_common_certificate_timestamp
            #     )

            # blob_for_storage_sequester_services = {}
            # for service_id, service in org.sequester_services.items():
            #     match service.service_type:
            #         case SequesterServiceType.STORAGE:
            #             blob_for_storage_sequester_services[service_id] = sequester_blob[service_id]
            #         case SequesterServiceType.WEBHOOK:
            #             assert service.webhook_url is not None
            #             match await self._sequester_service_send_webhook(
            #                 webhook_url=service.webhook_url,
            #                 organization_id=organization_id,
            #                 service_id=service_id,
            #                 sequester_blob=sequester_blob[service_id],
            #             ):
            #                 case None:
            #                     pass
            #                 case error:
            #                     return error
            #         case unknown:
            #             assert_never(unknown)

        else:
            if sequester_blob is not None:
                return VlobCreateBadOutcome.ORGANIZATION_NOT_SEQUESTERED

        # All checks are good, now we do the actual insertion

        try:
            vlob_atom_internal_id = await conn.fetchval(
                *_q_create(
                    organization_id=organization_id.str,
                    realm_id=realm_id,
                    author=author.str,
                    key_index=key_index,
                    vlob_id=vlob_id,
                    blob=blob,
                    blob_len=len(blob),
                    timestamp=timestamp,
                )
            )

        except asyncpg.UniqueViolationError:
            return VlobCreateBadOutcome.VLOB_ALREADY_EXISTS

        assert vlob_atom_internal_id is not None

        await self.event_bus.send(
            EventVlob(
                organization_id=organization_id,
                author=author,
                realm_id=realm_id,
                timestamp=timestamp,
                vlob_id=vlob_id,
                version=1,
                blob=None,  # TODO: use `blob if len(blob) < EVENT_VLOB_MAX_BLOB_SIZE else None`
                last_common_certificate_timestamp=timestamp,  # TODO: this is not the right timestamp
                last_realm_certificate_timestamp=timestamp,  # TODO: this is not the right timestamp
            )
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

    # @retry_on_unique_violation
    # async def update(
    #     self,
    #     organization_id: OrganizationID,
    #     author: DeviceID,
    #     encryption_revision: int,
    #     vlob_id: VlobID,
    #     version: int,
    #     timestamp: DateTime,
    #     blob: bytes,
    #     sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    # ) -> None:
    #     async with self.dbh.pool.acquire() as conn:
    #         sequester_blob = await self._extract_sequestered_data_and_proceed_webhook(
    #             conn,
    #             organization_id=organization_id,
    #             sequester_blob=sequester_blob,
    #             author=author,
    #             encryption_revision=encryption_revision,
    #             vlob_id=vlob_id,
    #             timestamp=timestamp,
    #         )

    #         return await query_update(
    #             conn,
    #             organization_id,
    #             author,
    #             encryption_revision,
    #             vlob_id,
    #             version,
    #             timestamp,
    #             blob,
    #             sequester_blob,
    #         )
