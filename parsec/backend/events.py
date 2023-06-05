# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from collections import deque
from typing import Awaitable, Callable, Type

import trio

from parsec._parsec import (
    BackendEvent,
    BackendEventCertificatesUpdated,
    BackendEventInviteStatusChanged,
    BackendEventMessageReceived,
    BackendEventPinged,
    BackendEventPkiEnrollmentUpdated,
    BackendEventRealmMaintenanceFinished,
    BackendEventRealmMaintenanceStarted,
    BackendEventRealmRolesUpdated,
    BackendEventRealmVlobsUpdated,
    authenticated_cmds,
)
from parsec.api.protocol.types import UserProfile
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.realm import BaseRealmComponent
from parsec.backend.utils import api, api_ws_cancel_on_client_sending_new_cmd

# TODO: make this configurable ?
BACKEND_EVENTS_LOCAL_CACHE_SIZE = 1024


def internal_to_api_v2_v3_events(
    event: BackendEvent,
) -> authenticated_cmds.v3.events_listen.APIEvent | None:
    event_listen_cmd_mod = authenticated_cmds.v3.events_listen
    if isinstance(event, BackendEventCertificatesUpdated):
        # Ignore this event
        return None
    elif isinstance(event, BackendEventPinged):
        return event_listen_cmd_mod.APIEventPinged(event.ping)
    elif isinstance(event, BackendEventMessageReceived):
        return event_listen_cmd_mod.APIEventMessageReceived(event.index)
    elif isinstance(event, BackendEventInviteStatusChanged):
        return event_listen_cmd_mod.APIEventInviteStatusChanged(
            token=event.token, invitation_status=event.status
        )
    elif isinstance(event, BackendEventRealmMaintenanceStarted):
        return event_listen_cmd_mod.APIEventRealmMaintenanceStarted(
            realm_id=event.realm_id,
            encryption_revision=event.encryption_revision,
        )
    elif isinstance(event, BackendEventRealmMaintenanceFinished):
        return event_listen_cmd_mod.APIEventRealmMaintenanceFinished(
            realm_id=event.realm_id,
            encryption_revision=event.encryption_revision,
        )
    elif isinstance(event, BackendEventRealmVlobsUpdated):
        return event_listen_cmd_mod.APIEventRealmVlobsUpdated(
            realm_id=event.realm_id,
            checkpoint=event.checkpoint,
            src_id=event.src_id,
            src_version=event.src_version,
        )
    elif isinstance(event, BackendEventRealmRolesUpdated):
        return event_listen_cmd_mod.APIEventRealmRolesUpdated(
            realm_id=event.realm_id,
            role=event.role,
        )
    else:
        assert isinstance(event, BackendEventPkiEnrollmentUpdated)
        return event_listen_cmd_mod.APIEventPkiEnrollmentUpdated()


def internal_to_api_events(
    event: BackendEvent,
) -> authenticated_cmds.latest.events_listen.APIEvent | None:
    event_listen_cmd_mod = authenticated_cmds.latest.events_listen

    if isinstance(event, BackendEventCertificatesUpdated):
        return event_listen_cmd_mod.APIEventCertificatesUpdated(event.index)
    elif isinstance(event, BackendEventPinged):
        return event_listen_cmd_mod.APIEventPinged(event.ping)
    elif isinstance(event, BackendEventMessageReceived):
        return event_listen_cmd_mod.APIEventMessageReceived(event.index, event.message)
    elif isinstance(event, BackendEventInviteStatusChanged):
        return event_listen_cmd_mod.APIEventInviteStatusChanged(
            token=event.token, invitation_status=event.status
        )
    elif isinstance(event, BackendEventRealmMaintenanceStarted):
        return event_listen_cmd_mod.APIEventRealmMaintenanceStarted(
            realm_id=event.realm_id,
            encryption_revision=event.encryption_revision,
        )
    elif isinstance(event, BackendEventRealmMaintenanceFinished):
        return event_listen_cmd_mod.APIEventRealmMaintenanceFinished(
            realm_id=event.realm_id,
            encryption_revision=event.encryption_revision,
        )
    elif isinstance(event, BackendEventRealmVlobsUpdated):
        return event_listen_cmd_mod.APIEventRealmVlobsUpdated(
            realm_id=event.realm_id,
            checkpoint=event.checkpoint,
            src_id=event.src_id,
            src_version=event.src_version,
        )
    elif isinstance(event, BackendEventRealmRolesUpdated):
        # Ignore this event
        return None
    else:
        assert isinstance(event, BackendEventPkiEnrollmentUpdated)
        return event_listen_cmd_mod.APIEventPkiEnrollmentUpdated()


