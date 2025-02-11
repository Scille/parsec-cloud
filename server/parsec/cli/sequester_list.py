# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
from typing import Any

import click

from parsec._parsec import (
    OrganizationID,
)
from parsec.cli.options import db_server_options, debug_config_options, logging_config_options
from parsec.cli.testbed import if_testbed_available
from parsec.cli.utils import cli_exception_handler, start_backend
from parsec.components.sequester import (
    BaseSequesterService,
    SequesterGetOrganizationServicesBadOutcome,
    WebhookSequesterService,
)
from parsec.config import BaseDatabaseConfig, DisabledBlockStoreConfig, LogLevel


class DevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            for key, value in (
                ("debug", True),
                ("db", "MOCKED"),
                ("with_testbed", "sequestered"),
                ("organization", "SequesteredOrgTemplate"),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(short_help="List sequester services in a given organization")
@click.option("--organization", type=OrganizationID, help="Organization ID", required=True)
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
            "Equivalent to `--debug --db=MOCKED --with-testbed=sequestered --organization SequesteredOrgTemplate`"
        ),
    )
)
def list_services(
    organization: OrganizationID,
    db: BaseDatabaseConfig,
    db_max_connections: int,
    db_min_connections: int,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    debug: bool,
    with_testbed: str | None = None,
    dev: bool = False,
) -> None:
    with cli_exception_handler(debug):
        asyncio.run(
            _list_services(
                db_config=db,
                debug=debug,
                with_testbed=with_testbed,
                organization_id=organization,
            )
        )


def _display_service(service: BaseSequesterService) -> None:
    display_service_id = click.style(service.service_id.hex, fg="yellow")
    display_service_label = click.style(service.service_label, fg="green")
    click.echo(f"Service {display_service_label} (id: {display_service_id})")
    click.echo(f"\tCreated on: {service.created_on}")
    click.echo(f"\tService type: {service.service_type}")
    if isinstance(service, WebhookSequesterService):
        click.echo(f"\tWebhook endpoint URL {service.webhook_url}")
    if service.is_revoked:
        display_revoked = click.style("Revoked", fg="red")
        click.echo(f"\t{display_revoked} on: {service.revoked_on}")


async def _list_services(
    db_config: BaseDatabaseConfig,
    debug: bool,
    with_testbed: str | None,
    organization_id: OrganizationID,
) -> None:
    # Can use a dummy blockstore config since we are not going to query it
    blockstore_config = DisabledBlockStoreConfig()

    async with start_backend(
        db_config=db_config,
        blockstore_config=blockstore_config,
        debug=debug,
        populate_with_template=with_testbed,
    ) as backend:
        # 1) Retrieve the organization and check it is compatible

        outcome = await backend.sequester.get_organization_services(organization_id=organization_id)
        match outcome:
            case list() as services:
                pass
            case SequesterGetOrganizationServicesBadOutcome.ORGANIZATION_NOT_FOUND:
                raise RuntimeError("Organization doesn't exist")
            case SequesterGetOrganizationServicesBadOutcome.SEQUESTER_DISABLED:
                raise RuntimeError("Organization is not sequestered")

        display_services_count = click.style(len(services), fg="green")
        click.echo(f"Found {display_services_count} sequester service(s)")

        # Display active services first, and order them by creation date

        active = (service for service in services if not service.is_revoked)
        for service in sorted(active, key=lambda s: s.created_on):
            print()
            _display_service(service)

        revoked = (service for service in services if service.is_revoked)
        for service in sorted(revoked, key=lambda s: s.created_on):
            print()
            _display_service(service)
