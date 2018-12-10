from trio import BrokenResourceError
import struct

from parsec.utils import ejson_dumps, ejson_loads


__all__ = ("TransportError", "BaseTransport", "TCPTransport", "PatateTCPTransport")


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

    async def recv(self) -> bytes:
        try:
            msg_size_raw = await self.stream.receive_some(4)
        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc
        if not msg_size_raw:
            # Empty body should normally never occurs, though it is sent
            # when peer closes connection
            raise TransportError("Peer has closed connection")

        msg_size, = struct.unpack("!L", msg_size_raw)
        if msg_size > self.MAX_MSG_SIZE:
            raise TransportError("Message too big")

        try:
            msg = await self.stream.receive_some(msg_size)
        except BrokenResourceError as exc:
            raise TransportError(*exc.args) from exc
        if not msg:
            # Empty body should normally never occurs, though it is sent
            # when peer closes connection
            raise TransportError("Peer has closed connection")

        return msg


# TODO: remove me !
class PatateTCPTransport(TCPTransport):
    async def send(self, msg: dict) -> None:
        try:
            msg = ejson_dumps(msg).encode("utf8")
        except Exception as exc:
            raise TransportError("Cannot serialize data.") from exc
        await super().send(msg)

    async def recv(self) -> dict:
        msg = await super().recv()
        try:
            return ejson_loads(msg.decode("utf8"))
        except Exception as exc:
            raise TransportError("Cannot deserialize data.") from exc
