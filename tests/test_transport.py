import pytest
import trio

from parsec.schema import UnknownCheckedSchema, fields
from parsec.api.transport import Transport
from parsec.api.protocole.base import Serializer


# TODO: basically a benchmark to showcase the performances issues with
# marshmallow/json serialization
@pytest.mark.slow
@pytest.mark.trio
async def test_big_buffer_bench(backend_addr):
    server_stream, client_stream = trio.testing.memory_stream_pair()

    client_transport = None
    server_transport = None

    async def _boot_server():
        nonlocal server_transport
        server_transport = await Transport.init_for_server(server_stream)

    async def _boot_client():
        nonlocal client_transport
        client_transport = await Transport.init_for_client(client_stream, backend_addr)

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_boot_client)
        nursery.start_soon(_boot_server)

    class Schema(UnknownCheckedSchema):
        data = fields.Bytes()

    schema = Serializer(Schema)

    # Base64 encoding of the bytes make the payload bigger once serialized
    # roughly_max = int(TCPTransport.MAX_MSG_SIZE * 2 / 3)
    roughly_max = int(Transport.RECEIVE_BYTES * 2 / 3)
    payload = {"data": b"x" * roughly_max}

    for _ in range(10):
        await client_transport.send(schema.dumps(payload))
        raw = await server_transport.recv()
        server_payload = schema.loads(raw)
        assert server_payload == payload
        del raw

        await server_transport.send(schema.dumps(server_payload))
        raw = await client_transport.recv()
        roundtrip_payload = schema.loads(raw)
        assert roundtrip_payload == payload
        del raw
