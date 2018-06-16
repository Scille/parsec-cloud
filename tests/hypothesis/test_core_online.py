import pytest
from hypothesis import note
from hypothesis.stateful import rule


@pytest.mark.slow
@pytest.mark.trio
async def test_online(
    TrioDriverRuleBasedStateMachine,
    server_factory,
    backend_factory,
    core_factory,
    core_sock_factory,
    device_factory,
):
    class CoreOnline(TrioDriverRuleBasedStateMachine):
        async def trio_runner(self, task_status):
            self.core_cmd = self.communicator.send
            self.device = device_factory()
            self.backend = await backend_factory(devices=[self.device])
            server = server_factory(self.backend.handle_client)
            self.core = await core_factory(
                devices=[self.device], config={"backend_addr": server.addr}
            )
            try:
                await self.core.login(self.device)
                sock = core_sock_factory(self.core)
                task_status.started()

                while True:
                    msg = await self.communicator.trio_recv()
                    await sock.send(msg)
                    rep = await sock.recv()
                    await self.communicator.trio_respond(rep)

            finally:
                await self.core.teardown()

        @rule()
        def get_core_state(self):
            rep = self.core_cmd({"cmd": "get_core_state"})
            note(rep)
            assert rep == {"status": "ok", "login": self.device.id, "backend_online": True}

    await CoreOnline.run_test()
