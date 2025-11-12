# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import asyncio
import hashlib
import uuid
from base64 import b64decode, b64encode
from collections import defaultdict
from collections.abc import AsyncIterator, Callable
from contextlib import asynccontextmanager
from typing import TYPE_CHECKING, Any

import anyio
import click
import structlog
from fastapi import APIRouter, BackgroundTasks, Request, Response
from fastapi.middleware.cors import CORSMiddleware

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    HumanHandle,
    OrganizationID,
    ParsecAddr,
    SecretKey,
    ValidationCode,
    authenticated_cmds,
)

if TYPE_CHECKING:
    from parsec.api import ApiFn
from parsec.client_context import AuthenticatedClientContext
from parsec.templates import get_environment

try:
    from parsec._parsec import testbed

    TESTBED_AVAILABLE = True

    type TestbedTemplateContent = testbed.TestbedTemplateContent  # pyright: ignore[reportRedeclaration]
    type TestbedTemplate = tuple[OrganizationID, int, testbed.TestbedTemplateContent]  # pyright: ignore[reportRedeclaration]
except ImportError:
    TESTBED_AVAILABLE = False
    type TestbedTemplate = tuple[OrganizationID, int, Any]
    type TestbedTemplateContent = Any

from parsec.asgi import AsgiApp, app_factory, serve_parsec_asgi_app
from parsec.backend import Backend, backend_factory
from parsec.cli.options import debug_config_options, logging_config_options
from parsec.cli.utils import cli_exception_handler
from parsec.config import (
    AllowedClientAgent,
    BackendConfig,
    LogLevel,
    MockedBlockStoreConfig,
    MockedDatabaseConfig,
    MockedEmailConfig,
    MockedSentEmail,
    OpenBaoAuthConfig,
    OpenBaoAuthType,
    OpenBaoConfig,
    PostgreSQLBlockStoreConfig,
    PostgreSQLDatabaseConfig,
)

logger: structlog.stdlib.BoundLogger = structlog.get_logger()


# Helper for other CLI commands to add dev-related options
def if_testbed_available[FC](decorator: Callable[[FC], FC]) -> Callable[[FC], FC]:
    if TESTBED_AVAILABLE:
        return decorator
    else:
        return lambda f: f


DEFAULT_PORT = 6770


class TestbedNotAvailable(Exception):
    pass


class UnknownTemplateError(Exception):
    pass


_ORG_ID_COUNT = 0


def next_organization_id(prefix: str) -> OrganizationID:
    """
    Convenient helper to generate unique organization IDs for the tests.

    Note the returned IDs are only unique across the lifetime of the process,
    but this is enough since the PostgreSQL database gets reset before each run.
    """
    global _ORG_ID_COUNT
    _ORG_ID_COUNT += 1
    return OrganizationID(f"{prefix}{_ORG_ID_COUNT}")


class TestbedBackend:
    __test__ = False  # Prevents Pytest from thinking this is a test class

    def __init__(
        self,
        backend: Backend,
        loaded_templates: dict[str, TestbedTemplate] | None = None,
    ):
        self.backend = backend
        self._account_count = 0
        self._load_template_lock = anyio.Lock()
        # keys: template ID, values: (to duplicate organization ID, CRC, template content)
        self._loaded_templates = {} if loaded_templates is None else loaded_templates
        self.template_per_org: dict[OrganizationID, TestbedTemplateContent] = {}
        # We expose some routes to simulate a OpenBao server, hence this field
        self.openbao_secrets: defaultdict[tuple[str, str], list[Any]] = defaultdict(list)

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
        new_org_id = next_organization_id(prefix="TestbedOrg")
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


testbed_router = APIRouter(tags=["testbed"])


