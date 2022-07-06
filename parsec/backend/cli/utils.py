# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import click
from functools import wraps

from parsec.backend.app import backend_app_factory
from parsec.backend.config import BackendConfig


# Simplified options to create a backend config for interactive use (i.e. when
# executing a one-shot command, by opposition of starting the Parsec server)
def interactive_backend_config_options(blockstore: bool = False):
    decorators = [
        click.option(
            "--db",
            required=True,
            envvar="PARSEC_DB",
            metavar="URL",
            help="""Database configuration.
    Allowed values:
    \b
    -`MOCKED`: Mocked in memory
    -`postgresql://<...>`: Use PostgreSQL database
    \b
    """,
        ),
        click.option(
            "--db-min-connections",
            default=5,
            show_default=True,
            envvar="PARSEC_DB_MIN_CONNECTIONS",
            help="Minimal number of connections to the database if using PostgreSQL",
        ),
        click.option(
            "--db-max-connections",
            default=7,
            show_default=True,
            envvar="PARSEC_DB_MAX_CONNECTIONS",
            help="Maximum number of connections to the database if using PostgreSQL",
        ),
    ]

    def _interactive_backend_config_options(fn):
        for decorator in decorators:
            fn = decorator(fn)
        @wraps(fn)
        def wrapper(
            db,
            db_min_connections,
            db_max_connections,
            **kwargs
        ):
            kwargs["backend_config"] = BackendConfig(
                db_url=db,
                db_min_connections=db_min_connections,
                db_max_connections=db_max_connections,

        blockstore_config: BaseBlockStoreConfig

        @property
        def administration_token(self) -> str:
            raise RuntimeError("Not available for interactive use")

        email_config: Union[SmtpEmailConfig, MockedEmailConfig]
        ssl_context: bool
        forward_proto_enforce_https: Optional[Tuple[bytes, bytes]]
        backend_addr: Optional[BackendAddr]

        debug: bool

        organization_bootstrap_webhook_url: Optional[str] = None
        organization_spontaneous_bootstrap: bool = False
        organization_initial_active_users_limit: Optional[int] = None
        organization_initial_user_profile_outsider_allowed: bool = True
            )

        return fn

    return _interactive_backend_config_options


@asynccontextmanager
async def run_pg_sequester_component(config: BackendDbConfig):
    event_bus = EventBus()
    dbh = PGHandler(config.db_url, config.db_min_connections, config.db_max_connections, event_bus)
    sequester_component = PGPSequesterComponent(dbh)

    async with open_service_nursery() as nursery:
        await dbh.init(nursery)
        try:
            yield sequester_component
        finally:
            await dbh.teardown()

