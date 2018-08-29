import pytest


@pytest.mark.trio
async def test_connection(core, core_sock_factory):
    async with core_sock_factory(core) as sock:
        await sock.send({"cmd": "get_core_state"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "login": None, "backend_online": False}

    # Deconnection, then reco
    async with core_sock_factory(core) as sock:
        await sock.send({"cmd": "get_core_state"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "login": None, "backend_online": False}


@pytest.mark.trio
async def test_multi_connections(core, core_sock_factory):
    async with core_sock_factory(core) as sock1:
        async with core_sock_factory(core) as sock2:

            await sock1.send({"cmd": "get_core_state"})
            await sock2.send({"cmd": "get_core_state"})

            rep1 = await sock1.recv()
            rep2 = await sock2.recv()

            assert rep1 == {"status": "ok", "login": None, "backend_online": False}
            assert rep2 == {"status": "ok", "login": None, "backend_online": False}

        # Sock 1 should not have been affected by sock 2 leaving
        await sock1.send({"cmd": "get_core_state"})
        rep = await sock1.recv()
        assert rep == {"status": "ok", "login": None, "backend_online": False}


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


@pytest.mark.trio
async def test_bad_cmd(alice_core_sock, monitor):
    await alice_core_sock.send({"cmd": "dummy"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "unknown_command", "reason": "Unknown command `dummy`"}


@pytest.mark.trio
async def test_bad_msg_format(alice_core_sock):
    await alice_core_sock.sockstream.send_all(b"fooo\n")
    rep = await alice_core_sock.recv()
    assert rep == {"status": "invalid_msg_format", "reason": "Invalid message format"}