def testbed_app_factory(testbed: TestbedBackend) -> AsgiApp:
    app = app_factory(testbed.backend)

    # Testbed server often runs in background, so it output on crash is often
    # not visible (e.g. on the CI). Hence it's convenient to have the client
    # print the stacktrace on our behalf.
    # Note the testbed server is only meant to be run for tests and on a local
    # local machine so this has no security implication.
    app.debug = True

    app.state.testbed = testbed

    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_methods=["*"],
        allow_headers=["*"],
    )

    app.include_router(testbed_router)

    return app


# We don't use json in the /testbed/... routes, this is to simplify
# as much as possible implementation on the client side


@testbed_router.post("/testbed/new/{template}")
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
        content=f"{new_org_id.str}\n{template_crc}".encode(),
    )


@testbed_router.post("/testbed/customize/{raw_organization_id}")
async def test_customize(raw_organization_id: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        return Response(status_code=400, content=b"")

    customization = await request.body()
    await testbed.customize_organization(organization_id, customization)

    return Response(status_code=200, content=b"")


@testbed_router.post("/testbed/drop/{raw_organization_id}")
async def test_drop(raw_organization_id: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        return Response(status_code=400, content=b"")

    # Dropping is idempotent, so no need for error handling
    await testbed.drop_organization(organization_id)

    return Response(status_code=200, content=b"")


class CorruptedCertificateGet:
    """
    Patch `certificate_get` API command to return an invalid certificate.
    This is useful to test the client behavior when the server is buggy/malicious.
    """

    def __init__(
        self,
        testbed: TestbedBackend,
        vanilla_api_certificate_get: ApiFn,  # pyright: ignore[reportMissingTypeArgument] Req/Rep are currently untyped
    ):
        self.testbed = testbed
        self.vanilla_api_certificate_get = vanilla_api_certificate_get
        self.organizations: set[OrganizationID] = set()

    async def __call__(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.certificate_get.Req,
    ):
        rep = await self.vanilla_api_certificate_get(client_ctx, req)

        if not isinstance(rep, authenticated_cmds.latest.certificate_get.RepOk):
            return rep

        return authenticated_cmds.latest.certificate_get.RepOk(
            common_certificates=[*rep.common_certificates, b"<dummy>"],
            sequester_certificates=rep.sequester_certificates,
            shamir_recovery_certificates=rep.shamir_recovery_certificates,
            realm_certificates=rep.realm_certificates,
        )

    @classmethod
    def get_or_install(cls, testbed: TestbedBackend) -> CorruptedCertificateGet:
        target = testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req]
        if isinstance(target, cls):
            return target

        vanilla_api_certificate_get = target
        corrupted = cls(testbed, vanilla_api_certificate_get)
        testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req] = corrupted
        return corrupted

    @staticmethod
    def is_installed(testbed: TestbedBackend) -> bool:
        target = testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req]
        return isinstance(target, CorruptedCertificateGet)

    @staticmethod
    def uninstall(testbed: TestbedBackend):
        target = testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req]
        assert isinstance(target, CorruptedCertificateGet)
        testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req] = (
            target.vanilla_api_certificate_get
        )


