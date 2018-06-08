import trio
import logbook
import json

from parsec.schema import UnknownCheckedSchema, fields
from parsec.core.base import BaseAsyncComponent
from parsec.core.devices_manager import Device
from parsec.core import backend_connection as bc


logger = logbook.Logger("parsec.core.backend_events_manager")


class BackendEventListenRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.String(required=True)
    subject = fields.String(missing=None)


backend_event_listen_rep_schema = BackendEventListenRepSchema()


class SubscribeBackendEventError(Exception):
    pass


class ListenBackendEventError(Exception):
    pass


class BackendEventsManager(BaseAsyncComponent):
    """
    Difference between signal and event:
    - signals are in-process notifications
    - events are message sent downstream from the backend to the client core
    """

    def __init__(self, device: Device, backend_addr: str, signal_ns):
        super().__init__()
        self.device = device
        self.backend_addr = backend_addr
        self._signal_ns = signal_ns
        self._nursery = None
        self._subscribed_events = set()

    async def _init(self, nursery):
        self._nursery = nursery
        self._event_listener_task_cancel_scope = await nursery.start(self._event_listener_task)

    async def _teardown(self):
        self._event_listener_task_cancel_scope.cancel()

    async def subscribe_backend_event(self, event, subject=None):
        """
        Raises:
            KeyError: if event/subject couple has already been previously subscribed.
        """
        async with self._lock:
            key = (event, subject)
            if key in self._subscribed_events:
                raise KeyError("%s@%s already subscribed" % key)

            self._subscribed_events.add(key)
            logger.debug("Subscribe %s@%s, restarting event listener" % (event, subject))
            await self._teardown()
            await self._init(self._nursery)

    async def unsubscribe_backend_event(self, event, subject=None):
        """
        Raises:
            KeyError: if event/subject couple has not been previously subscribed.
        """
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

            # Useful for tests to know when we actually start to listen for backend events
            self._signal_ns.signal("backend_event_manager_listener_started").send()

            while True:
                await sock.send({"cmd": "event_listen"})
                rep = await sock.recv()
                _, errors = backend_event_listen_rep_schema.load(rep)
                if errors:
                    raise ListenBackendEventError(
                        "Bad reponse %r while listening for event: %r" % (rep, errors)
                    )

                subject = rep.get("subject")
                event = rep.get("event")
                if subject is None:
                    self._signal_ns.signal(event).send()
                else:
                    self._signal_ns.signal(event).send(subject)

        with trio.open_cancel_scope() as cancel:
            task_status.started(cancel)

            while True:
                try:
                    sock = await bc.backend_connection_factory(self.backend_addr, self.device)
                    await _event_pump(sock)
                except (
                    bc.BackendNotAvailable,
                    trio.BrokenStreamError,
                    trio.ClosedStreamError,
                ) as exc:
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
                    self._signal_ns.signal("panic").send(exc)
