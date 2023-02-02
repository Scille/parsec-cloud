# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import AsyncContextManager, Callable

import pytest
import trio

from parsec._parsec import EntryName, LocalDevice
from parsec.core.logged_core import CoreConfig, LoggedCore
from parsec.core.types import DEFAULT_BLOCK_SIZE
from tests.common import RunningBackend, customize_fixtures


@pytest.mark.trio
@customize_fixtures(workspace_storage_cache_size=DEFAULT_BLOCK_SIZE)
async def test_remanence_monitor_single_device(
    running_backend: RunningBackend,
    alice_core: LoggedCore,
    remanence_monitor_event: trio.Event,
    monkeypatch,
):
    # Go fast
    alice_core.device.time_provider.mock_time(speed=1000.0)
    monkeypatch.setattr("parsec.utils.BALLPARK_ALWAYS_OK", True)

    # Create a workspace
    wid = await alice_core.user_fs.workspace_create(EntryName("w"))
    workspace = alice_core.user_fs.get_workspace(wid)

    # The manager is not started
    assert not workspace.is_block_remanent()
    info = workspace.get_remanence_manager_info()
    assert not info.is_prepared
    assert not info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 0
    assert info.local_and_remote_size == 0
    assert info.remote_only_size == 0
    with pytest.raises(RuntimeError):
        await workspace.wait_remanence_manager_prepared()

    # Let remanence manager run
    remanence_monitor_event.set()
    await workspace.wait_remanence_manager_prepared(wait_for_connection=True)
    await workspace.remanence_manager._prepared_event.wait()

    # The workspace is empty
    info = workspace.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 0
    assert info.local_and_remote_size == 0
    assert info.remote_only_size == 0

    # Write a file with a single block
    await workspace.write_bytes("/test.txt", b"hello")
    await workspace.sync()
    await alice_core.wait_idle_monitors()
    info = workspace.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 5
    assert info.local_and_remote_size == 5
    assert info.remote_only_size == 0

    # Write another file (the storage can only hold a single block)
    await workspace.mkdir("/more")
    await workspace.write_bytes("/more/test2.txt", b"world!")
    await workspace.sync()
    await alice_core.wait_idle_monitors()
    info = workspace.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 11
    assert info.local_and_remote_size == 6
    assert info.remote_only_size == 5

    # Enable block remanence (the missing block is downloaded)
    assert await workspace.enable_block_remanence()
    assert not await workspace.enable_block_remanence()
    assert workspace.is_block_remanent()
    await alice_core.wait_idle_monitors()
    info = workspace.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert info.is_block_remanent
    assert info.total_size == 11
    assert info.local_and_remote_size == 11
    assert info.remote_only_size == 0

    # Disable block remanence (the oldest block is removed)
    assert await workspace.disable_block_remanence()
    assert not await workspace.disable_block_remanence()
    assert not workspace.is_block_remanent()
    await alice_core.wait_idle_monitors()
    info = workspace.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 11
    assert info.local_and_remote_size == 5
    assert info.remote_only_size == 6

    # Read the missing block (blocks are swapped)
    assert await workspace.read_bytes("/more/test2.txt") == b"world!"
    await alice_core.wait_idle_monitors()
    info = workspace.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 11
    assert info.local_and_remote_size == 6
    assert info.remote_only_size == 5

    # Remove one of the files
    await workspace.unlink("/more/test2.txt")
    await workspace.sync()
    await alice_core.wait_idle_monitors()
    info = workspace.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 5
    assert info.local_and_remote_size == 0
    assert info.remote_only_size == 5

    # Remove the other file
    await workspace.unlink("/test.txt")
    await workspace.sync()
    await alice_core.wait_idle_monitors()
    info = workspace.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 0
    assert info.local_and_remote_size == 0
    assert info.remote_only_size == 0


