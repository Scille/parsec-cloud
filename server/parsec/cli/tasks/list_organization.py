# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager

import click
import pydantic

from parsec._parsec import EmailAddress, OrganizationID, ParsecAddr, SecretKey
from parsec.cli.options import asyncio_run, db_server_options, logging_config_options
from parsec.components.memory.datamodel import MemoryDatamodel
from parsec.components.memory.events import event_bus_factory
from parsec.components.memory.organization import MemoryOrganizationComponent
from parsec.components.organization import BaseOrganizationComponent, OrganizationDump
from parsec.components.postgresql.handler import asyncpg_pool_factory
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.config import (
    BackendConfig,
    BaseDatabaseConfig,
    LogLevel,
    MockedBlockStoreConfig,
    MockedEmailConfig,
    PostgreSQLDatabaseConfig,
)
from parsec.webhooks import MockedWebhooksComponent


@pydantic.dataclasses.dataclass()
class OrganizationInfo:
    id: OrganizationID
    is_bootstrapped: bool
    # bootstrapped_on: DateTime | None
    is_expired: bool
    # expired_on: DateTime | None
    # active_users_limit: ActiveUsersLimit
    user_profile_outsider_allowed: bool
    realm_minimum_archiving_period_before_deletion: int
    # tos: TermsOfService | None

    @classmethod
    def from_dump(cls, dump: OrganizationDump) -> OrganizationInfo:
        return OrganizationInfo(
            id=dump.organization_id,
            is_bootstrapped=dump.is_bootstrapped,
            is_expired=dump.is_expired,
            user_profile_outsider_allowed=dump.user_profile_outsider_allowed,
            realm_minimum_archiving_period_before_deletion=dump.realm_minimum_archiving_period_before_deletion,
        )


@click.command(name="list-organizations", short_help="List organization present on the server")
@db_server_options
@logging_config_options(default_log_level="INFO")
@asyncio_run
async def cmd(
    db: BaseDatabaseConfig,
    db_min_connections: int,
    db_max_connections: int,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
):
    backend_config = BackendConfig(
        debug=False,
        db_config=db,
        blockstore_config=MockedBlockStoreConfig(),
        email_config=MockedEmailConfig(sender=EmailAddress("tasks@parsec.local")),
        server_addr=ParsecAddr("tasks.parsec.local", None, True),
        administration_token="",
        fake_account_password_algorithm_seed=SecretKey.generate(),
    )
    async with organization_component_factory(backend_config) as component:
        orgs = await list_organizations(component)

    adapter = pydantic.TypeAdapter(
        dict[
            OrganizationID,
            OrganizationInfo,
        ]
    )
    click.echo_via_pager(adapter.dump_json(orgs, indent=4).decode())


@asynccontextmanager
async def organization_component_factory(
    config: BackendConfig,
) -> AsyncGenerator[BaseOrganizationComponent, None]:
    if config.db_config.is_mocked():
        data = MemoryDatamodel(
            {} if config.backend_mocked_data is None else config.backend_mocked_data
        )
        async with event_bus_factory() as event_bus:
            yield MemoryOrganizationComponent(data, event_bus, MockedWebhooksComponent(), config)

    else:
        assert isinstance(config.db_config, PostgreSQLDatabaseConfig)
        async with asyncpg_pool_factory(
            url=config.db_config.url,
            min_connections=config.db_config.min_connections,
            max_connections=config.db_config.max_connections,
        ) as pool:
            yield PGOrganizationComponent(
                pool=pool,
                webhooks=MockedWebhooksComponent(),
                config=None,  # pyright: ignore [reportArgumentType]
            )


async def list_organizations(
    component: BaseOrganizationComponent,
) -> dict[OrganizationID, OrganizationInfo]:
    return {
        id: OrganizationInfo.from_dump(dump)
        for id, dump in (await component.list_organizations()).items()
    }
