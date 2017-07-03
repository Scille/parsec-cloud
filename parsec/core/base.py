import attr
from blinker import signal

from effect2 import ComposedDispatcher, TypeDispatcher, base_dispatcher, base_asyncio_dispatcher


@attr.s
class EEvent:
    event = attr.ib()
    sender = attr.ib()


def perform_event(intent):
    signal(intent.event).send(intent.sender)


base_dispatcher = ComposedDispatcher([
    base_dispatcher,
    base_asyncio_dispatcher,
    TypeDispatcher({EEvent: perform_event})
])
