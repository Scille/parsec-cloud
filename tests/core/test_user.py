import pytest


from tests.common import connect_core, core_factory, freeze_time


@pytest.mark.trio
async def test_user_invite_not_logged(core):
    async with connect_core(core) as sock:
        await sock.send({"cmd": "user_invite", "user_id": "John"})
        rep = await sock.recv()
    assert rep == {"status": "login_required", "reason": "Login required"}


@pytest.mark.trio
async def test_user_invite_backend_offline(core, alice_core_sock):
    await alice_core_sock.send({"cmd": "user_invite", "user_id": "John"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "backend_not_availabled", "reason": "Backend not available"}


@pytest.mark.trio
async def test_user_invite_then_claim_ok(
    tmpdir, backend_addr, running_backend, alice_core_sock, mallory
):
    await alice_core_sock.send({"cmd": "user_invite", "user_id": mallory.user_id})
    rep = await alice_core_sock.recv()
    assert rep["status"] == "ok"
    assert rep["user_id"] == "mallory"
    assert rep["invitation_token"]
    invitation_token = rep["invitation_token"]

    # Create a brand new core and try to use the token to register
    mallory_core_conf_dir = tmpdir.mkdir("mallory_core")
    async with core_factory(
        base_settings_path=mallory_core_conf_dir, backend_addr=backend_addr
    ) as mallory_core:

        async with connect_core(mallory_core) as mallory_core_sock:
            await mallory_core_sock.send(
                {
                    "cmd": "user_claim",
                    "id": "mallory@device1",
                    "invitation_token": invitation_token,
                    "password": "S3cr37",
                }
            )
            rep = await mallory_core_sock.recv()
            assert rep == {"status": "ok"}

    # Recreate the core (but share config with the previous one !)
    async with core_factory(
        base_settings_path=mallory_core_conf_dir, backend_addr=backend_addr
    ) as mallory_core:

        # Finally make sure we can login as the new user
        async with connect_core(mallory_core) as mallory_core_sock:
            await mallory_core_sock.send(
                {"cmd": "login", "id": "mallory@device1", "password": "S3cr37"}
            )
            rep = await mallory_core_sock.recv()
            assert rep == {"status": "ok"}

            await mallory_core_sock.send({"cmd": "info"})
            rep = await mallory_core_sock.recv()
            assert rep == {"status": "ok", "id": "mallory@device1", "loaded": True}


@pytest.mark.trio
async def test_user_invite_then_claim_timeout(
    tmpdir, backend_addr, running_backend, alice_core_sock, mallory
):
    with freeze_time("2017-01-01"):
        await alice_core_sock.send({"cmd": "user_invite", "user_id": mallory.user_id})
        rep = await alice_core_sock.recv()
    assert rep["status"] == "ok"
    assert rep["user_id"] == "mallory"
    assert rep["invitation_token"]
    invitation_token = rep["invitation_token"]

    # Create a brand new core and try to use the token to register
    async with core_factory(
        base_settings_path=tmpdir.mkdir("mallory_core"), backend_addr=backend_addr
    ) as mallory_core:
        # Create a brand new core and try to use the token to register
        async with connect_core(mallory_core) as mallory_core_sock:
            with freeze_time("2017-01-02"):
                await mallory_core_sock.send(
                    {
                        "cmd": "user_claim",
                        "id": "mallory@device1",
                        "invitation_token": invitation_token,
                        "password": "S3cr37",
                    }
                )
                rep = await mallory_core_sock.recv()
            assert rep == {"status": "out_of_date_error", "reason": "Claim code is too old."}
