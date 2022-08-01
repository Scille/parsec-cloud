# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec import IS_OXIDIZED


@pytest.mark.trio
@pytest.mark.skipif(IS_OXIDIZED, reason="Oxidation doesn't implement WorkspaceStorageTimestamped")
async def test_sync_timestamp_consistency(alice_workspace):
    # Add more files and dirs, without time freezing
    # Not freezing the time is crucial here as it allows for
    # testing that the remote manifests are causally timestamped
    await alice_workspace.mkdir("/test_sync_dir")
    await alice_workspace.touch("/test_sync_file")
    await alice_workspace.write_bytes("/test_sync_file", b"123")

    # Synchronize everything at once
    await alice_workspace.sync()

    # Iterate over root versions
    versions = await alice_workspace.remote_loader.list_versions(alice_workspace.workspace_id)
    for version, (timestamp, _) in versions.items():

        # Get the corresponding timestamped workspace
        workspace = await alice_workspace.to_timestamped(timestamp)

        # Check root integrity
        async for path in workspace.iterdir("/"):
            await workspace.path_info(path)
