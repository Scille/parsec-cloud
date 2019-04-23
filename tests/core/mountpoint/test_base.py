# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from uuid import uuid4

import trio
import pytest
from pathlib import Path, PurePath
from unittest.mock import patch

from parsec.core.mountpoint import (
    mountpoint_manager_factory,
    MountpointDisabled,
    MountpointConfigurationError,
    MountpointAlreadyMounted,
    MountpointNotMounted,
    MountpointDriverCrash,
)
from parsec.core import logged_core_factory
from parsec.core.types import FsPath


@pytest.mark.trio
async def test_runner_not_available(alice_fs, event_bus):
    base_mountpoint = Path("/foo")

    with patch("parsec.core.mountpoint.manager.get_mountpoint_runner", return_value=None):
        with pytest.raises(RuntimeError):
            async with mountpoint_manager_factory(alice_fs, event_bus, base_mountpoint):
                pass


@pytest.mark.trio
async def test_mountpoint_disabled(alice_fs, event_bus):
    base_mountpoint = Path("/foo")

    wid = await alice_fs.workspace_create("/w")

    with patch("parsec.core.mountpoint.manager.get_mountpoint_runner", return_value=None):
        async with mountpoint_manager_factory(
            alice_fs, event_bus, base_mountpoint, enabled=False
        ) as mountpoint_manager:
            with pytest.raises(MountpointDisabled):
                await mountpoint_manager.mount_workspace(wid)


