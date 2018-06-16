import pytest


@pytest.mark.trio
async def test_event_api_subscribe_bad_event(core, alice_core_sock):
    await alice_core_sock.send({"cmd": "event_subscribe", "event": "dummy"})
    rep = await alice_core_sock.recv()
    assert rep == {"errors": {"event": ["Not a valid choice."]}, "status": "bad_message"}


@pytest.mark.trio
async def test_event_api_unsubscribe_bad_event(core, alice_core_sock):
    await alice_core_sock.send({"cmd": "event_unsubscribe", "event": "dummy"})
    rep = await alice_core_sock.recv()
    assert rep == {"errors": {"event": ["Not a valid choice."]}, "status": "bad_message"}


@pytest.mark.trio
async def test_event_api_subscribe_and_receive(core, alice_core_sock):
    await alice_core_sock.send({"cmd": "event_subscribe", "event": "pinged", "subject": "foo"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    await alice_core_sock.send({"cmd": "event_listen"})
    core.signal_ns.signal("not me !").send(None, ping="foo")
    core.signal_ns.signal("pinged").send(None, ping="not this one !")
    core.signal_ns.signal("pinged").send(None, ping="foo")
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}

    await alice_core_sock.send({"cmd": "event_listen", "wait": False})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # Unsubscribe event
    await alice_core_sock.send({"cmd": "event_unsubscribe", "event": "pinged", "subject": "foo"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # Nothing should be received now
    core.signal_ns.signal("not me !").send(None, ping="foo")
    core.signal_ns.signal("pinged").send(None, ping="not this one !")
    core.signal_ns.signal("pinged").send(None, ping="foo")

    await alice_core_sock.send({"cmd": "event_listen", "wait": False})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}
