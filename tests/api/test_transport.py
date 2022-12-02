# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from functools import partial

import pytest
import trio

from parsec.api.protocol.base import MsgpackSerializer
from parsec.api.transport import Transport, TransportClosedByPeer
from parsec.serde import BaseSchema, fields


async def serve_tcp_testbed(*conns):
    host = "127.0.0.1"
    send_channel, receive_channel = trio.open_memory_channel(0)

    async def _serve_client(stream):
        server_fn = await receive_channel.receive()
        transport = await Transport.init_for_server(stream)
        await server_fn(transport)

    async def _store_handlers(*, task_status=trio.TASK_STATUS_IGNORED):
        async with trio.open_service_nursery() as handler_nursery:
            task_status.started(handler_nursery)
            await trio.sleep_forever()

    async with trio.open_service_nursery() as nursery:
        handler_nursery = await nursery.start(_store_handlers)
        listeners = await nursery.start(
            partial(
                trio.serve_tcp,
                _serve_client,
                port=0,
                host="127.0.0.1",
                handler_nursery=handler_nursery,
            )
        )
        _, port, *_ = listeners[0].socket.getsockname()
        assert not handler_nursery.child_tasks

        for client_fn, server_fn in conns:
            stream = await trio.open_tcp_stream(host, port)
            await send_channel.send(server_fn)

            transport = await Transport.init_for_client(stream, host)
            await client_fn(transport)

            await trio.testing.wait_all_tasks_blocked()
            # No pending connections should remain
            assert not handler_nursery.child_tasks

        await send_channel.aclose()
        nursery.cancel_scope.cancel()


@pytest.mark.trio
@pytest.mark.parametrize("closing_end", ["client", "server"])
async def test_no_transport_leaks_one_end_close(closing_end):
    async def closing_end_fn(transport):
        with pytest.raises(TransportClosedByPeer):
            await transport.recv()
        await transport.aclose()

    async def listening_end_fn(transport):
        await transport.aclose()

    if closing_end == "server":
        await serve_tcp_testbed((listening_end_fn, closing_end_fn))
    else:
        await serve_tcp_testbed((closing_end_fn, listening_end_fn))


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
        client_transport = await Transport.init_for_client(
            client_stream, host=backend_addr.hostname
        )

    async with trio.open_service_nursery() as nursery:
        nursery.start_soon(_boot_client)
        nursery.start_soon(_boot_server)

    class Schema(BaseSchema):
        data = fields.Bytes()

    schema = MsgpackSerializer(Schema)

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


# TODO: test websocket can work with message sent across mutiple TCP frames
