# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger
from weakref import ref, WeakMethod, ReferenceType
from collections import defaultdict


logger = get_logger()


class EventWaiter:
    def __init__(self):
        self._event_occured = trio.Event()
        self._event_result = None

    def _cb(self, event, **kwargs):
        if self._event_occured.is_set():
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
        self._event_handlers = defaultdict(set)

    def send(self, event, **kwargs):
        logger.debug("send event", event_name=event, kwargs=kwargs)
        # Given event handlers are stored as weakrefs, any one of them
        # can become unavailable at any time.
        # In such case we perform a cleanup operation.
        # Note we don't want to use weakref's callback feature to do that
        # given it would mean event_handlers list could change size randomly
        # during iteration...
        need_clean = False

        for cb in self._event_handlers[event]:
            if isinstance(cb, ReferenceType):
                cb = cb()
                if not cb:
                    need_clean = True
                else:
                    cb(event, **kwargs)
            else:
                cb(event, **kwargs)

        if need_clean:
            self._event_handlers[event] = {cb for cb in self._event_handlers[event] if cb()}

    def waiter_on(self, event):
        ew = EventWaiter()
        self.connect(event, ew._cb, weak=True)
        return ew

    def waiter_on_first(self, *events):
        ew = EventWaiter()
        for event in events:
            self.connect(event, ew._cb, weak=True)
        return ew

    def connect(self, event, cb, weak=False):
        # logger.debug("connect event", event_name=event)
        if weak:
            try:
                weak = WeakMethod(cb)
            except TypeError:
                weak = ref(cb)

            self._event_handlers[event].add(weak)
        else:
            self._event_handlers[event].add(cb)
        self.send("event.connected", event_name=event)

    def disconnect(self, event, cb):
        # logger.debug("disconnect event", event_name=event, cb=cb)
        self._event_handlers[event].remove(cb)
        self.send("event.disconnected", event_name=event)
