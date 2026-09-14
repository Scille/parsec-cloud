from collections.abc import Callable, Coroutine
from datetime import datetime, timedelta
from typing import Any

import click
from structlog import get_logger

from parsec.cli.options import (
    Duration,
    asyncio_run,
    blockstore_server_options,
    db_server_options,
    debug_config_options,
    logging_config_options,
    sentry_config_options,
)
from parsec.config import BaseBlockStoreConfig, BaseDatabaseConfig, LogLevel

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
    old_date = datetime.utcnow() - remove_older_than
    logger.info("Will remove old organizations", old_date=old_date, interval=remove_older_than)
    pass
    await configure_sentry()

    pass
