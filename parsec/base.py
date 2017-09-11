import attr
from collections import defaultdict

from effect2 import (
    ComposedDispatcher, TypeDispatcher, base_dispatcher, base_asyncio_dispatcher, Effect
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
    # Use list instead of set to store intent_factories to trigger them in
    # a determinist order
    listeners = attr.ib(default=attr.Factory(lambda: defaultdict(list)))

    async def perform_trigger_event(self, intent):
        key = (intent.event, intent.sender)
        for intent_factory in self.listeners[key]:
            await Effect(intent_factory(intent.event, intent.sender))
        # Sender=None means registered for all senders
        key = (intent.event, None)
        for intent_factory in self.listeners[key]:
            await Effect(intent_factory(intent.event, intent.sender))

    async def perform_register_event(self, intent):
        key = (intent.event, intent.sender)
        if intent.intent_factory not in self.listeners[key]:
            self.listeners[key].append(intent.intent_factory)

    async def perform_unregister_event(self, intent):
        key = (intent.event, intent.sender)
        self.listeners[key].remove(intent.intent_factory)

    def get_dispatcher(self):
        return TypeDispatcher({
            EEvent: self.perform_trigger_event,
            ERegisterEvent: self.perform_register_event,
            EUnregisterEvent: self.perform_unregister_event
        })


base_dispatcher = ComposedDispatcher([
    base_dispatcher,
    base_asyncio_dispatcher,
])
