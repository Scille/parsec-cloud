# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
from collections import deque
from contextlib import asynccontextmanager, contextmanager
from dataclasses import dataclass, field
from enum import auto
from typing import AsyncIterator, Callable, Iterator, Sequence, Type, TypeAlias
from unittest.mock import ANY
from uuid import UUID

import anyio
from anyio.streams.memory import MemoryObjectReceiveStream, MemoryObjectSendStream

from parsec._parsec import DeviceID, OrganizationID, UserID, UserProfile, VlobID
from parsec.client_context import AuthenticatedClientContext
from parsec.config import BackendConfig
from parsec.events import (
    ClientBroadcastableEvent,
    Event,
    EventOrganizationConfig,
    EventOrganizationExpired,
    EventRealmCertificate,
    EventUserRevokedOrFrozen,
    EventUserUpdated,
)
from parsec.types import BadOutcomeEnum

PER_CLIENT_MAX_BUFFER_EVENTS = 100


ClientBroadcastableEventStream: TypeAlias = MemoryObjectReceiveStream[
    tuple[ClientBroadcastableEvent, bytes | None] | None
]


class EventWaiter:
    def __init__(self, filter: Callable[[Event], bool]):
        self._filter = filter
        self._event_occurred = asyncio.Event()
        self._event_result: Event | None = None

    def _cb(self, event: Event) -> None:
        if self._event_occurred.is_set():
            return
        if not self._filter(event):
            return
        self._event_result = event
        self._event_occurred.set()

    async def wait(self) -> Event:
        await self._event_occurred.wait()
        assert self._event_result is not None
        return self._event_result

    def clear(self) -> None:
        self._event_occurred = asyncio.Event()
        self._event_result = None


@dataclass(repr=False, eq=False)
class EventBusSpy:
    _connected: bool = False
    _events: list[Event] = field(default_factory=list)
    _waiters: set[Callable[[Event], None]] = field(default_factory=set)

    @property
    def events(self) -> list[Event]:
        if not self._connected:
            raise RuntimeError(
                "Spy is no longer connected to the event bus (using it outside of its context manager ?)"
            )
        return self._events

    def __repr__(self):
        return f"<{type(self).__name__}({self._events})>"

    def _on_event_cb(self, event: Event) -> None:
        self.events.append(event)
        for waiter in self._waiters.copy():
            waiter(event)

    def clear(self):
        self.events.clear()

    async def wait(self, expected_event_type: Type[Event]) -> Event:
        for occurred_event in reversed(self.events):
            if isinstance(occurred_event, expected_event_type):
                return occurred_event

        return await self._wait(expected_event_type)

    async def _wait(self, expected_event_type: Type[Event]) -> Event:
        send_channel, receive_channel = anyio.create_memory_object_stream(1)

        def _waiter(event: Event):
            if isinstance(event, expected_event_type):
                send_channel.send_nowait(event)
                self._waiters.remove(_waiter)

        self._waiters.add(_waiter)
        return await receive_channel.receive()

    async def wait_multiple(
        self, expected_event_types: Sequence[Type[Event]], in_order: bool = True
    ) -> None:
        try:
            self.assert_events_occurred(expected_event_types, in_order=in_order)
            return
        except AssertionError:
            pass

        done = anyio.Event()

        def _waiter(event: Event):
            try:
                self.assert_events_occurred(expected_event_types, in_order=in_order)
                self._waiters.remove(_waiter)
                done.set()
            except AssertionError:
                pass

        self._waiters.add(_waiter)
        await done.wait()

    async def wait_event_occurred(self, event: Event, ignore_event_id: bool = True):
        if ignore_event_id:
            event.event_id = ANY
        while True:
            for occurred in self.events:
                if occurred == event:
                    return
            else:
                await self._wait(type(event))

    def assert_event_occurred(self, event: Event, ignore_event_id: bool = True):
        if ignore_event_id:
            event.event_id = ANY
        for occurred in self.events:
            if occurred == event:
                break
        else:
            raise AssertionError(f"Event {event} didn't occurred")

    def assert_events_occurred(
        self,
        events: Sequence[Event | Type[Event]],
        in_order: bool = True,
        ignore_event_id: bool = True,
    ) -> None:
        if ignore_event_id:
            for event in events:
                if not isinstance(event, type):
                    event.event_id = ANY
        occured_events = self.events
        for event in events:
            for i, (occured_event, _) in enumerate(occured_events):
                if isinstance(event, type):
                    matches = isinstance(occured_event, event)
                else:
                    matches = occured_event == event
                if matches:
                    if in_order:
                        occured_events = occured_events[i + 1 :]
                    break
            else:
                raise AssertionError(f"Event {event} didn't occurred")

    def assert_events_exactly_occurred(
        self, events: list[Event], ignore_event_id: bool = True
    ) -> None:
        if ignore_event_id:
            for event in events:
                if not isinstance(event, type):
                    event.event_id = ANY
        assert events == self.events


