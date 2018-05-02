import pytest
import trio
import trio.testing
import json

from hypothesis import given, strategies as st
from string import printable

from parsec.networking import CookedSocket


json_nested = st.recursive(
    st.none()
    | st.booleans()
    | st.floats(allow_nan=False, allow_infinity=False)
    | st.text(printable),
    lambda children: st.lists(children) | st.dictionaries(st.text(printable), children),
)
json_dict = st.dictionaries(st.text(printable), json_nested)


# TODO: Improve this (see https://github.com/python-trio/pytest-trio/issues/42)
class TestCookedSocketHypothesis:

    @pytest.mark.slow
    @given(json_dict)
    async def test_cooked_socket_communication(self, payload):
        rserver, rclient = trio.testing.memory_stream_pair()
        cclient = CookedSocket(rclient)
        cserver = CookedSocket(rserver)

        await cclient.send(payload)
        server_payload = await cserver.recv()
        assert server_payload == payload

        await cserver.send(server_payload)
        roundtrip_payload = await cclient.recv()
        assert roundtrip_payload == payload

    def execute_example(self, f):
        # Nothing but a dirty hack...
        from unittest.mock import Mock

        node = Mock()
        node.function = f
        from pytest_trio.plugin import _trio_test_runner_factory

        return _trio_test_runner_factory(node)()


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
