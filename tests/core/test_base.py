import pytest


@pytest.mark.trio
async def test_offline_login_and_logout(
    server_factory, backend, backend_addr, device_factory, core_factory, core_sock_factory
):
    # Backend is not available, hence we can use a user not registered in it
    device = device_factory()
    async with core_factory(devices=[device]) as core:

        async with core_sock_factory(core) as sock:
            await sock.send({"cmd": "info"})
            rep = await sock.recv()
            assert rep == {"status": "ok", "loaded": False, "id": None}
            # Do the login
            await sock.send({"cmd": "login", "id": device.id, "password": "<secret>"})
            rep = await sock.recv()
            assert rep == {"status": "ok"}

            await sock.send({"cmd": "info"})
            rep = await sock.recv()
            assert rep == {"status": "ok", "loaded": True, "id": device.id}

        # Changing socket should not trigger logout
        async with core_sock_factory(core) as sock:

            await sock.send({"cmd": "info"})
            rep = await sock.recv()
            assert rep == {"status": "ok", "loaded": True, "id": device.id}
            # Actual logout
            await sock.send({"cmd": "logout"})
            rep = await sock.recv()
            assert rep == {"status": "ok"}
            await sock.send({"cmd": "info"})
            rep = await sock.recv()
            assert rep == {"status": "ok", "loaded": False, "id": None}

        # Startup backend and check exception is triggered given logged user is unknown
        server_factory(backend.handle_client, backend_addr)
        async with core_sock_factory(core) as sock:
            await sock.send({"cmd": "login", "id": device.id, "password": "<secret>"})
            rep = await sock.recv()
            assert rep == {"status": "ok"}
            await sock.send({"cmd": "get_core_state"})
            rep = await sock.recv()
            assert rep == {"status": "ok", "login": device.id, "backend_online": False}


@pytest.mark.trio
async def test_login_and_logout(alice, core, core_sock_factory):

    async with core_sock_factory(core) as sock:
        await sock.send({"cmd": "info"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "loaded": False, "id": None}
        # Do the login
        await sock.send({"cmd": "login", "id": alice.id, "password": "<secret>"})
        rep = await sock.recv()
        assert rep == {"status": "ok"}

        await sock.send({"cmd": "info"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "loaded": True, "id": alice.id}

    # Changing socket should not trigger logout
    async with core_sock_factory(core) as sock:
        await sock.send({"cmd": "info"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "loaded": True, "id": alice.id}
        # Actual logout
        await sock.send({"cmd": "logout"})
        rep = await sock.recv()
        assert rep == {"status": "ok"}
        await sock.send({"cmd": "info"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "loaded": False, "id": None}


@pytest.mark.trio
async def test_need_login_cmds(core, core_sock_factory):
    async with core_sock_factory(core) as sock:
        for cmd in [
            "logout",
            "file_create",
            "file_read",
            "file_write",
            "synchronize",
            "stat",
            "folder_create",
            "move",
            "delete",
            "file_truncate",
        ]:
            await sock.send({"cmd": cmd})
            rep = await sock.recv()
            assert rep == {"status": "login_required", "reason": "Login required"}
