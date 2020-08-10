# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest


@pytest.fixture
@pytest.mark.trio
async def alice_workspace(alice_user_fs, running_backend):
    wid = await alice_user_fs.workspace_create("w")
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.mkdir("/foo")
    await workspace.touch("/foo/bar")
    await workspace.touch("/foo/baz")
    await workspace.sync()
    return workspace


@pytest.fixture
@pytest.mark.trio
async def bob_workspace(bob_user_fs, running_backend):
    wid = await bob_user_fs.workspace_create("w")
    workspace = bob_user_fs.get_workspace(wid)
    await workspace.mkdir("/foo")
    await workspace.sync()
    return workspace
