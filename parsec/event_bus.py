import trio
from weakref import ref, WeakMethod, ReferenceType
from collections import defaultdict


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
        for cb in self._event_handlers[event]:
            if isinstance(cb, ReferenceType):
                cb = cb()
                assert cb is not None
            cb(event, **kwargs)

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
        if weak:

            def _disconnect(ref):
                self.disconnect(event, ref)

            try:
                weak = WeakMethod(cb, _disconnect)
            except TypeError:
                weak = ref(cb, _disconnect)

            self._event_handlers[event].add(weak)
        else:
            self._event_handlers[event].add(cb)

    def disconnect(self, event, cb):
        self._event_handlers[event].remove(cb)
