import os
import trio
import click

from parsec.logging import configure_logging
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.core.backend_connection import backend_anonymous_cmds_factory


async def _create_organization(debug, name, backend_addr):
    async with spinner("Creating organization in backend"):
        async with backend_anonymous_cmds_factory(backend_addr) as cmds:
            bootstrap_token = await cmds.organization_create(name)

    organization_addr = f"{backend_addr}/{name}?bootstrap-token={bootstrap_token}"
    organization_addr_display = click.style(organization_addr, fg="yellow")
    click.echo(f"Bootstrap organization url: {organization_addr_display}")


@click.command(short_help="create new organization")
@click.argument("name", required=True)
@click.option("--addr", "-B", required=True)
def create_organization(name, addr):
    debug = "DEBUG" in os.environ
    configure_logging(log_level="DEBUG" if debug else "WARNING")

    with cli_exception_handler(debug):
        trio.run(_create_organization, debug, name, addr)
