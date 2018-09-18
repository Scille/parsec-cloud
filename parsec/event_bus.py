import attr
import trio
import pendulum
from weakref import ref, ReferenceType
from contextlib import contextmanager
from collections import defaultdict


class ANY:
    def __repr__(self):
        return "<ANY>"


ANY = ANY()


def convert_dicts_with_any(a, b):
    if a is ANY or b is ANY:
        return ANY, ANY
    any_fields = [k for k, v in a.items() if v is ANY] + [k for k, v in b.items() if v is ANY]
    cooked_a = {k: v if k not in any_fields else ANY for k, v in a.items()}
    cooked_b = {k: v if k not in any_fields else ANY for k, v in b.items()}
    return cooked_a, cooked_b


def convert_events_with_any(a, b):
    if a.event is ANY or b.event is ANY:
        a_event = b_event = ANY
    else:
        a_event = a.event
        b_event = b.event

    a_kwargs, b_kwargs = convert_dicts_with_any(a.kwargs, b.kwargs)

    if a.dt is ANY or b.dt is ANY:
        a_dt = b_dt = ANY
    else:
        a_dt = a.dt
        b_dt = b.dt

    return SpiedEvent(a_event, a_kwargs, a_dt), SpiedEvent(b_event, b_kwargs, b_dt)


def convert_event_lists_with_any(a, b):
    converted = [convert_events_with_any(*x) for x in zip(a, b)]
    return (
        [ia for ia, _ in converted] + a[len(converted) :],
        [ib for _, ib in converted] + b[len(converted) :],
    )


@attr.s(frozen=True, slots=True)
class SpiedEvent:
    event = attr.ib()
    kwargs = attr.ib()
    dt = attr.ib(factory=pendulum.now)


@attr.s(repr=False)
class EventSpy:
    ANY = ANY  # Easier to use than doing an import
    events = attr.ib(factory=list)
    _waiters = attr.ib(factory=set)

    def __repr__(self):
        return f"<{type(self).__name__}({[e.event for e in self.events]})>"

    def _on_event_cb(self, event, **kwargs):
        cooked_event = SpiedEvent(event, kwargs)
        self.events.append(cooked_event)
        for waiter in self._waiters:
            waiter(cooked_event)

    def clear(self):
        self.events.clear()

    async def wait_for_backend_online(self):
        for occured_event in reversed(self.events):
            if occured_event.event == "backend.online":
                return occured_event
            elif occured_event.event == "backend.offline":
                break

        # Backend never been online, or has been disconnected in the meantime
        return await self._wait("backend.online")

    async def wait(self, event):
        for occured_event in self.events:
            if occured_event.event == event:
                return occured_event

        return await self._wait(event)

    async def _wait(self, event):
        catcher = trio.Queue(1)

        def waiter(cooked_event):
            if cooked_event.event == event:
                catcher.put_nowait(cooked_event)
                self._waiters.remove(waiter)

        self._waiters.add(waiter)
        return await catcher.get()

    def _cook_events_params(self, events):
        cooked_events = []
        for event in events:
            if isinstance(event, SpiedEvent):
                cooked_events.append(event)
            elif event is ANY:
                cooked_events.append(event)
            elif isinstance(event, str):
                cooked_events.append(SpiedEvent(event, ANY, ANY))
            elif isinstance(event, tuple):
                event = event + (ANY,) * (3 - len(event))
                cooked_events.append(SpiedEvent(*event))
            else:
                raise ValueError(
                    "`events` must be a list of `SpiedEvent`, `(<event>, <kwargs>, <dt>)` tuple or string"
                )
        return cooked_events

    def assert_occured(self, events, exact=True):
        if exact:
            self._assert_exactly_occured(events)
        else:
            self._assert_roughly_occured(events)

    def _assert_exactly_occured(self, events):
        events = self._cook_events_params(events)
        cooked_expected, cooked_observed = convert_event_lists_with_any(events, self.events)
        assert cooked_observed == cooked_expected

    def _assert_roughly_occured(self, events):
        events = self._cook_events_params(events)
        occured_events = iter(events)
        try:
            for i, expected in enumerate(events):
                while True:
                    candidate = next(occured_events)
                    cooked_candidate, cooked_expected = convert_events_with_any(candidate, expected)
                    if cooked_candidate == cooked_expected:
                        break
        except StopIteration:
            raise ValueError(f"Missing events {events[i:]}")


class EventWaiter:
    def __init__(self, event):
        self.event = event
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
        ew = EventWaiter(event)
        self.connect(event, ew._cb, weak=True)
        return ew

    def waiter_on_first(self, *events):
        ew = EventWaiter(event)
        for event in events:
            self.connect(event, ew._cb, weak=True)
        return ew

    def connect(self, event, cb, weak=False):
        if weak:

            def _disconnect(ref):
                self.disconnect(event, ref)

            self._event_handlers[event].add(ref(cb, _disconnect))
        else:
            self._event_handlers[event].add(cb)

    def disconnect(self, event, cb):
        self._event_handlers[event].remove(cb)

    # TODO: Legacy API
    def signal(self, name):
        @attr.s
        class Signal:
            event_bus = attr.ib()

            def send(self, *args, **kwargs):
                assert len(args) <= 1
                self.event_bus.send(name, **kwargs)

            def connect(self, cb, weak=False):
                # TODO: weak is not handled here...
                self.event_bus.connect(name, cb)

        return Signal(self)


class SpiedEventBus(EventBus):
    ANY = ANY  # Easier to use than doing an import

    def __init__(self):
        super().__init__()
        self._spies = []
        self.spy = self.create_spy()

    def send(self, event, **kwargs):
        for spy in self._spies:
            spy(event, **kwargs)
        super().send(event, **kwargs)

    def create_spy(self):
        spy = EventSpy()
        self._spies.append(spy._on_event_cb)
        return spy

    def destroy_spy(self, spy):
        self._spies.remove(spy._on_event_cb)

    @contextmanager
    def listen(self):
        spy = self.create_spy()
        yield spy
        self.destroy_spy(spy)
