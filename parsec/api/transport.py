import trio
from trio import BrokenResourceError
import struct
from structlog import get_logger
from urllib.parse import urlsplit
from wsproto.connection import WSConnection, ConnectionType
from wsproto.events import (
    ConnectionClosed,
    ConnectionEstablished,
    ConnectionRequested,
    BytesReceived,
    PingReceived,
)


__all__ = ("TransportError", "BaseTransport", "TCPTransport")


logger = get_logger()


class TransportError(Exception):
    pass


class BaseTransport:
    async def aclose(self) -> None:
        """
        Close the underlying stream.
        """
        raise NotImplementedError()

    async def send(self, msg: bytes) -> None:
        """
        Raises:
            TransportError
        """
        raise NotImplementedError()

    async def recv(self) -> bytes:
        """
        Raises:
            TransportError
        """
        raise NotImplementedError()


# Note we let `trio.ClosedResourceError` exceptions bubble up given
# they should be only raised in case of programming error.


class TCPTransport(BaseTransport):
    MAX_MSG_SIZE = 2 ** 20  # 1Mo

    def __init__(self, stream):
        self.stream = stream
        # vanilla_send_all_hook = self.stream.send_stream.send_all_hook
        # vanilla_receive_some_hook = self.stream.receive_stream.receive_some_hook
        # async def send_all_hook():
        #     print('send_all==>', self.stream.send_stream._outgoing._data)
        #     if vanilla_send_all_hook:
        #         await vanilla_send_all_hook()
        #     print('<==send_all', self.stream.send_stream._outgoing._data)
        # async def receive_some_hook():
        #     print('receive_some==>', self.stream.receive_stream._incoming._data)
        #     if vanilla_receive_some_hook:
        #         await vanilla_receive_some_hook()
        #     print('<==receive_some', self.stream.receive_stream._incoming._data)
        # self.stream.send_stream.send_all_hook = send_all_hook
        # self.stream.receive_stream.receive_some_hook = receive_some_hook

    async def aclose(self) -> None:
        try:
            await self.stream.aclose()
        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc

    async def send(self, msg: bytes) -> None:
        assert len(msg) <= self.MAX_MSG_SIZE
        try:
            await self.stream.send_all(struct.pack("!L", len(msg)))
            await self.stream.send_all(msg)
        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc

    async def _recv_exactly(self, size):
        out = bytearray(size)
        offset = 0
        try:
            while offset < size:
                buff = await self.stream.receive_some(size - offset)
                if not buff:
                    # Empty body should normally never occurs, though it is sent
                    # when peer closes connection
                    raise TransportError("Peer has closed connection")
                out[offset : offset + len(buff)] = buff
                offset += len(buff)

        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc

        return out

    async def recv(self) -> bytes:
        msg_size_raw = await self._recv_exactly(4)

        msg_size, = struct.unpack("!L", msg_size_raw)
        if msg_size > self.MAX_MSG_SIZE:
            raise TransportError("Message too big")

        return await self._recv_exactly(msg_size)


class WebsocketTransport(BaseTransport):
    RECEIVE_BYTES = 2 ** 20  # 1Mo

    def __init__(self, stream, ws):
        self.stream = stream
        self.ws = ws
        self._ws_events = ws.events()

    async def _next_ws_event(self):
        while True:
            try:
                return next(self._ws_events)

            except StopIteration as exc:
                # Not enough data to form an event
                await self._net_recv()
                self._ws_events = self.ws.events()

    async def _net_recv(self):
        try:
            in_data = await self.stream.receive_some(self.RECEIVE_BYTES)

        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc

        print(f"RECV {in_data!r}")
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
    async def init_for_client(cls, stream, host, resource):
        ws = WSConnection(ConnectionType.CLIENT, host=host, resource=resource)
        transport = WebsocketTransport(stream, ws)

        # Because this is a client websocket, wsproto has automatically queued up
        # a handshake, and we need to send it and wait for a response.
        await transport._net_send()
        event = await transport._next_ws_event()

        if isinstance(event, ConnectionEstablished):
            logger.debug("[C] WebSocket negotiation complete", ws_event=event)

        else:
            logger.warning("[C] Unexpected event during websocket handshake", ws_event=event)
            reason = f"[C] Unexpected event during websocket handshake: {event}"
            transport.ws.close(code=1000, reason=reason)
            await transport._net_send()
            raise TransportError(reason)

        return transport

    @classmethod
    async def init_for_server(cls, stream):
        ws = WSConnection(ConnectionType.SERVER)
        transport = WebsocketTransport(stream, ws)

        # Wait for client to init websocket handshake
        event = await transport._next_ws_event()
        print("event", event)
        if isinstance(event, ConnectionRequested):
            logger.debug("[S] Accepting WebSocket upgrade")
            transport.ws.accept(event)
            await transport._net_send()
            return transport

        logger.warning("[S] Unexpected event during websocket handshake", ws_event=event)
        reason = f"[S] Unexpected event during websocket handshake: {event}"
        transport.ws.close(code=1000, reason=reason)
        await transport._net_send()
        raise TransportError(reason)

    async def aclose(self) -> None:
        try:
            await self.stream.aclose()
        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc

    async def send(self, msg: bytes) -> None:
        """
        Raises:
            TransportError
        """
        self.ws.send_data(msg)
        await self._net_send()

    async def recv(self) -> bytes:
        """
        Raises:
            TransportError
        """
        while True:
            event = await self._next_ws_event()

            if isinstance(event, ConnectionClosed):
                logger.debug("Connection closed", code=event.code, reason=event.reason)
                raise TransportError("Peer has closed connection")

            elif isinstance(event, BytesReceived):
                return event.data

            elif isinstance(event, PingReceived):
                # wsproto handles ping events for you by placing a pong frame in
                # the outgoing buffer. You should not call pong() unless you want to
                # send an unsolicited pong frame.
                logger.debug("Received ping and sending pong")
                await self._net_send()

            else:
                logger.warning("Unexpected event", ws_event=event)
                raise TransportError("Unexpected event: {event}")


