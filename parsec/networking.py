"""
Defines client/server communication protocol on top of trio's Stream.
"""

import attr
import trio
import json
from structlog import get_logger
from collections import deque
from wsproto.connection import WSConnection, ConnectionType
from wsproto.events import (
    ConnectionClosed,
    ConnectionEstablished,
    ConnectionRequested,
    BytesReceived,
)

from parsec.utils import ejson_dumps


logger = get_logger()
next_conn_id = 0

RECEIVE_BYTES = 4096


async def serve_client(dispatch_request, conn) -> None:
    def _filter_big_fields(data):
        # As hacky as arbitrary... but works well so far !
        filtered_data = data.copy()
        try:
            if len(data["block"]) > 200:
                filtered_data["block"] = f"{data['block'][:100]}[...]{data['block'][-100:]}"
        except (KeyError, ValueError, TypeError):
            pass
        try:
            if len(data["blob"]) > 200:
                filtered_data["blob"] = f"{data['blob'][:100]}[...]{data['blob'][-100:]}"
        except (KeyError, ValueError, TypeError):
            pass
        return filtered_data

    try:
        sock = CoreAPICookedSocket(conn)
        while True:
            try:
                req = await sock.recv()
            except json.JSONDecodeError:
                rep = {"status": "invalid_msg_format", "reason": "Invalid message format"}
                await sock.send(rep)
                continue

            if not req:  # Client disconnected
                logger.debug("client disconnected")
                return

            logger.debug("req", req=_filter_big_fields(req))

            rep = await dispatch_request(req)

            logger.debug("rep", rep=_filter_big_fields(rep))

            await sock.send(rep)
    except trio.BrokenStreamError:
        # Client has closed connection
        pass
    except Exception as exc:
        # If we are here, something unexpected happened...
        logger.error("unexpected error", exc_info=exc)
        # TODO: do we need to close the socket (i.e. sockstream) here ?
        # or should we let the caller (most certainly the server) handle this ?
        await sock.aclose()
        raise


async def net_send(ws, conn):
    """ Write pending data from websocket to network. """
    out_data = ws.bytes_to_send()
    await conn.send_all(out_data)


async def net_recv(ws, conn):
    """ Read pending data from network into websocket. """
    in_data = await conn.receive_some(RECEIVE_BYTES)
    if not in_data:
        # A receive of zero bytes indicates the TCP socket has been closed. We
        # need to pass None to wsproto to update its internal state.
        ws.receive_bytes(None)
    else:
        ws.receive_bytes(in_data)


async def net_send_recv(ws, conn):
    """ Send pending data and then wait for response. """
    await net_send(ws, conn)
    await net_recv(ws, conn)


def get_next_conn_id():
    global next_conn_id
    next_conn_id += 1
    return next_conn_id


def client_cooked_socket_factory(conn, hostname, resource="server"):
    ws = WSConnection(ConnectionType.CLIENT, host=hostname, resource=resource)
    return CookedSocket(ws, conn)


def server_cooked_socket_factory(conn):
    conn_id = get_next_conn_id()
    ws = WSConnection(ConnectionType.SERVER)
    return CookedSocket(ws, conn, conn_id)


