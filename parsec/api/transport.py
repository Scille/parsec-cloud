from abc import ABC
from trio import BrokenResourceError
import struct

from parsec.utils import ejson_dumps, ejson_loads


__all__ = (
    "TransportError",
    "BrokenResourceError",
    "BaseTransport",
    "TCPTransport",
    "PatateTCPTransport",
)


class TransportError(Exception, ABC):
    pass


# Expose `trio.BrokenResourceError` as a child of `TransportError`
# Note we don't do the same for `trio.ClosedResourceError` given this
# exception should be only raised in case of programming error.
TransportError.register(BrokenResourceError)


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


class TCPTransport(BaseTransport):
    MAX_MSG_SIZE = 2 ** 20  # 1Mo

    def __init__(self, stream):
        self.stream = stream

    async def aclose(self) -> None:
        await self.stream.aclose()

    async def send(self, msg: bytes) -> None:
        assert len(msg) <= self.MAX_MSG_SIZE
        await self.stream.send_all(struct.pack("!L", len(msg)))
        await self.stream.send_all(msg)

    async def recv(self) -> bytes:
        msg_size_raw = await self.stream.receive_some(4)
        if not msg_size_raw:
            # Empty body should normally never occurs, though it is sent
            # when peer closes connection
            raise TransportError("Peer has closed connection")

        msg_size, = struct.unpack("!L", msg_size_raw)
        if msg_size > self.MAX_MSG_SIZE:
            raise TransportError("Message too big")

        msg = await self.stream.receive_some(msg_size)
        if not msg:
            # Empty body should normally never occurs, though it is sent
            # when peer closes connection
            raise TransportError("Peer has closed connection")

        return msg


# TODO: remove me !
class PatateTCPTransport(BaseTransport):
    MAX_MSG_SIZE = 2 ** 20  # 1Mo

    def __init__(self, stream):
        self.stream = stream

    async def aclose(self) -> None:
        await self.stream.aclose()

    async def send(self, msg: dict) -> None:
        msg = ejson_dumps(msg).encode("utf8")
        assert len(msg) <= self.MAX_MSG_SIZE
        await self.stream.send_all(struct.pack("!L", len(msg)))
        await self.stream.send_all(msg)

    async def recv(self) -> dict:
        msg_size_raw = await self.stream.receive_some(4)
        if not msg_size_raw:
            # Empty body should normally never occurs, though it is sent
            # when peer closes connection
            raise TransportError("Peer has closed connection")

        msg_size, = struct.unpack("!L", msg_size_raw)
        if msg_size > self.MAX_MSG_SIZE:
            raise TransportError("Message too big")

        msg = await self.stream.receive_some(msg_size)
        if not msg:
            # Empty body should normally never occurs, though it is sent
            # when peer closes connection
            raise TransportError("Peer has closed connection")

        return ejson_loads(msg.decode("utf8"))
