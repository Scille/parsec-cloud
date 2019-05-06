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
)
from parsec.core import logged_core_factory
from parsec.core.types import FsPath


@pytest.mark.trio
async def test_runner_not_available(alice_user_fs, event_bus):
    base_mountpoint = Path("/foo")

    with patch("parsec.core.mountpoint.manager.get_mountpoint_runner", return_value=None):
        with pytest.raises(RuntimeError):
            async with mountpoint_manager_factory(alice_user_fs, event_bus, base_mountpoint):
                pass


@pytest.mark.trio
async def test_mountpoint_disabled(alice_user_fs, event_bus):
    base_mountpoint = Path("/foo")

    wid = await alice_user_fs.workspace_create("/w")

    with patch("parsec.core.mountpoint.manager.get_mountpoint_runner", return_value=None):
        async with mountpoint_manager_factory(
            alice_user_fs, event_bus, base_mountpoint, enabled=False
        ) as mountpoint_manager:
            with pytest.raises(MountpointDisabled):
                await mountpoint_manager.mount_workspace(wid)


@pytest.mark.trio
async def test_mount_unknown_workspace(base_mountpoint, alice_user_fs, event_bus):
    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        wid = uuid4()
        with pytest.raises(MountpointConfigurationError) as exc:
            await mountpoint_manager.mount_workspace(wid)

        assert exc.value.args == (f"Workspace `{wid}` doesn't exist",)


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_base_mountpoint_not_created(base_mountpoint, alice_fs, alice_user_fs, event_bus):
    # Path should be created if it doesn' exist
    base_mountpoint = base_mountpoint / "dummy/dummy/dummy"
    mountpoint = f"{base_mountpoint.absolute()}/w"

    wid = await alice_user_fs.workspace_create("w")
    await alice_fs.touch("/w/bar.txt")

    bar_txt = trio.Path(f"{mountpoint}/bar.txt")

    # Now we can start fuse

    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:

        await mountpoint_manager.mount_workspace(wid)
        assert await bar_txt.exists()


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_mountpoint_path_already_in_use(
    base_mountpoint, running_backend, alice_user_fs, alice2_user_fs, alice_fs
):
    # Create a workspace and make it available in two devices
    wid = await alice_user_fs.workspace_create("w")
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.sync("/")
    await alice2_user_fs.sync()
    # Easily differenciate alice&alice2
    await alice2_user_fs.get_workspace(wid).touch("/I_am_alice2.txt")
    await alice_user_fs.get_workspace(wid).touch("/I_am_alice.txt")

    naive_workspace_path = (base_mountpoint / "w").absolute()

    # Default workspace path already exists, souldn't be able to use it
    await trio.Path(base_mountpoint / "w").mkdir(parents=True)
    await trio.Path(base_mountpoint / "w" / "bar.txt").touch()

    async with mountpoint_manager_factory(
        alice_user_fs, alice_user_fs.event_bus, base_mountpoint
    ) as alice_mountpoint_manager, mountpoint_manager_factory(
        alice2_user_fs, alice2_user_fs.event_bus, base_mountpoint
    ) as alice2_mountpoint_manager:
        # Alice mount the workspace first
        alice_mountpoint_path = await alice_mountpoint_manager.mount_workspace(wid)
        assert str(alice_mountpoint_path) == f"{naive_workspace_path} (2)"

        # Alice2 should also be able to mount the workspace without name clashing
        alice2_mountpoint_path = await alice2_mountpoint_manager.mount_workspace(wid)
        assert str(alice2_mountpoint_path) == f"{naive_workspace_path} (3)"

        # Finally make sure each workspace is well mounted
        assert await trio.Path(alice_mountpoint_path / "I_am_alice.txt").exists()
        assert await trio.Path(alice2_mountpoint_path / "I_am_alice2.txt").exists()


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.parametrize("manual_unmount", [True, False])
async def test_mount_and_explore_workspace(
    base_mountpoint, alice_fs, alice_user_fs, event_bus, manual_unmount
):
    # Populate a bit the fs first...

    wid = await alice_user_fs.workspace_create("w")
    await alice_fs.folder_create("/w/foo")
    await alice_fs.touch("/w/bar.txt")
    await alice_fs.file_write("/w/bar.txt", b"Hello world !")

    # Now we can start fuse

    with event_bus.listen() as spy:

        async with mountpoint_manager_factory(
            alice_user_fs, event_bus, base_mountpoint
        ) as mountpoint_manager:

            await mountpoint_manager.mount_workspace(wid)
            mountpoint_path = base_mountpoint / "w"

            spy.assert_events_occured(
                [
                    ("mountpoint.starting", {"mountpoint": mountpoint_path}),
                    ("mountpoint.started", {"mountpoint": mountpoint_path}),
                ]
            )

            # Finally explore the mountpoint

            def inspect_mountpoint():
                wksp_children = set(os.listdir(mountpoint_path))
                assert wksp_children == {"foo", "bar.txt"}

                bar_stat = os.stat(f"{mountpoint_path}/bar.txt")
                assert bar_stat.st_size == len(b"Hello world !")

                with open(f"{mountpoint_path}/bar.txt", "rb") as fd:
                    bar_txt = fd.read()
                assert bar_txt == b"Hello world !"

            # Note given python fs api is blocking, we must run it inside a thread
            # to avoid blocking the trio loop and ending up in a deadlock
            await trio.run_sync_in_worker_thread(inspect_mountpoint)

            if manual_unmount:
                await mountpoint_manager.unmount_workspace(wid)
                # Mountpoint should be stopped by now
                spy.assert_events_occured([("mountpoint.stopped", {"mountpoint": mountpoint_path})])

        if not manual_unmount:
            # Mountpoint should be stopped by now
            spy.assert_events_occured([("mountpoint.stopped", {"mountpoint": mountpoint_path})])


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.parametrize("manual_unmount", [True, False])
async def test_idempotent_mount(
    base_mountpoint, alice_fs, alice_user_fs, event_bus, manual_unmount
):
    mountpoint_path = base_mountpoint / "w"

    # Populate a bit the fs first...

    wid = await alice_user_fs.workspace_create("w")
    await alice_fs.touch("/w/bar.txt")

    bar_txt = trio.Path(f"{mountpoint_path}/bar.txt")

    # Now we can start fuse

    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
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
    mountpoint_path = base_mountpoint / "w"
    bar_txt = trio.Path(f"{mountpoint_path}/bar.txt")

    async with logged_core_factory(core_config, alice) as alice_core:
        wid = await alice_core.user_fs.workspace_create("w")
        await alice_core.fs.touch("/w/bar.txt")

        assert not await bar_txt.exists()

        await alice_core.mountpoint_manager.mount_workspace(wid)

        assert await bar_txt.exists()

    assert not await bar_txt.exists()


@pytest.mark.linux
def test_manifest_not_available(mountpoint_service):
    async def _bootstrap(user_fs, fs, mountpoint_manager):
        await user_fs.workspace_create("x")
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
async def test_get_path_in_mountpoint(base_mountpoint, alice_fs, alice_user_fs, event_bus):
    # Populate a bit the fs first...
    wid = await alice_user_fs.workspace_create("mounted_wksp")
    wid2 = await alice_user_fs.workspace_create("not_mounted_wksp")
    await alice_fs.touch("/mounted_wksp/bar.txt")
    await alice_fs.touch("/not_mounted_wksp/foo.txt")

    # Now we can start fuse
    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        await mountpoint_manager.mount_workspace(wid)

        bar_path = mountpoint_manager.get_path_in_mountpoint(wid, FsPath("/bar.txt"))

        assert isinstance(bar_path, PurePath)
        expected = base_mountpoint / f"mounted_wksp" / "bar.txt"
        assert str(bar_path) == str(expected.absolute())
        assert await trio.Path(bar_path).exists()

        with pytest.raises(MountpointNotMounted):
            mountpoint_manager.get_path_in_mountpoint(wid2, FsPath("/foo.txt"))
