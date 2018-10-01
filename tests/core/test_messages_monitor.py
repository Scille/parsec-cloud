import trio


async def test_new_sharing_trigger_event(alice_core, bob_core2):
    await alice_core.event_bus.spy.wait_for_backend_connection_ready()
    await bob_core2.event_bus.spy.wait_for_backend_connection_ready()

    # First, create a folder and sync it on backend
    await alice_core.fs.workspace_create("/foo")
    await alice_core.fs.sync("/foo")

    # Now we can share this workspace with Bob
    await alice_core.fs.share("/foo", recipient="bob")

    # Bob should get a notification
    with trio.fail_after(seconds=1):
        await bob_core2.event_bus.spy.wait(
            "sharing.new",
            kwargs={"path": f"/foo (shared by alice)", "access": bob_core2.event_bus.spy.ANY},
        )
