from trio import BrokenResourceError
import struct


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


# TODO: remove me !
class PatateTCPTransport(TCPTransport):
    async def send(self, msg: dict) -> None:
        # try:
        #     msg = ejson_dumps(msg).encode("utf8")
        # except Exception as exc:
        #     raise TransportError("Cannot serialize data.") from exc
        await super().send(msg)

    async def recv(self) -> dict:
        msg = await super().recv()
        return msg
        # try:
        #     return ejson_loads(msg.decode("utf8"))
        # except Exception as exc:
        #     raise TransportError("Cannot deserialize data.") from exc
