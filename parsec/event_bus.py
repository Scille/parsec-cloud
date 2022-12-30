# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import ContextManager, DefaultDict, Dict, Iterator, List, Tuple, TypeVar, Union

try:
    # Introduced in Python 3.8
    from typing import Protocol
except ImportError:
    from typing_extensions import Protocol  # type: ignore
from collections import defaultdict
from contextlib import contextmanager
from enum import Enum

import trio
from structlog import get_logger

from parsec._parsec import CoreEvent

logger = get_logger()


class MetaEvent(Enum):
    EVENT_CONNECTED = "event.connected"
    EVENT_DISCONNECTED = "event.disconnected"


CustomEvent = TypeVar("CustomEvent", bound=Enum)
AllEvent = Union[MetaEvent, CustomEvent]


class EventCallback(Protocol):
    def __call__(self, event: Enum | CoreEvent, **kwargs: object) -> None:
        ...


class EventFilterCallback(Protocol):
    def __call__(self, event: Enum | CoreEvent, **kwargs: object) -> bool:
        ...


class EventWaiter:
    def __init__(self, filter: EventFilterCallback | None):
        self._filter = filter
        self._event_occurred = trio.Event()
        self._event_result: Tuple[Enum | CoreEvent, Dict[str, object]] | None = None

    def _cb(self, event: Enum | CoreEvent, **kwargs: object) -> None:
        if self._event_occurred.is_set():
            return
        if self._filter and not self._filter(event, **kwargs):
            return
        self._event_result = (event, kwargs)
        self._event_occurred.set()

    async def wait(self) -> Tuple[Enum | CoreEvent, Dict[str, object]]:
        await self._event_occurred.wait()
        assert self._event_result is not None
        return self._event_result

    def clear(self) -> None:
        self._event_occurred = trio.Event()
        self._event_result = None


class EventBus:
    def __init__(self) -> None:
        self._event_handlers: DefaultDict[Enum | CoreEvent, List[EventCallback]] = defaultdict(list)

    def stats(self) -> Dict[Enum | CoreEvent, int]:
        return {event: len(cbs) for event, cbs in self._event_handlers.items() if cbs}

    def connection_context(self) -> "EventBusConnectionContext":
        return EventBusConnectionContext(self)

    def send(self, event: CoreEvent | Enum, **kwargs: object) -> None:
        # Do not log meta events (event.connected and event.disconnected)
        if "event_type" not in kwargs:
            logger.debug("Send event", event_type=event, **kwargs)
        for cb in self._event_handlers[event]:
            try:
                cb(event, **kwargs)
            except Exception:
                logger.exception(
                    "Unhandled exception in event bus callback",
                    callback=cb,
                    event_type=event,
                    **kwargs,
                )

    @contextmanager
    def waiter_on(
        self, event: Enum | CoreEvent, *, filter: EventFilterCallback | None = None
    ) -> Iterator[EventWaiter]:
        ew = EventWaiter(filter)
        self.connect(event, ew._cb)
        try:
            yield ew

        finally:
            self.disconnect(event, ew._cb)

    @contextmanager
    def waiter_on_first(
        self, *events: Enum, filter: EventFilterCallback | None = None
    ) -> Iterator[EventWaiter]:
        ew = EventWaiter(filter)
        for event in events:
            self.connect(event, ew._cb)
        try:
            yield ew

        finally:
            for event in events:
                self.disconnect(event, ew._cb)

    def connect(self, event: Enum | CoreEvent, cb: EventCallback) -> None:
        self._event_handlers[event].append(cb)
        self.send(MetaEvent.EVENT_CONNECTED, event_type=event)

    @contextmanager
    def connect_in_context(self, *events: Tuple[Enum | CoreEvent, EventCallback]) -> Iterator[None]:
        for event, cb in events:
            self.connect(event, cb)
        try:
            yield

        finally:
            for event, cb in events:
                self.disconnect(event, cb)

    def disconnect(self, event: Enum | CoreEvent, cb: EventCallback) -> None:
        self._event_handlers[event].remove(cb)
        self.send(MetaEvent.EVENT_DISCONNECTED, event_type=event)


class EventBusConnectionContext:
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus
        self.to_disconnect: List[Tuple[Enum, EventCallback]] = []

    def __enter__(self) -> "EventBusConnectionContext":
        return self

    def __exit__(self, exc_type, exc_value, traceback):  # type: ignore
        self.clear()

    def clear(self) -> None:
        for event, cb in self.to_disconnect:
            self.event_bus.disconnect(event, cb)
        self.to_disconnect.clear()

    def send(self, event: Enum, **kwargs: object) -> None:
        self.event_bus.send(event, **kwargs)

    def waiter_on(self, event: Enum) -> ContextManager[EventWaiter]:
        return self.event_bus.waiter_on(event)

    def waiter_on_first(self, *events: Enum) -> ContextManager[EventWaiter]:
        return self.event_bus.waiter_on_first(*events)

    def connect(self, event: Enum, cb: EventCallback) -> None:
        self.to_disconnect.append((event, cb))
        self.event_bus.connect(event, cb)

    def connect_in_context(self, *events: Tuple[Enum, EventCallback]) -> ContextManager[None]:
        return self.event_bus.connect_in_context(*events)

    def disconnect(self, event: Enum, cb: EventCallback) -> None:
        self.event_bus.disconnect(event, cb)
        self.to_disconnect.remove((event, cb))
