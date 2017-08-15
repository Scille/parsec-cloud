import attr
from collections import defaultdict

from effect2 import (
    ComposedDispatcher, TypeDispatcher, base_dispatcher, base_asyncio_dispatcher, Effect, do
)


@attr.s
class EEvent:
    event = attr.ib()
    sender = attr.ib()


@attr.s
class ERegisterEvent:
    intent_factory = attr.ib()
    event = attr.ib()
    sender = attr.ib(default=None)


@attr.s
class EUnregisterEvent:
    intent_factory = attr.ib()
    event = attr.ib()
    sender = attr.ib(default=None)


@attr.s
class EventComponent:
    listeners = attr.ib(default=attr.Factory(lambda: defaultdict(set)))

    @do
    def perform_trigger_event(self, intent):
        key = (intent.event, intent.sender)
        for intent_factory in self.listeners[key]:
            yield Effect(intent_factory(intent.event, intent.sender))

    @do
    def perform_register_event(self, intent):
        key = (intent.event, intent.sender)
        self.listeners[key].add(intent.intent_factory)

    @do
    def perform_unregister_event(self, intent):
        key = (intent.event, intent.sender)
        self.listeners[key].remove(intent.intent_factory)

    def get_dispatcher(self):
        return TypeDispatcher({
            EEvent: self.perform_trigger_event,
            ERegisterEvent: self.perform_register_event,
            EUnregisterEvent: self.perform_register_event
        })


base_dispatcher = ComposedDispatcher([
    base_dispatcher,
    base_asyncio_dispatcher,
])
