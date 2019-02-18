# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import click

from parsec.types import OrganizationID, BackendAddr, BackendOrganizationBootstrapAddr
from parsec.logging import configure_logging
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.core.backend_connection import backend_administrator_cmds_factory


async def _create_organization(debug, name, backend_addr, administrator_token):
    async with spinner("Creating organization in backend"):
        async with backend_administrator_cmds_factory(backend_addr, administrator_token) as cmds:
            bootstrap_token = await cmds.organization_create(name)

    organization_addr = BackendOrganizationBootstrapAddr.build(backend_addr, name, bootstrap_token)
    organization_addr_display = click.style(organization_addr, fg="yellow")
    click.echo(f"Bootstrap organization url: {organization_addr_display}")


@click.command(short_help="create new organization")
@click.argument("name", required=True, type=OrganizationID)
@click.option("--addr", "-B", required=True, type=BackendAddr)
@click.option("--administrator-token", "-T", required=True)
def create_organization(name, addr, administrator_token):
    debug = "DEBUG" in os.environ
    configure_logging(log_level="DEBUG" if debug else "WARNING")

    with cli_exception_handler(debug):
        trio.run(_create_organization, debug, name, addr, administrator_token)
