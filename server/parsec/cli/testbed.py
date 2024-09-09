# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import asyncio
import tempfile
from contextlib import asynccontextmanager
from typing import Any, AsyncIterator, TypeAlias

import anyio
import click
import structlog
from fastapi import BackgroundTasks, Request, Response
from fastapi.middleware.cors import CORSMiddleware

from parsec._parsec import OrganizationID, ParsecAddr

try:
    from parsec._parsec import testbed

    TESTBED_AVAILABLE = True

    TestbedTemplateContent: TypeAlias = testbed.TestbedTemplateContent  # pyright: ignore[reportRedeclaration]
    TestbedTemplate: TypeAlias = tuple[OrganizationID, int, testbed.TestbedTemplateContent]  # pyright: ignore[reportRedeclaration]
except ImportError:
    TESTBED_AVAILABLE = False
    TestbedTemplate: TypeAlias = tuple[OrganizationID, int, Any]  # pyright: ignore[reportRedeclaration]
    TestbedTemplateContent: TypeAlias = Any  # pyright: ignore[reportRedeclaration]

from parsec.asgi import app_factory, serve_parsec_asgi_app
from parsec.backend import Backend, backend_factory
from parsec.cli.options import debug_config_options, logging_config_options
from parsec.cli.utils import cli_exception_handler
from parsec.config import (
    BackendConfig,
    LogLevel,
    MockedBlockStoreConfig,
    MockedDatabaseConfig,
    MockedEmailConfig,
    PostgreSQLBlockStoreConfig,
    PostgreSQLDatabaseConfig,
)

logger: structlog.stdlib.BoundLogger = structlog.get_logger()


DEFAULT_PORT = 6770


class TestbedNotAvailable(Exception):
    pass


class UnknownTemplateError(Exception):
    pass


class TestbedBackend:
    __test__ = False  # Prevents Pytest from thinking this is a test class

    def __init__(
        self,
        backend: Backend,
        loaded_templates: dict[str, TestbedTemplate] | None = None,
    ):
        self.backend = backend
        self._org_count = 0
        self._load_template_lock = anyio.Lock()
        # keys: template ID, values: (to duplicate organization ID, CRC, template content)
        self._loaded_templates = {} if loaded_templates is None else loaded_templates
        self.template_per_org: dict[OrganizationID, TestbedTemplateContent] = {}

    async def get_template(self, template: str) -> TestbedTemplate:
        try:
            return self._loaded_templates[template]
        except KeyError:
            async with self._load_template_lock:
                # Ensure the template hasn't been loaded while we were waiting for the lock
                try:
                    return self._loaded_templates[template]

                except KeyError:
                    if not TESTBED_AVAILABLE:
                        raise TestbedNotAvailable()

                    # If it exists, template has not been loaded yet
                    maybe_template_content = testbed.test_get_testbed_template(  # pyright: ignore [reportPossiblyUnboundVariable]
                        template
                    )

                    if not maybe_template_content:
                        # No template with the given id
                        raise UnknownTemplateError(template)
                    template_content = maybe_template_content

                    template_crc = template_content.compute_crc()
                    template_org_id = await self.backend.test_load_template(template_content)
                    ret = (
                        template_org_id,
                        template_crc,
                        template_content,
                    )
                    self._loaded_templates[template] = ret
                    return ret

    async def new_organization(self, template: str) -> TestbedTemplate:
        template_org_id, template_crc, template_content = await self.get_template(template)

        self._org_count += 1
        new_org_id = OrganizationID(f"Org{self._org_count}")
        await self.backend.test_duplicate_organization(template_org_id, new_org_id)
        self.template_per_org[new_org_id] = template_content

        return (new_org_id, template_crc, template_content)

    async def customize_organization(self, id: OrganizationID, customization: bytes) -> None:
        template = self.template_per_org[id]
        cooked_customization = testbed.test_load_testbed_customization(template, customization)  # pyright: ignore [reportPossiblyUnboundVariable]
        await self.backend.test_customize_organization(id, template, cooked_customization)

    async def drop_organization(self, id: OrganizationID) -> None:
        await self.backend.test_drop_organization(id)
        del self.template_per_org[id]


app = app_factory()


# Testbed server often runs in background, so it output on crash is often
# not visible (e.g. on the CI). Hence it's convenient to have the client
# print the stacktrace on our behalf.
# Note the testbed server is only meant to be run for tests and on a local
# local machine so this has no security implication.
app.debug = True


# Must be overwritten before the app is started !
app.state.testbed = None

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


# We don't use json in the /testbed/... routes, this is to simplify
# as much as possible implementation on the client side


