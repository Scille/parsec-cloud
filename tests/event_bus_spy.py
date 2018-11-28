import trio
import attr
import pendulum
from contextlib import contextmanager
from unittest.mock import ANY

from parsec.event_bus import EventBus


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


def compare_events_with_any(a, b):
    ca, cb = convert_events_with_any(a, b)
    return ca == cb


def convert_event_lists_with_any(a, b):
    converted = [convert_events_with_any(*x) for x in zip(a, b)]
    return (
        [ia for ia, _ in converted] + a[len(converted) :],
        [ib for _, ib in converted] + b[len(converted) :],
    )


@attr.s(frozen=True, slots=True)
class SpiedEvent:
    event = attr.ib()
    kwargs = attr.ib(factory=dict)
    dt = attr.ib(factory=pendulum.now)


@attr.s(repr=False)
class EventBusSpy:
    ANY = ANY  # Easier to use than doing an import
    events = attr.ib(factory=list)
    _waiters = attr.ib(factory=set)

    def __repr__(self):
        return f"<{type(self).__name__}({[e.event for e in self.events]})>"

    def _on_event_cb(self, event, **kwargs):
        cooked_event = SpiedEvent(event, kwargs)
        self.events.append(cooked_event)
        for waiter in self._waiters.copy():
            waiter(cooked_event)

    def clear(self):
        self.events.clear()

    async def wait_for_backend_connection_ready(self):
        for occured_event in reversed(self.events):
            if occured_event.event == "backend.connection.ready":
                return occured_event
            elif occured_event.event == "backend.connection.lost":
                break

        return await self._wait(SpiedEvent("backend.connection.ready", dt=ANY))

    async def wait(self, event, dt=ANY, kwargs=ANY):
        expected = SpiedEvent(event, kwargs, dt)
        for occured_event in reversed(self.events):
            if compare_events_with_any(expected, occured_event):
                return occured_event

        return await self._wait(expected)

    async def _wait(self, cooked_expected_event):
        catcher = trio.Queue(1)

        def _waiter(cooked_event):
            if compare_events_with_any(cooked_expected_event, cooked_event):
                catcher.put_nowait(cooked_event)
                self._waiters.remove(_waiter)

        self._waiters.add(_waiter)
        return await catcher.get()

    async def wait_multiple(self, events):
        expected_events = self._cook_events_params(events)
        try:
            self.assert_events_occured(expected_events)
            return
        except AssertionError:
            pass

        done = trio.Event()

        def _waiter(cooked_event):
            try:
                self.assert_events_occured(expected_events)
                self._waiters.remove(_waiter)
                done.set()
            except AssertionError:
                pass

        self._waiters.add(_waiter)
        await done.wait()

    def _cook_events_params(self, events):
        cooked_events = [self._cook_event_params(event) for event in events]
        return cooked_events

    def _cook_event_params(self, event):
        if isinstance(event, SpiedEvent):
            return event
        elif event is ANY:
            return event
        elif isinstance(event, str):
            return SpiedEvent(event, ANY, ANY)
        elif isinstance(event, tuple):
            event = event + (ANY,) * (3 - len(event))
            return SpiedEvent(*event)
        else:
            raise ValueError(
                "event must be provided as `SpiedEvent`, `(<event>, <kwargs>, <dt>)` tuple or string"
            )

    def assert_event_occured(self, event, dt=ANY, kwargs=ANY):
        expected = SpiedEvent(event, kwargs, dt)
        for occured in self.events:
            cooked_occured, cooked_expected = convert_events_with_any(occured, expected)
            if cooked_occured == cooked_expected:
                break
        else:
            raise AssertionError(f"Event {cooked_expected} didn't occured")

    def assert_events_occured(self, events):
        expected_events = self._cook_events_params(events)
        occured_events = iter(self.events)
        try:
            for i, expected in enumerate(expected_events):
                while True:
                    occured = next(occured_events)
                    if compare_events_with_any(occured, expected):
                        break
        except StopIteration:
            raise AssertionError("Missing events: " + "\n".join([str(x) for x in events[i:]]))

    def assert_events_exactly_occured(self, events):
        events = self._cook_events_params(events)
        cooked_expected, cooked_observed = convert_event_lists_with_any(events, self.events)
        assert cooked_observed == cooked_expected


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
        spy = EventBusSpy()
        self._spies.append(spy._on_event_cb)
        return spy

    def destroy_spy(self, spy):
        self._spies.remove(spy._on_event_cb)

    @contextmanager
    def listen(self):
        spy = self.create_spy()
        yield spy
        self.destroy_spy(spy)