@pytest.mark.trio
async def test_mount_unknown_workspace(base_mountpoint, alice_fs, event_bus):
    async with mountpoint_manager_factory(
        alice_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        wid = uuid4()
        with pytest.raises(MountpointConfigurationError) as exc:
            await mountpoint_manager.mount_workspace(wid)

        assert exc.value.args == (f"Workspace `{wid}` doesn't exist",)


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_base_mountpoint_not_created(base_mountpoint, alice, alice_fs, event_bus):
    # Path should be created if it doesn' exist
    base_mountpoint = base_mountpoint / "dummy/dummy/dummy"
    mountpoint = f"{base_mountpoint.absolute()}/{alice.user_id}-w"

    wid = await alice_fs.workspace_create("/w")
    await alice_fs.touch("/w/bar.txt")

    bar_txt = trio.Path(f"{mountpoint}/bar.txt")

    # Now we can start fuse

    async with mountpoint_manager_factory(
        alice_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:

        await mountpoint_manager.mount_workspace(wid)
        assert await bar_txt.exists()


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.skipif(os.name == "nt", reason="TODO: Cause freeze in winfsp so far...")
async def test_mountpoint_already_in_use(base_mountpoint, alice, alice_fs, alice2_fs, event_bus):
    # Path should be created if it doesn' exist
    mountpoint = str(base_mountpoint.absolute() / f"{alice.user_id}-w")

    wid1 = await alice_fs.workspace_create("/w")
    await alice_fs.touch("/w/bar.txt")
    await alice2_fs.workspace_create("/w")
    await alice2_fs.touch("/w/bar.txt")

    bar_txt = trio.Path(f"{mountpoint}/bar.txt")

    # Now we can start fuse

    async with mountpoint_manager_factory(alice_fs, event_bus, base_mountpoint) as alice_mm:

        async with mountpoint_manager_factory(alice_fs, event_bus, base_mountpoint) as alice2_mm:

            await alice_mm.mount_workspace(wid1)
            assert await bar_txt.exists()

            with pytest.raises(MountpointDriverCrash) as exc:
                await alice2_mm.mount_workspace(wid1)
            assert exc.value.args == (f"Fuse has crashed on {mountpoint}: EPERM",)
            assert await bar_txt.exists()

        assert await bar_txt.exists()

    assert not await bar_txt.exists()


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.parametrize("manual_unmount", [True, False])
async def test_mount_and_explore_workspace(
    base_mountpoint, alice, alice_fs, event_bus, manual_unmount
):
    # Populate a bit the fs first...

    wid = await alice_fs.workspace_create("/w")
    await alice_fs.folder_create("/w/foo")
    await alice_fs.touch("/w/bar.txt")
    await alice_fs.file_write("/w/bar.txt", b"Hello world !")

    # Now we can start fuse

    with event_bus.listen() as spy:

        async with mountpoint_manager_factory(
            alice_fs, event_bus, base_mountpoint
        ) as mountpoint_manager:

            await mountpoint_manager.mount_workspace(wid)
            mountpoint = str(base_mountpoint.absolute() / f"{alice.user_id}-w")

            spy.assert_events_occured(
                [
                    ("mountpoint.starting", {"mountpoint": mountpoint}),
                    ("mountpoint.started", {"mountpoint": mountpoint}),
                ]
            )

            # Finally explore the mountpoint

            def inspect_mountpoint():
                wksp_children = set(os.listdir(mountpoint))
                assert wksp_children == {"foo", "bar.txt"}

                bar_stat = os.stat(f"{mountpoint}/bar.txt")
                assert bar_stat.st_size == len(b"Hello world !")

                with open(f"{mountpoint}/bar.txt", "rb") as fd:
                    bar_txt = fd.read()
                assert bar_txt == b"Hello world !"

            # Note given python fs api is blocking, we must run it inside a thread
            # to avoid blocking the trio loop and ending up in a deadlock
            await trio.run_sync_in_worker_thread(inspect_mountpoint)

            if manual_unmount:
                await mountpoint_manager.unmount_workspace(wid)
                # Mountpoint should be stopped by now
                spy.assert_events_occured([("mountpoint.stopped", {"mountpoint": mountpoint})])

        if not manual_unmount:
            # Mountpoint should be stopped by now
            spy.assert_events_occured([("mountpoint.stopped", {"mountpoint": mountpoint})])


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.parametrize("manual_unmount", [True, False])
async def test_idempotent_mount(base_mountpoint, alice, alice_fs, event_bus, manual_unmount):
    mountpoint = f"{base_mountpoint.absolute()}/{alice.user_id}-w"

    # Populate a bit the fs first...

    wid = await alice_fs.workspace_create("/w")
    await alice_fs.touch("/w/bar.txt")

    bar_txt = trio.Path(f"{mountpoint}/bar.txt")

    # Now we can start fuse

    async with mountpoint_manager_factory(
        alice_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:

        await mountpoint_manager.mount_workspace(wid)
        assert await bar_txt.exists()

        with pytest.raises(MountpointAlreadyMounted):
            await mountpoint_manager.mount_workspace(wid)
        assert await bar_txt.exists()

        await mountpoint_manager.unmount_workspace(wid)
        assert not await bar_txt.exists()

        with pytest.raises(MountpointNotMounted):
            await mountpoint_manager.unmount_workspace(wid)
        assert not await bar_txt.exists()

        await mountpoint_manager.mount_workspace(wid)
        assert await bar_txt.exists()


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_work_within_logged_core(base_mountpoint, core_config, alice, tmpdir):
    core_config = core_config.evolve(mountpoint_enabled=True, mountpoint_base_dir=base_mountpoint)
    mountpoint = f"{base_mountpoint.absolute()}/{alice.user_id}-w"
    bar_txt = trio.Path(f"{mountpoint}/bar.txt")

    async with logged_core_factory(core_config, alice) as alice_core:
        wid = await alice_core.fs.workspace_create("/w")
        await alice_core.fs.touch("/w/bar.txt")

        assert not await bar_txt.exists()

        await alice_core.mountpoint_manager.mount_workspace(wid)

        assert await bar_txt.exists()

    assert not await bar_txt.exists()


@pytest.mark.linux
def test_manifest_not_available(mountpoint_service):
    async def _bootstrap(fs, mountpoint_manager):
        await fs.workspace_create("/x")
        await fs.touch("/x/foo.txt")
        foo_access = fs._local_folder_fs.get_access(FsPath("/x/foo.txt"))
        fs._local_folder_fs.clear_manifest(foo_access)
        await mountpoint_manager.mount_all()

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)
    x_path = mountpoint_service.get_workspace_mountpoint("x")

    with pytest.raises(OSError) as exc:
        (x_path / "foo.txt").stat()
    if os.name == "nt":
        assert str(exc.value).startswith("[WinError 1231] The network location cannot be reached.")
    else:
        assert exc.value.args == (100, "Network is down")


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_get_path_in_mountpoint(base_mountpoint, alice, alice_fs, event_bus):
    # Populate a bit the fs first...
    wid = await alice_fs.workspace_create("/mounted_wksp")
    wid2 = await alice_fs.workspace_create("/not_mounted_wksp")
    await alice_fs.touch("/mounted_wksp/bar.txt")
    await alice_fs.touch("/not_mounted_wksp/foo.txt")

    # Now we can start fuse
    async with mountpoint_manager_factory(
        alice_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        await mountpoint_manager.mount_workspace(wid)

        bar_path = mountpoint_manager.get_path_in_mountpoint(wid, FsPath("/bar.txt"))

        assert isinstance(bar_path, PurePath)
        expected = base_mountpoint / f"{alice.user_id}-mounted_wksp" / "bar.txt"
        assert str(bar_path) == str(expected.absolute())
        assert await trio.Path(bar_path).exists()

        with pytest.raises(MountpointNotMounted):
            mountpoint_manager.get_path_in_mountpoint(wid2, FsPath("/foo.txt"))
