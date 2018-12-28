import pytest

from parsec.api.protocole import packb, unpackb
from parsec.api.transport import WebsocketTransport


@pytest.mark.trio
@pytest.mark.parametrize("bad_part", ["user_id", "device_name"])
async def test_handshake_unknown_device(bad_part, backend, server_factory, alice):
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await WebsocketTransport.init_for_client(stream, "foo", "bar")
        if bad_part == "user_id":
            identity = "zack@dummy"
        else:
            identity = f"{alice.user_id}@dummy"

        await transport.recv()  # Get challenge
        await transport.send(
            packb({"handshake": "answer", "identity": identity, "answer": b"fooo"})
        )
        result_req = await transport.recv()
        assert unpackb(result_req) == {"handshake": "result", "result": "bad_identity"}


@pytest.mark.trio
async def test_handshake_invalid_format(backend, server_factory):
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await WebsocketTransport.init_for_client(stream, "foo", "bar")

        await transport.recv()  # Get challenge
        req = {"handshake": "answer", "dummy": "field"}
        await transport.send(packb(req))
        result_req = await transport.recv()
        assert unpackb(result_req) == {"handshake": "result", "result": "bad_format"}