@testbed_router.post("/testbed/corrupt/{raw_organization_id}")
async def test_corruption(raw_organization_id: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        return Response(status_code=400, content=b"")

    req_body = await request.body()
    for corruption in req_body.split(b"\n"):
        match corruption:
            case b"certificates:true":
                corruption = CorruptedCertificateGet.get_or_install(testbed)
                corruption.organizations.add(organization_id)

            case b"certificates:false":
                if CorruptedCertificateGet.is_installed(testbed):
                    corruption = CorruptedCertificateGet.get_or_install(testbed)
                    try:
                        corruption.organizations.remove(organization_id)
                    except KeyError:
                        pass

            case unknown:
                return Response(
                    status_code=400,
                    content=(
                        b"Unknown corruption `"
                        + unknown
                        + b"`, only `certificates:[true|false]` is allowed !"
                    ),
                )

    return Response(status_code=200, content=b"")


def _openbao_entity_id_from_vault_token_header(request: Request) -> str | None:
    auth_token = request.headers.get("x-vault-token", "")
    if not auth_token.startswith("s."):
        return None
    return str(uuid.UUID(bytes=hashlib.sha256(auth_token.encode()).digest()[:16]))


@testbed_router.get("/testbed/mock/openbao/v1/auth/token/lookup-self")
async def test_openbao_auth_token_lookup_self(request: Request):
    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    # See https://openbao.org/api-docs/auth/token/#sample-response-2

    return {
        "data": {
            "accessor": "8609694a-cdbc-db9b-d345-e782dbb562ed",
            "creation_time": 1523979354,
            "creation_ttl": 2764800,
            "display_name": "oidc-tesla",
            "entity_id": entity_id,
            "expire_time": "2018-05-19T11:35:54.466476215-04:00",
            "explicit_max_ttl": 0,
            "id": "cf64a70f-3a12-3f6c-791d-6cef6d390eed",
            "identity_policies": ["dev-group-policy"],
            "issue_time": "2018-04-17T11:35:54.466476078-04:00",
            "meta": {"username": "tesla"},
            "num_uses": 0,
            "orphan": True,
            "path": "auth/oidc/login/tesla",
            "policies": ["default", "testgroup2-policy"],
            "renewable": True,
            "ttl": 2764790,
        }
    }


@testbed_router.get("/testbed/mock/openbao/v1/secret/parsec-keys/data/{secret_path:path}")
async def test_openbao_read_secret(secret_path: str, request: Request):
    testbed: TestbedBackend = request.app.state.testbed

    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    secret_versions = testbed.openbao_secrets[(entity_id, secret_path)]
    if not secret_versions:
        return Response(status_code=404)

    # See https://openbao.org/api-docs/secret/kv/kv-v2/#sample-response-1

    return {
        "data": {
            "data": secret_versions[-1],
            "metadata": {
                "created_time": "2018-03-22T02:24:06.945319214Z",
                "custom_metadata": {"owner": "alice", "mission_critical": "false"},
                "deletion_time": "",
                "destroyed": False,
                "version": len(secret_versions),
            },
        }
    }


@testbed_router.post("/testbed/mock/openbao/v1/secret/parsec-keys/data/{secret_path:path}")
async def test_openbao_create_secret(secret_path: str, request: Request):
    testbed: TestbedBackend = request.app.state.testbed

    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    # See https://openbao.org/api-docs/secret/kv/kv-v2/#sample-payload-1

    req = await request.json()
    cas = req.get("option", {}).get("cas")
    if not (cas is None or isinstance(cas, int)):
        return Response(status_code=400)
    secret_data = req.get("data")

    secret_current_version = len(testbed.openbao_secrets.get((entity_id, secret_path), []))
    if cas is not None and secret_current_version != cas:
        return Response(status_code=409)

    testbed.openbao_secrets[(entity_id, secret_path)].append(secret_data)

    # See https://openbao.org/api-docs/secret/kv/kv-v2/#sample-payload-1

    return {
        "data": {
            "created_time": "2018-03-22T02:36:43.986212308Z",
            "custom_metadata": {"owner": "alice", "mission_critical": "false"},
            "deletion_time": "",
            "destroyed": False,
            "version": 1,
        }
    }


@testbed_router.get("/testbed/mailbox/{raw_recipient}")
async def test_mailbox(raw_recipient: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed
    try:
        recipient = EmailAddress(raw_recipient)
    except ValueError:
        return Response(status_code=400, content=b"invalid email address")

    match testbed.backend.config.email_config:
        case MockedEmailConfig() as email_config:
            sent_emails = (mail for mail in email_config.sent_emails if mail.recipient == recipient)
        case _:
            return Response(
                status_code=400, content=b"mailbox is only available with MockedEmailConfig !"
            )

    # Body should be something like `<sender_email>\t<timestamp>\t<base64(body)`,
    # also multiple mails can be send one after the other separated by `\n`

    def _encode_mail(mail: MockedSentEmail) -> bytes:
        return f"{mail.sender.str}\t{mail.timestamp.to_rfc3339()}\t".encode() + b64encode(
            mail.body.encode("utf8")
        )

    rep_body = b"\n".join(_encode_mail(mail) for mail in sent_emails)

    return Response(status_code=200, content=rep_body)


@testbed_router.post("/testbed/account/new")
async def test_new_account(request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    req_body = await request.body()
    try:
        (
            raw_auth_method_id,
            raw_mac_key,
            raw_vault_key_access,
        ) = req_body.split(b"\n")

        auth_method_id = AccountAuthMethodID.from_hex(raw_auth_method_id.decode("ascii"))
        auth_method_mac_key = SecretKey(b64decode(raw_mac_key))
        vault_key_access = b64decode(raw_vault_key_access)

    except ValueError:
        return Response(
            status_code=400,
            content=b"invalid body, expected `<auth_method_id as hex>\nb64(<mac_key>)\nb64(<vault_key_access>)`",
        )

    testbed._account_count += 1
    human_handle = HumanHandle(
        email=EmailAddress(f"agent{testbed._account_count}@example.com"),
        label=f"Agent{testbed._account_count:0>3}",
    )

    validation_code = await testbed.backend.account.create_send_validation_email(
        DateTime.now(), human_handle.email
    )
    assert isinstance(validation_code, ValidationCode)

    # Discard the mail that have been generated during the account creation
    match testbed.backend.config.email_config:
        case MockedEmailConfig() as email_config:
            email_config.sent_emails = [
                mail for mail in email_config.sent_emails if mail.recipient != human_handle.email
            ]
        case _:
            pass

    outcome = await testbed.backend.account.create_proceed(
        now=DateTime.now(),
        validation_code=validation_code,
        vault_key_access=vault_key_access,
        human_handle=human_handle,
        created_by_user_agent="TestbedAgent",
        created_by_ip="",
        auth_method_id=auth_method_id,
        auth_method_mac_key=auth_method_mac_key,
        auth_method_password_algorithm=None,
    )
    assert outcome is None

    rep_body = f"{human_handle.str}".encode()

    return Response(status_code=200, content=rep_body)


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
            url=with_postgresql, min_connections=1, max_connections=5
        )

    jinja_env = get_environment(None)

    config = BackendConfig(
        jinja_env=jinja_env,
        debug=True,
        db_config=db_config,
        sse_keepalive=30,
        proxy_trusted_addresses=None,
        server_addr=server_addr,
        email_config=MockedEmailConfig(EmailAddress("no-reply@parsec.com")),
        blockstore_config=blockstore_config,
        administration_token="s3cr3t",
        fake_account_password_algorithm_seed=SecretKey(b"F" * 32),
        allowed_client_agent=AllowedClientAgent.NATIVE_OR_WEB,
        organization_spontaneous_bootstrap=True,
        openbao_config=OpenBaoConfig(
            server_url="openbao.parsec.invalid",
            secret_mount_path="secret/parsec-keys",
            auths=[
                OpenBaoAuthConfig(
                    id=OpenBaoAuthType.HEXAGONE,
                    mount_path="auth/oidc/hexagone",
                ),
                OpenBaoAuthConfig(
                    id=OpenBaoAuthType.PRO_CONNECT,
                    mount_path="auth/oidc/pro_connect",
                ),
            ],
        ),
        # Disable the rate limit
        email_rate_limit_cooldown_delay=0,
        email_rate_limit_max_per_hour=0,
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
# Add --debug & --version
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

                app = testbed_app_factory(testbed)
                await serve_parsec_asgi_app(
                    host=host,
                    port=port,
                    app=app,
                    proxy_trusted_addresses=None,
                    ssl_ciphers=["TLSv1"],
                )

                click.echo("bye ;-)")

    with cli_exception_handler(debug):
        asyncio.run(_run_testbed())
