import trio
import click

from parsec.cli_utils import spinner, cli_exception_handler
from parsec.types import UserID
from parsec.core.backend_connection import backend_cmds_factory
from parsec.core.invite_claim import generate_invitation_token, invite_and_create_user
from parsec.core.cli.utils import core_config_and_device_options


async def _invite_user(config, device, invited_user_id):
    async with backend_cmds_factory(
        device.backend_addr, device.device_id, device.signing_key
    ) as cmds:

        token = generate_invitation_token()

        backend_addr_display = click.style(device.backend_addr, fg="yellow")
        token_display = click.style(token, fg="yellow")
        click.echo(f"Backend url: {backend_addr_display}")
        click.echo(f"Invitation token: {token_display}")

        async with spinner("Waiting for invitation reply"):
            invite_device_id = await invite_and_create_user(device, cmds, invited_user_id, token)

        display_device = click.style(invite_device_id, fg="yellow")
        click.echo(f"Device {display_device} has been created")


@click.command()
@core_config_and_device_options
@click.argument("invited_user_id", type=UserID, required=True)
def invite_user(config, device, invited_user_id, **kwargs):
    with cli_exception_handler(config.debug):
        trio.run(_invite_user, config, device, invited_user_id)
