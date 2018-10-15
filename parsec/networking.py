"""
Defines client/server communication protocol on top of trio's Stream.
"""

import attr
import trio
import json
import logbook
import traceback
from collections import deque

from parsec.utils import ejson_dumps


logger = logbook.Logger("parsec.networking")


async def serve_client(dispatch_request, sockstream) -> None:
    def _filter_big_fields(data):
        # As hacky as arbitrary... but works well so far !
        return {
            **data,
            "block": f"{data['block'][:100]}[...]{data['block'][-100:]}"
            if "block" in data
            else data["block"],
            "blob": f"{data['blob'][:100]}[...]{data['blob'][-100:]}"
            if "blob" in data
            else data["blob"],
        }

    try:
        sock = CookedSocket(sockstream)
        while True:
            try:
                req = await sock.recv()
            except json.JSONDecodeError:
                rep = {"status": "invalid_msg_format", "reason": "Invalid message format"}
                await sock.send(rep)
                continue

            if not req:  # Client disconnected
                logger.debug("CLIENT DISCONNECTED")
                return

            logger.debug("REQ {}", _filter_big_fields(req))

            rep = await dispatch_request(req)

            logger.debug("REP {}", _filter_big_fields(rep))

            await sock.send(rep)
    except trio.BrokenStreamError:
        # Client has closed connection
        pass
    except Exception as exc:
        print(exc)
        # If we are here, something unexpected happened...
        tb = traceback.format_exc()
        logger.error(tb)
        # TODO: do we need to close the socket (i.e. sockstream) here ?
        # or should we let the caller (most certainly the server) handle this ?
        await sock.aclose()
        raise


@attr.s
class CookedSocket:
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

        payload = ejson_dumps(req).encode() + b"\n"
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
            self._recv_buff += await self.sockstream.receive_some(self.BUFFSIZE)
            if not self._recv_buff:
                # Empty body should normally never occurs, though it is sent
                # when peer closes connection
                raise trio.BrokenStreamError("Peer has closed connection")

            *reps, unfinished_msg = self._recv_buff.split(b"\n")
            self._reps_ready += reps
            self._recv_buff = unfinished_msg
            if not self._reps_ready:
                # Return in the loop to do a new BUFFSIZE read on the socket
                continue

            else:
                next_rep = self._reps_ready.popleft()
                return json.loads(next_rep.decode())
