import pytest

from parsec.api.transport import PatateTCPTransport


@pytest.mark.trio
@pytest.mark.parametrize("bad_part", ["user_id", "device_name"])
async def test_handshake_unknown_device(bad_part, backend, server_factory, alice):
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = PatateTCPTransport(stream)
        if bad_part == "user_id":
            identity = "zack@dummy"
        else:
            identity = f"{alice.user_id}@dummy"

        await transport.recv()  # Get challenge
        await transport.send({"handshake": "answer", "identity": identity, "answer": "fooo"})
        result_req = await transport.recv()
        assert result_req == {"handshake": "result", "result": "bad_identity"}


@pytest.mark.trio
async def test_handshake_invalid_format(backend, server_factory):
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = PatateTCPTransport(stream)

        await transport.recv()  # Get challenge
        await transport.send({"handshake": "answer", "dummy": "field"})
        result_req = await transport.recv()
        assert result_req == {"handshake": "result", "result": "bad_format"}