@app.post("/testbed/new/{template}")
async def test_new(template: str, request: Request, background_tasks: BackgroundTasks) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        (new_org_id, template_crc, _) = await testbed.new_organization(template)
    except UnknownTemplateError:
        return Response(
            status_code=404,
            content=b"unknown template",
        )

    match request.query_params.get("ttl"):
        case None as orga_life_limit:
            pass
        case str() as raw:
            try:
                orga_life_limit = float(raw)
            except ValueError:
                return Response(
                    status_code=400,
                    content=b"invalid ttl query param",
                )

    if orga_life_limit:

        async def _organization_garbage_collector():
            await asyncio.sleep(orga_life_limit)
            logger.info("Dropping testbed org due to time limit", organization=new_org_id.str)
            # Dropping is idempotent, so no need for error handling
            await testbed.backend.test_drop_organization(new_org_id)

        background_tasks.add_task(_organization_garbage_collector)

    return Response(
        status_code=200,
        content=f"{new_org_id.str}\n{template_crc}".encode("utf8"),
    )


@app.post("/testbed/customize/{raw_organization_id}")
async def test_customize(raw_organization_id: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        return Response(status_code=400, content=b"")

    customization = await request.body()
    await testbed.customize_organization(organization_id, customization)

    return Response(status_code=200, content=b"")


@app.post("/testbed/drop/{raw_organization_id}")
async def test_drop(raw_organization_id: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        return Response(status_code=400, content=b"")

    # Dropping is idempotent, so no need for error handling
    await testbed.drop_organization(organization_id)

    return Response(status_code=200, content=b"")


@asynccontextmanager
async def testbed_backend_factory(
    server_addr: ParsecAddr, with_postgresql: str | None
) -> AsyncIterator[TestbedBackend]:
    blockstore_config = (
        MockedBlockStoreConfig() if with_postgresql is None else PostgreSQLBlockStoreConfig()
    )
    # Same as the defaults in `db_server_options`
    if with_postgresql is None:
        db_config = MockedDatabaseConfig()
    else:
        db_config = PostgreSQLDatabaseConfig(
            url=with_postgresql, min_connections=5, max_connections=7
        )

    # TODO: avoid tempdir for email ?
    tmpdir = tempfile.mkdtemp(prefix="tmp-email-folder-")
    config = BackendConfig(
        debug=True,
        db_config=db_config,
        sse_keepalive=30,
        forward_proto_enforce_https=None,
        server_addr=server_addr,
        email_config=MockedEmailConfig("no-reply@parsec.com", tmpdir),
        blockstore_config=blockstore_config,
        administration_token="s3cr3t",
        organization_spontaneous_bootstrap=True,
    )
    async with backend_factory(config=config) as backend:
        yield TestbedBackend(backend=backend)


@click.command(
    context_settings={"max_content_width": 400},
    short_help="run the testbed server",
)
@click.option(
    "--host",
    "-H",
    default="127.0.0.1",
    show_default=True,
    envvar="PARSEC_HOST",
    help="Host to listen on",
)
@click.option(
    "--port",
    "-P",
    default=DEFAULT_PORT,
    type=int,
    show_default=True,
    envvar="PARSEC_PORT",
    help="Port to listen on",
)
@click.option(
    "--server-addr",
    envvar="PARSEC_SERVER_ADDR",
    default="parsec3://saas.parsec.invalid",
    show_default=True,
    metavar="URL",
    type=ParsecAddr.from_url,
    help="URL to reach this server (typically used in invitation emails)",
)
@click.option(
    "--with-postgresql",
    envvar="PARSEC_WITH_POSTGRESQL",
    default=None,
    show_default=True,
    metavar="WITH_POSTGRESQL",
    help="Use a postgresql database instead of the mocked one",
)
@click.option(
    "--stop-after-process",
    type=int,
    default=None,
    show_default=True,
    envvar="PARSEC_STOP_AFTER_PROCESS",
    help="Stop the server once the given process has terminated",
)
# Add --log-level/--log-format/--log-file
@logging_config_options(default_log_level="INFO")
# Add --debug
@debug_config_options
def testbed_cmd(
    host: str,
    port: int,
    server_addr: ParsecAddr,
    with_postgresql: str | None,
    stop_after_process: int | None,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    debug: bool,
) -> None:
    async def _run_testbed():
        # Task group must be enclosed by backend (and not the other way around !)
        # given we will sleep forever in it __aexit__ part
        async with anyio.create_task_group() as tg:
            if stop_after_process:

                async def _watch_and_stop_after_process(pid: int, cancel_scope: anyio.CancelScope):
                    while True:
                        await anyio.sleep(1)
                        # psutil is a dev dependency, so we cannot import it globally
                        import psutil

                        if not psutil.pid_exists(pid):
                            print(f"PID `{pid}` has left, closing server.")
                            cancel_scope.cancel()
                            break

                tg.start_soon(_watch_and_stop_after_process, stop_after_process, tg.cancel_scope)

            async with testbed_backend_factory(
                server_addr=server_addr, with_postgresql=with_postgresql
            ) as testbed:
                click.secho("All set !", fg="yellow")
                click.echo("Don't forget to export `TESTBED_SERVER` environ variable:")
                click.secho(
                    f"export TESTBED_SERVER='parsec3://127.0.0.1:{port}?no_ssl=true'",
                    fg="magenta",
                )

                app.state.testbed = testbed
                app.state.backend = testbed.backend
                await serve_parsec_asgi_app(host=host, port=port, app=app)

                click.echo("bye ;-)")

    with cli_exception_handler(debug):
        asyncio.run(_run_testbed())
