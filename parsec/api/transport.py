# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import uuid4
import trio
from trio import BrokenResourceError
from structlog import get_logger
from wsproto.frame_protocol import CloseReason
from wsproto.connection import WSConnection, ConnectionType
from wsproto.events import (
    ConnectionClosed,
    ConnectionEstablished,
    ConnectionRequested,
    BytesReceived,
    PingReceived,
    PongReceived,
)


__all__ = ("TransportError", "Transport")


logger = get_logger()


class TransportError(Exception):
    pass


class TransportClosedByPeer(TransportError):
    pass


# Note we let `trio.ClosedResourceError` exceptions bubble up given
# they should be only raised in case of programming error.


class Transport:
    RECEIVE_BYTES = 2 ** 20  # 1Mo

    def __init__(self, stream, ws):
        self.stream = stream
        self.ws = ws
        self.keepalive_time = 0
        self.conn_id = uuid4().hex
        self.logger = logger.bind(conn_id=self.conn_id)
        self._ws_events = ws.events()

    async def _next_ws_event(self):
        while True:
            try:
                return next(self._ws_events)

            except StopIteration:
                # Not enough data to form an event
                await self._net_recv()
                self._ws_events = self.ws.events()

    async def _net_recv(self):
        try:
            in_data = await self.stream.receive_some(self.RECEIVE_BYTES)

        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc

        if not in_data:
            # A receive of zero bytes indicates the TCP socket has been closed. We
            # need to pass None to wsproto to update its internal state.
            self.ws.receive_bytes(None)
        else:
            self.ws.receive_bytes(in_data)

    async def _net_send(self):
        out_data = self.ws.bytes_to_send()
        try:
            await self.stream.send_all(out_data)

        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc

    @classmethod
    async def init_for_client(cls, stream, host):
        ws = WSConnection(ConnectionType.CLIENT, host=host, resource="/")
        transport = cls(stream, ws)

        # Because this is a client websocket, wsproto has automatically queued up
        # a handshake, we need to send it and wait for a response.
        await transport._net_send()
        event = await transport._next_ws_event()

        if isinstance(event, ConnectionEstablished):
            transport.logger.debug("WebSocket negotiation complete", ws_event=event)

        else:
            transport.logger.warning("Unexpected event during WebSocket handshake", ws_event=event)
            reason = f"Unexpected event during WebSocket handshake: {event}"
            raise TransportError(reason)

        return transport

    @classmethod
    async def init_for_server(cls, stream):
        ws = WSConnection(ConnectionType.SERVER)
        transport = cls(stream, ws)

        # Wait for client to init WebSocket handshake
        event = await transport._next_ws_event()
        if isinstance(event, ConnectionRequested):
            transport.logger.debug("Accepting WebSocket upgrade")
            transport.ws.accept(event)
            await transport._net_send()
            return transport

        transport.logger.warning("Unexpected event during WebSocket handshake", ws_event=event)
        raise TransportError(f"Unexpected event during WebSocket handshake: {event}")

    async def aclose(self) -> None:
        try:
            self.ws.close(code=CloseReason.NORMAL_CLOSURE)
            await self._net_send()
            await self.stream.aclose()

        except (BrokenResourceError, TransportError):
            pass

    async def send(self, msg: bytes) -> None:
        """
        Raises:
            TransportError
        """
        self.ws.send_data(msg)
        await self._net_send()

    async def recv(self, keepalive: bool = False) -> bytes:
        """
        Raises:
            TransportError
        """
        data = bytearray()
        while True:
            if keepalive:
                with trio.move_on_after(self.keepalive_time) as cancel_scope:
                    event = await self._next_ws_event()
                if cancel_scope.cancel_called:
                    self.logger.debug("Sending keep alive ping")
                    self.ws.ping()
                    await self._net_send()
                    continue
            else:
                event = await self._next_ws_event()

            if isinstance(event, ConnectionClosed):
                self.logger.debug("Connection closed", code=event.code, reason=event.reason)
                raise TransportClosedByPeer("Peer has closed connection")

            elif isinstance(event, BytesReceived):
                # TODO: check that data doesn't go over MAX_BIN_LEN (1 MB)
                # Msgpack will refuse to unpack it so we should fail early on if that happens
                data += event.data
                if event.message_finished:
                    return data

            elif isinstance(event, PingReceived):
                # wsproto handles ping events for you by placing a pong frame in
                # the outgoing buffer. You should not call pong() unless you want to
                # send an unsolicited pong frame.
                self.logger.debug("Received ping and sending pong")
                await self._net_send()

            elif isinstance(event, PongReceived):
                # Nothing to do \o/
                self.logger.debug("Received pong")

            else:
                self.logger.warning("Unexpected event", ws_event=event)
                raise TransportError(f"Unexpected event: {event}")
