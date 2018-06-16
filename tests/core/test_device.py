import pytest
import trio


@pytest.mark.trio
@pytest.mark.parametrize(
    "cmd",
    [
        {"cmd": "device_declare", "device_name": "device2"},
        {
            "cmd": "device_configure",
            "device_id": "alice@device2",
            "password": "S3cr37",
            "configure_device_token": "123456",
        },
        {"cmd": "device_accept_configuration_try", "config_try_id": "123456"},
    ],
)
async def test_device_cmd_backend_offline(core, alice_core_sock, cmd):
    await alice_core_sock.send(cmd)
    rep = await alice_core_sock.recv()
    assert rep == {"status": "backend_not_availabled", "reason": "Backend not available"}


@pytest.mark.trio
async def test_device_declare_then_accepted(
    tmpdir, running_backend, core_factory, core_sock_factory, alice
):
    # -1) Given the core is initialized while the backend is online, we are
    # guaranteed the two are connected
    core = await core_factory()
    await core.login(alice)
    alice_core_sock = core_sock_factory(core)

    # 0) Initial device declare the new device

    await alice_core_sock.send({"cmd": "device_declare", "device_name": "device2"})
    rep = await alice_core_sock.recv()
    assert rep["status"] == "ok"
    configure_device_token = rep["configure_device_token"]

    # 1) Existing device start listening for device configuration

    print("start listening")
    await alice_core_sock.send({"cmd": "event_subscribe", "event": "device_try_claim_submitted"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}
    print("listening ok")

    # 2) Wannabe device spawn core and start configuration

    new_device_core = await core_factory(devices=[])
    new_device_core_sock = core_sock_factory(new_device_core)

    await new_device_core_sock.send(
        {
            "cmd": "device_configure",
            "device_id": "alice@device2",
            "password": "S3cr37",
            "configure_device_token": configure_device_token,
        }
    )

    # Here new_device_core should be on hold, waiting for existing device to
    # accept/refuse his configuration try

    # 3) Existing device receive configuration event

    await alice_core_sock.send({"cmd": "event_listen"})
    with trio.fail_after(1):
        rep = await alice_core_sock.recv()
    assert rep["status"] == "ok"
    assert rep["event"] == "device_try_claim_submitted"
    assert rep["device_name"] == "device2"
    assert rep["config_try_id"]

    config_try_id = rep["config_try_id"]

    # 4) Existing device accept configuration

    await alice_core_sock.send(
        {"cmd": "device_accept_configuration_try", "config_try_id": config_try_id}
    )
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # 5) Wannabe device get it answer: device has been accepted !

    with trio.fail_after(1):
        rep = await new_device_core_sock.recv()
    assert rep["status"] == "ok"

    # Device config should have been stored on local storage so restarting
    # core is not a trouble

    restarted_new_device_core = await core_factory(
        devices=[], config={"base_settings_path": new_device_core.config.base_settings_path}
    )
    restarted_new_device_core_sock = core_sock_factory(restarted_new_device_core)

    # 6) Now wannabe device can login as alice

    rep = await restarted_new_device_core_sock.send({"cmd": "list_available_logins"})
    rep = await restarted_new_device_core_sock.recv()
    assert rep == {"status": "ok", "devices": ["alice@device2"]}

    rep = await restarted_new_device_core_sock.send(
        {"cmd": "login", "id": "alice@device2", "password": "S3cr37"}
    )
    rep = await restarted_new_device_core_sock.recv()
    assert rep == {"status": "ok"}

    rep = await restarted_new_device_core_sock.send({"cmd": "info"})
    rep = await restarted_new_device_core_sock.recv()
    assert rep == {"status": "ok", "loaded": True, "id": "alice@device2"}
