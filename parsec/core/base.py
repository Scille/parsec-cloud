import attr
from blinker import signal

from effect import sync_performer, TypeDispatcher, base_dispatcher


@attr.s
class EEvent:
    event = attr.ib()
    sender = attr.ib()


@sync_performer
def perform_event(dispatcher, intent):
    signal(intent.event).send(intent.sender)


base_dispatcher = TypeDispatcher({**base_dispatcher.mapping, EEvent: perform_event})
