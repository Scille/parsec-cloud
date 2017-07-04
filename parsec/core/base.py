import attr
from blinker import signal

from effect2 import (
    ComposedDispatcher, TypeDispatcher, base_dispatcher, base_asyncio_dispatcher, Effect, do
)


@attr.s
class EEvent:
    event = attr.ib()
    sender = attr.ib()


@do
def perform_event(intent):
    effects = signal(intent.event).send(intent.sender)
    for callback, effect in effects:
        if effect:
            assert isinstance(effect, Effect), \
                "Event callback can only return Effect, %s did not." % callback
            yield effect


base_dispatcher = ComposedDispatcher([
    base_dispatcher,
    base_asyncio_dispatcher,
    TypeDispatcher({EEvent: perform_event})
])
