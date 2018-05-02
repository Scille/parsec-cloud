import pytest


@pytest.mark.trio
async def test_event_api_subscribe_bad_event(core, alice_core_sock):
    await alice_core_sock.send({"cmd": "event_subscribe", "event": "dummy"})
    rep = await alice_core_sock.recv()
    assert rep == {'errors': {'event': ['Not a valid choice.']}, 'status': 'bad_message'}


@pytest.mark.trio
async def test_event_api_unsubscribe_bad_event(core, alice_core_sock):
    await alice_core_sock.send({"cmd": "event_unsubscribe", "event": "dummy"})
    rep = await alice_core_sock.recv()
    assert rep == {'errors': {'event': ['Not a valid choice.']}, 'status': 'bad_message'}


@pytest.mark.trio
async def test_event_api_subscribe_and_receive(core, alice_core_sock):
    await alice_core_sock.send({"cmd": "event_subscribe", "event": "ping", "subject": "foo"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    await alice_core_sock.send({"cmd": "event_listen"})
    core.signal_ns.signal("not me !").send("foo")
    core.signal_ns.signal("ping").send("not this one !")
    core.signal_ns.signal("ping").send("foo")
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok", "event": "ping", "subject": "foo"}

    await alice_core_sock.send({"cmd": "event_listen", "wait": False})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # Unsubscribe event
    await alice_core_sock.send({"cmd": "event_unsubscribe", "event": "ping", "subject": "foo"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # Nothing should be received now
    core.signal_ns.signal("not me !").send("foo")
    core.signal_ns.signal("ping").send("not this one !")
    core.signal_ns.signal("ping").send("foo")

    await alice_core_sock.send({"cmd": "event_listen", "wait": False})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}
