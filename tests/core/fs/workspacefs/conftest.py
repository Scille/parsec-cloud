# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import EntryName
from parsec.core.fs import UserFS, WorkspaceFS


@pytest.fixture
@pytest.mark.trio
async def alice_workspace(alice_user_fs: UserFS, running_backend) -> WorkspaceFS:
    wid = await alice_user_fs.workspace_create(EntryName("w"))
    workspace = alice_user_fs.get_workspace(wid)

    await workspace.mkdir("/foo")
    await workspace.touch("/foo/bar")
    await workspace.touch("/foo/baz")
    await workspace.sync()
    await alice_user_fs.sync()

    return workspace


@pytest.fixture
@pytest.mark.trio
async def bob_workspace(bob_user_fs: UserFS, running_backend) -> WorkspaceFS:
    wid = await bob_user_fs.workspace_create(EntryName("w"))
    workspace = bob_user_fs.get_workspace(wid)

    await workspace.mkdir("/foo")
    await workspace.sync()
    await bob_user_fs.sync()

    return workspace
