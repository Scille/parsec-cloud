# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import math
import tempfile
from pathlib import Path
from typing import Any

import click

from parsec._parsec import ActiveUsersLimit, ParsecAddr
from parsec.asgi import app as parsec_app
from parsec.asgi import serve_parsec_asgi_app
from parsec.backend import backend_factory
from parsec.cli.options import (
    _split_with_escaping,
    blockstore_server_options,
    db_server_options,
    debug_config_options,
    logging_config_options,
    sentry_config_options,
)
from parsec.cli.utils import (
    cli_exception_handler,
)
from parsec.components.organization import TosLocale, TosUrl
from parsec.config import (
    BackendConfig,
    BaseBlockStoreConfig,
    BaseDatabaseConfig,
    EmailConfig,
    LogLevel,
    MockedEmailConfig,
    PostgreSQLDatabaseConfig,
    SmtpEmailConfig,
)
from parsec.logging import get_logger

logger = get_logger()

DEFAULT_PORT = 6777
DEFAULT_EMAIL_SENDER = "no-reply@parsec.com"


def _parse_organization_initial_tos_url(raw_param: str | None) -> dict[TosLocale, TosUrl] | None:
    if raw_param is None:
        return None

    locales = {}

    # 1) Split the comma-separated list of `<locale>:<url>`

    for item in _split_with_escaping(raw_param, ","):
        # 2) Split the `<locale>:<url>` pair
        try:
            locale, url = item.split(":", 1)
            if not locale or not url:
                raise ValueError
        except ValueError:
            raise click.BadParameter(
                "Invalid format, should be comma-separated list of `<locale>:<url>`"
            )

        locales[locale] = url

    return locales


class DevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            if "port" in opts:
                server_addr = "parsec3://localhost:" + str(opts["port"])
            else:
                server_addr = "parsec3://localhost:" + str(DEFAULT_PORT)
            for key, value in (
                ("debug", True),
                ("db", "MOCKED"),
                ("server_addr", server_addr),
                ("email_sender", "no-reply@parsec.com"),
                ("email_host", "MOCKED"),
                ("blockstore", ("MOCKED",)),
                ("administration_token", "s3cr3t"),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(
    context_settings={"max_content_width": 400},
    short_help="run the server",
    epilog="""Note each parameter has a corresponding environ variable with the `PARSEC_` prefix
(e.g. `--email-port=42` parameter is equivalent to environ variable `PARSEC_EMAIL_PORT=42`).

\b
Parameters can also be specified by using the special environment variable `PARSEC_CMD_ARGS`.
All available command line arguments can be used and environ variables within it will be expanded.
For instance:

    $ DB_URL=postgres:///parsec PARSEC_CMD_ARGS='--db=$DB_URL --host=0.0.0.0' parsec run
""",
)
@click.option(
    "--host",
    "-H",
    default="127.0.0.1",
    show_default=True,
    envvar="PARSEC_HOST",
    show_envvar=True,
    help="Host to listen on",
)
@click.option(
    "--port",
    "-P",
    default=DEFAULT_PORT,
    type=int,
    show_default=True,
    envvar="PARSEC_PORT",
    show_envvar=True,
    help="Port to listen on",
)
@db_server_options
@click.option(
    "--maximum-database-connection-attempts",
    default=10,
    show_default=True,
    envvar="PARSEC_MAXIMUM_DATABASE_CONNECTION_ATTEMPTS",
    show_envvar=True,
    help="Maximum number of attempts at connecting to the database (0 means never retry)",
)
@click.option(
    "--pause-before-retry-database-connection",
    type=float,
    default=1.0,
    show_default=True,
    envvar="PARSEC_PAUSE_BEFORE_RETRY_DATABASE_CONNECTION",
    show_envvar=True,
    help="Number of seconds before a new attempt at connecting to the database",
)
@blockstore_server_options
@click.option(
    "--administration-token",
    required=True,
    envvar="PARSEC_ADMINISTRATION_TOKEN",
    show_envvar=True,
    metavar="TOKEN",
    help="Secret token to access the Administration API",
)
@click.option(
    "--spontaneous-organization-bootstrap",
    envvar="PARSEC_SPONTANEOUS_ORGANIZATION_BOOTSTRAP",
    show_envvar=True,
    is_flag=True,
    help="""Allow organization bootstrap without prior creation.

Without this flag, an organization must be created by administration (see
 `parsec core create_organization` command) before bootstrap can occur.

With this flag, the server allows anybody to bootstrap an organization
by providing an empty bootstrap token given 1) the organization is not bootstrapped yet
and 2) the organization hasn't been created by administration (which would act as a
reservation and change the bootstrap token)
""",
)
@click.option(
    "--organization-bootstrap-webhook",
    envvar="PARSEC_ORGANIZATION_BOOTSTRAP_WEBHOOK",
    show_envvar=True,
    metavar="URL",
    help="""URL to notify 3rd party service that a new organization has been bootstrapped.

Each time an organization is bootstrapped, an HTTP POST will be send to the URL
with an `application/json` body with the following fields:
organization_id, device_id, device_label (can be null), human_email (can be null), human_label (can be null)
""",
)
@click.option(
    "--organization-initial-active-users-limit",
    envvar="PARSEC_ORGANIZATION_INITIAL_ACTIVE_USERS_LIMIT",
    show_envvar=True,
    help="Non-revoked users limit used to configure newly created organizations (default: no limit)",
    type=int,
)
@click.option(
    "--organization-initial-user-profile-outsider-allowed",
    envvar="PARSEC_ORGANIZATION_INITIAL_USER_PROFILE_OUTSIDER_ALLOWED",
    show_envvar=True,
    help="Allow the outsider profiles for the newly created organizations (default: True)",
    default=True,
    type=bool,
)
@click.option(
    "--organization-initial-tos",
    envvar="PARSEC_ORGANIZATION_INITIAL_TOS",
    show_envvar=True,
    callback=lambda ctx, param, value: _parse_organization_initial_tos_url(value),
    help="""Terms Of Service (TOS) used to configure newly created organizations (default: no TOS)

The TOS should be provided as a comma-separated list of `<locale>:<url>` (with any comma
in the URL escaped with a backslash).

For instance: `en_US:https://example.com/tos_en,fr_FR:https://example.com/tos_fr`.
""",
    default=None,
)
@click.option(
    "--server-addr",
    envvar="PARSEC_SERVER_ADDR",
    show_envvar=True,
    required=True,
    metavar="URL",
    type=ParsecAddr.from_url,
    help="URL to reach this server (typically used in invitation emails)",
)
@click.option(
    "--email-host",
    envvar="PARSEC_EMAIL_HOST",
    show_envvar=True,
    required=True,
    help="The host to use for sending email, set to `MOCKED` to output emails to temporary file",
)
@click.option(
    "--email-port",
    envvar="PARSEC_EMAIL_PORT",
    show_envvar=True,
    type=int,
    default=25,
    show_default=True,
    help="Port to use for the SMTP server defined in EMAIL_HOST",
)
@click.option(
    "--email-host-user",
    envvar="PARSEC_EMAIL_HOST_USER",
    help="Username to use for the SMTP server defined in EMAIL_HOST",
)
@click.option(
    "--email-host-password",
    envvar="PARSEC_EMAIL_HOST_PASSWORD",
    show_envvar=True,
    help=(
        "Password to use for the SMTP server defined in EMAIL_HOST."
        " This setting is used in conjunction with EMAIL_HOST_USER when authenticating to the SMTP server."
    ),
)
@click.option(
    "--email-use-ssl",
    envvar="PARSEC_EMAIL_USE_SSL",
    show_envvar=True,
    is_flag=True,
    help=(
        "Whether to use a TLS (secure) connection when talking to the SMTP server."
        " This is used for explicit TLS connections, generally on port 587."
    ),
)
@click.option(
    "--email-use-tls",
    envvar="PARSEC_EMAIL_USE_TLS",
    show_envvar=True,
    is_flag=True,
    help=(
        "Whether to use an implicit TLS (secure) connection when talking to the SMTP server."
        " In most email documentation this type of TLS connection is referred to as SSL."
        " It is generally used on port 465."
        " Note that --email-use-tls/--email-use-ssl are mutually exclusive,"
        " so only set one of those settings to True."
    ),
)
@click.option(
    "--email-sender",
    envvar="PARSEC_EMAIL_SENDER",
    show_envvar=True,
    metavar="EMAIL",
    help="Sender address used in sent emails",
)
@click.option(
    "--proxy-trusted-addresses",
    envvar="PARSEC_PROXY_TRUSTED_ADDRESSES",
    type=str,
    show_envvar=True,
    help="""\b
        Comma-separated list of IP Addresses, IP Networks or literals to trust with proxy headers.
        Use "*" to trust all proxies. If not provided, the gunicorn/uvicorn `FORWARDED_ALLOW_IPS`
        environment variable is used, defaulting to trusting only localhost if absent.
        """,
)
@click.option(
    "--ssl-keyfile",
    type=click.Path(exists=True, dir_okay=False),
    envvar="PARSEC_SSL_KEYFILE",
    show_envvar=True,
    help="SSL key file. This setting enables serving Parsec over SSL.",
)
@click.option(
    "--ssl-certfile",
    type=click.Path(exists=True, dir_okay=False),
    envvar="PARSEC_SSL_CERTFILE",
    show_envvar=True,
    help="SSL certificate file. This setting enables serving Parsec over SSL.",
)
# Add --log-level/--log-format/--log-file
@logging_config_options(default_log_level="INFO")
# Add --sentry-url
@sentry_config_options(configure_sentry=True)
@click.option(
    "--dev",
    cls=DevOption,
    is_flag=True,
    is_eager=True,
    help=(
        "Equivalent to `--debug --db=MOCKED"
        " --blockstore=MOCKED --administration-token=s3cr3t"
        " --email-sender=no-reply@parsec.com --email-host=MOCKED"
        " --server-addr=parsec3://localhost:<port>(?no_ssl=False if ssl is not set)`"
    ),
)
@click.option(
    "--sse-keepalive",
    default=30,
    show_default=True,
    type=float,
    callback=lambda ctx, param, value: math.inf if value is None or value <= 0 else value,
    envvar="PARSEC_SSE_KEEPALIVE",
    show_envvar=True,
    help="Keep SSE connection open by sending keepalive messages to client (pass <= 0 to disable)",
)
# Add --debug
@debug_config_options
def run_cmd(
    host: str,
    port: int,
    db: BaseDatabaseConfig,
    db_min_connections: int,
    db_max_connections: int,
    sse_keepalive: float,
    maximum_database_connection_attempts: int,
    pause_before_retry_database_connection: float,
    blockstore: BaseBlockStoreConfig,
    administration_token: str,
    spontaneous_organization_bootstrap: bool,
    organization_bootstrap_webhook: str | None,
    organization_initial_active_users_limit: int | None,
    organization_initial_user_profile_outsider_allowed: bool,
    organization_initial_tos: dict[TosLocale, TosUrl] | None,
    server_addr: ParsecAddr,
    email_host: str,
    email_port: int,
    email_host_user: str | None,
    email_host_password: str | None,
    email_use_ssl: bool,
    email_use_tls: bool,
    email_sender: str | None,
    proxy_trusted_addresses: str | None,
    ssl_keyfile: Path | None,
    ssl_certfile: Path | None,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    sentry_dsn: str | None,
    sentry_environment: str,
    debug: bool,
    dev: bool,
) -> None:
    # Set min and max connections
    if isinstance(db, PostgreSQLDatabaseConfig):
        db.min_connections = db_min_connections
        db.max_connections = db_max_connections

    # Start a local server

    with cli_exception_handler(debug):
        email_config: EmailConfig
        if email_host == "MOCKED":
            tmpdir = tempfile.mkdtemp(prefix="tmp-email-folder-")
            if email_sender:
                email_config = MockedEmailConfig(sender=email_sender, tmpdir=tmpdir)
            else:
                email_config = MockedEmailConfig(sender=DEFAULT_EMAIL_SENDER, tmpdir=tmpdir)
        else:
            if not email_sender:
                raise ValueError("--email-sender is required when --email-host is provided")
            email_config = SmtpEmailConfig(
                host=email_host,
                port=email_port,
                host_user=email_host_user,
                host_password=email_host_password,
                use_ssl=email_use_ssl,
                use_tls=email_use_tls,
                sender=email_sender,
            )

        app_config = BackendConfig(
            administration_token=administration_token,
            db_config=db,
            sse_keepalive=sse_keepalive,
            blockstore_config=blockstore,
            email_config=email_config,
            proxy_trusted_addresses=proxy_trusted_addresses,
            server_addr=server_addr,
            debug=debug,
            organization_bootstrap_webhook_url=organization_bootstrap_webhook,
            organization_spontaneous_bootstrap=spontaneous_organization_bootstrap,
            organization_initial_active_users_limit=ActiveUsersLimit.limited_to(
                organization_initial_active_users_limit
            )
            if organization_initial_active_users_limit is not None
            else ActiveUsersLimit.NO_LIMIT,
            organization_initial_user_profile_outsider_allowed=organization_initial_user_profile_outsider_allowed,
            organization_initial_tos=organization_initial_tos,
        )

        click.echo(
            f"Starting Parsec server on {host}:{port}"
            f"(db={app_config.db_config.type}"
            f" blockstore={app_config.blockstore_config.type}"
            f" email={email_config.type}"
            f" telemetry={'on' if sentry_dsn else 'off'}"
            f" server_addr={app_config.server_addr.to_url() if app_config.server_addr else ''}"
            ")"
        )
        try:
            retry_policy = RetryPolicy(
                maximum_database_connection_attempts, pause_before_retry_database_connection
            )
            asyncio.run(
                _run_backend(
                    host=host,
                    port=port,
                    ssl_certfile=ssl_certfile,
                    ssl_keyfile=ssl_keyfile,
                    retry_policy=retry_policy,
                    app_config=app_config,
                )
            )
        # Ignore some noisy cancellation
        # Note that this is no longer necessary with the latest anyio version (version 4.6)
        # This can be verified using the following protocol:
        # - Run the parsec server with a postgre database
        # - Stop the postgre server
        # - Start the postgre server
        # - The parsec server should restart automatically
        # - Stop the parsec server with a KeyboardInterrupt
        # - Check that the cancelled error does not appear
        except asyncio.CancelledError:
            pass
        finally:
            click.echo("bye ;-)")


class RetryPolicy:
    def __init__(self, maximum_attempts: int = 10, pause_before_retry: float = 1.0):
        self.maximum_attempts = maximum_attempts
        self.pause_before_retry = pause_before_retry
        self.current_attempt = 0  # No attempt at the moment

    def new_attempt(self) -> None:
        self.current_attempt += 1

    def success(self) -> None:
        self.current_attempt = 0

    def is_expired(self) -> bool:
        return self.current_attempt >= self.maximum_attempts

    async def pause(self) -> None:
        await asyncio.sleep(self.pause_before_retry)


async def _run_backend(
    host: str,
    port: int,
    ssl_certfile: Path | None,
    ssl_keyfile: Path | None,
    retry_policy: RetryPolicy,
    app_config: BackendConfig,
) -> None:
    # Loop over connection attempts
    while True:
        try:
            # New connection attempt
            retry_policy.new_attempt()

            # Run the backend app (and connect to the database)
            async with backend_factory(config=app_config) as backend:
                # Connection is successful, reset the retry policy
                retry_policy.success()

                # Serve backend through TCP
                parsec_app.state.backend = backend
                await serve_parsec_asgi_app(
                    app=parsec_app,
                    host=host,
                    port=port,
                    ssl_certfile=ssl_certfile,
                    ssl_keyfile=ssl_keyfile,
                    proxy_trusted_addresses=app_config.proxy_trusted_addresses,
                )
                return

        except ConnectionError as exc:
            # The maximum number of attempt is reached
            if retry_policy.is_expired():
                raise
            # Connection with the DB is dead, restart everything
            logger.warning(
                f"Database connection lost ({exc}), retrying in {retry_policy.pause_before_retry} seconds"
            )
            await retry_policy.pause()
