# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from asyncio import TaskGroup
from collections.abc import Callable, Coroutine
from typing import Any

import click

from parsec._parsec import EmailAddress, OrganizationID, ParsecAddr, SecretKey
from parsec.backend import backend_factory
from parsec.cli.options import (
    asyncio_run,
    blockstore_server_options,
    db_server_options,
    debug_config_options,
    logging_config_options,
    sentry_config_options,
)
from parsec.components.blockstore import BaseBlockStoreComponent
from parsec.components.organization import BaseOrganizationComponent, WipeOrganizationBadOutcome
from parsec.config import (
    BackendConfig,
    BaseBlockStoreConfig,
    BaseDatabaseConfig,
    LogLevel,
    MockedEmailConfig,
)
from parsec.logging import get_logger

logger = get_logger()


@click.command(name="wipe-organization", short_help="Wipe organization from server")
@click.argument("organizations", type=OrganizationID, nargs=-1)
@db_server_options
@blockstore_server_options
# Add --log-level/--log-format/--log-file
@logging_config_options(default_log_level="INFO")
# Add --sentry-environment/--sentry-dsn
@sentry_config_options
@debug_config_options
@click.pass_context
@asyncio_run
async def cmd(
    ctx: click.Context,
    organizations: list[OrganizationID],
    db: BaseDatabaseConfig,
    db_min_connections: int,
    db_max_connections: int,
    blockstore: BaseBlockStoreConfig,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    sentry_dsn: str | None,
    sentry_environment: str,
    sentry_traces_sample_rate: float | None,
    sentry_profiles_sample_rate: float | None,
    configure_sentry: Callable[[], Coroutine[Any, Any, None]],
    debug: bool,
):
    # Early exit is not organizations is provided
    if not organizations:
        return

    logger.info("Will wipe organizations", count=len(organizations))
    await configure_sentry()
    config = BackendConfig(
        debug=False,
        db_config=db,
        blockstore_config=blockstore,
        email_config=MockedEmailConfig(sender=EmailAddress("tasks@parsec.local")),
        server_addr=ParsecAddr("tasks.parsec.local", None, True),
        administration_token="",
        fake_account_password_algorithm_seed=SecretKey.generate(),
    )

    async with backend_factory(config) as backend, TaskGroup() as tg:
        for id in organizations:
            tg.create_task(wipe_organization(backend.organization, backend.blockstore, id))


async def wipe_organization(
    component: BaseOrganizationComponent, blockstore: BaseBlockStoreComponent, id: OrganizationID
):
    logger.debug("Wiping organization", organization_id=id.str)
    await blockstore.wipe_organization_data(id)
    match await component.wipe_organization(id):
        case WipeOrganizationBadOutcome.ORGANIZATION_NOT_FOUND:
            logger.warn("Organization not found", organization_id=id.str)
        case None:
            logger.info("Organization wiped", organization_id=id.str)
