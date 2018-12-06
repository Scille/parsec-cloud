import pytest
import trio

from parsec.schema import UnknownCheckedSchema, fields
from parsec.api.transport import PatateTCPTransport


# TODO: basically a benchmark to showcase the performances issues with
# marshmallow/json serialization
@pytest.mark.slow
@pytest.mark.trio
async def test_big_buffer_bench():
    server_stream, client_stream = trio.testing.memory_stream_pair()

    server_transport = PatateTCPTransport(server_stream)
    client_transport = PatateTCPTransport(client_stream)

    class Schema(UnknownCheckedSchema):
        data = fields.Base64Bytes()

    schema = Schema(strict=True)

    # Base64 encoding of the bytes make the payload bigger once serialized
    roughly_max = int(PatateTCPTransport.MAX_MSG_SIZE * 2 / 3)
    payload = {"data": b"x" * roughly_max}

    for _ in range(10):
        await client_transport.send(schema.dump(payload).data)
        raw = await server_transport.recv()
        server_payload = schema.load(raw).data
        assert server_payload == payload
        del raw

        await server_transport.send(schema.dump(server_payload).data)
        raw = await client_transport.recv()
        roundtrip_payload = schema.load(raw).data
        assert roundtrip_payload == payload
        del raw
