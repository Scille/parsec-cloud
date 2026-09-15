# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from asyncio import TaskGroup
from collections.abc import Callable, Coroutine
from datetime import UTC, datetime, timedelta
from typing import Any

import click
import pydantic
from structlog import get_logger

from parsec._parsec import DateTime, EmailAddress, OrganizationID, ParsecAddr, SecretKey
from parsec.cli.options import (
    Duration,
    asyncio_run,
    blockstore_server_options,
    db_server_options,
    debug_config_options,
    logging_config_options,
    sentry_config_options,
)
from parsec.cli.tasks.list_organization import organization_component_factory
from parsec.components.organization import BaseOrganizationComponent
from parsec.config import (
    BackendConfig,
    BaseBlockStoreConfig,
    BaseDatabaseConfig,
    LogLevel,
    MockedEmailConfig,
)

logger = get_logger()


@click.command(
    name="delete-organization", short_help="Delete organization older that a provided date"
)
@click.option("--force", help="Force deletion of old organizations", is_flag=True)
@click.option(
    "--remove-older-than", type=Duration(), help="Remove organization older than provided duration"
)
@db_server_options
@blockstore_server_options
# Add --log-level/--log-format/--log-file
@logging_config_options(default_log_level="INFO")
# Add --sentry-environment/--sentry-dsn
@sentry_config_options
@debug_config_options
@asyncio_run
async def cmd(
    force: bool,
    remove_older_than: timedelta,
    db: BaseDatabaseConfig,
    db_min_connections: int,
    db_max_connections: int,
    # skip_database_migrations_check: bool,
    # maximum_database_connection_attempts: int,
    # pause_before_retry_database_connection: float,
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
    now = datetime.now(tz=UTC)
    old_date = now - remove_older_than
    logger.info("Will remove old organizations", old_date=old_date, interval=remove_older_than)
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

    async with organization_component_factory(config) as component:
        deleted_orgs = await delete_old_organizations(
            DateTime.from_rfc3339(old_date.isoformat()), component
        )

    adapter = pydantic.TypeAdapter(list[OrganizationID])
    click.echo_via_pager(adapter.dump_json(deleted_orgs, indent=4).decode())


async def delete_old_organizations(
    old_than: DateTime, component: BaseOrganizationComponent
) -> list[OrganizationID]:
    to_remove = [
        org_id
        for org_id, org in (await component.list_organizations()).items()
        if org.created_on <= old_than
    ]
    async with TaskGroup() as tg:
        for id in to_remove:
            tg.create_task(component.delete_organization(id))
    return to_remove