@pytest.mark.trio
@customize_fixtures(workspace_storage_cache_size=DEFAULT_BLOCK_SIZE)
async def test_remanence_monitor_multiple_device(
    running_backend,
    alice_core: LoggedCore,
    alice2_core: LoggedCore,
    remanence_monitor_event: trio.Event,
    monkeypatch,
):
    # Go fast
    alice_core.device.time_provider.mock_time(speed=1000.0)
    alice2_core.device.time_provider.mock_time(speed=1000.0)
    monkeypatch.setattr("parsec.utils.BALLPARK_ALWAYS_OK", True)

    # Create a workspace
    wid = await alice_core.user_fs.workspace_create(EntryName("w"))
    await alice_core.user_fs.sync()
    await alice_core.wait_idle_monitors()
    await alice2_core.wait_idle_monitors()
    workspace1 = alice_core.user_fs.get_workspace(wid)
    workspace2 = alice2_core.user_fs.get_workspace(wid)

    # The managers are started
    remanence_monitor_event.set()
    await workspace1.wait_remanence_manager_prepared(wait_for_connection=True)
    await workspace2.wait_remanence_manager_prepared(wait_for_connection=True)

    # First device writes a file with a single block
    await workspace1.write_bytes("/test.txt", b"hello")
    await workspace1.sync()
    await alice_core.wait_idle_monitors()
    info = workspace1.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 5
    assert info.local_and_remote_size == 5
    assert info.remote_only_size == 0

    # Second device is notified (but doesn't have the block locally)
    await alice2_core.wait_idle_monitors()
    info = workspace2.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 5
    assert info.local_and_remote_size == 0
    assert info.remote_only_size == 5

    # Second device enable block remanence
    await workspace2.enable_block_remanence()
    await alice2_core.wait_idle_monitors()
    info = workspace2.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert info.is_block_remanent
    assert info.total_size == 5
    assert info.local_and_remote_size == 5
    assert info.remote_only_size == 0

    # First device writes another file
    await workspace1.mkdir("/more")
    await workspace1.write_bytes("/more/test2.txt", b"world!")
    await workspace1.sync()
    await alice_core.wait_idle_monitors()
    info = workspace1.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 11
    assert info.local_and_remote_size == 6
    assert info.remote_only_size == 5

    # Second device gets the second block
    await alice2_core.wait_idle_monitors()
    info = workspace2.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert info.is_block_remanent
    assert info.total_size == 11
    assert info.local_and_remote_size == 11
    assert info.remote_only_size == 0

    # First device removes one of the files
    await workspace1.unlink("/more/test2.txt")
    await workspace1.sync()
    await alice_core.wait_idle_monitors()
    info = workspace1.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 5
    assert info.local_and_remote_size == 0
    assert info.remote_only_size == 5

    # Second device gets notified
    await alice2_core.wait_idle_monitors()
    info = workspace2.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert info.is_block_remanent
    assert info.total_size == 5
    assert info.local_and_remote_size == 5
    assert info.remote_only_size == 0

    # First device removes the last file
    await workspace1.unlink("/test.txt")
    await workspace1.sync()
    await alice_core.wait_idle_monitors()
    info = workspace1.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert not info.is_block_remanent
    assert info.total_size == 0
    assert info.local_and_remote_size == 0
    assert info.remote_only_size == 0

    # Second device gets notified
    await alice2_core.wait_idle_monitors()
    info = workspace2.get_remanence_manager_info()
    assert info.is_prepared
    assert info.is_running
    assert info.is_block_remanent
    assert info.total_size == 0
    assert info.local_and_remote_size == 0
    assert info.remote_only_size == 0


@pytest.mark.trio
@customize_fixtures(workspace_storage_cache_size=DEFAULT_BLOCK_SIZE)
async def test_remanence_monitor_with_core_restart(
    core_config: CoreConfig,
    running_backend: RunningBackend,
    alice: LocalDevice,
    core_factory: Callable[[LocalDevice], AsyncContextManager[LoggedCore]],
    initialize_local_user_manifest,
    remanence_monitor_event: trio.Event,
    monkeypatch,
):
    # Initialize local user manifest
    await initialize_local_user_manifest(
        core_config.data_base_dir,
        alice,
        initial_user_manifest="v1",
    )

    # Unlock the remanence monitor
    remanence_monitor_event.set()

    # Go fast
    alice.time_provider.mock_time(speed=1000.0)
    monkeypatch.setattr("parsec.utils.BALLPARK_ALWAYS_OK", True)

    # First instanciation
    async with core_factory(alice) as alice_core:

        # Create a workspace
        wid = await alice_core.user_fs.workspace_create(EntryName("w"))
        workspace = alice_core.user_fs.get_workspace(wid)

        # The managers are started
        await workspace.wait_remanence_manager_prepared(wait_for_connection=True)
        await workspace.enable_block_remanence()
        await workspace.write_bytes("/test.txt", b"hello")
        await alice_core.wait_idle_monitors()
        await workspace.mkdir("/more")
        await workspace.write_bytes("/more/test2.txt", b"world!")
        await workspace.sync()

        # Fill up workspace
        await alice_core.wait_idle_monitors()
        info = workspace.get_remanence_manager_info()
        assert info.is_prepared
        assert info.is_running
        assert info.is_block_remanent
        assert info.total_size == 11
        assert info.local_and_remote_size == 11
        assert info.remote_only_size == 0

    # Second instanciation
    async with core_factory(alice) as alice_core:

        # Check thate hasn't changed
        workspace = alice_core.user_fs.get_workspace(wid)
        await workspace.wait_remanence_manager_prepared(wait_for_connection=True)
        await alice_core.wait_idle_monitors()
        info = workspace.get_remanence_manager_info()
        assert info.is_prepared
        assert info.is_running
        assert info.is_block_remanent
        assert info.total_size == 11
        assert info.local_and_remote_size == 11
        assert info.remote_only_size == 0

        # Disable block remanence
        await workspace.disable_block_remanence()
        await alice_core.wait_idle_monitors()
        info = workspace.get_remanence_manager_info()
        assert info.is_prepared
        assert info.is_running
        assert not info.is_block_remanent
        assert info.total_size == 11
        assert info.local_and_remote_size == 6
        assert info.remote_only_size == 5

        # Go offline and enable it back
        with running_backend.offline():
            await workspace.enable_block_remanence()
            await alice_core.wait_idle_monitors()
            info = workspace.get_remanence_manager_info()
            assert info.is_prepared
            assert not info.is_running
            assert info.is_block_remanent
            assert info.total_size == 11
            assert info.local_and_remote_size == 6
            assert info.remote_only_size == 5

        # Wait to go back online
        with trio.fail_after(1):
            while not workspace.get_remanence_manager_info().is_running:
                await trio.sleep(0.001)

        await alice_core.wait_idle_monitors()
        info = workspace.get_remanence_manager_info()
        assert info.is_prepared
        assert info.is_running
        assert info.is_block_remanent
        assert info.total_size == 11
        assert info.local_and_remote_size == 11
        assert info.remote_only_size == 0
