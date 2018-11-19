import pytest
import trio
import trio.testing
import json
from wsproto.connection import WSConnection, ConnectionType
from wsproto.events import ConnectionRequested, ConnectionEstablished

from hypothesis import given, strategies as st
from string import printable

from parsec.networking import CookedSocket, net_send


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
    ws_client = WSConnection(ConnectionType.CLIENT, host="localhost", resource="server")
    ws_server = WSConnection(ConnectionType.SERVER)
    cclient = CookedSocket(ws_client, rclient)
    cserver = CookedSocket(ws_server, rserver)

    ws_server.receive_bytes(ws_client.bytes_to_send())
    event = next(ws_server.events())
    assert isinstance(event, ConnectionRequested)
    ws_server.accept(event)
    ws_client.receive_bytes(ws_server.bytes_to_send())
    assert isinstance(next(ws_client.events()), ConnectionEstablished)

    await cclient.send(payload)
    server_payload = await cserver.recv()
    assert server_payload == payload

    await cserver.send(server_payload)
    roundtrip_payload = await cclient.recv()
    assert roundtrip_payload == payload


class TestCookedSocket:
    def setup_method(self):
        self.rserver, self.rclient = trio.testing.memory_stream_pair()
        self.ws_client = WSConnection(ConnectionType.CLIENT, host="localhost", resource="server")
        self.ws_server = WSConnection(ConnectionType.SERVER)

        self.ws_server.receive_bytes(self.ws_client.bytes_to_send())
        event = next(self.ws_server.events())
        assert isinstance(event, ConnectionRequested)
        self.ws_server.accept(event)
        self.ws_client.receive_bytes(self.ws_server.bytes_to_send())
        assert isinstance(next(self.ws_client.events()), ConnectionEstablished)

        self.cclient = CookedSocket(self.ws_client, self.rclient)
        self.cserver = CookedSocket(self.ws_server, self.rserver)

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
        self.cserver.ws.send_data(b"dummy\n")
        await net_send(self.cserver.ws, self.cserver.conn)
        with pytest.raises(json.JSONDecodeError):
            with trio.move_on_after(1):
                await self.cclient.recv()