class EventBus:
    def __init__(self):
        self._listeners: list[Callable[[Event], None]] = []

    def _dispatch_incoming_event(self, event: Event) -> None:
        for listener in self._listeners:
            listener(event)

    def connect(self, cb: Callable[[Event], None]) -> None:
        self._listeners.append(cb)

    def disconnect(self, cb: Callable[[Event], None]) -> None:
        try:
            self._listeners.remove(cb)
        except ValueError:
            pass

    @contextmanager
    def create_waiter(self, filter: Callable[[Event], bool]) -> Iterator[EventWaiter]:
        waiter = EventWaiter(filter)
        self.connect(waiter._cb)
        try:
            yield waiter
        finally:
            self.disconnect(waiter._cb)

    @contextmanager
    def spy(self) -> Iterator[EventBusSpy]:
        """Only for tests !"""
        spy = EventBusSpy()
        self.connect(spy._on_event_cb)
        spy._connected = True
        try:
            yield spy

        finally:
            self.disconnect(spy._on_event_cb)
            spy._connected = False

    async def send(self, event: Event) -> None:
        raise NotImplementedError

    def send_nowait(self, event: Event) -> None:
        raise NotImplementedError


@dataclass(slots=True)
class RegisteredClient:
    channel_sender: MemoryObjectSendStream[tuple[Event, bytes]]
    organization_id: OrganizationID
    device_id: DeviceID
    user_id: UserID
    realms: set[VlobID]
    profile: UserProfile
    cancel_scope: anyio.CancelScope


class SseAPiEventsListenBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


