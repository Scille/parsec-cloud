# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import asyncio
import math
from contextlib import asynccontextmanager
from typing import AsyncIterator, override

import anyio
from anyio.streams.memory import MemoryObjectReceiveStream, MemoryObjectSendStream

from parsec._parsec import OrganizationID, UserID, UserProfile, VlobID
from parsec.components.events import BaseEventsComponent, EventBus, SseAPiEventsListenBadOutcome
from parsec.components.memory.datamodel import MemoryDatamodel
from parsec.events import Event, EventOrganizationConfig
from parsec.logging import get_logger

logger = get_logger()


class MemoryEventBus(EventBus):
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
async def event_bus_factory() -> AsyncIterator[MemoryEventBus]:
    # TODO: add typing once use anyio>=4 (currently incompatible with fastapi)
    send_events_channel, receive_events_channel = anyio.create_memory_object_stream(math.inf)
    receive_events_channel: MemoryObjectReceiveStream[Event]

    event_bus = MemoryEventBus(send_events_channel)

    async def _pump_events():
        async for event in receive_events_channel:
            await asyncio.sleep(0)  # Sleep to force some asynchronousness

            logger.info_with_debug_extra(
                "Received internal event",
                event_type=event.type,
                event_id=event.event_id.hex,
                organization=event.organization_id.str,
                debug_extra={"event_detail": event},
            )

            event_bus._dispatch_incoming_event(event)

    async with anyio.create_task_group() as tg:
        tg.start_soon(_pump_events)

        yield event_bus

        tg.cancel_scope.cancel()


class MemoryEventsComponent(BaseEventsComponent):
    def __init__(self, data: MemoryDatamodel, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._data = data

    @override
    async def _get_registration_info_for_user(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> tuple[EventOrganizationConfig, UserProfile, set[VlobID]] | SseAPiEventsListenBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return SseAPiEventsListenBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return SseAPiEventsListenBadOutcome.ORGANIZATION_EXPIRED

        try:
            user = org.users[user_id]
        except KeyError:
            return SseAPiEventsListenBadOutcome.AUTHOR_NOT_FOUND
        if user.is_revoked:
            return SseAPiEventsListenBadOutcome.AUTHOR_REVOKED

        org_config = EventOrganizationConfig(
            organization_id=org.organization_id,
            user_profile_outsider_allowed=org.user_profile_outsider_allowed,
            active_users_limit=org.active_users_limit,
        )

        user_realms = set()
        for realm in org.realms.values():
            if realm.get_current_role_for(user_id) is not None:
                user_realms.add(realm.realm_id)

        return org_config, user.current_profile, user_realms
