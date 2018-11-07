import pytest
from pendulum import Pendulum

from tests.common import freeze_time


@pytest.mark.trio
async def test_lazy_root_manifest_generation(
    backend_factory, backend_addr, server_factory, device_factory, fs_factory
):
    with freeze_time("2000-01-01"):
        zack1 = device_factory("zack", "1", user_manifest_in_v0=True)

    async with fs_factory(zack1) as fs:
        with freeze_time("2000-01-02"):
            stat = await fs.stat("/")

        assert stat == {
            "type": "root",
            "created": Pendulum(2000, 1, 2),
            "updated": Pendulum(2000, 1, 2),
            "base_version": 0,
            "is_folder": True,
            "is_placeholder": True,
            "need_sync": True,
            "children": [],
        }

        async with backend_factory(devices=[zack1]) as backend:
            async with server_factory(backend.handle_client, backend_addr):
                with freeze_time("2000-01-03"):
                    await fs.sync("/")

        stat = await fs.stat("/")
        assert stat == {
            "type": "root",
            "created": Pendulum(2000, 1, 2),
            "updated": Pendulum(2000, 1, 2),
            "base_version": 1,
            "is_folder": True,
            "is_placeholder": False,
            "need_sync": False,
            "children": [],
        }


@pytest.mark.trio
async def test_concurrent_devices_agreed_on_root_manifest(
    backend_factory, backend_addr, server_factory, device_factory, fs_factory
):
    with freeze_time("2000-01-01"):
        zack1 = device_factory("zack", "1", user_manifest_in_v0=True)

    with freeze_time("2000-01-02"):
        zack2 = device_factory("zack", "2", user_manifest_in_v0=True)

    async with fs_factory(zack1) as fs1, fs_factory(zack2) as fs2:

        with freeze_time("2000-01-03"):
            await fs1.workspace_create("/from_1")
        with freeze_time("2000-01-04"):
            await fs2.workspace_create("/from_2")

        async with backend_factory(devices=[zack1, zack2]) as backend:
            async with server_factory(backend.handle_client, backend_addr):

                with freeze_time("2000-01-05"):
                    await fs1.sync("/")

                with freeze_time("2000-01-06"):
                    await fs2.sync("/")

                with freeze_time("2000-01-07"):
                    await fs1.sync("/")

        stat1 = await fs1.stat("/")
        stat2 = await fs2.stat("/")
        assert stat1 == {
            "type": "root",
            "created": Pendulum(2000, 1, 3),
            "updated": Pendulum(2000, 1, 4),
            "base_version": 2,
            "is_folder": True,
            "is_placeholder": False,
            "need_sync": False,
            "children": ["from_1", "from_2"],
        }
        assert stat1 == stat2
