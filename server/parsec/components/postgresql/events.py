# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import math
from contextlib import asynccontextmanager
from typing import AsyncIterator, override

import anyio
import asyncpg
from anyio.streams.memory import MemoryObjectReceiveStream, MemoryObjectSendStream

from parsec._parsec import (
    ActiveUsersLimit,
    OrganizationID,
    UserID,
    UserProfile,
    VlobID,
)
from parsec.components.events import BaseEventsComponent, EventBus, SseAPiEventsListenBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.handler import parse_signal, send_signal
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.realm import PGRealmComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import Q, transaction
from parsec.config import BackendConfig
from parsec.events import Event, EventOrganizationConfig
from parsec.logging import get_logger

logger = get_logger()


class PGEventBus(EventBus):
    def __init__(self, send_events_channel: MemoryObjectSendStream[Event]):
        super().__init__()
        self._send_events_channel = send_events_channel

    @override
    async def send(self, event: Event) -> None:
        await self._send_events_channel.send(event)

    @override
    def send_nowait(self, event: Event) -> None:
        self._send_events_channel.send_nowait(event)


@asynccontextmanager
async def event_bus_factory(db_url: str) -> AsyncIterator[PGEventBus]:
    # TODO: add typing once use anyio>=4 (currently incompatible with fastapi)
    send_events_channel, receive_events_channel = anyio.create_memory_object_stream(math.inf)
    receive_events_channel: MemoryObjectReceiveStream[Event]

    event_bus = PGEventBus(send_events_channel)
    _connection_lost = False

    def _on_notification_conn_termination(conn: object) -> None:
        nonlocal _connection_lost
        _connection_lost = True
        task_group.cancel_scope.cancel()

    async def _pump_events():
        async for event in receive_events_channel:
            logger.info_with_debug_extra(
                "Received internal event",
                event_type=event.type,
                event_id=event.event_id.hex,
                organization=event.organization_id.str,
                debug_extra={"event_detail": event},
            )

            await send_signal(notification_conn, event)

    def _on_notification(conn: object, pid: int, channel: str, payload: object) -> None:
        assert isinstance(payload, str)
        try:
            event = parse_signal(payload)
        except ValueError as exc:
            logger.warning(
                "Invalid notif received", pid=pid, channel=channel, payload=payload, exc_info=exc
            )
            return
        event_bus._dispatch_incoming_event(event)

    # This connection is dedicated to the notifications listening, so it
    # would only complicate stuff to include it into the connection pool
    notification_conn = await asyncpg.connect(db_url)
    try:
        try:
            async with anyio.create_task_group() as task_group:
                notification_conn.add_termination_listener(_on_notification_conn_termination)
                await notification_conn.add_listener("app_notification", _on_notification)
                task_group.start_soon(_pump_events)
                yield event_bus
                task_group.cancel_scope.cancel()
        finally:
            if _connection_lost:
                raise ConnectionError("PostgreSQL notification query has been lost")
    finally:
        await notification_conn.close()


_q_get_orga_and_user_infos = Q("""
WITH my_organization AS (
    SELECT
        _id,
        is_expired,
        user_profile_outsider_allowed,
        active_users_limit
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

my_user AS (
    SELECT
        _id,
        frozen,
        (revoked_on IS NOT NULL) AS revoked,
        user_id,
        current_profile
    FROM user_
    WHERE
        user_id = $user_id
        AND organization = (SELECT _id FROM my_organization)
    LIMIT 1
),

-- Retrieve the last role for each realm the user is or used to be part of...
my_realms_last_roles AS (
    SELECT DISTINCT ON (realm)
        realm,
        role
    FROM realm_user_role
    WHERE user_ = (SELECT my_user._id FROM my_user)
    ORDER BY realm ASC, certified_on DESC
),

-- ...and only keep the realm the user is still part of
my_realms AS (
    SELECT
        (SELECT realm.realm_id FROM realm WHERE realm._id = my_realms_last_roles.realm)
    FROM my_realms_last_roles
    WHERE role IS NOT NULL
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_is_expired,
    (SELECT user_profile_outsider_allowed FROM my_organization) AS organization_user_profile_outsider_allowed,
    (SELECT active_users_limit FROM my_organization) AS organization_active_users_limit,
    (SELECT _id FROM my_user) AS user_internal_id,
    (SELECT frozen FROM my_user) AS user_is_frozen,
    (SELECT revoked FROM my_user) AS user_is_revoked,
    (SELECT user_id FROM my_user) AS user_id,
    (SELECT current_profile FROM my_user) AS user_current_profile,
    (SELECT ARRAY_AGG(realm_id) FROM my_realms) as user_realms
""")


class PGEventsComponent(BaseEventsComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig, event_bus: EventBus):
        super().__init__(config, event_bus)
        self.pool = pool
        self.organization: PGOrganizationComponent
        self.user: PGUserComponent
        self.realm: PGRealmComponent

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

    @override
    @transaction
    async def _get_registration_info_for_user(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, user_id: UserID
    ) -> tuple[EventOrganizationConfig, UserProfile, set[VlobID]] | SseAPiEventsListenBadOutcome:
        row = await conn.fetchrow(
            *_q_get_orga_and_user_infos(organization_id=organization_id.str, user_id=user_id)
        )
        assert row is not None

        # 1) Check organization

        match row["organization_internal_id"]:
            case int():
                pass
            case None:
                return SseAPiEventsListenBadOutcome.ORGANIZATION_NOT_FOUND
            case unknown:
                assert False, repr(unknown)

        match row["organization_is_expired"]:
            case False:
                pass
            case True:
                return SseAPiEventsListenBadOutcome.ORGANIZATION_EXPIRED
            case unknown:
                assert False, repr(unknown)

        match row["organization_user_profile_outsider_allowed"]:
            case bool() as organization_user_profile_outsider_allowed:
                pass
            case unknown:
                assert False, repr(unknown)

        match row["organization_active_users_limit"]:
            case None as organization_active_users_limit:
                organization_active_users_limit = ActiveUsersLimit.NO_LIMIT
            case int() as organization_active_users_limit:
                organization_active_users_limit = ActiveUsersLimit.limited_to(
                    organization_active_users_limit
                )
            case unknown:
                assert False, repr(unknown)

        # 2) Check user

        match row["user_internal_id"]:
            case int():
                pass
            case None:
                return SseAPiEventsListenBadOutcome.AUTHOR_NOT_FOUND
            case unknown:
                assert False, repr(unknown)

        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case unknown:
                assert False, repr(unknown)

        match row["user_is_frozen"]:
            case False:
                pass
            case True:
                return SseAPiEventsListenBadOutcome.AUTHOR_REVOKED
            case unknown:
                assert False, repr(unknown)

        match row["user_is_revoked"]:
            case False:
                pass
            case True:
                return SseAPiEventsListenBadOutcome.AUTHOR_REVOKED
            case unknown:
                assert False, repr(unknown)

        match row["user_current_profile"]:
            case str() as raw_user_current_profile:
                user_current_profile = UserProfile.from_str(raw_user_current_profile)
            case unknown:
                assert False, repr(unknown)

        # 3) Check realms

        match row["user_realms"]:
            case list() as raw_user_realms:
                user_realms = set(VlobID.from_hex(x) for x in raw_user_realms)
            case None:
                user_realms = set()
            case unknown:
                assert False, repr(unknown)

        org_config = EventOrganizationConfig(
            organization_id=organization_id,
            user_profile_outsider_allowed=organization_user_profile_outsider_allowed,
            active_users_limit=organization_active_users_limit,
        )

        return org_config, user_current_profile, user_realms
