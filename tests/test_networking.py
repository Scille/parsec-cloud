import pytest
import trio
import trio.testing
import json

# from hypothesis import given, strategies as st
# from string import printable; from pprint import pprint

from parsec.networking import CookedSocket

from tests.open_tcp_stream_mock_wrapper import offline


# TODO: use hypothesis once @given support async functions
# json_nested = st.recursive(st.none() | st.booleans() | st.floats() | st.text(printable),
#     lambda children: st.lists(children) | st.dictionaries(st.text(printable), children))
# json = st.dictionaries(st.text(printable), json_nested)


class TestCookedSocket:

    def setup_method(self):
        self.rserver, self.rclient = trio.testing.memory_stream_pair()
        self.cclient = CookedSocket(self.rclient)
        self.cserver = CookedSocket(self.rserver)

    @pytest.mark.trio
    @pytest.mark.parametrize(
        "payload",
        [{}, {"foo": 42, "spam": None}, {"foo": "bar", "spam": [1, "two", {"three": True}]}],
    )
    async def test_base(self, payload):
        await self.cclient.send(payload)
        server_payload = await self.cserver.recv()
        assert server_payload == payload

        await self.cserver.send(server_payload)
        roundtrip_payload = await self.cclient.recv()
        assert roundtrip_payload == payload

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
