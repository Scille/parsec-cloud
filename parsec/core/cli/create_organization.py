# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import click

from parsec.utils import trio_run
from parsec.types import BackendAddr, BackendOrganizationBootstrapAddr
from parsec.api.protocol import OrganizationID
from parsec.logging import configure_logging
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.core.backend_connection import backend_administration_cmds_factory


async def _create_organization(debug, name, backend_addr, administration_token):
    async with spinner("Creating organization in backend"):
        async with backend_administration_cmds_factory(backend_addr, administration_token) as cmds:
            bootstrap_token = await cmds.organization_create(name)

    organization_addr = BackendOrganizationBootstrapAddr.build(backend_addr, name, bootstrap_token)
    organization_addr_display = click.style(organization_addr, fg="yellow")
    click.echo(f"Bootstrap organization url: {organization_addr_display}")


@click.command(short_help="create new organization")
@click.argument("name", required=True, type=OrganizationID)
@click.option("--addr", "-B", required=True, type=BackendAddr)
@click.option("--administration-token", "-T", required=True)
def create_organization(name, addr, administration_token):
    debug = "DEBUG" in os.environ
    configure_logging(log_level="DEBUG" if debug else "WARNING")

    with cli_exception_handler(debug):
        trio_run(_create_organization, debug, name, addr, administration_token)
