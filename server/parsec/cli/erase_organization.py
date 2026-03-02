# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
from typing import Any

import click

from parsec._parsec import (
    OrganizationID,
)
from parsec.cli.options import (
    db_server_options,
    debug_config_options,
    logging_config_options,
)
from parsec.cli.testbed import if_testbed_available
from parsec.cli.utils import cli_exception_handler, spinner, start_backend
from parsec.components.organization import OrganizationEraseBadOutcome
from parsec.config import (
    BaseDatabaseConfig,
    DisabledBlockStoreConfig,
    LogLevel,
    MockedBlockStoreConfig,
)


class DevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            for key, value in (
                ("debug", True),
                ("db", "MOCKED"),
                ("with_testbed", "coolorg"),
                ("organization", "CoolorgOrgTemplate"),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(short_help="Erase an organization from the database")
@click.option("--organization", type=OrganizationID, help="Organization ID", required=True)
@click.option("--yes", is_flag=True, help="Don't ask for confirmation before proceeding")
@db_server_options
# Add --log-level/--log-format/--log-file
@logging_config_options(default_log_level="INFO")
# Add --debug & --version
@debug_config_options
@if_testbed_available(
    click.option("--with-testbed", help="Start by populating with a testbed template")
)
@if_testbed_available(
    click.option(
        "--dev",
        cls=DevOption,
        is_flag=True,
        is_eager=True,
        help=(
            "Equivalent to `--debug --db=MOCKED --with-testbed=coolorg --organization CoolorgOrgTemplate`"
        ),
    )
)
def erase_organization(
    organization: OrganizationID,
    db: BaseDatabaseConfig,
    db_max_connections: int,
    db_min_connections: int,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    yes: bool,
    debug: bool,
    with_testbed: str | None = None,
    dev: bool = False,
) -> None:
    with cli_exception_handler(debug):
        asyncio.run(
            _erase_organization(
                yes=yes,
                db_config=db,
                debug=debug,
                with_testbed=with_testbed,
                organization_id=organization,
            )
        )


async def _erase_organization(
    db_config: BaseDatabaseConfig,
    yes: bool,
    debug: bool,
    with_testbed: str | None,
    organization_id: OrganizationID,
) -> None:
    # Can use a dummy blockstore config since we are not going to query it
    if with_testbed is None:
        blockstore_config = DisabledBlockStoreConfig()
    else:
        # Testbed template might need to create some blocks
        blockstore_config = MockedBlockStoreConfig()

    display_org = click.style(organization_id.str, fg="yellow")
    click.echo(
        f"You are about to entirely erase the {display_org} organization from the database, this action cannot be undone."
    )

    display_bucket_path = click.style(f"{organization_id.str}/", fg="yellow")
    click.echo("Notes:")
    click.echo(
        "- No trace of the organization will remain, so it will be possible to re-create another organization with the same name."
    )
    click.echo(
        f"- The organization's blocks won't be erased from the blockstore, you should manually remove the {display_bucket_path} top level directory from it."
    )
    click.echo("")

    if not yes:
        confirmation = click.prompt("To confirm, type the name of the organization")
        if confirmation != organization_id.str:
            raise RuntimeError("Organization name does not match, aborting")

    async with start_backend(
        db_config=db_config,
        blockstore_config=blockstore_config,
        debug=debug,
        populate_with_template=with_testbed,
    ) as backend:
        async with spinner("Removing from database..."):
            outcome = await backend.organization.erase(id=organization_id)
            match outcome:
                case None:
                    pass
                case OrganizationEraseBadOutcome.ORGANIZATION_NOT_FOUND:
                    raise RuntimeError("Organization doesn't exist")
