# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import math
from contextlib import asynccontextmanager
from typing import AsyncIterator, override

import anyio
import asyncpg
from anyio.streams.memory import MemoryObjectReceiveStream, MemoryObjectSendStream
from structlog import get_logger

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    UserID,
    UserProfile,
    VlobID,
)
from parsec.components.events import BaseEventsComponent, EventBus, SseAPiEventsListenBadOutcome
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.handler import parse_signal, send_signal
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import Q, q_realm, q_user_internal_id, transaction
from parsec.components.user import CheckDeviceBadOutcome
from parsec.config import BackendConfig
from parsec.events import Event, EventOrganizationConfig

logger = get_logger()

_q_get_realms_for_user = Q(
    f"""
SELECT DISTINCT ON(realm) { q_realm(_id="realm_user_role.realm", select="realm_id") } as realm_id, role
FROM  realm_user_role
WHERE user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
ORDER BY realm, certified_on DESC
"""
)


async def query_get_realms_for_user(
    conn: AsyncpgConnection, organization_id: OrganizationID, user: UserID
) -> dict[VlobID, RealmRole]:
    rep = await conn.fetch(
        *_q_get_realms_for_user(organization_id=organization_id.str, user_id=user.str)
    )
    return {
        VlobID.from_hex(row["realm_id"]): RealmRole.from_str(row["role"])
        for row in rep
        if row["role"] is not None
    }


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
async def event_bus_factory(url: str) -> AsyncIterator[PGEventBus]:
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
            logger.info("Broadcasting", event_=event)
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
    notification_conn = await asyncpg.connect(url)
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


class PGEventsComponent(BaseEventsComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig, event_bus: EventBus):
        super().__init__(config, event_bus)
        self.pool = pool
        self.organization: PGOrganizationComponent
        self.user: PGUserComponent

    def register_components(
        self, organization: PGOrganizationComponent, user: PGUserComponent, **kwargs
    ) -> None:
        self.organization = organization
        self.user = user

    @override
    @transaction
    async def _get_registration_info_for_author(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, author: DeviceID
    ) -> tuple[EventOrganizationConfig, UserProfile, set[VlobID]] | SseAPiEventsListenBadOutcome:
        match await self.organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return SseAPiEventsListenBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as org:
                pass

        if org.is_expired:
            return SseAPiEventsListenBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return SseAPiEventsListenBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return SseAPiEventsListenBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return SseAPiEventsListenBadOutcome.AUTHOR_REVOKED
            case (UserProfile() as profile, DateTime()):
                pass

        org_config = EventOrganizationConfig(
            organization_id=org.organization_id,
            user_profile_outsider_allowed=org.user_profile_outsider_allowed,
            active_users_limit=org.active_users_limit,
        )

        mapping = await query_get_realms_for_user(conn, organization_id, author.user_id)
        return org_config, profile, set(mapping.keys())
