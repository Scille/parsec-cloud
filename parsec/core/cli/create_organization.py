import os
import trio
import click

from parsec.logging import configure_logging
from parsec.core.backend_connection import backend_anonymous_cmds_factory
from parsec.spinner import spinner


async def _create_organization(debug, name, backend_addr):
    try:
        async with spinner("Creating organization in backend"):
            async with backend_anonymous_cmds_factory(backend_addr) as cmds:
                bootstrap_token = await cmds.organization_create(name)

    except Exception as exc:
        click.echo(click.style("Error: ", fg="red") + str(exc))
        if debug:
            raise
        else:
            raise SystemExit(1)

    organization_addr = f"{backend_addr}/{name}?bootstrap-token={bootstrap_token}"
    organization_addr_display = click.style(organization_addr, fg="yellow")
    click.echo(f"Bootstrap organization url: {organization_addr_display}")


@click.command()
@click.argument("name", required=True)
@click.option("--addr", "-B", required=True)
def create_organization(name, addr):
    debug = "DEBUG" in os.environ
    configure_logging(log_level="DEBUG" if debug else "WARNING")
    trio.run(_create_organization, debug, name, addr)