def _is_event_for_our_client(
    client_ctx: AuthenticatedClientContext,
    event: BackendEvent,
) -> bool:
    if event.organization_id != client_ctx.organization_id:
        return False

    if isinstance(event, (BackendEventCertificatesUpdated, BackendEventCertificatesUpdated)):
        # No more filter
        return True

    elif isinstance(event, BackendEventRealmRolesUpdated):
        # Note for this event we don't filter out the ones sent by the client's
        # device, there is two reason for this:
        # 1) A user cannot change it own role, so this case should never occur
        # 2) Returning this event inform the peer we are ready to send it
        #    `realm.vlobs_updated` events on this realm (especially useful during tests)
        return event.user == client_ctx.user_id

    elif isinstance(event, BackendEventPinged):
        # Filter out the event we originated from
        return event.author != client_ctx.device_id

    elif isinstance(
        event,
        (
            BackendEventRealmVlobsUpdated,
            BackendEventRealmMaintenanceFinished,
            BackendEventRealmMaintenanceStarted,
        ),
    ):
        # Filter out the event we originated from and the one from realms we are not part of
        return event.author != client_ctx.device_id and event.realm_id in client_ctx.realms

    elif isinstance(event, BackendEventMessageReceived):
        return event.recipient == client_ctx.user_id

    elif isinstance(event, BackendEventInviteStatusChanged):
        return event.greeter == client_ctx.user_id

    else:
        assert isinstance(event, BackendEventPkiEnrollmentUpdated)
        return client_ctx.profile == UserProfile.ADMIN


