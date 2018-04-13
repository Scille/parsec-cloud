import pytest

from tests.common import connect_core, run_app


@pytest.mark.trio
async def test_connection(core):
    async with connect_core(core) as sock:
        await sock.send({"cmd": "get_core_state"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "login": None, "backend_online": False}
    # Deconnection, then reco
    async with connect_core(core) as sock:
        await sock.send({"cmd": "get_core_state"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "login": None, "backend_online": False}


@pytest.mark.trio
async def test_multi_connections(core):
    async with connect_core(core) as sock1:
        async with connect_core(core) as sock2:

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
async def test_offline_login_and_logout(backend, core, mallory):
    # Backend is not available, hence we can use a user not registered in it
    core.devices_manager.register_new_device(
        mallory.id,
        mallory.user_privkey.encode(),
        mallory.device_signkey.encode(),
        "<secret>",
    )

    async with connect_core(core) as sock:
        await sock.send({"cmd": "info"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "loaded": False, "id": None}
        # Do the login
        await sock.send({"cmd": "login", "id": mallory.id, "password": "<secret>"})
        rep = await sock.recv()
        assert rep == {"status": "ok"}

        await sock.send({"cmd": "info"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "loaded": True, "id": mallory.id}

    # Changing socket should not trigger logout
    async with connect_core(core) as sock:
        await sock.send({"cmd": "info"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "loaded": True, "id": mallory.id}
        # Actual logout
        await sock.send({"cmd": "logout"})
        rep = await sock.recv()
        assert rep == {"status": "ok"}
        await sock.send({"cmd": "info"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "loaded": False, "id": None}

    # Startup backend and check exception is triggered given logged user is unknown
    async with run_app(backend):
        async with connect_core(core) as sock:
            await sock.send({"cmd": "login", "id": mallory.id, "password": "<secret>"})
            rep = await sock.recv()
            assert rep == {"status": "ok"}
            await sock.send({"cmd": "get_core_state"})
            rep = await sock.recv()
            assert (
                rep
                == {"status": "ok", "login": "mallory@test", "backend_online": False}
            )


@pytest.mark.trio
async def test_login_and_logout(core, alice):
    async with connect_core(core) as sock:
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
    async with connect_core(core) as sock:
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
async def test_need_login_cmds(core):
    async with connect_core(core) as sock:
        for cmd in [
            "logout",
            "file_create",
            "file_read",
            "file_write",
            "synchronize",
            "flush",
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
async def test_bad_cmd(alice_core_sock):
    await alice_core_sock.send({"cmd": "dummy"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "unknown_command", "reason": "Unknown command `dummy`"}


@pytest.mark.trio
async def test_bad_msg_format(alice_core_sock):
    await alice_core_sock.sockstream.send_all(b"fooo\n")
    rep = await alice_core_sock.recv()
    assert rep == {"status": "invalid_msg_format", "reason": "Invalid message format"}
