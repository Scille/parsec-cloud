# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from contextlib import asynccontextmanager
from pathlib import Path

from parsec.core.config import CoreConfig
from parsec.core.types import BackendAddr
from parsec.core.core_events import CoreEvent
from parsec.core.logged_core import logged_core_factory
from parsec.core.backend_connection import BackendConnStatus

from tests.common.trio_clock import real_clock_timeout


@pytest.fixture
def core_config(tmpdir, backend_addr, unused_tcp_port, fixtures_customization):
    if fixtures_customization.get("fake_preferred_org_creation_backend_addr", False):
        backend_addr = BackendAddr.from_url(f"parsec://127.0.0.1:{unused_tcp_port}")

    tmpdir = Path(tmpdir)
    return CoreConfig(
        config_dir=tmpdir / "config",
        data_base_dir=tmpdir / "data",
        mountpoint_base_dir=tmpdir / "mnt",
        preferred_org_creation_backend_addr=backend_addr,
        gui_language=fixtures_customization.get("gui_language"),
    )


@pytest.fixture
def core_factory(request, running_backend_ready, event_bus_factory, core_config):
    @asynccontextmanager
    async def _core_factory(device, event_bus=None):
        # Ensure test doesn't stay frozen if a bug in a fixture prevent the
        # backend from starting
        async with real_clock_timeout():
            await running_backend_ready()
        event_bus = event_bus or event_bus_factory()

        with event_bus.listen() as spy:
            async with logged_core_factory(core_config, device, event_bus) as core:
                # On startup core is always considered offline.
                # Hence we risk concurrency issues if the connection to backend
                # switches online concurrently with the test.
                if "running_backend" in request.fixturenames:
                    await spy.wait_with_timeout(
                        CoreEvent.BACKEND_CONNECTION_CHANGED,
                        {"status": BackendConnStatus.READY, "status_exc": spy.ANY},
                    )
                assert core.are_monitors_idle()

                yield core

    return _core_factory


@pytest.fixture
async def alice_core(
    core_config, fixtures_customization, initialize_local_user_manifest, core_factory, alice
):
    initial_user_manifest = fixtures_customization.get("alice_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        core_config.data_base_dir, alice, initial_user_manifest=initial_user_manifest
    )
    async with core_factory(alice) as core:
        yield core


@pytest.fixture
async def alice2_core(
    core_config, fixtures_customization, initialize_local_user_manifest, core_factory, alice2
):
    initial_user_manifest = fixtures_customization.get("alice2_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        core_config.data_base_dir, alice2, initial_user_manifest=initial_user_manifest
    )
    async with core_factory(alice2) as core:
        yield core


@pytest.fixture
async def otheralice_core(core_config, initialize_local_user_manifest, core_factory, otheralice):
    await initialize_local_user_manifest(
        core_config.data_base_dir, otheralice, initial_user_manifest="v1"
    )
    async with core_factory(otheralice) as core:
        yield core


@pytest.fixture
async def adam_core(
    core_config, fixtures_customization, initialize_local_user_manifest, core_factory, adam
):
    initial_user_manifest = fixtures_customization.get("adam_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        core_config.data_base_dir, adam, initial_user_manifest=initial_user_manifest
    )
    async with core_factory(adam) as core:
        yield core


@pytest.fixture
async def bob_core(
    core_config, fixtures_customization, initialize_local_user_manifest, core_factory, bob
):
    initial_user_manifest = fixtures_customization.get("bob_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        core_config.data_base_dir, bob, initial_user_manifest=initial_user_manifest
    )
    async with core_factory(bob) as core:
        yield core