@attr.s
class CookedSocket:
    BUFFSIZE = 4049

    ws = attr.ib()
    conn = attr.ib()
    conn_id = attr.ib()
    events = attr.ib(init=False)

    @events.default
    def _events(self):
        return self.ws.events()

    @conn_id.default
    def _conn_id(self, id=0):
        return id

    async def init(self):
        # Because this is a client WebSocket, wsproto has automatically queued up
        # a handshake, and we need to send it and wait for a response.
        await net_send_recv(self.ws, self.conn)
        event = next(self.events)
        assert isinstance(event, ConnectionEstablished)

    async def aclose(self) -> None:
        """
        Close the underlying conn.
        """
        try:
            self.ws.close(code=1000)
        except AttributeError:
            return
        # After sending the closing frame, we won't get any more events. The server
        # should send a reply and then close the connection, so we need to receive
        # twice:
        # await net_send_recv(self.ws, self.conn)  # TODO usefull or not?
        await self.conn.aclose()

    async def send(self, req: dict) -> None:
        """
        Args:
            req: Dictionary data that will be serialized and sent.

        Raises:
            TypeError: if provided req is not a valid JSON serializable object.
            trio.ResourceBusyError: if another task is already executing a send_all(),
                wait_send_all_might_not_block(), or HalfCloseableStream.send_eof() on this stream.
            trio.BrokenStreamError: if something has gone wrong, and the stream is broken.
            trio.ClosedStreamError: if you already closed this stream object.
        """
        if not isinstance(req, dict):
            raise TypeError("req must be a dict")

        payload = ejson_dumps(req).encode() + b"\n"
        try:
            self.ws.send_data(payload)
        except AttributeError:
            raise trio.BrokenStreamError()
        await net_send(self.ws, self.conn)

    async def recv(self) -> dict:
        """
        Returns:
            Received data deserialized as a dictionary.

        Raises:
            json.JSONDecodeError: if returned message is not valid JSON data.
            trio.ResourceBusyError: if two tasks attempt to call receive_some()
                on the same stream at the same time.
            trio.BrokenStreamError: if something has gone wrong, and the stream is broken.
            trio.ClosedStreamError: if you already closed this stream object.
        """
        await net_recv(self.ws, self.conn)

        try:
            event = next(self.events)
        except StopIteration as exc:
            raise trio.BrokenStreamError() from exc

        if isinstance(event, ConnectionRequested):
            self.ws.accept(event)
        elif isinstance(event, ConnectionClosed):
            await net_send(self.ws, self.conn)
            raise trio.BrokenStreamError()
        elif isinstance(event, BytesReceived):
            return json.loads(event.data.decode())

        await net_send(self.ws, self.conn)


@attr.s
class CoreAPICookedSocket:
    BUFFSIZE = 4049

    sockstream = attr.ib()
    _recv_buff = attr.ib(default=attr.Factory(bytearray))
    _reps_ready = attr.ib(default=attr.Factory(deque))

    async def aclose(self) -> None:
        """
        Close the underlying sockstream.
        """
        await self.sockstream.aclose()

    async def send(self, req: dict) -> None:
        """
        Args:
            req: Dictionary data that will be serialized and sent.

        Raises:
            TypeError: if provided req is not a valid JSON serializable object.
            trio.ResourceBusyError: if another task is already executing a send_all(),
                wait_send_all_might_not_block(), or HalfCloseableStream.send_eof() on this stream.
            trio.BrokenStreamError: if something has gone wrong, and the stream is broken.
            trio.ClosedStreamError: if you already closed this stream object.
        """
        if not isinstance(req, dict):
            raise TypeError("req must be a dict")

        payload = ejson_dumps(req).encode("utf8") + b"\n"
        await self.sockstream.send_all(payload)

    async def recv(self) -> dict:
        """
        Returns:
            Received data deserialized as a dictionary.

        Raises:
            json.JSONDecodeError: if returned message is not valid JSON data.
            trio.ResourceBusyError: if two tasks attempt to call receive_some()
                on the same stream at the same time.
            trio.BrokenStreamError: if something has gone wrong, and the stream is broken.
            trio.ClosedStreamError: if you already closed this stream object.
        """
        while True:
            new_buff = await self.sockstream.receive_some(self.BUFFSIZE)
            if not new_buff:
                # Empty body should normally never occurs, though it is sent
                # when peer closes connection
                raise trio.BrokenStreamError("Peer has closed connection")
            self._recv_buff += new_buff

            *reps, unfinished_msg = self._recv_buff.split(b"\n")
            self._reps_ready += reps
            self._recv_buff = unfinished_msg
            if not self._reps_ready:
                # Return in the loop to do a new BUFFSIZE read on the socket
                continue

            else:
                next_rep = self._reps_ready.popleft()
                return json.loads(next_rep.decode("utf8"))
