import pytest
from hypothesis import note
from hypothesis_trio.stateful import run_state_machine_as_test

from tests.hypothesis.common import rule


@pytest.mark.slow
def test_online(tcp_stream_spy, BaseCoreWithBackendStateMachine, hypothesis_settings):
    class CoreOnline(BaseCoreWithBackendStateMachine):
        backend_online = True

        @rule()
        async def get_core_state(self):
            print("get state...")
            rep = await self.core_cmd({"cmd": "get_core_state"})
            note(rep)
            print(rep)
            assert rep == {
                "status": "ok",
                "login": self.device.id,
                "backend_online": self.backend_online,
            }

        @rule()
        async def switch_backend_availability(self):
            if self.backend_online:
                tcp_stream_spy.switch_offline(self.backend_info.server.addr)
            else:
                tcp_stream_spy.switch_online(self.backend_info.server.addr)
            self.backend_online = not self.backend_online

    run_state_machine_as_test(CoreOnline, settings=hypothesis_settings)
