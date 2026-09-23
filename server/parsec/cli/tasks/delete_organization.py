# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from asyncio import TaskGroup
from collections.abc import Callable, Coroutine, Iterator
from typing import Any, Literal

import click
from typing_extensions import TypeIs

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
from parsec.components.organization import BaseOrganizationComponent, DeleteOrganizationBadOutcome
from parsec.config import (
    BackendConfig,
    BaseBlockStoreConfig,
    BaseDatabaseConfig,
    LogLevel,
    MockedEmailConfig,
)
from parsec.logging import get_logger

logger = get_logger()


@click.command(
    name="delete-organization", short_help="Delete organization older that a provided date"
)
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

    logger.info("Will remove organizations", count=len(organizations))
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

    try:
        async with backend_factory(config) as backend, TaskGroup() as tg:
            for id in organizations:
                tg.create_task(delete_organization(backend.organization, backend.blockstore, id))
    except* DeleteException as e:
        for exception in walk_exceptions(e):
            logger.error(
                "Failed to remove an organization", id=exception.id, outcome=exception.outcome
            )
        ctx.exit(1)


async def delete_organization(
    component: BaseOrganizationComponent, blockstore: BaseBlockStoreComponent, id: OrganizationID
):
    logger.debug("Deleting organization", organization_id=id.str)
    await blockstore.delete_whole_organization_data(id)
    match await component.delete_organization(id):
        case DeleteOrganizationBadOutcome.ORGANIZATION_NOT_FOUND:
            pass
        case None:
            pass
        case _ as bad_outcome:
            raise DeleteException(id, bad_outcome)


class DeleteException(Exception):
    def __init__(
        self,
        id: OrganizationID,
        outcome: Literal[DeleteOrganizationBadOutcome.FAIL_TO_REMOVE_DATA]
        | Literal[DeleteOrganizationBadOutcome.FAIL_TO_REMOVE_METADATA],
        *args: object,
    ) -> None:
        self.id = id
        self.outcome = outcome
        super().__init__(*args)


def walk_exceptions[E: BaseException](exec: E | BaseExceptionGroup[E]) -> Iterator[E]:
    if _is_group(exec):
        for sub_exec in exec.exceptions:
            yield from walk_exceptions(sub_exec)
    else:
        yield exec


def _is_group[E: BaseException](exec: E | BaseExceptionGroup[E]) -> TypeIs[BaseExceptionGroup[E]]:
    return isinstance(exec, BaseExceptionGroup)
