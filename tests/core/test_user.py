import pytest


from tests.common import freeze_time


@pytest.mark.trio
async def test_user_invite_not_logged(core, core_sock_factory):
    sock = core_sock_factory(core)
    await sock.send({"cmd": "user_invite", "user_id": "John"})
    rep = await sock.recv()
    assert rep == {"status": "login_required", "reason": "Login required"}


@pytest.mark.trio
async def test_user_invite_backend_offline(core, alice_core_sock):
    await alice_core_sock.send({"cmd": "user_invite", "user_id": "John"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "backend_not_available", "reason": "Backend not available"}


@pytest.mark.trio
async def test_user_invite_then_claim_ok(
    running_backend, core_factory, core_sock_factory, alice_core_sock
):
    await alice_core_sock.send({"cmd": "user_invite", "user_id": "mallory"})
    rep = await alice_core_sock.recv()
    assert rep["status"] == "ok"
    assert rep["user_id"] == "mallory"
    assert rep["invitation_token"]
    invitation_token = rep["invitation_token"]

    # Create a brand new core and try to use the token to register
    new_core = await core_factory(devices=[])
    new_core_sock = core_sock_factory(new_core)
    await new_core_sock.send(
        {
            "cmd": "user_claim",
            "id": "mallory@device1",
            "invitation_token": invitation_token,
            "password": "S3cr37",
        }
    )
    rep = await new_core_sock.recv()
    assert rep == {"status": "ok"}

    # Recreate the core (but share config with the previous one !)
    restarted_new_core = await core_factory(
        devices=[], config={"base_settings_path": new_core.config.base_settings_path}
    )

    # Finally make sure we can login as the new user
    restarted_new_core_sock = core_sock_factory(restarted_new_core)
    await restarted_new_core_sock.send(
        {"cmd": "login", "id": "mallory@device1", "password": "S3cr37"}
    )
    rep = await restarted_new_core_sock.recv()
    assert rep == {"status": "ok"}

    await restarted_new_core_sock.send({"cmd": "info"})
    rep = await restarted_new_core_sock.recv()
    assert rep == {"status": "ok", "id": "mallory@device1", "loaded": True}


@pytest.mark.trio
async def test_user_invite_then_claim_timeout(
    running_backend, core_factory, core_sock_factory, alice_core_sock
):
    with freeze_time("2017-01-01"):
        await alice_core_sock.send({"cmd": "user_invite", "user_id": "mallory"})
        rep = await alice_core_sock.recv()
    assert rep["status"] == "ok"
    assert rep["user_id"] == "mallory"
    assert rep["invitation_token"]
    invitation_token = rep["invitation_token"]

    # Create a brand new core and try to use the token to register
    new_core = await core_factory(devices=[])
    new_core_sock = core_sock_factory(new_core)
    with freeze_time("2017-01-02"):
        await new_core_sock.send(
            {
                "cmd": "user_claim",
                "id": "mallory@device1",
                "invitation_token": invitation_token,
                "password": "S3cr37",
            }
        )
        rep = await new_core_sock.recv()
    assert rep == {"status": "out_of_date_error", "reason": "Claim code is too old."}
