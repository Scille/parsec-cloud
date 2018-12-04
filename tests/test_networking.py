import pytest
import trio
import trio.testing
import json

from hypothesis import given, strategies as st
from string import printable

from parsec.networking import CookedSocket
from parsec.schema import UnknownCheckedSchema, fields


json_nested_strategy = st.recursive(
    st.none()
    | st.booleans()
    | st.floats(allow_nan=False, allow_infinity=False)
    | st.text(printable),
    lambda children: st.lists(children, max_size=3)
    | st.dictionaries(st.text(printable), children, max_size=3),
)
json_dict_strategy = st.dictionaries(st.text(printable), json_nested_strategy)


@pytest.mark.slow
@pytest.mark.trio
@given(payload=json_dict_strategy)
async def test_cooked_socket_communication(payload):
    rserver, rclient = trio.testing.memory_stream_pair()
    cclient = CookedSocket(rclient)
    cserver = CookedSocket(rserver)

    await cclient.send(payload)
    server_payload = await cserver.recv()
    assert server_payload == payload

    await cserver.send(server_payload)
    roundtrip_payload = await cclient.recv()
    assert roundtrip_payload == payload


class TestCookedSocket:
    def setup_method(self):
        self.rserver, self.rclient = trio.testing.memory_stream_pair()
        self.cclient = CookedSocket(self.rclient)
        self.cserver = CookedSocket(self.rserver)

    @pytest.mark.trio
    async def test_peer_diconnected_during_send(self):
        await self.rserver.aclose()
        with pytest.raises(trio.BrokenStreamError):
            with trio.move_on_after(1):
                await self.cclient.send({"foo": "bar"})

    @pytest.mark.trio
    async def test_peer_diconnected_during_recv(self):
        await self.rserver.aclose()
        with pytest.raises(trio.BrokenStreamError):
            with trio.move_on_after(1):
                await self.cclient.recv()

    @pytest.mark.trio
    @pytest.mark.parametrize(
        "payload", [None, object(), [], {"foo": 42, "bar": {"spam": object()}}]
    )
    async def test_send_invalid_message(self, payload):
        with pytest.raises(TypeError):
            with trio.move_on_after(1):
                await self.cclient.send(payload)

    @pytest.mark.trio
    async def test_receive_invalid_message(self):
        await self.rserver.send_all(b"dummy\n")
        with pytest.raises(json.JSONDecodeError):
            with trio.move_on_after(1):
                await self.cclient.recv()


@pytest.mark.slow
@pytest.mark.trio
async def test_big_buffer_bench():
    rserver, rclient = trio.testing.memory_stream_pair()
    cclient = CookedSocket(rclient)
    cserver = CookedSocket(rserver)
    MAX_MSG_SIZE = 2 ** 20

    class Schema(UnknownCheckedSchema):
        data = fields.Base64Bytes()

    schema = Schema(strict=True)

    payload = {"data": b"x" * (MAX_MSG_SIZE - 50)}

    for _ in range(10):
        await cclient.send(schema.dump(payload).data)
        raw = await cserver.recv()
        server_payload = schema.load(raw).data
        assert server_payload == payload
        del raw

        await cserver.send(schema.dump(server_payload).data)
        raw = await cclient.recv()
        roundtrip_payload = schema.load(raw).data
        assert roundtrip_payload == payload
        del raw
