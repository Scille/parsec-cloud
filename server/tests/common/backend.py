# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import asyncio
from collections.abc import AsyncIterator, Awaitable, Callable

import pytest

from parsec._parsec import DateTime, EmailAddress, ParsecAddr, SecretKey
from parsec.asgi import AsgiApp, app_factory
from parsec.backend import Backend, backend_factory
from parsec.cli.testbed import TestbedBackend, TestbedTemplate
from parsec.components.memory.organization import MemoryOrganization, OrganizationID
from parsec.config import (
    BackendConfig,
    BaseBlockStoreConfig,
    BaseDatabaseConfig,
    MockedEmailConfig,
)
from parsec.templates import get_environment
from tests.common.postgresql import reset_postgresql_testbed

SERVER_DOMAIN = "parsec.invalid"


@pytest.fixture(scope="session")
def backend_mocked_data() -> dict[OrganizationID, MemoryOrganization]:
    return {}


@pytest.fixture
def backend_config(
    db_config: BaseDatabaseConfig,
    blockstore_config: BaseBlockStoreConfig,
    backend_mocked_data: dict[OrganizationID, MemoryOrganization],
) -> BackendConfig:
    return BackendConfig(
        jinja_env=get_environment(None),
        debug=True,
        db_config=db_config,
        sse_keepalive=30,
        proxy_trusted_addresses=None,
        server_addr=ParsecAddr(hostname=SERVER_DOMAIN, port=None, use_ssl=True),
        email_config=MockedEmailConfig(EmailAddress("no-reply@parsec.com")),
        blockstore_config=blockstore_config,
        administration_token="s3cr3t",
        fake_account_password_algorithm_seed=SecretKey(b"F" * 32),
        organization_spontaneous_bootstrap=False,
        organization_bootstrap_webhook_url=None,
        backend_mocked_data=backend_mocked_data,
        email_rate_limit_cooldown_delay=60,
        email_rate_limit_max_per_hour=3,
    )


@pytest.fixture
async def backend(backend_config: BackendConfig) -> AsyncIterator[Backend]:
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


# Fastapi's ASGI app object is very costly to build: the tests are basically
# 2x slower without this caching optimization !
__backend_cache = None


@pytest.fixture
def app(backend: Backend, monkeypatch: pytest.MonkeyPatch) -> AsgiApp:
    global __backend_cache
    if __backend_cache is None:
        asgi_app = __backend_cache = app_factory(backend)
    else:
        # `FastAPI.state` stays between runs, so must clean it manually
        asgi_app = __backend_cache
        asgi_app.state._state.clear()
        asgi_app.state.backend = backend
    return asgi_app


# TODO: Replace with `type` once the linter supports it
type TestbedTemplateDict = dict[str, TestbedTemplate]


@pytest.fixture(scope="session")
def loaded_templates() -> TestbedTemplateDict:
    # Session cache for loaded templates
    return {}


@pytest.fixture
def testbed(backend: Backend, loaded_templates: TestbedTemplateDict) -> TestbedBackend:
    return TestbedBackend(backend, loaded_templates)


@pytest.fixture
def ballpark_always_ok(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr("parsec.ballpark.BALLPARK_ALWAYS_OK", True)


@pytest.fixture
def timestamp_out_of_ballpark() -> DateTime:
    return DateTime.now().subtract(seconds=3600)


@pytest.fixture
async def reset_testbed(
    request: pytest.FixtureRequest,
    loaded_templates: dict[str, str],
    backend_mocked_data: dict[OrganizationID, MemoryOrganization],
) -> Callable[[], Awaitable[None]]:
    """Fixture providing a helper to fully reset the testbed.

    This is **not** done between all tests in order to speed up the test suite (
    template are loaded once).

    You probably don't need it, this code is only kept given it is useful when
    debugging a test.
    """

    async def _reset_testbed():
        loaded_templates.clear()
        backend_mocked_data.clear()
        if request.config.getoption("--postgresql"):
            await reset_postgresql_testbed()

    return _reset_testbed
