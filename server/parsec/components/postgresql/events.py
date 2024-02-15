# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import math
from contextlib import asynccontextmanager
from typing import AsyncIterator, override

import anyio
import asyncpg
from anyio.streams.memory import MemoryObjectReceiveStream, MemoryObjectSendStream
from structlog import get_logger

from parsec._parsec import OrganizationID, UserID, UserProfile, VlobID
from parsec.components.events import BaseEventsComponent, EventBus, SseAPiEventsListenBadOutcome
from parsec.components.postgresql.handler import parse_signal, send_signal
from parsec.config import BackendConfig
from parsec.events import Event, EventOrganizationConfig

logger = get_logger()

# async def _send_event(
#     event: Event,
#     conn: asyncpg.Connection | None = None,
# ) -> None:
#     if conn is None:
#         async with dbh.pool.acquire() as conn:
#             await send_signal(conn, event)
#     else:
#         await send_signal(conn, event)


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
    def __init__(self, pool: asyncpg.Pool, config: BackendConfig, event_bus: EventBus):
        super().__init__(config, event_bus)
        self._pool = pool

    @override
    async def _get_registration_info_for_user(
        self, organization_id: OrganizationID, author: UserID
    ) -> tuple[EventOrganizationConfig, UserProfile, set[VlobID]] | SseAPiEventsListenBadOutcome:
        raise NotImplementedError
        # try:
        #     org = self._data.organizations[organization_id]
        # except KeyError:
        #     return SseAPiEventsListenBadOutcome.ORGANIZATION_NOT_FOUND
        # if org.is_expired:
        #     return SseAPiEventsListenBadOutcome.ORGANIZATION_EXPIRED

        # try:
        #     user = org.users[author]
        # except KeyError:
        #     return SseAPiEventsListenBadOutcome.AUTHOR_NOT_FOUND
        # if user.is_revoked:
        #     return SseAPiEventsListenBadOutcome.AUTHOR_REVOKED

        # org_config = EventOrganizationConfig(
        #     organization_id=org.organization_id,
        #     user_profile_outsider_allowed=org.user_profile_outsider_allowed,
        #     active_users_limit=org.active_users_limit,
        # )

        # user_realms = set()
        # for realm in org.realms.values():
        #     if realm.get_current_role_for(author) is not None:
        #         user_realms.add(realm.realm_id)

        # return org_config, user.current_profile, user_realms