class ClientTransportFactory:
    def __init__(self, addr: str, certfile: str = None, keyfile: str = None):
        assert not (keyfile and not certfile)

        self.addr = addr
        parsed_addr = urlsplit(addr)

        if parsed_addr.scheme == "ws":
            self._transport_cls = self._make_ws_transport
            use_ssl = False

        elif parsed_addr.scheme == "wss":
            self._transport_cls = self._make_ws_transport
            use_ssl = True

        elif parsed_addr.scheme == "tcp":
            self._transport_cls = self._make_tcp_transport
            use_ssl = False

        elif parsed_addr.scheme == "tcp+ssl":
            self._transport_cls = self._make_tcp_transport
            use_ssl = True

        else:
            raise ValueError(f"Unknown scheme in {addr} (must be ws, wss, tcp or tcp+ssl)")

        self.hostname = parsed_addr.hostname
        self.port = parsed_addr.port or (443 if not use_ssl else 80)
        self.path = parsed_addr.path

        if not use_ssl:
            self.ssl_context = None

        else:
            self.ssl_context = trio.ssl.create_default_context(trio.ssl.Purpose.CLIENT_AUTH)
            if certfile:
                self.ssl_context.load_cert_chain(certfile, keyfile)
            else:
                self.ssl_context.load_default_certs()

    async def _make_ws_transport(self, stream):
        return await WebsocketTransport.init_for_client(stream, self.hostname, self.path)

    async def _make_tcp_transport(self, stream):
        return TCPTransport(stream)

    async def new_transport(self) -> WebsocketTransport:
        try:
            stream = await trio.open_tcp_stream(self.hostname, self.port)

        except OSError as exc:
            logger.debug("Impossible to connect to backend", reason=exc)
            raise TransportError(exc) from exc

        if self.ssl_context:
            stream = trio.ssl.SSLStream(stream, self.ssl_context, server_hostname=self.hostname)

        return await self._transport_cls(stream)


class ServerTransportFactory:
    def __init__(self, scheme: str, certfile: str = None, keyfile: str = None):
        assert not (keyfile and not certfile)

        if scheme == "ws":
            self._transport_cls = self._make_ws_transport
            use_ssl = False

        elif scheme == "wss":
            self._transport_cls = self._make_ws_transport
            use_ssl = True

        elif scheme == "tcp":
            self._transport_cls = self._make_tcp_transport
            use_ssl = False

        elif scheme == "tcp+ssl":
            self._transport_cls = self._make_tcp_transport
            use_ssl = True

        else:
            raise ValueError(f"Unknown scheme `{scheme}` (must be ws, wss, tcp or tcp+ssl)")

        if not use_ssl:
            self.ssl_context = None

        else:
            self.ssl_context = trio.ssl.create_default_context(trio.ssl.Purpose.SERVER_AUTH)
            if certfile:
                self.ssl_context.load_cert_chain(certfile, keyfile)
            else:
                self.ssl_context.load_default_certs()

    async def _make_ws_transport(self, stream) -> WebsocketTransport:
        return await WebsocketTransport.init_for_server(stream)

    async def _make_tcp_transport(self, stream) -> TCPTransport:
        return TCPTransport(stream)

    async def wrap_with_transport(self, stream) -> BaseTransport:
        if self.ssl_context:
            stream = trio.ssl.SSLStream(stream, self.ssl_context, server_side=True)

        return await self._transport_cls(stream)
