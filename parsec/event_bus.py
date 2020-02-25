# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import List
import trio
from structlog import get_logger
from collections import defaultdict
from contextlib import contextmanager


logger = get_logger()


class EventWaiter:
    def __init__(self, filter):
        self._filter = filter
        self._event_occured = trio.Event()
        self._event_result = None

    def _cb(self, event: str, **kwargs):
        if self._event_occured.is_set():
            return
        if self._filter and not self._filter(event, **kwargs):
            return
        self._event_result = (event, kwargs)
        self._event_occured.set()

    async def wait(self):
        await self._event_occured.wait()
        return self._event_result

    def clear(self):
        self._event_occured = trio.Event()
        self._event_result = None


class EventBus:
    def __init__(self):
        self._event_handlers = defaultdict(list)

    def stats(self):
        return {event: len(cbs) for event, cbs in self._event_handlers.items() if cbs}

    def connection_context(self):
        return EventBusConnectionContext(self)

    def send(self, event: str, **kwargs):
        # Do not log meta events (event.connected and event.disconnected)
        if "event_name" not in kwargs:
            logger.debug("Send event", event_name=event, **kwargs)
        for cb in self._event_handlers[event]:
            try:
                cb(event, **kwargs)
            except Exception:
                logger.exception(
                    "Unhandled exception in event bus callback",
                    callback=cb,
                    event_name=event,
                    **kwargs
                )

    @contextmanager
    def waiter_on(self, event: str, *, filter=None):
        ew = EventWaiter(filter)
        self.connect(event, ew._cb)
        try:
            yield ew

        finally:
            self.disconnect(event, ew._cb)

    @contextmanager
    def waiter_on_first(self, *events: List[str], filter=None):
        ew = EventWaiter(filter)
        for event in events:
            self.connect(event, ew._cb)
        try:
            yield ew

        finally:
            for event in events:
                self.disconnect(event, ew._cb)

    def connect(self, event: str, cb):
        self._event_handlers[event].append(cb)
        self.send("event.connected", event_name=event)

    @contextmanager
    def connect_in_context(self, *events: List[str]):
        for event, cb in events:
            self.connect(event, cb)
        try:
            yield

        finally:
            for event, cb in events:
                self.disconnect(event, cb)

    def disconnect(self, event: str, cb):
        self._event_handlers[event].remove(cb)
        self.send("event.disconnected", event_name=event)


class EventBusConnectionContext:
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus
        self.to_disconnect = []

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.clear()

    def clear(self):
        for event, cb in self.to_disconnect:
            self.event_bus.disconnect(event, cb)
        self.to_disconnect.clear()

    def send(self, event: str, **kwargs):
        self.event_bus.send(event, **kwargs)

    def waiter_on(self, event: str):
        return self.event_bus.waiter_on(event)

    def waiter_on_first(self, *events: List[str]):
        return self.event_bus.waiter_on_first(*events)

    def connect(self, event: str, cb):
        self.to_disconnect.append((event, cb))
        self.event_bus.connect(event, cb)

    def connect_in_context(self, *events: List[str]):
        return self.event_bus.connect_in_context(*events)

    def disconnect(self, event: str, cb):
        self.event_bus.disconnect(event, cb)
        self.to_disconnect.remove((event, cb))
