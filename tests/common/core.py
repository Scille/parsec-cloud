# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncContextManager, Callable

import pytest
import trio

from parsec._parsec import CoreEvent, LocalDevice
from parsec.core.backend_connection import BackendConnStatus
from parsec.core.config import CoreConfig
from parsec.core.logged_core import LoggedCore, logged_core_factory
from parsec.core.types import BackendAddr
from tests.common.trio_clock import real_clock_timeout


@pytest.fixture
def core_config(tmpdir, backend_addr, unused_tcp_port, fixtures_customization):
    if fixtures_customization.get("fake_preferred_org_creation_backend_addr", False):
        backend_addr = BackendAddr.from_url(f"parsec://127.0.0.1:{unused_tcp_port}")

    workspace_storage_cache_size = fixtures_customization.get("workspace_storage_cache_size")
    if workspace_storage_cache_size is None:
        kwargs = {}
    else:
        kwargs = {"workspace_storage_cache_size": workspace_storage_cache_size}

    tmpdir = Path(tmpdir)
    return CoreConfig(
        config_dir=tmpdir / "config",
        data_base_dir=tmpdir / "data",
        mountpoint_base_dir=tmpdir / "mnt",
        preferred_org_creation_backend_addr=backend_addr,
        gui_language=fixtures_customization.get("gui_language"),
        **kwargs,
    )


CoreFactory = Callable[..., AsyncContextManager[LoggedCore]]


@pytest.fixture
def core_factory(request, running_backend_ready, event_bus_factory, core_config) -> CoreFactory:
    @asynccontextmanager
    async def _core_factory(device: LocalDevice, event_bus=None) -> AsyncContextManager[LoggedCore]:
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
                await core.wait_idle_monitors()
                assert core.are_monitors_idle()

                yield core

    return _core_factory


@pytest.fixture
async def alice_core(
    core_config,
    fixtures_customization,
    initialize_local_user_manifest,
    core_factory: CoreFactory,
    alice: LocalDevice,
) -> AsyncContextManager[LoggedCore]:
    initial_user_manifest = fixtures_customization.get("alice_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        core_config.data_base_dir, alice, initial_user_manifest=initial_user_manifest
    )
    async with core_factory(alice) as core:
        yield core


@pytest.fixture
async def alice2_core(
    core_config,
    fixtures_customization,
    initialize_local_user_manifest,
    core_factory: CoreFactory,
    alice2: LocalDevice,
) -> AsyncContextManager[LoggedCore]:
    initial_user_manifest = fixtures_customization.get("alice2_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        core_config.data_base_dir, alice2, initial_user_manifest=initial_user_manifest
    )
    async with core_factory(alice2) as core:
        yield core


@pytest.fixture
async def otheralice_core(
    core_config, initialize_local_user_manifest, core_factory: CoreFactory, otheralice: LocalDevice
) -> AsyncContextManager[LoggedCore]:
    await initialize_local_user_manifest(
        core_config.data_base_dir, otheralice, initial_user_manifest="v1"
    )
    async with core_factory(otheralice) as core:
        yield core


@pytest.fixture
async def adam_core(
    core_config,
    fixtures_customization,
    initialize_local_user_manifest,
    core_factory: CoreFactory,
    adam: LocalDevice,
) -> AsyncContextManager[LoggedCore]:
    initial_user_manifest = fixtures_customization.get("adam_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        core_config.data_base_dir, adam, initial_user_manifest=initial_user_manifest
    )
    async with core_factory(adam) as core:
        yield core


@pytest.fixture
async def bob_core(
    core_config,
    fixtures_customization,
    initialize_local_user_manifest,
    core_factory: CoreFactory,
    bob: LocalDevice,
) -> AsyncContextManager[LoggedCore]:
    initial_user_manifest = fixtures_customization.get("bob_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        core_config.data_base_dir, bob, initial_user_manifest=initial_user_manifest
    )
    async with core_factory(bob) as core:
        yield core


@pytest.fixture
def global_core_monitors_freeze(monkeypatch: pytest.MonkeyPatch):
    # In theory the `LoggedCore` (unlike `UserFS` alone) will run monitors in the
    # background. However this is an issue in some tests given it cause flakiness due
    # to those concurrent operations (e.g. in `test_sync_monitor_stateful`, message
    # monitor wakes up after every `create_sharing` rule to process the sharing message).
    # So instead we use the mockpoints in the monitors to make sure they don't do
    # actual work until we allow them to do so.

    monitor_are_unfrozen = trio.Event()

    async def _wait_for_monitor_unfrozen():
        await monitor_are_unfrozen.wait()

    monkeypatch.setattr(
        "parsec.core.sync_monitor.freeze_sync_monitor_mockpoint", _wait_for_monitor_unfrozen
    )
    monkeypatch.setattr(
        "parsec.core.messages_monitor.freeze_messages_monitor_mockpoint", _wait_for_monitor_unfrozen
    )
    monkeypatch.setattr(
        "parsec.core.remanence_monitor.freeze_remanence_monitor_mockpoint",
        _wait_for_monitor_unfrozen,
    )

    def _global_core_monitors_freeze(frozen: bool):
        nonlocal monitor_are_unfrozen
        if frozen:
            if monitor_are_unfrozen.is_set():
                monitor_are_unfrozen = trio.Event()
        else:
            if not monitor_are_unfrozen.is_set():
                monitor_are_unfrozen.set()

    return _global_core_monitors_freeze
