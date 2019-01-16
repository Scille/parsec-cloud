import trio
import click

from parsec.cli_utils import spinner, cli_exception_handler
from parsec.types import DeviceName
from parsec.core.invite_claim import generate_invitation_token, invite_and_create_device
from parsec.core.cli.utils import core_config_and_device_options
from parsec.core.backend_connection import backend_cmds_factory


async def _invite_device(config, device, new_device_name):
    token = generate_invitation_token()

    organization_addr_display = click.style(device.organization_addr, fg="yellow")
    token_display = click.style(token, fg="yellow")
    click.echo(f"Backend url: {organization_addr_display}")
    click.echo(f"Invitation token: {token_display}")

    async with backend_cmds_factory(
        device.organization_addr, device.device_id, device.signing_key
    ) as cmds:

        async with spinner("Waiting for invitation reply"):
            await invite_and_create_device(device, cmds, new_device_name, token)

        display_device = click.style(f"{device.device_name}@{new_device_name}", fg="yellow")
        click.echo(f"Device {display_device} is ready !")


@click.command()
@core_config_and_device_options
@click.argument("new_device_name", type=DeviceName, required=True)
def invite_device(config, device, new_device_name, **kwargs):
    with cli_exception_handler(config.debug):
        trio.run(_invite_device, config, device, new_device_name)
