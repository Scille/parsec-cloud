"""
Defines client/server communication protocol on top of trio's Stream.
"""

import attr
import trio
import json
import logbook
import traceback
from collections import deque


logger = logbook.Logger("parsec.networking")


@attr.s
class ClientContext:

    @property
    def ctxid(self):
        return id(self)

    registered_events = attr.ib(default=attr.Factory(dict))
    pending_events = attr.ib(default=attr.Factory(dict))


async def serve_client(dispatch_request, sockstream) -> None:
    try:
        ctx = ClientContext()
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

            logger.debug("REQ {}", req)
            rep = await dispatch_request(req, ctx)
            logger.debug("REP {}", rep)
            await sock.send(rep)
    except trio.BrokenStreamError:
        # Client has closed connection
        pass
    except Exception:
        # If we are here, something unexpected happened...
        logger.error(traceback.format_exc())
        # TODO: do we need to close the socket (i.e. sockstream) here ?
        # or should we let the caller (most certainly the server) handle this ?
        await sock.aclose()
        raise


@attr.s
class CookedSocket:
    BUFFSIZE = 4049

    sockstream = attr.ib()
    _recv_buff = attr.ib(default=attr.Factory(bytearray))
    _msgs_ready = attr.ib(default=attr.Factory(deque))

    async def aclose(self):
        await self.sockstream.aclose()

    async def send(self, msg):
        await self.sockstream.send_all(json.dumps(msg).encode() + b"\n")

    async def recv(self):
        while True:
            self._recv_buff += await self.sockstream.receive_some(self.BUFFSIZE)
            if not self._recv_buff:
                # Empty body should normally never occurs, though it is sent
                # when peer closes connection
                raise trio.BrokenStreamError("Peer has closed connection")

            *msgs, unfinished_msg = self._recv_buff.split(b"\n")
            self._msgs_ready += msgs
            self._recv_buff = unfinished_msg
            if not self._msgs_ready:
                # Return in the loop to do a new BUFFSIZE read on the socket
                continue

            else:
                next_msg = self._msgs_ready.popleft()
                return json.loads(next_msg.decode())
