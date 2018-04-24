import trio
import blinker
import logbook
import json

from parsec.schema import UnknownCheckedSchema, fields
from parsec.core.base import BaseAsyncComponent
from parsec.core.devices_manager import Device
from parsec.core import backend_connection as bc


logger = logbook.Logger("parsec.core.events_manager")


class EventListenRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.String(required=True)
    subject = fields.String(missing=None)


event_listen_rep_schema = EventListenRepSchema()


class SubscribeBackendEventError(Exception):
    pass


class ListenBackendEventError(Exception):
    pass


class EventsManager(BaseAsyncComponent):
    """
    Difference between signal and event:
    - signals are in-process notifications
    - events are message sent downstream from the backend to the client core
    """

    def __init__(self, device: Device, backend_addr: str):
        super().__init__()
        self.device = device
        self.backend_addr = backend_addr
        self._ns = blinker.Namespace()
        self._nursery = None
        self._subscribed_events = []

    async def _init(self, nursery):
        self._nursery = nursery
        self._event_listener_task_cancel_scope = await nursery.start(self._event_listener_task)

    async def _teardown(self):
        self._event_listener_task_cancel_scope.cancel()

    def signal(self, name: str) -> blinker.NamedSignal:
        return self._ns.signal(name)

    async def subscribe_backend_event(self, event, subject=None):
        async with self._lock:
            self._subscribed_events.append((event, subject))
            logger.debug("Subscribe %s@%s, restarting event listener" % (event, subject))
            await self._teardown()
            await self._init(self._nursery)

    async def unsubscribe_backend_event(self, event, subject=None):
        async with self._lock:
            self._subscribed_events.remove((event, subject))
            logger.debug("Unsubscribe %s@%s, restarting event listener" % (event, subject))
            await self._teardown()
            await self._init(self._nursery)

    async def _event_listener_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        # Copy `self._subscribed_event` to avoid concurrent modifications
        subscribed_events = self._subscribed_events.copy()

        async def _event_pump(sock):
            # TODO: allow to subscribe to multiple events in a single query...
            for event, subject in subscribed_events:
                await sock.send({"cmd": "event_subscribe", "event": event, "subject": subject})
                rep = await sock.recv()
                if rep["status"] != "ok":
                    raise SubscribeBackendEventError(
                        "Cannot subscribe to event `%s@%s`: %r" % (event, subject, rep)
                    )

            while True:
                await sock.send({"cmd": "event_listen"})
                rep = await sock.recv()
                _, errors = event_listen_rep_schema.load(rep)
                if errors:
                    raise ListenBackendEventError(
                        "Bad reponse %r while listening for event: %r" % (rep, errors)
                    )

                subject = rep.get("subject")
                event = rep.get("event")
                if subject is None:
                    self._ns.signal(event).send()
                else:
                    self._ns.signal(event).send(subject)

        with trio.open_cancel_scope() as cancel:
            task_status.started(cancel)

            while True:
                try:
                    sock = await bc.backend_connection_factory(self.backend_addr, self.device)
                    await _event_pump(sock)
                except (bc.BackendNotAvailable, trio.BrokenStreamError) as exc:
                    # In case of connection failure, wait a bit and restart
                    logger.debug("Connection lost with backend ({}), restarting connection...", exc)
                    await trio.sleep(1)
                except (SubscribeBackendEventError, ListenBackendEventError, json.JSONDecodeError):
                    logger.exception("Invalid response sent by backend, restarting connection...")
                    await trio.sleep(1)
                except bc.HandshakeError as exc:
                    # Handshake error means there is no need retrying the connection
                    # Only thing we can do is sending a signal to notify the
                    # trouble...
                    # TODO: think about this kind of signal format
                    self._ns.signal("panic").send(exc)
