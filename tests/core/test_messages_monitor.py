# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pytest
from unittest.mock import ANY

from parsec.core.types import WorkspaceEntry, WorkspaceRole


@pytest.mark.trio
async def test_new_sharing_trigger_event(alice_core, bob_core, running_backend):
    await alice_core.event_bus.spy.wait_for_backend_connection_ready()
    await bob_core.event_bus.spy.wait_for_backend_connection_ready()

    # First, create a folder and sync it on backend
    wid = await alice_core.user_fs.workspace_create("foo")
    workspace = alice_core.user_fs.get_workspace(wid)
    await workspace.sync("/")

    # Now we can share this workspace with Bob
    with bob_core.event_bus.listen() as spy:
        await alice_core.user_fs.workspace_share(wid, recipient="bob", role=WorkspaceRole.MANAGER)

        # Bob should get a notification
        with trio.fail_after(seconds=1):
            await spy.wait(
                "sharing.granted",
                kwargs={
                    "new_entry": WorkspaceEntry(
                        name="foo (shared by alice)",
                        id=wid,
                        key=ANY,
                        encryption_revision=1,
                        granted_on=ANY,
                        role=WorkspaceRole.MANAGER,
                    )
                },
            )
