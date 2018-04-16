import trio
import logbook
from urllib.parse import urlparse

from parsec.utils import CookedSocket
from parsec.handshake import ClientHandshake, AnonymousClientHandshake
from parsec.utils import ParsecError


logger = logbook.Logger("parsec.core.backend_connection")


class BackendError(ParsecError):
    pass


class BackendNotAvailable(BackendError):
    pass


class BackendConcurrencyError(BackendError):
    pass


class BackendConnection:

    def __init__(self, device, addr, signal_ns):
        self.handshake_id = device.id
        self.handshake_signkey = device.device_signkey
        self.addr = urlparse(addr)
        self.signal_ns = signal_ns
        self.nursery = None
        self._lock = trio.Lock()
        self._sock = None
        self._event_listener_task_cancel_scope = None
        self._subscribed_events = []

    async def _socket_connection_factory(self):
        logger.debug("connecting to backend {}:{}", self.addr.hostname, self.addr.port)
        sockstream = await trio.open_tcp_stream(self.addr.hostname, self.addr.port)
        try:
            logger.debug("handshake as {}", self.handshake_id)
            sock = CookedSocket(sockstream)
            if self.handshake_id == "anonymous":
                ch = AnonymousClientHandshake()
            else:
                ch = ClientHandshake(self.handshake_id, self.handshake_signkey)
            challenge_req = await sock.recv()
            answer_req = ch.process_challenge_req(challenge_req)
            await sock.send(answer_req)
            result_req = await sock.recv()
            ch.process_result_req(result_req)
        except Exception as exc:
            logger.debug("handshake failed {!r}", exc)
            await sockstream.aclose()
            raise exc

        return sock

    async def _init_send_connection(self):
        if self._sock:
            await self._sock.aclose()
        self._sock = await self._socket_connection_factory()

    async def _naive_send(self, req):
        if not self._sock:
            raise BackendNotAvailable()

        await self._sock.send(req)
        return await self._sock.recv()

    async def send(self, req):
        async with self._lock:
            # Try to use the already connected socket
            try:
                logger.debug("send {}", req)
                rep = await self._naive_send(req)
                logger.debug("recv {}", rep)
                return rep

            except (
                BackendNotAvailable, trio.BrokenStreamError, trio.ClosedStreamError
            ) as exc:
                logger.debug("retrying, cannot reach backend: {!r}", exc)
                try:
                    # If it failed, reopen the socket and retry the request
                    await self._init_send_connection()
                    logger.debug("send {}", req)
                    rep = await self._naive_send(req)
                    logger.debug("recv {}", rep)
                    return rep

                except (OSError, trio.BrokenStreamError) as e:
                    logger.debug("aborting, cannot reach backend: {!r}", e)
                    # Failed again, it seems we are offline
                    raise BackendNotAvailable() from e

    async def _event_listener_task(self, *, task_status=trio.TASK_STATUS_IGNORED):

        async def _event_pump(sock, signal_ns, subscribed_event):
            for event, subject in subscribed_event:
                await sock.send(
                    {"cmd": "event_subscribe", "event": event, "subject": subject}
                )
                rep = await sock.recv()
                if rep["status"] != "ok":
                    # TODO: better exception
                    raise BackendError(rep)

            while True:
                await sock.send({"cmd": "event_listen"})
                rep = await sock.recv()
                if rep["status"] != "ok":
                    raise BackendError(rep)

                if rep.get("subject") is None:
                    signal_ns.signal(rep["event"]).send()
                else:
                    signal_ns.signal(rep["event"]).send(rep["subject"])

        with trio.open_cancel_scope() as cancel:
            task_status.started(cancel)

            while True:
                try:
                    sock = await self._socket_connection_factory()
                    await _event_pump(sock, self.signal_ns, self._subscribed_events)
                except (
                    OSError,
                    BackendNotAvailable,
                    trio.BrokenStreamError,
                    trio.ClosedStreamError,
                ):
                    # In case of connection failure, wait a bit and restart
                    await trio.sleep(1)

    async def init(self, nursery):
        self.nursery = nursery
        self._event_listener_task_cancel_scope = await nursery.start(
            self._event_listener_task
        )

    # Try to open connection with the backend to save time for first
    # request
    # try:
    #     async with self._lock:
    #         await self._init_send_connection()
    # except (OSError, trio.BrokenStreamError):
    #     pass

    async def teardown(self):
        self._event_listener_task_cancel_scope.cancel()
        if self._sock:
            await self._sock.aclose()

    async def ping(self):
        await self.send({"cmd": "ping", "ping": ""})

    async def subscribe_event(self, event, subject=None):
        self._subscribed_events.append((event, subject))
        self._event_listener_task_cancel_scope.cancel()
        self._event_listener_task_cancel_scope = await self.nursery.start(
            self._event_listener_task
        )

    async def unsubscribe_event(self, event, subject=None):
        self._subscribed_events.remove((event, subject))
        self._event_listener_task_cancel_scope.cancel()
        self._event_listener_task_cancel_scope = await self.nursery.start(
            self._event_listener_task
        )


class AnonymousBackendConnection(BackendConnection):

    def __init__(self, addr):
        self.handshake_id = "anonymous"
        self.handshake_signkey = None
        self.addr = urlparse(addr)
        self._lock = trio.Lock()
        self._sock = None

    async def init(self, nursery):
        # TODO: Avoid this ugly code copy/paste
        self.nursery = nursery
        # Try to open connection with the backend to save time for first
        # request
        try:
            async with self._lock:
                await self._init_send_connection()
        except (OSError, trio.BrokenStreamError):
            pass

    async def teardown(self):
        if self._sock:
            await self._sock.aclose()


async def backend_send_anonymous_cmd(addr, cmd):
    conn = AnonymousBackendConnection(addr)
    try:
        return await conn.send(cmd)

    finally:
        await conn.teardown()
