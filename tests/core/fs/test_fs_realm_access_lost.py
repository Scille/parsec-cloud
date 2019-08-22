# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.fs import FSWorkspaceNoAccess

from tests.common import create_shared_workspace


@pytest.mark.trio
async def test_manifest_no_access(running_backend, alice_user_fs, bob_user_fs):
    wid = await create_shared_workspace("w", alice_user_fs, bob_user_fs)
    alice_w = alice_user_fs.get_workspace(wid)
    bob_w = bob_user_fs.get_workspace(wid)
    await alice_w.touch("/foo.txt")
    await alice_w.sync("/")
    await bob_w.sync("/")

    # Remove acces to bob
    await alice_user_fs.workspace_share(wid, bob_user_fs.device.user_id, None)

    with pytest.raises(FSWorkspaceNoAccess) as exc:
        await bob_w.path_info("/foo.txt")
    assert str(exc.value) == "Cannot load manifest: no read access"

    await bob_w.touch("/bar.txt")
    with pytest.raises(FSWorkspaceNoAccess) as exc:
        await bob_w.sync("/bar.txt")
    assert str(exc.value) == "Cannot upload manifest: no write access"


@pytest.mark.trio
async def test_block_no_access(running_backend, alice_user_fs, bob_user_fs):
    wid = await create_shared_workspace("w", alice_user_fs, bob_user_fs)
    alice_w = alice_user_fs.get_workspace(wid)
    bob_w = bob_user_fs.get_workspace(wid)
    await alice_w.touch("/foo.txt")
    await alice_w.write_bytes("/foo.txt", b"foo data")
    await alice_w.sync("/")
    await bob_w.sync("/")

    # Ensure file manifest is in cache (but not blocks)
    await bob_w.path_info("/foo.txt")

    # Remove acces to bob
    await alice_user_fs.workspace_share(wid, bob_user_fs.device.user_id, None)

    # Try to acces blocks
    with pytest.raises(FSWorkspaceNoAccess) as exc:
        await bob_w.read_bytes("/foo.txt")
    assert str(exc.value) == "Cannot load block: no read access"

    await bob_w.touch("/bar.txt")
    await bob_w.write_bytes("/bar.txt", b"bar data")
    with pytest.raises(FSWorkspaceNoAccess) as exc:
        await bob_w.sync("/bar.txt")
    assert str(exc.value) == "Cannot upload block: no write access"
