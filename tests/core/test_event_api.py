import pytest


@pytest.mark.trio
async def test_event_api_subscribe_and_receive(core, alice_core_sock):
    # TODO: should fail once event name filter is implemented
    await alice_core_sock.send({"cmd": "event_subscribe", "event": "foo", "subject": "bar"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    await alice_core_sock.send({"cmd": "event_listen"})
    core.signal_ns.signal("not me !").send("bar")
    core.signal_ns.signal("foo").send("not this one !")
    core.signal_ns.signal("foo").send("bar")
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok", "event": "foo", "subject": "bar"}

    await alice_core_sock.send({"cmd": "event_listen", "wait": False})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # Unsubscribe event
    await alice_core_sock.send({"cmd": "event_unsubscribe", "event": "foo", "subject": "bar"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # Nothing should be received now
    core.signal_ns.signal("not me !").send("bar")
    core.signal_ns.signal("foo").send("not this one !")
    core.signal_ns.signal("foo").send("bar")

    await alice_core_sock.send({"cmd": "event_listen", "wait": False})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}
