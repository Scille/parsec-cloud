# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import asyncio
from typing import AsyncGenerator, Iterator

import pytest

from parsec._parsec import ParsecAddr
from parsec.asgi import AsgiApp
from parsec.asgi import app as asgi_app
from parsec.backend import Backend, backend_factory
from parsec.cli.testbed import TestbedBackend
from parsec.config import BackendConfig, BaseBlockStoreConfig, MockedEmailConfig

SERVER_DOMAIN = "parsec.invalid"


@pytest.fixture
def backend_config(
    tmpdir: str, db_url: str, blockstore_config: BaseBlockStoreConfig
) -> BackendConfig:
    return BackendConfig(
        debug=True,
        db_url=db_url,
        db_min_connections=1,
        db_max_connections=1,
        sse_keepalive=30,
        forward_proto_enforce_https=None,
        server_addr=ParsecAddr(hostname=SERVER_DOMAIN, port=None, use_ssl=True),
        email_config=MockedEmailConfig("no-reply@parsec.com", tmpdir),
        blockstore_config=blockstore_config,
        administration_token="s3cr3t",
        organization_spontaneous_bootstrap=False,
        organization_bootstrap_webhook_url=None,
    )


@pytest.fixture
async def backend(backend_config: BackendConfig) -> AsyncGenerator[Backend, None]:
    # pytest-asyncio use different coroutines to run the init and teardown parts
    # of async generator fixtures.
    # However anyio's task group are required to run there async context manager init
    # and teardown from the same coroutine.
    # So the solution is to start a dedicated task.

    started = asyncio.Event()
    should_stop = asyncio.Event()
    backend = None

    async def _run_backend():
        nonlocal backend
        async with backend_factory(config=backend_config) as backend:
            started.set()
            await should_stop.wait()

    async with asyncio.TaskGroup() as tg:
        tg.create_task(_run_backend())
        await started.wait()
        assert isinstance(backend, Backend)
        yield backend
        should_stop.set()


@pytest.fixture
def app(backend: Backend, monkeypatch: pytest.MonkeyPatch) -> Iterator[AsgiApp]:
    # `FastAPI.state` stays between runs, so must clean it manually
    asgi_app.state._state.clear()
    asgi_app.state.backend = backend
    yield asgi_app


@pytest.fixture
def testbed(backend: Backend) -> TestbedBackend:
    return TestbedBackend(backend)


@pytest.fixture
def ballpark_always_ok(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr("parsec.ballpark.BALLPARK_ALWAYS_OK", True)