class BaseEventsComponent:
    def __init__(self, config: BackendConfig, event_bus: EventBus):
        self._event_bus = event_bus
        # Key is `id(client_ctx)`
        self._registered_clients: dict[int, RegisteredClient] = {}
        # Keep in cache the last dispatched events so that we can handle SSE reconnection
        # with the `Last-Event-Id` header
        self._last_events_cache: deque[ClientBroadcastableEvent] = deque(
            maxlen=config.sse_events_cache_size
        )
        self._event_bus.connect(self._on_event)
        # Note we don't have a `__del__` to disconnect from the event bus: the lifetime
        # of this component is basically equivalent of the one of the event bus anyway

    def _on_event(self, event: Event) -> None:
        if isinstance(event, EventOrganizationExpired):
            for registered in self._registered_clients.values():
                if registered.organization_id == event.organization_id:
                    registered.cancel_scope.cancel()
            return

        if isinstance(event, EventUserRevokedOrFrozen):
            for registered in self._registered_clients.values():
                if (
                    registered.organization_id == event.organization_id
                    and registered.user_id == event.user_id
                ):
                    registered.cancel_scope.cancel()
            return

        if not isinstance(event, ClientBroadcastableEvent):
            # The event is only meant for cross-server communicated, skip it
            return

        # It's likely the latest api is the most used, hence we only dump the
        # event once for this case
        apiv4_sse_payload = event.dump_as_apiv4_sse_payload()

        self._last_events_cache.append(event)
        for registered in self._registered_clients.values():
            if not event.is_event_for_client(registered):
                if (
                    isinstance(event, EventRealmCertificate)
                    and event.organization_id == registered.organization_id
                    and event.user_id == registered.user_id
                ):
                    # This is a special case: the current certificate is new a sharing
                    # for our user (hence he doesn't know yet he should be interested
                    # in this realm !).
                    registered.realms.add(event.realm_id)
                else:
                    # The event is not meant for this client, skip it
                    continue

            # The current certificate is a new unsharing for our user. It is then
            # last event our user will receive about this realm, hence we update
            # the list of realm we are interested about accordingly.
            if (
                isinstance(event, EventRealmCertificate)
                and event.role_removed
                and event.user_id == registered.user_id
            ):
                registered.realms.discard(event.realm_id)

            try:
                registered.channel_sender.send_nowait((event, apiv4_sse_payload))
            except anyio.WouldBlock:
                # Client is lagging too much behind, kill it
                registered.cancel_scope.cancel()

    async def _register_client(
        self,
        client_ctx: AuthenticatedClientContext,
        last_event_id: UUID | None,
        cancel_scope: anyio.CancelScope,
    ) -> (
        tuple[EventOrganizationConfig, ClientBroadcastableEventStream]
        | SseAPiEventsListenBadOutcome
    ):
        # To register the client for events, we must first know which realm it is part of
        # (in order to handle realm-related events).
        # However we have to be careful about concurrent events while requesting the
        # database for this list:
        # - Start listening events to collect any change in realm sharing/unsharing.
        # - Then fetch from database the realms shared with our user.
        # - Finally update the list of realms according to the collected changes.

        realms_changed: dict[VlobID, bool] = {}
        user_profile_changed: UserProfile | None = None

        def _collect_realm_changes(event: Event):
            nonlocal user_profile_changed

            if event.organization_id != client_ctx.organization_id:
                return

            if isinstance(event, EventUserUpdated):
                user_profile_changed = event.new_profile

            if isinstance(event, EventRealmCertificate):
                if event.role_removed:
                    realms_changed[event.realm_id] = False
                else:
                    realms_changed[event.realm_id] = True

        self._event_bus.connect(_collect_realm_changes)
        outcome = await self._get_registration_info_for_user(
            organization_id=client_ctx.organization_id, user_id=client_ctx.user_id
        )
        match outcome:
            case (initial_organization_config_event, user_profile, realms):
                pass
            case SseAPiEventsListenBadOutcome() as error:
                return error

        # It is fine to stop listening for event here given there is no asynchronous
        # operation between here and the client registration in `self._registered_clients`
        # (i.e. disconnect and client registration are one atomic operation given the
        # application is monothreaded).
        self._event_bus.disconnect(_collect_realm_changes)
        if user_profile_changed is not None:
            user_profile = user_profile_changed
        for realm_id, is_share in realms_changed.items():
            if is_share:
                realms.add(realm_id)
            else:  # Unshare
                realms.discard(realm_id)

        channel_sender, channel_receiver = anyio.create_memory_object_stream(
            max_buffer_size=PER_CLIENT_MAX_BUFFER_EVENTS
        )
        registered = RegisteredClient(
            channel_sender=channel_sender,
            organization_id=client_ctx.organization_id,
            device_id=client_ctx.device_id,
            user_id=client_ctx.user_id,
            realms=realms,
            profile=user_profile,
            cancel_scope=cancel_scope,
        )
        self._registered_clients[id(client_ctx)] = registered

        # Finally populate the event channel with the event that have been missed
        # since `last_event_id`.
        # Again, this must be an atomic with the client registration, otherwise a
        # concurrent event may be handled by `_on_event` callback and also appear
        # in the cache (and in the end we will send to the client this event twice !)
        if last_event_id is not None:
            missed_events: deque[ClientBroadcastableEvent] = deque()
            # It is likely the client has missed only few events, hence we
            # iter over the events starting by the most recent.
            for event in reversed(self._last_events_cache):
                if event.event_id == last_event_id:
                    # Found the last event !
                    # Now we can populate the channel with all the missed event.
                    for event in missed_events:
                        channel_sender.send_nowait((event, None))
                    break
                if event.is_event_for_client(registered):
                    if len(missed_events) >= PER_CLIENT_MAX_BUFFER_EVENTS:
                        # We missed too many events
                        channel_sender.send_nowait(None)
                        break
                    missed_events.appendleft(event)
            else:
                # Cannot find the last event referred by the ID, just consider it is too old
                channel_sender.send_nowait(None)

        return initial_organization_config_event, channel_receiver

    async def _get_registration_info_for_user(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> tuple[EventOrganizationConfig, UserProfile, set[VlobID]] | SseAPiEventsListenBadOutcome:
        raise NotImplementedError

    @asynccontextmanager
    async def sse_api_events_listen(
        self, client_ctx: AuthenticatedClientContext, last_event_id: UUID | None
    ) -> AsyncIterator[
        tuple[EventOrganizationConfig, ClientBroadcastableEventStream]
        | SseAPiEventsListenBadOutcome
    ]:
        async with anyio.open_cancel_scope() as cancel_scope:
            outcome = await self._register_client(
                client_ctx=client_ctx, last_event_id=last_event_id, cancel_scope=cancel_scope
            )
            try:
                yield outcome
            finally:
                if not isinstance(outcome, SseAPiEventsListenBadOutcome):
                    # It's vital to unregister the client here given the memory location of the
                    # client (and hence the id resulting of it) will most likely be re-used !
                    self._registered_clients.pop(id(client_ctx))
