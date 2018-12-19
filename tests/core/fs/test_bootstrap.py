import pytest
from pendulum import Pendulum

from tests.common import freeze_time


@pytest.mark.trio
async def test_lazy_root_manifest_generation(
    backend_factory, backend_addr, server_factory, device_factory, local_db_factory, fs_factory
):
    with freeze_time("2000-01-01"):
        zack1 = device_factory("zack", "1")
        zack1_local_db = local_db_factory(zack1, user_manifest_in_v0=True)

    async with fs_factory(zack1, zack1_local_db) as fs:
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
    backend_factory, backend_addr, server_factory, device_factory, local_db_factory, fs_factory
):
    with freeze_time("2000-01-01"):
        zack1 = device_factory("zack", "1")
        zack1_local_db = local_db_factory(zack1, user_manifest_in_v0=True)

    with freeze_time("2000-01-02"):
        zack2 = device_factory("zack", "2")
        zack2_local_db = local_db_factory(zack2, user_manifest_in_v0=True)

    async with fs_factory(zack1, zack1_local_db) as fs1, fs_factory(zack2, zack2_local_db) as fs2:

        with freeze_time("2000-01-03"):
            await fs1.workspace_create("/from_1")
        with freeze_time("2000-01-04"):
            await fs2.workspace_create("/from_2")

        async with backend_factory(devices=[zack1, zack2]) as backend:
            async with server_factory(backend.handle_client, backend_addr):

                with fs1.event_bus.listen() as spy:
                    with freeze_time("2000-01-05"):
                        await fs1.sync("/")
                date_sync = Pendulum(2000, 1, 5)
                spy.assert_events_exactly_occured(
                    [
                        ("fs.entry.minimal_synced", {"path": "/", "id": spy.ANY}, date_sync),
                        ("fs.entry.minimal_synced", {"path": "/from_1", "id": spy.ANY}, date_sync),
                        ("fs.entry.synced", {"path": "/", "id": spy.ANY}, date_sync),
                        ("fs.entry.synced", {"path": "/from_1", "id": spy.ANY}, date_sync),
                    ]
                )

                with fs2.event_bus.listen() as spy:
                    with freeze_time("2000-01-06"):
                        await fs2.sync("/")
                date_sync = Pendulum(2000, 1, 6)
                spy.assert_events_exactly_occured(
                    [
                        ("fs.entry.minimal_synced", {"path": "/", "id": spy.ANY}, date_sync),
                        ("fs.entry.minimal_synced", {"path": "/from_2", "id": spy.ANY}, date_sync),
                        ("fs.entry.synced", {"path": "/", "id": spy.ANY}, date_sync),
                        ("fs.entry.synced", {"path": "/from_2", "id": spy.ANY}, date_sync),
                    ]
                )

                with fs1.event_bus.listen() as spy:
                    with freeze_time("2000-01-07"):
                        await fs1.sync("/")
                date_sync = Pendulum(2000, 1, 7)
                spy.assert_events_exactly_occured(
                    [
                        (
                            "fs.entry.remote_changed",
                            {"path": "/", "id": spy.ANY},
                            date_sync,
                        )  # TODO: really needed ?
                    ]
                )

        stat1 = await fs1.stat("/")
        stat2 = await fs2.stat("/")
        assert stat1 == {
            "type": "root",
            "created": Pendulum(2000, 1, 3),
            "updated": Pendulum(2000, 1, 4),
            "base_version": 3,
            "is_folder": True,
            "is_placeholder": False,
            "need_sync": False,
            "children": ["from_1", "from_2"],
        }
        assert stat1 == stat2