class EventsComponent:
    def __init__(
        self, realm_component: BaseRealmComponent, send_event: Callable[..., Awaitable[None]]
    ):
        self._realm_component = realm_component
        # Keep in cache the last dispatched events so that we can handle SSE reconnection
        # with the `Last-Event-Id` header
        self._events_cache: deque[tuple[str, BackendEvent]] = deque(
            maxlen=BACKEND_EVENTS_LOCAL_CACHE_SIZE
        )
        self.send = send_event

    def add_event_to_cache(self, event_id: str, event: BackendEvent) -> None:
        self._events_cache.append((event_id, event))

    def _get_client_missed_events_since(
        self, client_ctx: AuthenticatedClientContext, last_event_id: str
    ) -> deque[tuple[str, BackendEvent] | None]:
        # We use a VlobID purely for convenience given we need a serializable UUID type
        events = iter(self._events_cache)
        for candidate_event_id, _ in events:
            if candidate_event_id == last_event_id:
                break

        else:
            return deque((None,))

        return deque(event for event in events if _is_event_for_our_client(client_ctx, event[1]))

    @api
    async def apiv2v3_events_subscribe(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.v3.events_subscribe.Req,
    ) -> authenticated_cmds.v3.events_subscribe.Rep:
        # Only Websocket transport need to subscribe events
        if client_ctx.event_bus_ctx:
            await self.connect_events(client_ctx)
        return authenticated_cmds.v3.events_subscribe.RepOk()

    async def connect_events(
        self, client_ctx: AuthenticatedClientContext, last_event_id: str | None = None
    ) -> deque[tuple[str, BackendEvent] | None]:
        def _on_event(
            event: Type[BackendEvent],
            event_id: str,
            payload: BackendEvent,
        ) -> None:
            if _is_event_for_our_client(client_ctx, payload):
                # Keep up to date the list of realms the user should be notified of
                if isinstance(payload, BackendEventRealmRolesUpdated):
                    if payload.role is None:
                        client_ctx.realms.discard(payload.realm_id)
                    else:
                        client_ctx.realms.add(payload.realm_id)

                try:
                    client_ctx.send_events_channel.send_nowait((event_id, payload))
                except trio.WouldBlock:
                    client_ctx.close_connection_asap()

        # Command should be idempotent
        if client_ctx.events_subscribed:
            return deque()

        # Connect the new callbacks
        client_ctx.event_bus_ctx.connect(
            BackendEventCertificatesUpdated,
            _on_event,  # type: ignore
        )
        client_ctx.event_bus_ctx.connect(
            BackendEventPinged,
            _on_event,  # type: ignore
        )
        client_ctx.event_bus_ctx.connect(
            BackendEventRealmVlobsUpdated,
            _on_event,  # type: ignore
        )
        client_ctx.event_bus_ctx.connect(
            BackendEventRealmMaintenanceStarted,
            _on_event,  # type: ignore
        )
        client_ctx.event_bus_ctx.connect(
            BackendEventRealmMaintenanceFinished,
            _on_event,  # type: ignore
        )
        client_ctx.event_bus_ctx.connect(
            BackendEventMessageReceived,
            _on_event,  # type: ignore
        )
        client_ctx.event_bus_ctx.connect(
            BackendEventInviteStatusChanged,
            _on_event,  # type: ignore
        )

        client_ctx.event_bus_ctx.connect(
            BackendEventPkiEnrollmentUpdated,
            _on_event,  # type: ignore
        )

        # Final event to keep up to date the list of realm we should listen on
        client_ctx.event_bus_ctx.connect(
            BackendEventRealmRolesUpdated,
            _on_event,  # type: ignore
        )

        # We must do that here to be right after even bus connection, but before any
        # async operation, otherwise a concurrent event may be handled by the registered
        # callbacks and also appear in the cache (and in the end we will send to the
        # client this event twice !)
        if last_event_id is not None:
            new_events = self._get_client_missed_events_since(client_ctx, last_event_id)
        else:
            # Returning `None` means we couldn't retreive the last event, however here
            # we were not asked to retreive it... which is equivalent to retreiving
            # the very last event
            new_events = deque()

        # Finally populate the list of realm we should listen on
        realms_for_user = await self._realm_component.get_realms_for_user(
            client_ctx.organization_id, client_ctx.user_id
        )
        client_ctx.realms = set(realms_for_user.keys())
        client_ctx.events_subscribed = True

        return new_events

    @api_ws_cancel_on_client_sending_new_cmd
    @api
    async def api_v2_v3_events_listen(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.v3.events_listen.Req,
    ) -> authenticated_cmds.v3.events_listen.Rep:
        while True:
            if req.wait:
                _, event = await client_ctx.receive_events_channel.receive()

            else:
                try:
                    _, event = client_ctx.receive_events_channel.receive_nowait()
                except trio.WouldBlock:
                    return authenticated_cmds.v3.events_listen.RepNoEvents()

            unit = internal_to_api_v2_v3_events(event)
            if not unit:
                # Ignore the current event
                continue

            return authenticated_cmds.v3.events_listen.RepOk(unit)

    @api_ws_cancel_on_client_sending_new_cmd
    @api
    async def api_events_listen(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.events_listen.Req,
    ) -> authenticated_cmds.latest.events_listen.Rep:
        return authenticated_cmds.latest.events_listen.RepNotAvailable()

    async def sse_api_events_listen(
        self, client_ctx: AuthenticatedClientContext, last_event_id: str | None
    ) -> Callable[[], Awaitable[tuple[str, authenticated_cmds.latest.events_listen.Rep] | None]]:
        missed_events = await self.connect_events(client_ctx, last_event_id)
        if missed_events is None:
            missed_events = deque((None,))

        async def _next_event_cb() -> tuple[
            str, authenticated_cmds.latest.events_listen.Rep
        ] | None:
            while True:
                # First return the events the client has missed since it last deconnection

                if missed_events:
                    missed_event = missed_events.popleft()
                    if missed_event is None:
                        # Return `None`` to signify we couldn't retreive the last event
                        return None
                    else:
                        missed_event_id, missed_event_payload = missed_event
                        unit = internal_to_api_events(missed_event_payload)
                        if not unit:
                            continue

                        return (
                            missed_event_id,
                            authenticated_cmds.latest.events_listen.RepOk(unit),
                        )

                # Then switch back to the current events

                (event_id, event) = await client_ctx.receive_events_channel.receive()

                unit = internal_to_api_events(event)
                if not unit:
                    # Ignore the current event
                    continue

                return (event_id, authenticated_cmds.latest.events_listen.RepOk(unit))

        return _next_event_cb
